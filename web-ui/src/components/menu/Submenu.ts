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

import {
  setElementAttribute,
  setElementTextContent,
  updateStateAndRender,
} from 'util/RenderUtils';
import './Submenu.css';

/**
 * Max width for vertical menu.
 */
const maxWidth = 576;

export default class Submenu extends HTMLElement {
  static get observedAttributes() {
    return ['title', 'key', 'expand'];
  }

  #initialized = false;

  private _title: string | null = null;

  private _key: string | null = null;

  private _expand = false;

  init() {
    if (!this.#initialized) {
      this._title = this.getAttribute('title');
      this._key = this.getAttribute('key');
      this._expand = this.hasAttribute('expand')
        ? this.getAttribute('expand') === 'true'
        : false;

      // add <a> at beginning
      const innerHtml = this.innerHTML;
      this.innerHTML = `
        <button key="${this._key}">
          <span class="sf-expand-collapse-indicator">›</span>&nbsp;<span>${this._title}</span>
        </button>
        <div role="none">
          ${innerHtml}
        </div>
        `;

      this.#initialized = true;
    }
  }

  render() {
    const button = this.getElementsByTagName('button').item(0) as HTMLButtonElement;
    const buttonIndicatorSpan = button.querySelector(':nth-child(1)') as HTMLSpanElement;
    const buttonTitleSpan = button.querySelector(':nth-child(2)') as HTMLSpanElement;

    setElementAttribute(this, 'role', 'menu');
    setElementAttribute(button, 'key', this._key);
    setElementAttribute(button, 'title', this._title);
    setElementAttribute(buttonIndicatorSpan, 'key', this._key);
    setElementAttribute(buttonTitleSpan, 'key', this._key);
    setElementTextContent(buttonTitleSpan, this._title);
    if (this._expand) {
      setElementAttribute(this, 'expand', 'true');
      this.classList.add('sf-submenu-expand');
    } else {
      setElementAttribute(this, 'expand', 'false');
      this.classList.remove('sf-submenu-expand');
      const subMenus = this.getElementsByClassName('sf-submenu-expand');
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

  onMouseEnter = (e: Event) => {
    // only take action if screen is small
    if (window.innerWidth > maxWidth) {
      e.stopPropagation();
      this._expand = true;
      this.render();
    }
  };

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  onMouseLeave = (e: Event) => {
    // only take action if screen is small
    if (window.innerWidth > maxWidth) {
      this._expand = false;
      this.render();
    }
  };

  onClick = (e: MouseEvent) => {
    const key = (e?.target as Element | null)?.getAttribute('key');
    if (key === this._key && !(e.target instanceof Submenu)) {
      e.stopPropagation();
      e.preventDefault();
      this._expand = !this._expand;
      this.click();
      this.render();
    }
  };

  connectedCallback() {
    this.init();
    this._title = this.getAttribute('title');
    this._key = this.getAttribute('key');
    this._expand = this.getAttribute('expand') === 'true';
    this.addEventListener('mouseenter', this.onMouseEnter);
    this.addEventListener('mouseleave', this.onMouseLeave);
    this.addEventListener('click', this.onClick);
    this.render();
  }

  disconnectedCallback() {
    this.removeEventListener('mouseenter', this.onMouseEnter);
    this.removeEventListener('mouseleave', this.onMouseLeave);
    this.removeEventListener('click', this.onClick);
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
    updateStateAndRender(this, 'title', '_title', name, newValue);
    updateStateAndRender(this, 'key', '_key', name, newValue);
    updateStateAndRender(this, 'expand', '_expand', name, newValue === 'true');
  }
}

customElements.define('sf-submenu', Submenu);
