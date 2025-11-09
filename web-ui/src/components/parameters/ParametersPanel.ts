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
import { isSameUrl } from 'util/UrlUtils';
import './Parameter';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'model/Channel';
import Message from 'model/Message';
import './ParametersPanel.css';

// The number of parameters to append to the DOM before yielding.
const parametersBatchSize = 100;

const html = `
  <h1>Heading 1</h1>
  <div>
    <ul></ul>
  </div>
`;

const nodeSelectedEvent = 'sf-tree-node-selected';
const nodeDeselectedEvent = 'sf-tree-node-deselected';
const nodeDataUpdatedEvent = 'sf-tree-node-data-updated';

export default class ParametersPanel extends HTMLElement {
  static get observedAttributes() {
    return ['title'];
  }

  #initialized = false;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #handles: any = [];

  private _title: string | null = null;

  #url: URL | null = null;

  #data: { key?: string; value: string | boolean | number | bigint }[] = [];

  get data() {
    return this.#data;
  }

  set data(
    data: { key?: string; value: string | boolean | number | bigint }[],
  ) {
    this.#data = data;
    this.render();
  }

  init() {
    if (!this.#initialized) {
      // noop
      this.#initialized = true;
    }
  }

  render() {
    this.innerHTML = html;

    const heading = this.querySelector('h1');
    heading!.textContent = this._title === null ? '' : this._title;

    if (this.#data !== null && this.#data.length > 0) {
      heading?.classList.add('populated');
    } else {
      heading?.classList.remove('populated');
    }

    const ulOld = this.querySelector('ul')!;
    const ulNew = document.createElement('ul');
    const div = ulOld!.parentElement;
    div!.replaceChild(ulNew, ulOld);
    // don't await the async function as it would block the main thread
    ParametersPanel.renderAsync(ulNew!, this.data);
  }

  static async renderAsync(
    ul: HTMLUListElement,
    parameters: { key?: string; value: string | boolean | number | bigint }[],
  ) {
    let index = 0;
    for (const param of parameters) {
      if (ul.parentElement === null || !ul.isConnected) {
        // the ul is no longer attached to the DOM
        break;
      }
      if (index % parametersBatchSize === 0) {
        // let other events get processed after every n parameters first
        // this allows for a responsive UI in case of many parameters
        /* eslint-disable-next-line no-await-in-loop */
        await new Promise((resolve) => {
          setTimeout(resolve, 0);
        });
      }
      const li = document.createElement('li');
      const parameterEl = document.createElement('sf-parameter');
      if (typeof param.key !== 'undefined' && param.key !== null) {
        parameterEl.setAttribute('key', param.key);
      }
      parameterEl.setAttribute('value', param.value?.toString() ?? null);
      li.append(parameterEl);
      ul.appendChild(li);
      index += 1;
    }
  }

  handleParametersChanged(message: Message) {
    const url = new URL(message.detail.url);
    const sameUrl = isSameUrl(this.#url, url);
    if (sameUrl && message.name === nodeDeselectedEvent) {
      this.#url = null;
      this.data = [];
    } else if (!sameUrl && message.name === nodeSelectedEvent) {
      this.#url = url;
      this.data = message.detail.parameters;
    } else if (sameUrl && message.name === nodeDataUpdatedEvent) {
      this.data = message.detail.parameters;
    }
  }

  connectedCallback() {
    this.init();
    const handle0 = this.#channel.addListener(
      nodeSelectedEvent,
      this.handleParametersChanged.bind(this),
    );
    const handle1 = this.#channel.addListener(
      nodeDeselectedEvent,
      this.handleParametersChanged.bind(this),
    );
    const handle2 = this.#channel.addListener(
      nodeDataUpdatedEvent,
      this.handleParametersChanged.bind(this),
    );
    this.#handles.push(handle0, handle1, handle2);
    this._title = this.getAttribute('title');
    this.render();
  }

  disconnectedCallback() {
    for (const handle of this.#handles) {
      this.#channel.removeListener(handle);
    }
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
    updateStateAndRender(this, 'title', '_title', name, newValue);
  }
}

customElements.define('sf-parameters-panel', ParametersPanel);
