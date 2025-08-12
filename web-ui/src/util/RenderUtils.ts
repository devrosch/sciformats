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

/* eslint no-param-reassign: ["error", { "props": false }] */

/**
 * Set or remove the attribute of an HTML element.
 * The attribute is only set if it does not already exist or does not have the value.
 * @param element HTML element to set the attribute on.
 * @param attributeName Attribute name.
 * @param attributeValue Attribute value. If the value is null and the attribute exists,
 * the attribute is removed.
 */
export const setElementAttribute = (
  element: HTMLElement,
  attributeName: string,
  attributeValue: string | null,
) => {
  const attr = element.getAttribute(attributeName);
  if (attributeValue === null) {
    if (attr !== null) {
      element.removeAttribute(attributeName);
    }
  } else if (attributeValue !== attr) {
    element.setAttribute(attributeName, attributeValue);
  }
};

/**
 * Sets the text content of an element.
 * The text content is only set if it is different fromn the provided text.
 * @param element HTML element to set the text content for.
 * @param text Text content to set.
 */
export const setElementTextContent = (
  element: HTMLElement,
  text: string | null,
) => {
  const content = element.textContent;
  if (content !== text) {
    element.textContent = text;
  }
};

/**
 * Checks if an attribute value change should result in a component's state change.
 * If so, the component's state is updated and its render() method called.
 * @param instance The component holding state and exposing the attribute.
 * @param expectedAttributeName The name of the attribute to observe.
 * @param instanceFieldName The component's field corresponding to the attribute's state.
 * @param attributeName The name of the attribute that changed.
 * @param newAttributeValue The new value of the attribute.
 */
export const updateStateAndRender = (
  instance: any,
  expectedAttributeName: string,
  instanceFieldName: string,
  attributeName: string,
  newAttributeValue: any,
) => {
  if (
    expectedAttributeName === attributeName &&
    instance[instanceFieldName] !== newAttributeValue
  ) {
    instance[instanceFieldName] = newAttributeValue;
    instance.render();
  }
};
