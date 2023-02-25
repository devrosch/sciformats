/* disable rule for now until more helpers are added */
/* eslint-disable import/prefer-default-export */

/**
 * Set the attribute of an HTML element.
 * The attribute is only set if it does not already exist or does not have the value.
 * @param element HTML element to set the attribute on.
 * @param attributeName Attribute name.
 * @param attributeValue Attribute value.
 */
export const setElementAttribute = (
  element: HTMLElement,
  attributeName: string,
  attributeValue: string,
) => {
  const attr = element.getAttribute(attributeName);
  if (attributeValue !== attr) {
    element.setAttribute(attributeName, attributeValue);
  }
};
