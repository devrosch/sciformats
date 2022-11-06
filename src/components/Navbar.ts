import 'components/menu/Menu';
import Menu from 'components/menu/Menu';
import './Navbar.css'

const template = `
  <a href="#" class="sf-logo" key="sf-navbar-logo">Logo</a>
  <nav>
    <ul is="sf-menu"></ul>
  </nav>
  <a href="#" class="sf-hamburger" key="sf-navbar-hamburger">☰</a>
`;

export default class Navbar extends HTMLElement {
  constructor() {
    super();
    console.log('Navbar constructor() called');
  }

  #showMenu: boolean = false;

  init() {
    if (this.children.length !== 3
      || !(this.children.item(0) instanceof HTMLAnchorElement)
      || this.children.item(1)?.nodeName !== 'NAV'
      || !(this.children.item(2) instanceof HTMLAnchorElement)) {
      // init
      this.innerHTML = template;
    }
  }

  render() {
    this.init();
    const menu = this.querySelector('ul[is="sf-menu"]') as Menu;
    if (this.#showMenu) {
      menu.classList.add('sf-show-menu');
    } else {
      menu.classList.remove('sf-show-menu');
    }
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
        console.log('show vertical menu: ' + this.#showMenu);
        this.render();
        break;
      default:
        break;
    }
  }

  connectedCallback() {
    console.log('Navbar connectedCallback() called');
    this.addEventListener('click', this.onClick.bind(this));
    this.render();
  }

  disconnectedCallback() {
    console.log('Navbar disconnectedCallback() called');
    this.removeEventListener('click', this.onClick.bind(this));
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
