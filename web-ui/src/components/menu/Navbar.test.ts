/* eslint-disable no-duplicate-imports */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Message from 'model/Message';
import Menu from './Menu';
import './NavbarMatchMediaMock'; // mock window.matchMedia()
import './Navbar'; // for side effects
import Navbar from './Navbar';
import AboutDialog from './AboutDialog';

// make sure modifier keys for Linux are expected
jest.mock('util/SysInfoProvider', () => ({
  detectOS: () => 'Linux/Unix',
}));

const appElement = 'sf-app';
const element = 'sf-navbar';

const testEventDispatchedForClickedKey = (
  key: string,
  expectedEventName: string,
  done: (error?: any) => any,
) => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector(element) as Navbar;
  expect(navbar).toBeTruthy();

  const mockElement = document.createElement('a');
  mockElement.setAttribute('key', key);
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  const listener = (message: Message) => {
    try {
      expect(message.name).toBe(expectedEventName);
      done();
    } catch (error) {
      done(error);
    }
  };

  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.addListener(expectedEventName, listener);

  navbar.onClick(mouseEvent);
};

const testEventDispatchedForShortcut = (
  key: string,
  expectedEventName: string,
  done: (error?: any) => any,
) => {
  document.body.innerHTML = `<${element}></${element}>`;
  const navbar = document.body.querySelector(element) as Navbar;
  navbar.activateShortcuts();
  expect(navbar).toBeTruthy();

  const event = new KeyboardEvent(
    'keydown',
    // modifier keys for Linux, guaranteed to be used by mock at beginning of file
    {
      key,
      shiftKey: true,
      ctrlKey: false,
      altKey: true,
      metaKey: false,
    },
  );
  const listener = (message: Message) => {
    try {
      expect(message.name).toBe(expectedEventName);
      done();
    } catch (error) {
      done(error);
    }
  };
  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.addListener(expectedEventName, listener);

  document.dispatchEvent(event);
};

const file = new File(['dummy'], 'test.txt');
const file2 = new File(['dummy2'], 'test2.txt');
const item = {
  webkitGetAsEntry() {
    return { isFile: true };
  },
};
let dataTransfer: DataTransfer;

beforeAll(() => {
  // see: https://github.com/jsdom/jsdom/issues/3294
  HTMLDialogElement.prototype.showModal = jest.fn();
  HTMLDialogElement.prototype.close = jest.fn();
});

beforeEach(() => {
  dataTransfer = {
    files: [file, file2],
    items: [item, item],
    dropEffect: 'none',
  } as unknown as DataTransfer;
});

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-navbar renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();

  expect(navbar.children).toHaveLength(4);
  expect(navbar.children.item(0)?.nodeName).toBe('IMG');
  expect(navbar.children.item(1)?.nodeName).toBe('A');
  expect(navbar.children.item(2)?.nodeName).toBe('NAV');
  expect(navbar.children.item(3)?.nodeName).toBe('SF-ABOUT-DIALOG');
});

test('sf-navbar hamburger menu toggles menu visibility', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();
  const menu = navbar.querySelector('sf-menu') as Menu;
  expect(menu).toBeTruthy();

  const mockShowMenu = jest.fn((show) => show);
  menu.showMenu = mockShowMenu;

  const mockElement = document.createElement('a');
  mockElement.setAttribute('key', 'sf-navbar-hamburger');
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  navbar.onClick(mouseEvent);
  expect(mouseEvent.stopPropagation).toHaveBeenCalledTimes(1);
  expect(mouseEvent.preventDefault).toHaveBeenCalledTimes(1);
  expect(mockShowMenu).toHaveBeenCalledTimes(1);
  expect(mockShowMenu.mock.results[0].value).toBe(true);
  navbar.onClick(mouseEvent);
  expect(mouseEvent.stopPropagation).toHaveBeenCalledTimes(2);
  expect(mouseEvent.preventDefault).toHaveBeenCalledTimes(2);
  expect(mockShowMenu).toHaveBeenCalledTimes(2);
  expect(mockShowMenu.mock.results[1].value).toBe(false);
});

test('sf-navbar menu item click closes menu', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();
  const menu = navbar.querySelector('sf-menu') as Menu;
  expect(menu).toBeTruthy();

  const mockShowMenu = jest.fn((show) => show);
  menu.showMenu = mockShowMenu;

  const mockElement = document.createElement('a');
  mockElement.setAttribute('key', 'sf-menu-item-1');
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  navbar.onClick(mouseEvent);
  expect(mockShowMenu).toHaveBeenCalledTimes(1);
  expect(mockShowMenu.mock.results[0].value).toBe(false);
});

test('sf-navbar screen change closes menu', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();
  const menu = navbar.querySelector('sf-menu') as Menu;
  expect(menu).toBeTruthy();

  const mockShowMenu = jest.fn((show) => show);
  menu.showMenu = mockShowMenu;

  const e = {} as unknown as MediaQueryListEvent;
  navbar.handleScreenChange(e);

  expect(mockShowMenu).toHaveBeenCalledTimes(1);
  expect(mockShowMenu.mock.results[0].value).toBe(false);
});

