/* eslint-disable import/no-duplicates */
import 'components/menu/NavbarMatchMediaMock'; // mock window.matchMedia()
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import './App'; // for side effects
import App from './App';

const element = 'sf-app';

const file = new File(['dummy'], 'test.txt');
const file2 = new File(['dummy2'], 'test2.txt');
const item = {
  webkitGetAsEntry() { return { isFile: true }; },
};
let dataTransfer: DataTransfer;

beforeAll(() => {
  window.crypto.randomUUID = jest.fn(() => 'aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee');
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

test('sf-app renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  const app = document.body.querySelector('sf-app') as App;
  expect(app).toBeTruthy();

  expect(app.children).toHaveLength(3);
  expect(app.children.item(0)?.nodeName).toBe('DIV');
  expect(app.children.item(1)?.nodeName).toBe('DIV');
  expect(app.children.item(2)?.nodeName).toBe('DIV');
});

test('sf-app dispatches custom event on file drop', () => {
  document.body.innerHTML = `<${element}/>`;
  const app = document.body.querySelector('sf-app') as App;
  const channel = CustomEventsMessageBus.getDefaultChannel();

  // simulate DragEvent, not supported by jsdom
  const event = new Event('drop') as any;
  event.dataTransfer = dataTransfer;

  const customEventHandler = jest.fn((e) => e.detail.files);
  const handle = channel.addListener('sf-file-open-requested', customEventHandler);
  app.dispatchEvent(event as DragEvent);
  expect(customEventHandler).toHaveBeenCalledTimes(1);
  channel.removeListener(handle);

  const receivedFiles = customEventHandler.mock.results[0];
  expect(receivedFiles.value.length).toBe(2);
  const receivedFile = receivedFiles.value[0] as File;
  expect(receivedFile.name).toBe('test.txt');
});

test('sf-app prevents default and stops propagation on dragenter', () => {
  document.body.innerHTML = `<${element}/>`;
  const app = document.body.querySelector('sf-app') as App;

  // simulate DragEvent, not supported by jsdom
  const event = new Event('dragenter') as DragEvent;
  event.preventDefault = jest.fn();
  event.stopPropagation = jest.fn();
  app.dispatchEvent(event);

  expect(event.preventDefault).toBeCalledTimes(1);
  expect(event.stopPropagation).toBeCalledTimes(1);
});

test('sf-app prevents shows copy symbol on dragover', () => {
  document.body.innerHTML = `<${element}/>`;
  const app = document.body.querySelector('sf-app') as App;

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
