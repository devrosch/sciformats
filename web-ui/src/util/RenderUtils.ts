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
  if (expectedAttributeName === attributeName
    && instance[instanceFieldName] !== newAttributeValue) {
    instance[instanceFieldName] = newAttributeValue;
    instance.render();
  }
};
