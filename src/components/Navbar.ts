/* eslint-disable import/no-duplicates */
import 'components/menu/Menu'; // for side effects
import Menu from 'components/menu/Menu';
import './Navbar.css';

const template = `
  <a href="#" class="sf-logo" key="sf-navbar-logo">Logo</a>
  <a href="#" class="sf-hamburger" key="sf-navbar-hamburger">☰</a>
  <nav>
    <ul is="sf-menu"></ul>
  </nav>
`;

const mediaQuery = window.matchMedia('screen and (max-width: 576px)');

export default class Navbar extends HTMLElement {
  constructor() {
    super();
    console.log('Navbar constructor() called');
  }

  #showMenu: boolean = false;

  init() {
    if (this.children.length !== 3
      || !(this.children.item(0) instanceof HTMLAnchorElement)
      || !(this.children.item(1) instanceof HTMLAnchorElement)
      || this.children.item(2)?.nodeName !== 'NAV') {
      // init
      this.innerHTML = template;
    }
  }

  render() {
    this.init();
    const menu = this.querySelector('ul[is="sf-menu"]') as Menu;
    menu.showMenu(this.#showMenu);
  }

  // eslint-disable-next-line class-methods-use-this
  onClick(e: MouseEvent) {
    console.log('Navbar item clicked.');
    if (!(e.target instanceof Element)) {
      return;
    }
    e.preventDefault();
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

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
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

  adoptedCallback() {
    console.log('Navbar adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Navbar attributeChangedCallback() called');
  }
}

console.log('define "sf-menu"');
customElements.define('sf-navbar', Navbar);
