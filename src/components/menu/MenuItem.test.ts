/* eslint-disable import/no-duplicates */
import 'components/menu/MenuItem'; // for side effects
import MenuItem from 'components/menu/MenuItem';

const element = 'sf-menu-item';
const keyAttr = 'key';
const key = 'abc';
const key2 = 'abc2';
const titleAttr = 'title';
const title = 'def';
const title2 = 'def2';

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-menu-item renders and observes attribute changes', async () => {
  document.body.innerHTML = `<${element} ${keyAttr}="${key}" ${titleAttr}="${title}"/>`;
  const menuItem = document.body.querySelector(element) as MenuItem;
  expect(menuItem).toBeTruthy();
  expect(menuItem.getAttribute(titleAttr)).toBe(title);
  expect(menuItem.getAttribute(keyAttr)).toBe(key);

  const a = menuItem.querySelector('a') as HTMLAnchorElement;
  expect(a).toBeTruthy();
  expect(a.getAttribute(titleAttr)).toBe(title);
  expect(a.getAttribute(keyAttr)).toBe(key);
  // a.innerText not available in JSDOM
  // see: https://github.com/jsdom/jsdom/issues/1245
  expect(a.textContent).toBe(title);

  menuItem.setAttribute(keyAttr, key2);
  expect(menuItem.getAttribute(keyAttr)).toBe(key2);
  expect(a.getAttribute(keyAttr)).toBe(key2);

  menuItem.setAttribute(titleAttr, title2);
  expect(menuItem.getAttribute(titleAttr)).toBe(title2);
  expect(a.getAttribute(titleAttr)).toBe(title2);
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
