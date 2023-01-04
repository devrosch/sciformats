import './DataChart.css';

import Channel from 'model/Channel';
import Message from 'model/Message';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import { isSameUrl } from 'util/UrlUtils';
// import * as Plotly from 'plotly.js';
import * as Plotly from 'plotly.js-dist-min';

const template = '<div id="sf-data-chart-placeholder"/>';

const nodeSelectedEvent = 'sf-tree-node-selected';
const nodeDeselectedEvent = 'sf-tree-node-deselected';
const nodeDataUpdatedEvent = 'sf-tree-node-data-updated';

export default class DataChart extends HTMLElement {
  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #handles: any = [];

  #url: URL | null = null;

  #chartContainer: Plotly.Root | null = null;

  #chartState = {
    data: {
      x: [] as number[],
      y: [] as number[],
      type: 'scatter',
      mode: 'lines',
    },
    layout: {
      // no title
      // smaller margins
      margin: {
        l: 50,
        r: 20,
        b: 50,
        t: 30,
        pad: 10,
      },
      xaxis: {
        title: 'x',
        showgrid: true,
        zeroline: false,
        showline: true,
      },
      yaxis: {
        title: 'y',
        showgrid: true,
        zeroline: false,
        showline: true,
      },
    },
    config: {
      responsive: true,
      displaylogo: false,
    },
  };

  constructor() {
    super();
    console.log('DataChart constructor() called');
  }

  get data() {
    const x = this.#chartState.data.x;
    const y = this.#chartState.data.y;
    return DataChart.fromXyArrays({ x, y });
  }

  set data(data: { x: number, y: number }[]) {
    const xyData = DataChart.toXyArrays(data);
    this.#chartState.data.x = xyData.x;
    this.#chartState.data.y = xyData.y;
    this.render();
  }

  init() {
    if (this.children.length !== 1
      || (this.children.item(0)?.nodeName !== 'DIV')
      || this.#chartContainer === null) {
      // avoid initial flash of incorrectly sized chart => hide
      this.classList.add('init');

      this.innerHTML = template;
      this.#chartContainer = this.querySelector('#sf-data-chart-placeholder') as Plotly.Root;
      Plotly.newPlot(
        this.#chartContainer!,
        [this.#chartState.data] as Plotly.Data[],
        this.#chartState.layout,
        this.#chartState.config,
      );
      // initial resize to panel before 'responsive' config kicks in
      Plotly.Plots.resize(this.#chartContainer!);

      // unhide chart
      setTimeout(() => { this.classList.remove('init'); }, 100);
    }
  }

  static toXyArrays(data: { x: number, y: number }[]) {
    const xArray: number[] = [];
    const yArray: number[] = [];
    for (const xyPair of data) {
      xArray.push(xyPair.x);
      yArray.push(xyPair.y);
    }
    return { x: xArray, y: yArray };
  }

  static fromXyArrays(data: { x: number[], y: number[] }) {
    const xyArray: { x: number, y: number }[] = [];
    for (let index = 0; index < data.x.length; index += 1) {
      const x = data.x[index];
      const y = data.y[index];
      xyArray.push({ x, y });
    }
    return xyArray;
  }

  render() {
    this.init();
    const mode = this.#chartState.data.x.length > 1 ? 'lines' : 'markers';
    this.#chartState.data.mode = mode;
    Plotly.newPlot(
      this.#chartContainer!,
      [this.#chartState.data] as Plotly.Data[],
      this.#chartState.layout,
      this.#chartState.config,
    );
  }

  handleDataChanged(message: Message) {
    console.log('DataChart handleDataChanged() called');
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
    console.log('DataChart connectedCallback() called');
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
    console.log('DataChart disconnectedCallback() called');
    for (const handle of this.#handles) {
      this.#channel.removeListener(handle);
    }
  }

  adoptedCallback() {
    console.log('DataChart adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('DataChart attributeChangedCallback() called');
  }
}

console.log('define "sf-data-chart"');
customElements.define('sf-data-chart', DataChart);
