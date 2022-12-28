/* eslint-disable import/no-duplicates */
import 'components/menu/Submenu'; // for side effects
import Submenu from 'components/menu/Submenu';

const element = 'sf-submenu';
const keyAttr = 'key';
const key = 'abc';
const key2 = 'abc2';
const titleAttr = 'title';
const title = 'def';
const title2 = 'def2';
const expandAttr = 'expand';
const expand = false;
const expand2 = true;

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-menu-item renders and observes attribute changes', async () => {
  document.body.innerHTML = `
    <${element}
    ${keyAttr}="${key}"
    ${titleAttr}="${title}"
    ${expandAttr}="${expand}"/>`;
  const submenu = document.body.querySelector(element) as Submenu;
  expect(submenu).toBeTruthy();
  expect(submenu.getAttribute(titleAttr)).toBe(title);
  expect(submenu.getAttribute(keyAttr)).toBe(key);
  expect(submenu.getAttribute(expandAttr)).toBe(`${expand}`);

  const a = submenu.querySelector('a') as HTMLAnchorElement;
  expect(a).toBeTruthy();
  expect(a.getAttribute(titleAttr)).toBe(title);
  expect(a.getAttribute(keyAttr)).toBe(key);
  // a.innerText not available in JSDOM
  // see: https://github.com/jsdom/jsdom/issues/1245
  expect(a.textContent).toBe(`▸ ${title}`);

  submenu.setAttribute(keyAttr, key2);
  expect(submenu.getAttribute(keyAttr)).toBe(key2);
  expect(a.getAttribute(keyAttr)).toBe(key2);

  submenu.setAttribute(titleAttr, title2);
  expect(submenu.getAttribute(titleAttr)).toBe(title2);
  expect(a.getAttribute(titleAttr)).toBe(title2);

  submenu.setAttribute(expandAttr, `${expand2}`);
  expect(submenu.getAttribute(expandAttr)).toBe(`${expand2}`);
  expect(a.textContent).toBe(`▾ ${title2}`);
});

test('sf-submenu expands on click, event does not get propagated', async () => {
  document.body.innerHTML = `<${element} ${keyAttr}="${key}" ${titleAttr}="${title}"/>`;
  const submenu = document.body.querySelector(element) as Submenu;
  expect(submenu).toBeTruthy();
  const a = submenu.querySelector('a') as HTMLAnchorElement;
  expect(a).toBeTruthy();
  expect(a.textContent).toBe(`▸ ${title}`);

  const clickHandler = jest.fn();
  document.body.addEventListener('click', clickHandler);
  a.click();
  document.body.removeEventListener('click', clickHandler);
  expect(a.textContent).toBe(`▾ ${title}`);
  expect(clickHandler).toHaveBeenCalledTimes(0);
});

test('sf-submenu expands/collapses on mouse enter/leave', async () => {
  document.body.innerHTML = `<${element} ${keyAttr}="${key}" ${titleAttr}="${title}"/>`;
  const submenu = document.body.querySelector(element) as Submenu;
  expect(submenu).toBeTruthy();
  const a = submenu.querySelector('a') as HTMLAnchorElement;
  expect(a).toBeTruthy();
  expect(a.textContent).toBe(`▸ ${title}`);
  submenu.onMouseEnter(new Event('onmouseenter'));
  expect(a.textContent).toBe(`▾ ${title}`);
  submenu.onMouseLeave(new Event('onmouseleave'));
  expect(a.textContent).toBe(`▸ ${title}`);
});

test('sf-submenu "expand" attribute sets visibility', async () => {
  document.body.innerHTML = `<${element}/>`;
  const submenu = document.body.querySelector(element) as Submenu;
  expect(submenu).toBeTruthy();

  expect(submenu.getAttribute('expand')).toBe('false');
  expect(submenu.classList).not.toContain('sf-submenu-expand');

  submenu.setAttribute('expand', 'true');
  expect(submenu.getAttribute('expand')).toBe('true');
  expect(submenu.classList).toContain('sf-submenu-expand');

  submenu.removeAttribute('expand');
  expect(submenu.getAttribute('expand')).toBe('false');
  expect(submenu.classList).not.toContain('sf-submenu-expand');
});

test('sf-submenu collapses nested submenus on collapse', async () => {
  document.body.innerHTML = `
    <${element} key="sub-1">
      <${element} key="sub-2"></${element}>
    </${element}>`;
  const parentSubmenu = document.body.querySelector(element) as Submenu;
  expect(parentSubmenu).toBeTruthy();
  const nestedSubmenu = parentSubmenu.querySelector(element) as Submenu;
  expect(nestedSubmenu).toBeTruthy();

  expect(parentSubmenu.getAttribute('expand')).toBe('false');
  expect(parentSubmenu.classList).not.toContain('sf-submenu-expand');
  expect(nestedSubmenu.getAttribute('expand')).toBe('false');
  expect(nestedSubmenu.classList).not.toContain('sf-submenu-expand');

  parentSubmenu.setAttribute('expand', 'true');
  nestedSubmenu.setAttribute('expand', 'true');
  expect(parentSubmenu.getAttribute('expand')).toBe('true');
  expect(parentSubmenu.classList).toContain('sf-submenu-expand');
  expect(nestedSubmenu.getAttribute('expand')).toBe('true');
  expect(nestedSubmenu.classList).toContain('sf-submenu-expand');

  parentSubmenu.removeAttribute('expand');
  expect(parentSubmenu.getAttribute('expand')).toBe('false');
  expect(parentSubmenu.classList).not.toContain('sf-submenu-expand');
  expect(nestedSubmenu.getAttribute('expand')).toBe('false');
  expect(nestedSubmenu.classList).not.toContain('sf-submenu-expand');
});
