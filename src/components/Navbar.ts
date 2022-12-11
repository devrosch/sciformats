/* eslint-disable import/no-duplicates */
import Channel from 'model/Channel';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import './menu/Menu'; // for side effects
import Menu from './menu/Menu';
import './menu/AboutDialog'; // for side effects
import AboutDialog from './menu/AboutDialog';
import './Navbar.css';

const template = `
  <a href="#" class="sf-logo" key="sf-navbar-logo">Logo</a>
  <a href="#" class="sf-hamburger" key="sf-navbar-hamburger">☰</a>
  <nav>
    <ul is="sf-menu"></ul>
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
    const menu = this.querySelector('ul[is="sf-menu"]') as Menu;
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
