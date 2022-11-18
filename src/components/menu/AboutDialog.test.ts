/* eslint-disable import/no-duplicates */
import './AboutDialog'; // for side effects
import AboutDialog from './AboutDialog';

const element = 'sf-about-dialog';

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

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-about-dialog renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  const aboutDialog = document.body.querySelector(element) as AboutDialog;
  expect(aboutDialog).toBeTruthy();

  const dialog = aboutDialog.querySelector('dialog') as HTMLDialogElement;
  expect(dialog).toBeTruthy();
});

test('sf-about-dialog showModal() makes dialog open/close', async () => {
  document.body.innerHTML = `<${element}/>`;
  const aboutDialog = document.body.querySelector(element) as AboutDialog;
  expect(aboutDialog).toBeTruthy();
  const dialog = document.querySelector('dialog') as HTMLDialogElement;
  expect(dialog).toBeTruthy();

  expect(aboutDialog.hasAttribute('open')).toBeFalsy();
  expect(dialog.hasAttribute('open')).toBeFalsy();
  expect(showModal).toHaveBeenCalledTimes(0);
  expect(close).toHaveBeenCalledTimes(0);
  aboutDialog.showModal(true);
  dialog.setAttribute('open', ''); // mock showModel() does not set open attribute
  expect(aboutDialog.hasAttribute('open')).toBeTruthy();
  expect(showModal).toHaveBeenCalledTimes(1);
  expect(close).toHaveBeenCalledTimes(0);
  aboutDialog.showModal(false);
  expect(showModal).toHaveBeenCalledTimes(1);
  expect(close).toHaveBeenCalledTimes(1);
});

test('sf-about-dialog clicking anywhere closes dialog', async () => {
  document.body.innerHTML = `<${element}/>`;
  const aboutDialog = document.body.querySelector(element) as AboutDialog;
  expect(aboutDialog).toBeTruthy();
  const dialog = document.querySelector('dialog') as HTMLDialogElement;
  expect(dialog).toBeTruthy();

  const mouseEvent = {
    target: dialog,
    stopPropagation: jest.fn(),
  } as unknown as MouseEvent;

  aboutDialog.showModal(true);
  dialog.setAttribute('open', ''); // mock showModel() does not set open attribute

  expect(close).toHaveBeenCalledTimes(0);
  aboutDialog.onClick(mouseEvent);
  expect(close).toHaveBeenCalledTimes(1);
});
