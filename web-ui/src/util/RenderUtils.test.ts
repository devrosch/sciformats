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

import { setElementAttribute, setElementTextContent } from 'util/RenderUtils';

const elementName = 'div';
const attributeName = 'test-attr';
const attributeInitialValue = 'initial-value';
const attributeValue = 'test-value';

afterEach(() => {
  document.body.innerHTML = '';
});

test('setElementAttribute() creates attribute if it does not yet exists', async () => {
  document.body.innerHTML = `<${elementName}></${elementName}>`;
  const element = document.body.querySelector(elementName) as HTMLElement;
  expect(element).toBeTruthy();
  expect(element.getAttribute(attributeName)).toBeNull();

  setElementAttribute(element, attributeName, attributeValue);
  expect(element.getAttribute(attributeName)).toBe(attributeValue);
});

test('setElementAttribute() updates existing attribute', async () => {
  document.body.innerHTML = `<${elementName} ${attributeName}="${attributeInitialValue}"></${elementName}>`;
  const element = document.body.querySelector(elementName) as HTMLElement;
  expect(element).toBeTruthy();
  expect(element.getAttribute(attributeName)).toBe(attributeInitialValue);

  setElementAttribute(element, attributeName, attributeValue);
  expect(element.getAttribute(attributeName)).toBe(attributeValue);
});

test('setElementAttribute() does not update existing attribute if it already has value to be set', async () => {
  document.body.innerHTML = `<${elementName} ${attributeName}="${attributeInitialValue}"></${elementName}>`;
  const element = document.body.querySelector(elementName) as HTMLElement;
  expect(element).toBeTruthy();
  expect(element.getAttribute(attributeName)).toBe(attributeInitialValue);

  const spy = jest.spyOn(element, 'setAttribute');

  setElementAttribute(element, attributeName, attributeValue);
  expect(spy).toHaveBeenCalledTimes(1);
  setElementAttribute(element, attributeName, attributeValue);
  expect(spy).toHaveBeenCalledTimes(1);
});

test('setElementAttribute(null) removes existing attribute', async () => {
  document.body.innerHTML = `<${elementName} ${attributeName}="${attributeInitialValue}"></${elementName}>`;
  const element = document.body.querySelector(elementName) as HTMLElement;
  expect(element).toBeTruthy();
  expect(element.getAttribute(attributeName)).toBe(attributeInitialValue);

  const spySet = jest.spyOn(element, 'setAttribute');
  const spyRemove = jest.spyOn(element, 'removeAttribute');

  setElementAttribute(element, attributeName, null);
  expect(spySet).toHaveBeenCalledTimes(0);
  expect(spyRemove).toHaveBeenCalledTimes(1);
  expect(element.hasAttribute(attributeName)).toBeFalsy();
});

test('setTextContent() sets element text content', async () => {
  document.body.innerHTML = `<${elementName}></${elementName}>`;
  const element = document.body.querySelector(elementName) as HTMLElement;
  expect(element).toBeTruthy();

  const spySet = jest.spyOn(element, 'textContent', 'set');

  expect(element.textContent).toBe('');
  setElementTextContent(element, 'test');
  expect(spySet).toHaveBeenCalledTimes(1);
  expect(element.textContent).toBe('test');

  setElementTextContent(element, 'test');
  // not called again if text is already
  expect(spySet).toHaveBeenCalledTimes(1);

  setElementTextContent(element, null);
  expect(spySet).toHaveBeenCalledTimes(2);
  expect(element.textContent).toBe('');
});
