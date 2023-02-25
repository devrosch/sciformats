/* disable rule for now until more helpers are added */
/* eslint-disable import/prefer-default-export */

/**
 * Set or remove the attribute of an HTML element.
 * The attribute is only set if it does not already exist or does not have the value.
 * @param element HTML element to set the attribute on.
 * @param attributeName Attribute name.
 * @param attributeValue Attribute value. If the value is null and the attribute exists, it is removed.
 */
export const setElementAttribute = (
  element: HTMLElement,
  attributeName: string,
  attributeValue: string | null,
) => {
  const attr = element.getAttribute(attributeName);
  if (attributeValue === null) {
    if (attr !== null) {
      element.removeAttribute(attributeName)
    }
  }
  else if (attributeValue !== attr) {
    element.setAttribute(attributeName, attributeValue);
  }
};
