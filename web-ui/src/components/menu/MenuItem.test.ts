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
import 'components/menu/MenuItem'; // for side effects
import MenuItem from 'components/menu/MenuItem';

const element = 'sf-menu-item';
const keyAttr = 'key';
const key = 'abc';
const key2 = 'abc2';
const titleAttr = 'title';
const title = 'def';
const title2 = 'def2';
const shortcutAttr = 'shortcut';
const shortcut = 'Ctrl-A';
const roleAttr = 'role';
const role = 'none';
const aRole = 'menuitem';

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-menu-item renders and observes attribute changes', async () => {
  document.body.innerHTML = `
    <${element} ${keyAttr}="${key}" ${titleAttr}="${title}" ${shortcutAttr}="${shortcut}"/>`;
  const menuItem = document.body.querySelector(element) as MenuItem;
  expect(menuItem).toBeTruthy();
  expect(menuItem.getAttribute(titleAttr)).toBe(title);
  expect(menuItem.getAttribute(keyAttr)).toBe(key);
  expect(menuItem.getAttribute(roleAttr)).toBe(role);

  const a = menuItem.querySelector('a') as HTMLAnchorElement;
  expect(a).toBeTruthy();
  expect(a.getAttribute(roleAttr)).toBe(aRole);

  const nameSpan = a.children.item(0) as HTMLSpanElement;
  expect(nameSpan).toBeTruthy();
  // a.innerText not available in JSDOM
  // see: https://github.com/jsdom/jsdom/issues/1245
  expect(nameSpan.textContent).toBe(title);

  const shortcutSpan = a.children.item(1) as HTMLSpanElement;
  expect(shortcutSpan).toBeTruthy();
  expect(shortcutSpan.textContent).toBe(shortcut);

  menuItem.setAttribute(keyAttr, key2);
  expect(menuItem.getAttribute(keyAttr)).toBe(key2);

  menuItem.setAttribute(titleAttr, title2);
  expect(menuItem.getAttribute(titleAttr)).toBe(title2);
  expect(nameSpan.textContent).toBe(title2);
});

test('sf-menu-item generates click events', async () => {
  document.body.innerHTML = `<${element} ${keyAttr}="${key}" ${titleAttr}="${title}"/>`;
  const menuItem = document.body.querySelector(element) as MenuItem;
  expect(menuItem).toBeTruthy();
  const a = menuItem.querySelector('a') as HTMLAnchorElement;
  expect(a).toBeTruthy();

  const clickHandler = jest.fn((e) => e.target.getAttribute(keyAttr));
  document.body.addEventListener('click', clickHandler);
  a.click();
  document.body.removeEventListener('click', clickHandler);
  expect(clickHandler).toHaveBeenCalledTimes(1);
  expect(clickHandler.mock.results[0].value).toBe(key);
});
