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

import { setElementAttribute } from 'util/RenderUtils';
import './Dialog.css';

export default class Dialog extends HTMLElement {
  #initialized = false;
  #open = false;
  #message = '';

  constructor() {
    super();
    console.log('Dialog constructor() called');
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = `
        <dialog>
          <div class="dialog-content">
            <p></p>
            <form method="dialog">
              <button autofocus>OK</button>
            </form>
          </div>
        </dialog>
        `;
      this.#initialized = true;
    }
  }

  render() {
    const dialog = this.getElementsByTagName('dialog').item(
      0,
    ) as HTMLDialogElement;
    const p = dialog.querySelector('p')!;
    p.textContent = this.#message;

    if (this.#open) {
      if (!dialog.hasAttribute('open')) {
        dialog.showModal();
      }
      setElementAttribute(this, 'open', '');
    } else {
      if (dialog.hasAttribute('open')) {
        dialog.close();
      }
      setElementAttribute(this, 'open', null);
    }
  }

  showModal(show: boolean) {
    this.#open = show;
    this.render();
  }

  showMessage(message: string) {
    this.#message = message;
    this.showModal(true);
  }

  handleOutsideSelection = (e: MouseEvent) => {
    console.log('handleOutsideSelection() called');
    const node = e.target as Node;
    if (node === this.querySelector('dialog')) {
      // close whenever click ouside window occured
      this.#message = '';
      this.showModal(false);
    }
  };

  connectedCallback() {
    console.log('Splash connectedCallback() called');
    this.init();
    this.#open = this.hasAttribute('open');
    document.addEventListener('click', this.handleOutsideSelection);
    this.render();
  }

  disconnectedCallback() {
    console.log('Dialog disconnectedCallback() called');
    document.removeEventListener('click', this.handleOutsideSelection);
  }

  /* eslint-disable-next-line class-methods-use-this */
  adoptedCallback() {
    console.log('Dialog adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Dialog attributeChangedCallback() called');
    this.init();
  }
}

console.log('define "sf-dialog');
customElements.define('sf-dialog', Dialog);
