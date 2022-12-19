/* eslint-disable import/no-duplicates */
import Message from 'model/Message';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Menu from './Menu';
import './NavbarMatchMediaMock'; // mock window.matchMedia()
import './Navbar'; // for side effects
import Navbar from './Navbar';
import AboutDialog from './AboutDialog';

const element = 'sf-navbar';

const testEventDispatchedForClickedKey = (
  key: string,
  expectedEventName: string,
  done: (error?: any) => any,
) => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector(element) as Navbar;
  expect(navbar).toBeTruthy();

  const mockElement = document.createElement('a');
  mockElement.setAttribute('key', key);
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  const listener = (message: Message) => {
    try {
      expect(message.name).toBe(expectedEventName);
      done();
    } catch (error) {
      done(error);
    }
  };

  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.addListener(expectedEventName, listener);

  navbar.onClick(mouseEvent);
};

beforeAll(() => {
  // see: https://github.com/jsdom/jsdom/issues/3294
  HTMLDialogElement.prototype.showModal = jest.fn();
  HTMLDialogElement.prototype.close = jest.fn();
});

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-navbar renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();

  expect(navbar.children).toHaveLength(4);
  expect(navbar.children.item(0)?.nodeName).toBe('A');
  expect(navbar.children.item(1)?.nodeName).toBe('A');
  expect(navbar.children.item(2)?.nodeName).toBe('NAV');
  expect(navbar.children.item(3)?.nodeName).toBe('SF-ABOUT-DIALOG');
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
  mockElement.setAttribute('key', 'sf-navbar-hamburger');
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
  mockElement.setAttribute('key', 'sf-menu-item-1');
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
  mockElement.setAttribute('key', 'any');
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  navbar.handleOutsideSelection(mouseEvent);

  expect(mockShowMenu).toHaveBeenCalledTimes(1);
  expect(mockShowMenu.mock.results[0].value).toBe(false);
});

test('sf-navbar - about click opens AboutDialog', async () => {
  document.body.innerHTML = `<${element}/>`;
  const navbar = document.body.querySelector('sf-navbar') as Navbar;
  expect(navbar).toBeTruthy();
  const aboutDialog = navbar.querySelector('sf-about-dialog') as AboutDialog;
  expect(aboutDialog).toBeTruthy();

  const showModalMock = jest.fn((show) => show);
  aboutDialog.showModal = showModalMock;

  const mockElement = document.createElement('a');
  mockElement.setAttribute('key', 'sf-about');
  const mouseEvent = {
    target: mockElement,
    stopPropagation: jest.fn(),
    preventDefault: jest.fn(),
  } as unknown as MouseEvent;

  expect(showModalMock).toHaveBeenCalledTimes(0);
  navbar.onClick(mouseEvent);
  expect(showModalMock).toHaveBeenCalledTimes(1);
});

test('close event dispatched when "file - close" is clicked', (done) => {
  testEventDispatchedForClickedKey('sf-file-close', 'sf-file-close-requested', done);
});

test('close event dispatched when "file - close all" is clicked', (done) => {
  testEventDispatchedForClickedKey('sf-file-close-all', 'sf-file-close-all-requested', done);
});
