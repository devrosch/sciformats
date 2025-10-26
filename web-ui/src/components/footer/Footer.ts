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

import { isSameUrl } from 'util/UrlUtils';
import { setElementAttribute, setElementTextContent } from 'util/RenderUtils';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'model/Channel';
import Message from 'model/Message';
import './Footer.css';

const template = `
  <span></span>
`;

export default class Footer extends HTMLElement {
  #initialized = false;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #handles: any = [];

  #url: URL | null = null;

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    const span = this.querySelector('span') as HTMLSpanElement;
    const url = this.#url === null ? null : this.#url.toString();
    setElementAttribute(span, 'title', url);
    setElementTextContent(span, url);
  }

  handleUrlChanged(message: Message) {
    const url = new URL(message.detail.url);
    const sameUrl = isSameUrl(this.#url, url);
    if (sameUrl && message.name === 'sf-tree-node-deselected') {
      this.#url = null;
      this.render();
    } else if (!sameUrl && message.name === 'sf-tree-node-selected') {
      this.#url = url;
      this.render();
    }
  }

  connectedCallback() {
    this.init();
    const handle0 = this.#channel.addListener(
      'sf-tree-node-selected',
      this.handleUrlChanged.bind(this),
    );
    const handle1 = this.#channel.addListener(
      'sf-tree-node-deselected',
      this.handleUrlChanged.bind(this),
    );
    this.#handles.push(handle0, handle1);
    this.render();
  }

  disconnectedCallback() {
    for (const handle of this.#handles) {
      this.#channel.removeListener(handle);
    }
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
  }
}

customElements.define('sf-footer', Footer);
