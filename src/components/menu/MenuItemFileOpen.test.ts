/* eslint-disable import/no-duplicates */
import 'components/menu/MenuItemFileOpen'; // for side effects
import MenuItemFileOpen from 'components/menu/MenuItemFileOpen';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';

const element = 'sf-menu-item-file-open';
const keyAttr = 'key';
const key = 'abc';
const key2 = 'abc2';
const titleAttr = 'title';
const title = 'def';
const title2 = 'def2';
const inputKeyPostfix = '-input';
const aKeyPostfix = '-input-a';

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-menu-item-file-open renders and observes attribute changes', async () => {
  document.body.innerHTML = `<li is="${element}" ${keyAttr}="${key}" ${titleAttr}="${title}"/>`;
  const menuItem = document.body.querySelector('li') as MenuItemFileOpen;
  expect(menuItem).toBeTruthy();
  expect(menuItem.getAttribute(titleAttr)).toBe(title);
  expect(menuItem.getAttribute(keyAttr)).toBe(key);

  const input = menuItem.querySelector('input') as HTMLInputElement;
  expect(input).toBeTruthy();
  expect(input.getAttribute(keyAttr)).toBe(key + inputKeyPostfix);
  const a = menuItem.querySelector('label > a') as HTMLAnchorElement;
  expect(a).toBeTruthy();
  expect(a.getAttribute(keyAttr)).toBe(key + aKeyPostfix);
  expect(a.textContent).toBe(title);

  menuItem.setAttribute(keyAttr, key2);
  expect(menuItem.getAttribute(keyAttr)).toBe(key2);
  expect(input.getAttribute(keyAttr)).toBe(key2 + inputKeyPostfix);
  expect(a.getAttribute(keyAttr)).toBe(key2 + aKeyPostfix);

  menuItem.setAttribute(titleAttr, title2);
  expect(menuItem.getAttribute(titleAttr)).toBe(title2);
  expect(a.textContent).toBe(title2);
});

test('sf-menu-item-file-open a click event results in input click event', async () => {
  document.body.innerHTML = `<li is="${element}" ${keyAttr}="${key}" ${titleAttr}="${title}"/>`;
  const menuItem = document.body.querySelector('li') as MenuItemFileOpen;
  const input = document.body.querySelector('input') as HTMLInputElement;
  expect(menuItem).toBeTruthy();
  expect(input).toBeTruthy();

  const mockElement = document.createElement('a');
  mockElement.setAttribute('key', `${key}-input-a`);
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  input.onclick = jest.fn();

  menuItem.onClick(mouseEvent);
  expect(mouseEvent.stopPropagation).toHaveBeenCalledTimes(1);
  expect(mouseEvent.preventDefault).toHaveBeenCalledTimes(1);
  expect(input.onclick).toHaveBeenCalledTimes(1);
});

test('sf-menu-item-file-open stops propagation of input click events', async () => {
  document.body.innerHTML = `<li is="${element}" ${keyAttr}="${key}" ${titleAttr}="${title}"/>`;
  const menuItem = document.body.querySelector('li') as MenuItemFileOpen;
  expect(menuItem).toBeTruthy();

  const mockElement = document.createElement('input');
  mockElement.setAttribute('key', `${key}-input`);
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
  } as unknown as MouseEvent;

  menuItem.onClick(mouseEvent);
  expect(mouseEvent.stopPropagation).toHaveBeenCalledTimes(1);
});

test('sf-menu-item-file-open dispatches click and custom event on file upload', (done) => {
  document.body.innerHTML = `<li is="${element}" ${keyAttr}="${key}" ${titleAttr}="${title}"/>`;
  const menuItem = document.body.querySelector('li') as MenuItemFileOpen;
  const channel = CustomEventsMessageBus.getDefaultChannel();

  const file = new File(['dummy'], 'test.txt');
  const file2 = new File(['dummy2'], 'test2.txt');
  const event = {
    target: {
      files: [file, file2],
    },
  } as unknown as Event;

  const customEventHandler = jest.fn((e) => e.detail.files);
  const clickEventHandler = jest.fn();
  const handle = channel.addListener('sf-files-open-requested', customEventHandler);
  document.addEventListener('click', clickEventHandler);
  menuItem.onChange(event);
  channel.removeListener(handle);
  document.removeEventListener('click', clickEventHandler);

  expect(customEventHandler).toHaveBeenCalledTimes(1);
  expect(clickEventHandler).toHaveBeenCalledTimes(1);

  const receivedFiles = customEventHandler.mock.results[0];
  expect(receivedFiles.value.length).toBe(2);
  const receivedFile = receivedFiles.value[0] as File;
  expect(receivedFile.name).toBe('test.txt');

  // receivedFile.text() does not work in Jest
  // https://github.com/jsdom/jsdom/issues/2555
  const reader = new FileReader();
  reader.onload = () => {
    const text = reader.result;
    try {
      expect(text).toBe('dummy');
      done();
    } catch (error) {
      done(error);
    }
  };
  reader.readAsText(receivedFile);
});
