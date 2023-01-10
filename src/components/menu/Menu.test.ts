/* eslint-disable import/no-duplicates */
import 'components/menu/Menu'; // for side effects
import Menu from 'components/menu/Menu';

const element = 'sf-menu';

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-menu renders', async () => {
  document.body.innerHTML = `
  <${element}>
    <sf-submenu key="sf-submenu-file" title="File"></sf-submenu>
    <sf-menu-item key="sf-menu-item-2" title="Menu Item 2"></sf-menu-item>
  </${element}>
  `;
  const menu = document.body.querySelector(element) as Menu;
  expect(menu).toBeTruthy();
  expect(menu.getAttribute('role')).toBe('menubar');

  const submenuFile = menu.querySelector('sf-submenu[key="sf-submenu-file"]');
  expect(submenuFile).toBeTruthy();
  const menuItem = menu.querySelector('sf-menu-item[key="sf-menu-item-2"]');
  expect(menuItem).toBeTruthy();
});

test('sf-menu showMenu() sets CSS class and for "false" argument collapses all submenus', async () => {
  document.body.innerHTML = `
  <${element}>
    <sf-submenu key="sf-submenu-1" title="Submenu 1"></sf-submenu>
    <sf-submenu key="sf-submenu-2" title="Submenu 2"></sf-submenu>
  </${element}>
  `;
  const menu = document.body.querySelector(element) as Menu;
  expect(menu).toBeTruthy();
  expect(menu.classList).not.toContain('sf-show-menu');
  const submenus = menu.querySelectorAll('sf-submenu');
  expect(submenus.length).toBeGreaterThan(0);
  for (const submenu of submenus) {
    submenu.setAttribute('expand', 'true');
  }

  menu.showMenu(true);
  expect(menu.classList).toContain('sf-show-menu');
  for (const submenu of submenus) {
    expect(submenu.getAttribute('expand')).toBe('true');
  }

  menu.showMenu(false);
  expect(menu.classList).not.toContain('sf-show-menu');
  for (const submenu of submenus) {
    expect(submenu.getAttribute('expand')).toBe('false');
  }
});

test('sf-menu submenu click closes other submenu hierarchies', async () => {
  document.body.innerHTML = `
  <${element}>
    <sf-submenu key="sf-submenu-1" title="Submenu 1" expand="false"></sf-submenu>
    <sf-submenu key="sf-submenu-2" title="Submenu 2" expand="true"></sf-submenu>
  </${element}>
  `;
  const menu = document.body.querySelector(element) as Menu;
  expect(menu).toBeTruthy();
  const submenu1 = menu.querySelector('[key="sf-submenu-1"]') as HTMLElement;
  expect(submenu1).toBeTruthy();
  expect(submenu1?.getAttribute('expand')).toBe('false');
  const submenu2 = menu.querySelector('[key="sf-submenu-2"]');
  expect(submenu2).toBeTruthy();
  expect(submenu2?.getAttribute('expand')).toBe('true');

  submenu1.dispatchEvent(new Event('click', { bubbles: true }));
  expect(submenu2?.getAttribute('expand')).toBe('false');
});
