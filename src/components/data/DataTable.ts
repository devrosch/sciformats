import Channel from 'model/Channel';
import Message from 'model/Message';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import { isSameUrl } from 'util/UrlUtils';
import './DataTable.css';

const template = `
  <table>
  <thead>
    <tr>
      <th>x</th>
      <th>y</th>
    </tr>
  </thead>
  <tbody>
  </tbody>
  </table>
`;

const nodeSelectedEvent = 'sf-tree-node-selected';
const nodeDeselectedEvent = 'sf-tree-node-deselected';
const nodeDataUpdatedEvent = 'sf-tree-node-data-updated';

export default class DataTable extends HTMLElement {
  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #handles: any = [];

  #url: URL | null = null;

  #data: { x: number, y: number }[] = [];

  constructor() {
    super();
    console.log('DataTable constructor() called');
  }

  get data() {
    return this.#data;
  }

  set data(data: { x: number, y: number }[]) {
    this.#data = data;
    this.render();
  }

  init() {
    if (this.children.length !== 1
      || (this.children.item(0)?.nodeName !== 'TABLE')) {
      this.innerHTML = template;
    }
  }

  render() {
    this.init();

    const tBody = this.querySelector('table > tbody') as HTMLTableSectionElement;
    tBody.innerHTML = '';

    for (const data of this.#data) {
      const tr = document.createElement('tr');
      const tdX = document.createElement('td');
      tdX.textContent = String(data.x);
      tr.append(tdX);
      const tdY = document.createElement('td');
      tdY.textContent = String(data.y);
      tr.append(tdY);
      tBody.append(tr);
    }
  }

  handleDataChanged(message: Message) {
    console.log('DataTable handleDataChanged() called');
    const url = new URL(message.detail.url);
    const sameUrl = isSameUrl(this.#url, url);
    if (!sameUrl && message.name === nodeSelectedEvent) {
      this.#url = url;
      this.data = message.detail.data;
    } else if (sameUrl && message.name === nodeDeselectedEvent) {
      this.#url = null;
      this.data = [];
    } else if (sameUrl && message.name === nodeDataUpdatedEvent) {
      this.data = message.detail.data;
    }
  }

  connectedCallback() {
    console.log('DataTable connectedCallback() called');
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
    console.log('DataTable disconnectedCallback() called');
    for (const handle of this.#handles) {
      this.#channel.removeListener(handle);
    }
  }

  adoptedCallback() {
    console.log('DataTable adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('DataTable attributeChangedCallback() called');
  }
}

console.log('define "sf-data-table"');
customElements.define('sf-data-table', DataTable);
