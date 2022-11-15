/* eslint-disable import/no-duplicates */
import './NavbarMatchMediaMock'; // mock window.matchMedia()
import './Navbar'; // for side effects
import Navbar from './Navbar';
import Menu from 'components/menu/Menu';

const element = 'sf-navbar';

beforeAll(() => {
  window.matchMedia = jest.fn(() => {
    return {
      matches: false,
      addListener: jest.fn(),
      removeListener: jest.fn(),
    }
  }) as any;
});

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-navbar renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();

  expect(navbar.children).toHaveLength(3);
  expect(navbar.children.item(0)?.nodeName).toBe('A');
  expect(navbar.children.item(1)?.nodeName).toBe('A');
  expect(navbar.children.item(2)?.nodeName).toBe('NAV');
});

test('sf-navbar hamburger menu toggles menu visibility', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();
  const menu = navbar.querySelector('ul[is="sf-menu"]') as Menu;
  expect(menu).toBeTruthy();

  const mockShowMenu = jest.fn((show) => show);
  menu.showMenu = mockShowMenu;

  const mockElement = document.createElement('a');
  mockElement.setAttribute('key', `sf-navbar-hamburger`);
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  navbar.onClick(mouseEvent);
  expect(mouseEvent.stopPropagation).toHaveBeenCalledTimes(1);
  expect(mouseEvent.preventDefault).toHaveBeenCalledTimes(1);
  expect(mockShowMenu).toHaveBeenCalledTimes(1);
  expect(mockShowMenu.mock.results[0].value).toBe(true);
  navbar.onClick(mouseEvent);
  expect(mouseEvent.stopPropagation).toHaveBeenCalledTimes(2);
  expect(mouseEvent.preventDefault).toHaveBeenCalledTimes(2);
  expect(mockShowMenu).toHaveBeenCalledTimes(2);
  expect(mockShowMenu.mock.results[1].value).toBe(false);
});

test('sf-navbar menu item click closes menu', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();
  const menu = navbar.querySelector('ul[is="sf-menu"]') as Menu;
  expect(menu).toBeTruthy();

  const mockShowMenu = jest.fn((show) => show);
  menu.showMenu = mockShowMenu;

  const mockElement = document.createElement('a');
  mockElement.setAttribute('key', `sf-menu-item-1`);
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  navbar.onClick(mouseEvent);
  expect(mockShowMenu).toHaveBeenCalledTimes(1);
  expect(mockShowMenu.mock.results[0].value).toBe(false);
});

test('sf-navbar screen change closes menu', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();
  const menu = navbar.querySelector('ul[is="sf-menu"]') as Menu;
  expect(menu).toBeTruthy();

  const mockShowMenu = jest.fn((show) => show);
  menu.showMenu = mockShowMenu;

  const e = {} as unknown as MediaQueryListEvent;
  navbar.handleScreenChange(e);

  expect(mockShowMenu).toHaveBeenCalledTimes(1);
  expect(mockShowMenu.mock.results[0].value).toBe(false);
});

test('sf-navbar screen change closes menu', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();
  const menu = navbar.querySelector('ul[is="sf-menu"]') as Menu;
  expect(menu).toBeTruthy();

  const mockShowMenu = jest.fn((show) => show);
  menu.showMenu = mockShowMenu;

  const mockElement = document.createElement('a');
  mockElement.setAttribute('key', `any`);
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  navbar.handleOutsideSelection(mouseEvent);

  expect(mockShowMenu).toHaveBeenCalledTimes(1);
  expect(mockShowMenu.mock.results[0].value).toBe(false);
});
