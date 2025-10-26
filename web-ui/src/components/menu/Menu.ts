/**
 * Copyright (c) 2025 Robert Schiwon
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/* eslint-disable no-duplicate-imports */
import './MenuItem';
import './MenuItemFileOpen';
import './Submenu';
import { setElementAttribute } from 'util/RenderUtils';
import Submenu from './Submenu';
import './Menu.css';

export default class Menu extends HTMLElement {
  #initialized = false;

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
        if (
          subMenu.hasAttribute('expand') &&
          subMenu.getAttribute('expand') !== 'false'
        ) {
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
          if (
            child.hasAttribute('expand') &&
            child.getAttribute('expand') !== 'false'
          ) {
            child.setAttribute('expand', 'false');
          }
        }
      }
    }
  };

  connectedCallback() {
    this.init();
    this.addEventListener('click', this.handleClick);
    this.render();
  }

  disconnectedCallback() {
    this.removeEventListener('click', this.handleClick);
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
  }
}

customElements.define('sf-menu', Menu);
