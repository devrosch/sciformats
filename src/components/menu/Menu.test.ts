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
  const aFileOpen = document.body.querySelector('a[key="sf-file-open"]') as HTMLAnchorElement;
  expect(aFileOpen).toBeTruthy();
  const aMenuItem2 = document.body.querySelector('a[key="sf-menu-item-2"]') as HTMLAnchorElement;
  expect(aFileOpen).toBeTruthy();
  const aFileClose = document.body.querySelector('a[key="sf-file-close"]') as HTMLAnchorElement;
  expect(aFileOpen).toBeTruthy();

  // logs "TODO" for file open/close
  const logSpy = jest.spyOn(console, 'log');
  aFileOpen.click();
  expect(logSpy).toHaveBeenLastCalledWith(expect.stringMatching(/.*TODO.*/));
  aMenuItem2.click();
  expect(logSpy).not.toHaveBeenLastCalledWith(expect.stringMatching(/.*TODO.*/));
  aFileClose.click();
  expect(logSpy).toHaveBeenLastCalledWith(expect.stringMatching(/.*TODO.*/));
});
