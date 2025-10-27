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
import './MenuItem.css';

const template = `
  <button>
    <span class="menu-item-name"></span>
    <span class="menu-item-shortcut"></span>
  </button>
`;

export default class MenuItem extends HTMLElement {
  static get observedAttributes() {
    return ['title', 'shortcut'];
  }

  #initialized = false;

  private _title: string | null = null;

  private _shortcut: string | null = null;

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    const button = this.getElementsByTagName('button').item(
      0,
    ) as HTMLButtonElement;
    const nameSpan = button.children.item(0) as HTMLSpanElement;
    const shortcutSpan = button.children.item(1) as HTMLSpanElement;
    setElementAttribute(this, 'role', 'none');
    setElementAttribute(button, 'role', 'menuitem');
    setElementTextContent(nameSpan, this._title);
    setElementTextContent(shortcutSpan, this._shortcut);
  }

  onClick = (e: Event) => {
    if (e.target !== this) {
      // click event from child element
      e.stopPropagation();
      this.click();
    }
  };

  connectedCallback() {
    this.init();
    this._title = this.getAttribute('title');
    this._shortcut = this.getAttribute('shortcut');
    this.addEventListener('click', this.onClick);
    this.render();
  }

  disconnectedCallback() {
    this.removeEventListener('click', this.onClick);
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
    updateStateAndRender(this, 'title', '_title', name, newValue);
    updateStateAndRender(this, 'shortcut', '_shortcut', name, newValue);
  }
}

customElements.define('sf-menu-item', MenuItem);