test('sf-navbar selction outside menu closes menu', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();
  const menu = navbar.querySelector('sf-menu') as Menu;
  expect(menu).toBeTruthy();

  const mockShowMenu = jest.fn((show) => show);
  menu.showMenu = mockShowMenu;

  const mockElement = document.createElement('a');
  mockElement.setAttribute('key', 'any');
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  navbar.handleOutsideSelection(mouseEvent);

  expect(mockShowMenu).toHaveBeenCalledTimes(1);
  expect(mockShowMenu.mock.results[0].value).toBe(false);
});

test('sf-navbar - about click opens AboutDialog', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();
  const aboutDialog = navbar.querySelector('sf-about-dialog') as AboutDialog;
  expect(aboutDialog).toBeTruthy();

  const showModalMock = jest.fn((show) => show);
  aboutDialog.showModal = showModalMock;

  const mockElement = document.createElement('a');
  mockElement.setAttribute('key', 'sf-about');
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  expect(showModalMock).toHaveBeenCalledTimes(0);
  navbar.onClick(mouseEvent);
  expect(showModalMock).toHaveBeenCalledTimes(1);
});

test('sf-navbar - file export event dispatched when "export - json" is clicked', (done) => {
  testEventDispatchedForClickedKey(
    'sf-export-json',
    'sf-file-export-requested',
    done,
  );
});

test('sf-navbar - close event dispatched when "file - close" is clicked', (done) => {
  testEventDispatchedForClickedKey(
    'sf-file-close',
    'sf-file-close-requested',
    done,
  );
});

test('sf-navbar - close event dispatched when "file - close all" is clicked', (done) => {
  testEventDispatchedForClickedKey(
    'sf-file-close-all',
    'sf-file-close-all-requested',
    done,
  );
});

test('sf-navbar dispatches custom event on file drop', () => {
  document.body.innerHTML = `
    <${appElement}>
      <${element}></${element}>
    </${appElement}>`;
  const app = document.body.querySelector(appElement) as HTMLElement;
  const navbar = document.body.querySelector(element) as Navbar;
  navbar.activateDragAndDrop(app);
  const channel = CustomEventsMessageBus.getDefaultChannel();

  // simulate DragEvent, not supported by jsdom
  const event = new Event('drop') as any;
  event.dataTransfer = dataTransfer;

  const customEventHandler = jest.fn((e) => e.detail.files);
  const handle = channel.addListener(
    'sf-file-open-requested',
    customEventHandler,
  );
  app.dispatchEvent(event as DragEvent);
  expect(customEventHandler).toHaveBeenCalledTimes(1);
  channel.removeListener(handle);

  const receivedFiles = customEventHandler.mock.results[0];
  expect(receivedFiles.value.length).toBe(2);
  const receivedFile = receivedFiles.value[0] as File;
  expect(receivedFile.name).toBe('test.txt');
});

test('sf-navbar prevents default and stops propagation on dragenter', () => {
  document.body.innerHTML = `
    <${appElement}>
      <${element}></${element}>
    </${appElement}>`;
  const app = document.body.querySelector(appElement) as HTMLElement;
  const navbar = document.body.querySelector(element) as Navbar;
  navbar.activateDragAndDrop(app);

  // simulate DragEvent, not supported by jsdom
  const event = new Event('dragenter') as any;
  event.preventDefault = jest.fn();
  event.stopPropagation = jest.fn();
  app.dispatchEvent(event as DragEvent);

  expect(event.preventDefault).toBeCalledTimes(1);
  expect(event.stopPropagation).toBeCalledTimes(1);
});

test('sf-navbar shows copy symbol on dragover', () => {
  document.body.innerHTML = `
    <${appElement}>
      <${element}></${element}>
    </${appElement}>`;
  const app = document.body.querySelector(appElement) as HTMLElement;
  const navbar = document.body.querySelector(element) as Navbar;
  navbar.activateDragAndDrop(app);

  // simulate DragEvent, not supported by jsdom
  const event = new Event('dragover') as any;
  event.dataTransfer = dataTransfer;
  event.preventDefault = jest.fn();
  event.stopPropagation = jest.fn();
  app.dispatchEvent(event as DragEvent);

  expect(event.preventDefault).toBeCalledTimes(1);
  expect(event.stopPropagation).toBeCalledTimes(1);
  expect(event.dataTransfer.dropEffect).toBe('copy');
});

test('sf-navbar handles json export shortcut', (done) => {
  testEventDispatchedForShortcut('j', 'sf-file-export-requested', done);
});

test('sf-navbar handles file close shortcut', (done) => {
  testEventDispatchedForShortcut('c', 'sf-file-close-requested', done);
});

test('sf-navbar handles file close all shortcut', (done) => {
  testEventDispatchedForShortcut('q', 'sf-file-close-all-requested', done);
});
