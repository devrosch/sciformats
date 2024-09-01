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
