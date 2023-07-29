import { isSameUrl } from 'util/UrlUtils';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'model/Channel';
import Message from 'model/Message';
import PeakTable from 'model/PeakTable';
import './DataPeaks.css';

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

export default class DataPeaks extends HTMLElement {
  #initialized = false;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #handles: any = [];

  #url: URL | null = null;

  #data: PeakTable | null = null;

  constructor() {
    super();
    console.log('DataPeaks constructor() called');
  }

  get data() {
    return this.#data;
  }

  set data(data: PeakTable | null) {
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
      th.textContent = column.value;
      headerTr.append(th);
    });

    const rows = this.#data.peaks;

    rows.forEach((row) => {
      const tr = document.createElement('tr');
      columns.forEach((column) => {
        const td = document.createElement('td');
        const value: string = row.has(column.key) ? row.get(column.key) as string : '';
        td.textContent = value;
        tr.append(td);
      });
      tbody.append(tr);
    });
  }

  handleDataChanged(message: Message) {
    console.log('DataPeaks handleDataChanged() called');
    const url = new URL(message.detail.url);
    const sameUrl = isSameUrl(this.#url, url);
    if (!sameUrl && message.name === nodeSelectedEvent) {
      this.#url = url;
      this.data = message.detail.peakTable;
    } else if (sameUrl && message.name === nodeDeselectedEvent) {
      this.#url = null;
      this.data = null;
    } else if (sameUrl && message.name === nodeDataUpdatedEvent) {
      this.data = message.detail.peakTable;
    }
  }

  connectedCallback() {
    console.log('DataPeaks connectedCallback() called');
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
    console.log('DataPeaks disconnectedCallback() called');
    for (const handle of this.#handles) {
      this.#channel.removeListener(handle);
    }
  }

  adoptedCallback() {
    console.log('DataPeaks adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('DataPeaks attributeChangedCallback() called');
    this.init();
  }
}

console.log('define "sf-data-peaks"');
customElements.define('sf-data-peaks', DataPeaks);
