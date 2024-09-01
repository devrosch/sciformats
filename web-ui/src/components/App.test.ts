/* eslint-disable no-duplicate-imports */
import 'components/menu/NavbarMatchMediaMock'; // mock window.matchMedia()
import './App'; // for side effects
import App from './App';

jest.mock('util/WorkerUtils', () => ({
  initWorkerCpp: jest.fn(),
  initWorkerRs: jest.fn(),
}));

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
});

const element = 'sf-app';

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
  jest.restoreAllMocks();
});

test('sf-app renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  const app = document.body.querySelector('sf-app') as App;
  expect(app).toBeTruthy();

  expect(app.children).toHaveLength(4);
  expect(app.children.item(0)?.nodeName).toBe('SF-SPLASH');
  expect(app.children.item(1)?.nodeName).toBe('DIV');
  expect(app.children.item(2)?.nodeName).toBe('DIV');
  expect(app.children.item(3)?.nodeName).toBe('DIV');
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
