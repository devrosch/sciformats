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
import './Parameter'; // for side effects
import Parameter from './Parameter';

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-parameter renders', async () => {
  const element = 'sf-parameter';
  const keyAttr = 'key';
  const valueAttr = 'value';
  const key = 'abc';
  const value = 'def';

  document.body.innerHTML = `<${element}/>`;
  expect(document.body.innerHTML).not.toContain(key);
  expect(document.body.innerHTML).not.toContain(value);
  expect(document.body.textContent).not.toContain(`${key}: ${value}`);

  const parameter = document.body.querySelector(element) as Parameter;
  parameter.setAttribute(keyAttr, key);
  parameter.setAttribute(valueAttr, value);
  expect(document.body.innerHTML).toContain(key);
  expect(document.body.innerHTML).toContain(value);
  expect(document.body.textContent).toContain(`${key}: ${value}`);
});
