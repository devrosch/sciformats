/* eslint-disable no-duplicate-imports */
import './MenuItem';
import './MenuItemFileOpen';
import './Submenu';
import { setElementAttribute } from 'util/RenderUtils';
import Submenu from './Submenu';
import './Menu.css';

export default class Menu extends HTMLElement {
  #initialized = false;

  constructor() {
    super();
    console.log('Menu constructor() called');
  }

  /* eslint-disable class-methods-use-this */
  init() {
    if (!this.#initialized) {
      // noop, menu items should be inserted by parent
      this.#initialized = true;
    }
  }

  render() {
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
    this.init();
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
    this.init();
  }
}

console.log('define "sf-menu"');
customElements.define('sf-menu', Menu);
