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
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import './Footer'; // for side effects
import Footer from './Footer';

const element = 'sf-footer';
const data: { x: number; y: number }[] = [];
const urlChild2 = new URL('file:///test/path/root.txt#/child 2');

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-footer renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  const footer = document.body.querySelector(element);
  expect(footer).toBeTruthy();
  const span = footer?.querySelector('span');
  expect(span).toBeTruthy();
  expect(span?.textContent).toBe('');
});

test('sf-footer reacts to sf-tree-node-(de)selected events', async () => {
  const footer = new Footer();
  document.body.append(footer);
  const channel = CustomEventsMessageBus.getDefaultChannel();

  const span = footer.querySelector('span');
  expect(span).toBeTruthy();
  expect(span?.textContent).toBe('');

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data,
    parameters: null,
  });
  expect(span?.textContent).toBe(urlChild2.toString());

  channel.dispatch('sf-tree-node-deselected', { url: urlChild2 });
  expect(span?.textContent).toBe('');
});
