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
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'model/Channel';
import Message from 'model/Message';
import Table from 'model/Table';
import './DataTable.css';

const template = `
  <table>
    <thead>
    </thead>
    <tbody>
    </tbody>
  </table> 
`;

const nodeSelectedEvent = 'sf-tree-node-selected';
const nodeDeselectedEvent = 'sf-tree-node-deselected';
const nodeDataUpdatedEvent = 'sf-tree-node-data-updated';

export default class DataTable extends HTMLElement {
  #initialized = false;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #handles: any = [];

  #url: URL | null = null;

  #data: Table | null = null;

  constructor() {
    super();
  }

  get data() {
    return this.#data;
  }

  set data(data: Table | null) {
    this.#data = data;
    this.render();
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    const table = this.querySelector('table') as HTMLTableElement;
    const thead = table.querySelector('thead') as HTMLTableSectionElement;
    const tbody = table.querySelector('tbody') as HTMLTableSectionElement;

    // clear table
    thead.textContent = null;
    tbody.textContent = null;

    if (this.#data === null) {
      return;
    }

    const columns = this.#data.columnNames;

    const headerTr = document.createElement('tr');
    thead.append(headerTr);
    columns.forEach((column) => {
      const th = document.createElement('th');
      th.textContent = column.name;
      headerTr.append(th);
    });

    const rows = this.#data.rows;

    rows.forEach((row) => {
      const tr = document.createElement('tr');
      columns.forEach((column) => {
        const td = document.createElement('td');
        const value: string = Object.prototype.hasOwnProperty.call(
          row,
          column.key,
        )
          ? (row[column.key] as string)
          : '';
        td.textContent = value;
        tr.append(td);
      });
      tbody.append(tr);
    });
  }

  handleDataChanged(message: Message) {
    const url = new URL(message.detail.url);
    const sameUrl = isSameUrl(this.#url, url);
    if (!sameUrl && message.name === nodeSelectedEvent) {
      this.#url = url;
      this.data = message.detail.table;
    } else if (sameUrl && message.name === nodeDeselectedEvent) {
      this.#url = null;
      this.data = null;
    } else if (sameUrl && message.name === nodeDataUpdatedEvent) {
      this.data = message.detail.table;
    }
  }

  connectedCallback() {
    this.init();
    const handle0 = this.#channel.addListener(
      nodeSelectedEvent,
      this.handleDataChanged.bind(this),
    );
    const handle1 = this.#channel.addListener(
      nodeDeselectedEvent,
      this.handleDataChanged.bind(this),
    );
    const handle2 = this.#channel.addListener(
      nodeDataUpdatedEvent,
      this.handleDataChanged.bind(this),
    );
    this.#handles.push(handle0, handle1, handle2);
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

customElements.define('sf-data-table', DataTable);
