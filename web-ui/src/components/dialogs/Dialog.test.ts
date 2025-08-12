/**
 * Copyright (c) 2025 Robert Schiwon
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/* eslint-disable no-duplicate-imports */
import './Dialog'; // for side effects
import Dialog from './Dialog';

const element = 'sf-dialog';

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

test('sf-dialog renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  const dialog = document.body.querySelector(element) as Dialog;
  expect(dialog).toBeTruthy();

  const htmlDialog = dialog.querySelector('dialog') as HTMLDialogElement;
  expect(htmlDialog).toBeTruthy();
});

test('sf-dialog showMessage() open dialog with message', async () => {
  document.body.innerHTML = `<${element}/>`;
  const dialog = document.body.querySelector(element) as Dialog;
  expect(dialog).toBeTruthy();
  const htmlDialog = document.querySelector('dialog') as HTMLDialogElement;
  expect(htmlDialog).toBeTruthy();

  expect(dialog.hasAttribute('open')).toBeFalsy();
  expect(htmlDialog.hasAttribute('open')).toBeFalsy();
  expect(showModal).toHaveBeenCalledTimes(0);
  expect(close).toHaveBeenCalledTimes(0);
  dialog.showMessage('Test message.');
  // dialog.showModal(true);
  htmlDialog.setAttribute('open', ''); // mock showModel() does not set open attribute
  expect(dialog.hasAttribute('open')).toBeTruthy();
  expect(showModal).toHaveBeenCalledTimes(1);
  expect(close).toHaveBeenCalledTimes(0);
  expect(htmlDialog.textContent?.includes('Test message.')).toBeTruthy();
  dialog.showModal(false);
  expect(showModal).toHaveBeenCalledTimes(1);
  expect(close).toHaveBeenCalledTimes(1);
});

test('sf-dialog clicking anywhere closes dialog', async () => {
  document.body.innerHTML = `<${element}/>`;
  const dialog = document.body.querySelector(element) as Dialog;
  expect(dialog).toBeTruthy();
  const htmlDialog = document.querySelector('dialog') as HTMLDialogElement;
  expect(htmlDialog).toBeTruthy();

  const mouseEvent = {
    target: htmlDialog,
    stopPropagation: jest.fn(),
  } as unknown as MouseEvent;

  dialog.showModal(true);
  htmlDialog.setAttribute('open', ''); // mock showModel() does not set open attribute

  expect(close).toHaveBeenCalledTimes(0);
  dialog.handleOutsideSelection(mouseEvent);
  expect(close).toHaveBeenCalledTimes(1);
});
