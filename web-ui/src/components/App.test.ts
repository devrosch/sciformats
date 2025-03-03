/* eslint-disable no-duplicate-imports */
import 'components/menu/NavbarMatchMediaMock'; // mock window.matchMedia()
import './App'; // for side effects
import App from './App';
import Message from 'model/Message';
import ErrorParser from 'model/ErrorParser';
import MockParser from 'model/__mocks__/MockParser';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import AboutDialog from 'components/dialogs/AboutDialog';
import Splash from 'components/dialogs/Splash';
import Tree from 'components/tree/Tree';
import LocalParserRepository from 'model/LocalParserRepository';

const element = 'sf-app';
const fileOpenRequestEvent = 'sf-file-open-requested';
const fileOpenedEvent = 'sf-file-opened';
const fileCloseRequestEvent = 'sf-file-close-requested';
const fileClosedEvent = 'sf-file-closed';
const fileCloseAllRequestEvent = 'sf-file-close-all-requested';
const fileExportRequestEvent = 'sf-file-export-requested';
const fileExportedEvent = 'sf-file-exported';
const showAboutRequestEvent = 'sf-show-about-requested';
const errorEvent = 'sf-error';
const warningEvent = 'sf-warning';
const fileContent = 'abc';
const blob = new Blob([fileContent]);
const fileName = 'dummy.txt';
const errorFileName = 'ErrorFile.txt';
const errorMessage = 'Error message.';

jest.mock('util/WorkerUtils', () => ({
  initWorkerCpp: jest.fn(),
  initWorkerRs: jest.fn(),
}));
const mockAddRootNode = jest.fn(() => {
  /* noop */
});
const mockRemoveSelectedNode = jest.fn(
  () => new URL('file:///aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx#'),
);
const mockRemoveAllNodes = jest.fn(() => [
  new URL('file:///aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx#'),
  new URL('file:///some.other.url'),
]);
const mockGetSelectedNodeParser = jest.fn(() => ({
  rootUrl: new URL('file:///aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx#'),
  open: jest.fn(),
  read: jest.fn(),
  export: jest.fn(),
  close: jest.fn(),
}));
const mockGetSelectedNodeExportErrorParser = jest.fn(() => ({
  rootUrl: new URL('file:///aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx#'),
  open: jest.fn(),
  read: jest.fn(),
  export: jest.fn(() => {
    throw new Error('Export error.');
  }),
  close: jest.fn(),
}));
const mockErrorParser = new ErrorParser(
  new URL(`file:///${errorFileName}`),
  errorMessage,
);
jest.mock('model/LocalParserRepository');
jest.mock('util/FileUtils');

// TODO: put elsewhere (duplicate in AboutDialog)
const showModal = jest.fn();
const close = jest.fn();

beforeAll(() => {
  // see: https://github.com/jsdom/jsdom/issues/3294
  HTMLDialogElement.prototype.showModal = showModal;
  HTMLDialogElement.prototype.close = close;
});

beforeEach(() => {
  // see: https://github.com/jsdom/jsdom/issues/3294
  showModal.mockClear();
  close.mockClear();
  // required?
  mockAddRootNode.mockClear();
  mockRemoveSelectedNode.mockClear();
  mockRemoveAllNodes.mockClear();
  mockGetSelectedNodeParser.mockClear();

  // default mock, set up in beforeEach() to be overridable per test without side effects
  // @ts-ignore: mockImplementation() not declared in type but mock
  LocalParserRepository.mockImplementation(() => ({
    findParser: async (file: File) =>
      file.name === errorFileName ? mockErrorParser : new MockParser(file),
  }));
});

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
  jest.restoreAllMocks();
});

const waitForInit = async (): Promise<App> => {
  document.body.innerHTML = `<${element}/>`;
  const app = document.body.querySelector(element) as App;
  const splash = app.querySelector('sf-splash') as Splash;
  // wait for initialization to complete
  while (splash.hasAttribute('open')) {
    /* eslint-disable-next-line no-await-in-loop */
    await new Promise((r) => setTimeout(r, 1));
  }

  const tree = app.querySelector('sf-tree') as Tree;
  tree.addRootNode = mockAddRootNode;
  tree.removeSelectedNode = mockRemoveSelectedNode;
  tree.removeAllNodes = mockRemoveAllNodes;
  tree.getSelectedNodeParser = mockGetSelectedNodeParser;

  return app;
};

const prepareListener = (
  expectedEventType: string,
  expectedNumCalls = 1,
): [any, Promise<unknown>] => {
  const channel = CustomEventsMessageBus.getDefaultChannel();
  // workaround for using "done" in async method
  // see: https://github.com/facebook/jest/issues/11404
  let done: (value: unknown) => void = () => {
    /* noop */
  };
  const callbackResolved = new Promise((resolve) => {
    done = resolve;
  });
  let numCalls = 0;
  const listener = (message: Message) => {
    numCalls += 1;
    // the test assertions
    expect(message.name).toBe(expectedEventType);
    if (numCalls == expectedNumCalls) {
      done(null);
    }
  };
  const handle = channel.addListener(expectedEventType, listener);

  return [handle, callbackResolved];
};

