/* eslint-disable import/no-duplicates */
import 'components/menu/Menu'; // for side effects
import MenuItem from 'components/menu/Menu';

const element = 'sf-menu';

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-menu renders', async () => {
  document.body.innerHTML = `<ul is="${element}"></ul>`;
  const menu = document.body.querySelector('ul') as MenuItem;
  expect(menu).toBeTruthy();

  const submenuFile = menu.querySelector('li[key="sf-submenu-file"]');
  expect(submenuFile).toBeTruthy();
  const menuItem = menu.querySelector('li[key="sf-menu-item-2"]');
  expect(menuItem).toBeTruthy();
});

test('sf-menu listenes to click events', async () => {
  document.body.innerHTML = `<ul is="${element}"></ul>`;
  const menu = document.body.querySelector('ul') as MenuItem;
  expect(menu).toBeTruthy();
  const aFileClose = document.body.querySelector('a[key="sf-file-close"]') as HTMLAnchorElement;
  expect(aFileClose).toBeTruthy();

  // logs "TODO" for file open/close
  const logSpy = jest.spyOn(console, 'log');
  aFileClose.click();
  expect(logSpy).toHaveBeenLastCalledWith(expect.stringMatching(/.*TODO.*/));
});

test('sf-menu showMenu() sets CSS class and for "false" argument collapses all submenus', async () => {
  document.body.innerHTML = `<ul is="${element}"></ul>`;
  const menu = document.body.querySelector('ul') as MenuItem;
  expect(menu).toBeTruthy();
  expect(menu.classList).not.toContain('sf-show-menu');
  const submenus = menu.querySelectorAll('li[is="sf-submenu"]');
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
