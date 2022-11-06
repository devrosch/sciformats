import 'components/menu/Menu';
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

  init() {
    this.innerHTML = template;
  }

  render() {
    this.init();
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