test('sf-app renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  const app = document.body.querySelector('sf-app') as App;
  expect(app).toBeTruthy();

  expect(app.children).toHaveLength(6);
  expect(app.children.item(0)?.nodeName).toBe('SF-SPLASH');
  expect(app.children.item(1)?.nodeName).toBe('SF-DIALOG');
  expect(app.children.item(2)?.nodeName).toBe('SF-ABOUT-DIALOG');
  expect(app.children.item(3)?.nodeName).toBe('DIV');
  expect(app.children.item(4)?.nodeName).toBe('DIV');
  expect(app.children.item(5)?.nodeName).toBe('DIV');
});

test('sf-app supresses dragging UI elements', async () => {
  document.body.innerHTML = `<${element}/>`;
  const app = document.body.querySelector('sf-app') as App;

  // inspired by: https://www.freecodecamp.org/news/how-to-write-better-tests-for-drag-and-drop-operations-in-the-browser-f9a131f0b281/
  const createEvent = (type: string, properties = {}) => {
    const event = new Event(type, { bubbles: true });
    Object.assign(event, properties);
    return event;
  };

  const preventDefaultMock = jest.fn();
  const dragStartEvent = createEvent('dragstart', {
    preventDefault: preventDefaultMock,
  });

  app.dispatchEvent(dragStartEvent);

  expect(preventDefaultMock).toHaveBeenCalled();
});

test('sf-app listenes to file open events', async () => {
  await waitForInit();
  const [handle, callbackResolved] = prepareListener(fileOpenedEvent);

  const channel = CustomEventsMessageBus.getDefaultChannel();
  const file = new File([blob], fileName);
  channel.dispatch(fileOpenRequestEvent, { files: [file] });

  await callbackResolved;
  expect(mockAddRootNode).toHaveBeenCalledTimes(1);

  channel.removeListener(handle);
});

test('sf-app dispatches warning event when finding a parser fails', async () => {
  // override default mock
  // @ts-ignore: mockImplementation() not declared in type but mock, override default mock
  LocalParserRepository.mockImplementation(() => ({
    findParser: async () => {
      throw new Error('findParser() error');
    },
  }));

  await waitForInit();
  const [handle, callbackResolved] = prepareListener(warningEvent);

  const channel = CustomEventsMessageBus.getDefaultChannel();
  const file = new File([blob], errorFileName);
  channel.dispatch(fileOpenRequestEvent, { files: [file] });

  await callbackResolved;
  expect(mockAddRootNode).toHaveBeenCalledTimes(0);

  channel.removeListener(handle);
});

test('sf-app shows error and dispatches error event when file open fails', async () => {
  await waitForInit();
  const [handle, callbackResolved] = prepareListener(errorEvent);

  const channel = CustomEventsMessageBus.getDefaultChannel();
  const file = new File([blob], errorFileName);
  channel.dispatch(fileOpenRequestEvent, { files: [file] });

  await callbackResolved;
  expect(mockAddRootNode).toHaveBeenCalledTimes(1);
  expect(mockAddRootNode).toHaveBeenCalledWith(mockErrorParser);

  channel.removeListener(handle);
});

test('sf-app listenes to file close events', async () => {
  await waitForInit();
  const [handle, callbackResolved] = prepareListener(fileClosedEvent);

  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.dispatch(fileCloseRequestEvent, null);

  await callbackResolved;
  expect(mockRemoveSelectedNode).toHaveBeenCalledTimes(1);

  channel.removeListener(handle);
});

test('sf-app listenes to file close all events', async () => {
  await waitForInit();
  const [handle, callbackResolved] = prepareListener(fileClosedEvent, 2);

  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.dispatch(fileCloseAllRequestEvent, null);

  await callbackResolved;
  expect(mockRemoveAllNodes).toHaveBeenCalledTimes(1);

  channel.removeListener(handle);
});

test('sf-app listenes to file export events', async () => {
  await waitForInit();
  const [handle, callbackResolved] = prepareListener(fileExportedEvent);

  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.dispatch(fileExportRequestEvent, null);

  await callbackResolved;

  channel.removeListener(handle);
});

test('sf-app shows error and dispatches error event when file export fails', async () => {
  await waitForInit();
  const [handle, callbackResolved] = prepareListener(errorEvent);
  const tree = document.querySelector('sf-tree') as Tree;
  tree.getSelectedNodeParser = mockGetSelectedNodeExportErrorParser;
  showModal.mockClear();

  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.dispatch(fileExportRequestEvent, null);

  await callbackResolved;
  expect(showModal).toHaveBeenCalledTimes(1);

  channel.removeListener(handle);
});

test('sf-app file export shows error when no node is selected', async () => {
  await waitForInit();
  const tree = document.querySelector('sf-tree') as Tree;
  tree.getSelectedNodeParser = jest.fn(() => null);
  showModal.mockClear();

  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.dispatch(fileExportRequestEvent, null);

  expect(showModal).toHaveBeenCalledTimes(1);
});

test('sf-app listenes to show about events', async () => {
  const spy = jest.spyOn(App.prototype, 'handleShowAboutDialog');
  await waitForInit();
  const aboutDialog = document.body.querySelector(
    'sf-about-dialog',
  ) as AboutDialog;
  expect(aboutDialog).toBeTruthy();
  const showModalMock = jest.fn((show) => show);
  aboutDialog.showModal = showModalMock;

  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.dispatch(showAboutRequestEvent, null);

  expect(spy).toHaveBeenCalledTimes(1);
  expect(showModalMock).toHaveBeenCalledTimes(1);

  spy.mockClear();
});
