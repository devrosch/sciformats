import './MenuItem';
import './MenuItemFileOpen';
import './Submenu';
import './Menu.css';

// no template with slots required/possible
// see: https://stackoverflow.com/a/67333433
// maybe use html-template-loader instead
// https://stackoverflow.com/questions/37818401/importing-html-files-with-es6-template-string-loader
// <ul> required because of: https://stackoverflow.com/a/20550925
const template = `
  <sf-submenu key="sf-submenu-file" title="File">
    <ul>
      <sf-menu-item-file-open key="sf-file-open" title="Open..."></sf-menu-item-file-open>
      <sf-menu-item key="sf-file-close" title="Close"></sf-menu-item>
      <sf-menu-item key="sf-file-close-all" title="Close All"></sf-menu-item>
    </ul>
  </sf-submenu>
  <sf-menu-item key="sf-menu-item-2" title="Menu Item 2"></sf-menu-item>
  <sf-submenu key="sf-submenu-1" title="Submenu 1">
    <ul>
      <sf-menu-item key="sf-menu-item-3" title="Menu Item 3"></sf-menu-item>
      <sf-submenu key="sf-submenu-2" title="Submenu 2">
        <ul>
          <sf-menu-item key="sf-menu-item-5" title="Menu Item 5"></sf-menu-item>
          <sf-menu-item key="sf-menu-item-6" title="Menu Item 6"></sf-menu-item>
          <sf-menu-item key="sf-menu-item-7" title="Menu Item 7"></sf-menu-item>
        </ul>
      </sf-submenu>
      <sf-menu-item key="sf-menu-item-4" title="Menu Item 4"></sf-menu-item>
    </ul>
  </sf-submenu>
  <sf-menu-item key="sf-about" title="About..."></sf-menu-item>
`;

export default class Menu extends HTMLElement {
  constructor() {
    super();
    console.log('Menu constructor() called');
  }

  init() {
    this.innerHTML = template;
  }

  render() {
    this.init();
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
