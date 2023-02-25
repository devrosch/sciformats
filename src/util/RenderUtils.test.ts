import { setElementAttribute } from 'util/RenderUtils';

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