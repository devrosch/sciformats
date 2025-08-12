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

import { updateStateAndRender } from 'util/RenderUtils';

const keyValueTemplate = '<span></span>: <span></span>';
const valueOnlyTemplate = '<span></span>';

export default class Parameter extends HTMLElement {
  static get observedAttributes() {
    return ['key', 'value'];
  }

  #initialized = false;

  private _key = null as string | null;

  private _value = null as string | boolean | number | bigint | null;

  constructor() {
    super();
    console.log('Parameter constructor() called');
  }

  init() {
    if (!this.#initialized) {
      // noop
      this.#initialized = true;
    }
  }

  render() {
    if (this._key === '') {
      this.innerHTML = valueOnlyTemplate;
      const valueSpan = this.querySelector('span');
      valueSpan!.textContent = this._value?.toString() ?? null;
    } else {
      this.innerHTML = keyValueTemplate;
      const spans = this.querySelectorAll('span');
      const keySpan = spans[0];
      const valueSpan = spans[1];
      keySpan.textContent = this._key === null ? '' : this._key;
      valueSpan.textContent =
        this._value === null ? '' : this._value?.toString();
    }
  }

  connectedCallback() {
    console.log('Parameter connectedCallback() called');
    this.init();
    const key = this.getAttribute('key');
    const value = this.getAttribute('value');
    this._key = key;
    this._value = value;
    this.render();
  }

  /* eslint-disable-next-line class-methods-use-this */
  disconnectedCallback() {
    console.log('Parameter disconnectedCallback() called');
  }

  /* eslint-disable-next-line class-methods-use-this */
  adoptedCallback() {
    console.log('Parameter adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Parameter attributeChangedCallback() called');
    this.init();
    updateStateAndRender(this, 'key', '_key', name, newValue);
    updateStateAndRender(this, 'value', '_value', name, newValue);
    this.render();
  }
}

console.log('define "sf-parameter"');
customElements.define('sf-parameter', Parameter);
