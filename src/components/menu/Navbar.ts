/* eslint-disable import/no-duplicates */
import Channel from 'model/Channel';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import './Menu'; // for side effects
import Menu from './Menu';
import './AboutDialog'; // for side effects
import AboutDialog from './AboutDialog';
import './Navbar.css';

// no template with slots required/possible
// see: https://stackoverflow.com/a/67333433
// maybe use html-template-loader instead
// https://stackoverflow.com/questions/37818401/importing-html-files-with-es6-template-string-loader
// <div>s required for relative/absolute placement of child elements
const template = `
  <a href="#" class="sf-logo" key="sf-navbar-logo">Logo</a>
  <a href="#" class="sf-hamburger" key="sf-navbar-hamburger">☰</a>
  <nav>
    <sf-menu role="menubar">
      <sf-submenu key="sf-submenu-file" title="File" role="menu">
        <sf-menu-item-file-open key="sf-file-open" title="Open..." role="menuitem"></sf-menu-item-file-open>
        <sf-menu-item key="sf-file-close" title="Close" role="menuitem"></sf-menu-item>
        <sf-menu-item key="sf-file-close-all" title="Close All" role="menuitem"></sf-menu-item>
      </sf-submenu>
      <sf-menu-item key="sf-menu-item-2" title="Menu Item 2" role="menuitem"></sf-menu-item>
      <sf-submenu key="sf-submenu-1" title="Submenu 1" role="menu">
        <sf-menu-item key="sf-menu-item-3" title="Menu Item 3" role="menuitem"></sf-menu-item>
        <sf-submenu key="sf-submenu-2" title="Submenu 2" role="menu">
          <sf-menu-item key="sf-menu-item-5" title="Menu Item 5" role="menuitem"></sf-menu-item>
          <sf-menu-item key="sf-menu-item-6" title="Menu Item 6" role="menuitem"></sf-menu-item>
          <sf-menu-item key="sf-menu-item-7" title="Menu Item 7" role="menuitem"></sf-menu-item>
        </sf-submenu>
        <sf-menu-item key="sf-menu-item-4" title="Menu Item 4" role="menuitem"></sf-menu-item>
      </sf-submenu>
      <sf-menu-item key="sf-about" title="About..." role="menuitem"></sf-menu-item>
    </sf-menu>
  </nav>
  <sf-about-dialog/>
`;

const events = {
  fileCloseRequested: 'sf-file-close-requested',
  fileCloseAllRequested: 'sf-file-close-all-requested',
};

const mediaQuery = window.matchMedia('screen and (max-width: 576px)');

export default class Navbar extends HTMLElement {
  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  constructor() {
    super();
    console.log('Navbar constructor() called');
  }

  #showMenu: boolean = false;

  init() {
    if (this.children.length !== 4
      || !(this.children.item(0) instanceof HTMLAnchorElement)
      || !(this.children.item(1) instanceof HTMLAnchorElement)
      || this.children.item(2)?.nodeName !== 'NAV'
      || !(this.children.item(3) instanceof AboutDialog)) {
      // init
      this.innerHTML = template;
    }
  }

  render() {
    this.init();
    const menu = this.querySelector('sf-menu') as Menu;
    menu.showMenu(this.#showMenu);
  }

  showAboutDialog() {
    const aboutDialog = this.querySelector('sf-about-dialog') as AboutDialog;
    aboutDialog.showModal(true);
  }

  // eslint-disable-next-line class-methods-use-this
  onClick(e: MouseEvent) {
    console.log('Navbar item clicked.');
    if (!(e.target instanceof Element)) {
      return;
    }
    e.preventDefault();
    e.stopPropagation();
    const key = e?.target?.getAttribute('key');
    console.log({ key });
    if (!key) {
      return;
    }

    switch (key) {
      case 'sf-navbar-hamburger':
        this.#showMenu = !this.#showMenu;
        this.render();
        break;
      case 'sf-file-close':
        this.#channel.dispatch(events.fileCloseRequested, null);
        this.#showMenu = false;
        this.render();
        break;
      case 'sf-file-close-all':
        this.#channel.dispatch(events.fileCloseAllRequested, null);
        this.#showMenu = false;
        this.render();
        break;
      case 'sf-about':
        this.showAboutDialog();
        this.#showMenu = false;
        this.render();
        break;
      default:
        this.#showMenu = false;
        this.render();
        break;
    }
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  handleScreenChange(e: MediaQueryListEvent) {
    // close menu including submenus whenever screen layout crosses threshold
    this.#showMenu = false;
    this.render();
  }

  handleOutsideSelection(e: MouseEvent) {
    console.log('handleOutsideSelection() called');
    const node = e.target as Node;
    if (!this.contains(node)) {
      // close menu including submenus whenever click ouside navbar occured
      this.#showMenu = false;
      this.render();
    }
  }

  connectedCallback() {
    console.log('Navbar connectedCallback() called');
    this.addEventListener('click', this.onClick.bind(this));
    mediaQuery.addEventListener('change', this.handleScreenChange.bind(this));
    document.addEventListener('click', this.handleOutsideSelection.bind(this));
    this.render();
  }

  disconnectedCallback() {
    console.log('Navbar disconnectedCallback() called');
    this.removeEventListener('click', this.onClick.bind(this));
    mediaQuery.removeEventListener('change', this.handleScreenChange.bind(this));
    document.removeEventListener('click', this.handleOutsideSelection.bind(this));
  }
}

console.log('define "sf-menu"');
customElements.define('sf-navbar', Navbar);
