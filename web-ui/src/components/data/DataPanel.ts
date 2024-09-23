/* eslint-disable no-duplicate-imports */
import './DataChart';
import DataChart from './DataChart';
import './DataTable';
import './DataData';
import './DataPanel.css';
import Message from 'model/Message';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'model/Channel';
import NodeData from 'model/NodeData';
import { isSameUrl } from 'util/UrlUtils';

const template = `
  <div id="sf-data-tabs" class="tabs">
    <button id="sf-data-chart-link" class="tab-link">Chart</button>
    <button id="sf-data-data-link" class="tab-link">Data</button>
    <button id="sf-data-table-link" class="tab-link">Table</button>
  </div>

  <div id="sf-data-chart-panel" class="tab-content">
    <sf-data-chart></sf-data-chart>
  </div>
  <div id="sf-data-data-panel" class="tab-content">
    <sf-data-data></sf-data-data>
  </div>
  <div id="sf-data-table-panel" class="tab-content">
    <sf-data-table></sf-data-table>
  </div>
`;

const nodeSelectedEvent = 'sf-tree-node-selected';
const nodeDeselectedEvent = 'sf-tree-node-deselected';
const nodeDataUpdatedEvent = 'sf-tree-node-data-updated';

export default class DataPanel extends HTMLElement {
  #initialized = false;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #handles: any = [];

  #url: URL | null = null;

  #populated = {
    xyData: false,
    table: false,
    parameters: false,
  };

  #active = 'chart';

  constructor() {
    super();
    console.log('DataPanel constructor() called');
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    const tabLinks = this.querySelectorAll('#sf-data-tabs > button');
    for (const link of tabLinks) {
      if (link.id === `sf-data-${this.#active}-link`) {
        link.classList.add('active');
      } else {
        link.classList.remove('active');
      }
    }

    const chartTabLink = this.querySelector('#sf-data-chart-link');
    const dataTabLink = this.querySelector('#sf-data-data-link');
    const tableTabLink = this.querySelector('#sf-data-table-link');
    if (this.#populated.xyData) {
      chartTabLink?.classList.add('populated');
      dataTabLink?.classList.add('populated');
    } else {
      chartTabLink?.classList.remove('populated');
      dataTabLink?.classList.remove('populated');
    }
    if (this.#populated.table) {
      tableTabLink?.classList.add('populated');
    } else {
      tableTabLink?.classList.remove('populated');
    }

    const panels = this.querySelectorAll('.tab-content');
    for (const panel of panels) {
      if (panel.id === `sf-data-${this.#active}-panel`) {
        panel.classList.add('active');
        // make sure that chart exactly fits available space
        const chart = panel.querySelector('sf-data-chart');
        if (chart !== null) {
          (chart as DataChart).resize();
        }
      } else {
        panel.classList.remove('active');
      }
    }
  }

  onClick = (e: MouseEvent) => {
    console.log('DataPanel item clicked.');
    e.preventDefault();
    const id = (e?.target as Element | null)?.getAttribute('id');
    console.log({ id });

    switch (id) {
      case 'sf-data-chart-link':
        this.#active = 'chart';
        this.render();
        break;
      case 'sf-data-data-link':
        this.#active = 'data';
        this.render();
        break;
      case 'sf-data-table-link':
        this.#active = 'table';
        this.render();
        break;
      default:
      // noop
    }
  };

  handleDataChanged(message: Message) {
    console.log('DataPanel handleDataChanged() called');
    const url = new URL(message.detail.url);
    const sameUrl = isSameUrl(this.#url, url);
    if (
      (!sameUrl && message.name === nodeSelectedEvent) ||
      (sameUrl && message.name === nodeDataUpdatedEvent)
    ) {
      this.#url = url;
      const data = message.detail as NodeData;
      this.#populated = {
        xyData: data.data && data.data.length > 0,
        table:
          data.table &&
          data.table.columnNames &&
          data.table.columnNames.length > 0,
        parameters: data.parameters && data.parameters.length > 0,
      };
      this.render();
    } else if (sameUrl && message.name === nodeDeselectedEvent) {
      this.#url = null;
      this.#populated = {
        xyData: false,
        table: false,
        parameters: false,
      };
      this.render();
    }
  }

  connectedCallback() {
    console.log('DataPanel connectedCallback() called');
    this.init();
    this.addEventListener('click', this.onClick);
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
    console.log('DataPanel disconnectedCallback() called');
    this.removeEventListener('click', this.onClick);
    for (const handle of this.#handles) {
      this.#channel.removeListener(handle);
    }
  }

  /* eslint-disable-next-line class-methods-use-this */
  adoptedCallback() {
    console.log('DataPanel adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('DataPanel attributeChangedCallback() called');
    this.init();
  }
}

console.log('define "sf-data-panel"');
customElements.define('sf-data-panel', DataPanel);
