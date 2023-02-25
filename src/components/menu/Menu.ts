/* eslint-disable import/no-duplicates */
import './MenuItem';
import './MenuItemFileOpen';
import './Submenu';
import Submenu from './Submenu';
import { setElementAttribute } from 'util/RenderUtils';
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
    setElementAttribute(this, 'role', 'menubar');
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

  handleClick = (e: MouseEvent) => {
    if (e.target instanceof Submenu) {
      e.stopPropagation();
      e.preventDefault();
      const topLevelSubmenu = e.target.closest('sf-menu > sf-submenu');
      // close all submenus that are not parents of the clicked submenu
      for (const child of this.children) {
        if (child instanceof Submenu && child !== topLevelSubmenu) {
          if (child.hasAttribute('expand')
            && child.getAttribute('expand') !== 'false') {
            child.setAttribute('expand', 'false');
          }
        }
      }
    }
  };

  connectedCallback() {
    console.log('Menu connectedCallback() called');
    this.addEventListener('click', this.handleClick);
    this.render();
  }

  disconnectedCallback() {
    console.log('Menu disconnectedCallback() called');
    this.removeEventListener('click', this.handleClick);
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
