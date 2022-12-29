import './MenuItem';
import './MenuItemFileOpen';
import './Submenu';
import './Menu.css';

export default class Menu extends HTMLElement {
  constructor() {
    super();
    console.log('Menu constructor() called');
  }

  /* eslint-disable class-methods-use-this */
  init() {
    // noop, menu items should be inserted by parent
  }

  render() {
    this.init();
    const role = this.hasAttribute('role') ? this.getAttribute('role') : '';
    if (role !== 'menu') {
      this.setAttribute('role', 'menubar');
    }
  }

  showMenu(show: boolean) {
    if (show) {
      this.classList.add('sf-show-menu');
    } else {
      this.classList.remove('sf-show-menu');
      const subMenus = this.querySelectorAll('sf-submenu');
      for (const subMenu of subMenus) {
        if (subMenu.hasAttribute('expand')
          && subMenu.getAttribute('expand') !== 'false') {
          subMenu.setAttribute('expand', 'false');
        }
      }
    }
  }

  connectedCallback() {
    console.log('Menu connectedCallback() called');
    this.render();
  }

  disconnectedCallback() {
    console.log('Menu disconnectedCallback() called');
  }

  adoptedCallback() {
    console.log('Menu adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Menu attributeChangedCallback() called');
  }
}

console.log('define "sf-menu"');
customElements.define('sf-menu', Menu);
