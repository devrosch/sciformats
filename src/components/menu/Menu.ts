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
  <li is="sf-submenu" key="sf-submenu-file" title="File">
    <ul>
      <li is="sf-menu-item-file-open" key="sf-file-open" title="Open..."></li>
      <li is="sf-menu-item" key="sf-file-close" title="Close"></li>
      <li is="sf-menu-item" key="sf-file-close-all" title="Close All"></li>
    </ul>
  </li>
  <li is="sf-menu-item" key="sf-menu-item-2" title="Menu Item 2"></li>
  <li is="sf-submenu" key="sf-submenu-1" title="Submenu 1">
    <ul>
      <li is="sf-menu-item" key="sf-menu-item-3" title="Menu Item 3"></li>
      <li is="sf-submenu" key="sf-submenu-2" title="Submenu 2">
        <ul>
          <li is="sf-menu-item" key="sf-menu-item-5" title="Menu Item 5"></li>
          <li is="sf-menu-item" key="sf-menu-item-6" title="Menu Item 6"></li>
          <li is="sf-menu-item" key="sf-menu-item-7" title="Menu Item 7"></li>
        </ul>
      <li is="sf-menu-item" key="sf-menu-item-4" title="Menu Item 4"></li>
    </ul>
  </li>
  <li is="sf-menu-item" key="sf-about" title="About..."></li>
`;

export default class Menu extends HTMLUListElement {
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
      const subMenus = this.querySelectorAll('li[is="sf-submenu"]');
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
customElements.define('sf-menu', Menu, { extends: 'ul' });
