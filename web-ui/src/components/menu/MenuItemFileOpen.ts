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

import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'util/CustomEventsChannel';
import { setElementAttribute, updateStateAndRender } from 'util/RenderUtils';
import MenuItem from './MenuItem';
import './MenuItemFileOpen.css';

const menuItemKeyPostfix = 'input-menu-item';

const template = `
  <input id="sf-file-open-input" class="sf-file-open-input" tabindex="-1" key="sf-file-open-input" type="file" multiple="true"/>
  <label for="sf-file-open-input" class="sf-file-input-label">
    <sf-menu-item key="sf-file-open-${menuItemKeyPostfix}"></sf-menu-item>
  </label>  
`;

export default class MenuItemFileOpen extends HTMLElement {
  static get observedAttributes() {
    return ['title', 'key', 'shortcut'];
  }

  #initialized = false;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  private _title: string | null = null;

  private _key: string | null = null;

  private _shortcut: string | null = null;

  #shortcutActive = false;

  constructor() {
    super();
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    setElementAttribute(this, 'role', 'none');

    const input = this.getElementsByTagName('input').item(
      0,
    ) as HTMLInputElement;
    setElementAttribute(input, 'key', `${this._key}-input`);

    const label = this.querySelector('label') as HTMLLabelElement;
    setElementAttribute(label, 'key', `${this._key}-input-label`);

    const menuItem = this.querySelector('label > sf-menu-item') as MenuItem;
    setElementAttribute(menuItem, 'key', `${this._key}-${menuItemKeyPostfix}`);
    setElementAttribute(menuItem, 'title', this._title);
    setElementAttribute(menuItem, 'shortcut', this._shortcut);
    if (this.#shortcutActive) {
      menuItem.setAttribute('accesskey', 'o');
    }
  }

  activateShortcut() {
    this.#shortcutActive = true;
    this.init();
    this.render();
  }

  onClick = (e: MouseEvent) => {
    const key = (e?.target as Element | null)?.getAttribute('key');
    if (!key) {
      return;
    }
    if (key === `${this._key}-${menuItemKeyPostfix}`) {
      e.stopPropagation();
      e.preventDefault();
      const input = this.getElementsByTagName('input').item(
        0,
      ) as HTMLInputElement;
      input.click();
    }
    if (key === `${this._key}-input`) {
      e.stopPropagation();
    }
  };

  onChange = (e: Event) => {
    const input = e.target as HTMLInputElement;
    const selectedFiles = input.files;
    if (selectedFiles === null || typeof selectedFiles === 'undefined') {
      return;
    }
    const files = Array.from(selectedFiles);
    // reset file input, see https://stackoverflow.com/questions/20549241/how-to-reset-input-type-file
    // if not reset, opening the same file again does not fire on change event => cannot be opened
    input.value = '';
    // notify other application components
    this.#channel.dispatch('sf-file-open-requested', { files });
    // notify parents, so that menu might be closed
    this.click();
  };

  connectedCallback() {
    this.init();
    this._title = this.getAttribute('title');
    this._key = this.getAttribute('key');
    this._shortcut = this.getAttribute('shortcut');
    this.addEventListener('click', this.onClick);
    this.addEventListener('change', this.onChange);
    this.render();
  }

  disconnectedCallback() {
    this.removeEventListener('click', this.onClick);
    this.removeEventListener('change', this.onChange);
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
    updateStateAndRender(this, 'title', '_title', name, newValue);
    updateStateAndRender(this, 'key', '_key', name, newValue);
    updateStateAndRender(this, 'shortcut', '_shortcut', name, newValue);
  }
}

customElements.define('sf-menu-item-file-open', MenuItemFileOpen);
