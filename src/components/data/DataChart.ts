import './DataChart.css';

import Channel from 'model/Channel';
import Message from 'model/Message';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import { isSameUrl } from 'util/UrlUtils';
// import * as Plotly from 'plotly.js';
import * as Plotly from 'plotly.js-dist-min';

const template = `<div id="sf-data-chart-placeholder"/>`;

export default class DataChart extends HTMLElement {
  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #handles: any = [];

  #url: URL | null = null;

  #data: { x: number, y: number }[] = [];

  #chartContainer: Plotly.Root | null = null;

  constructor() {
    super();
    console.log('DataChart constructor() called');
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
      || (this.children.item(0)?.nodeName !== 'DIV')
      || this.#chartContainer === null) {
      this.innerHTML = template;

      this.#chartContainer = this.querySelector('#sf-data-chart-placeholder') as Plotly.Root;

      const data: Plotly.Data[] = [{
        x: [],
        y: [],
        type: 'scatter',
        mode: 'lines',
      }];
  
      const layout = {
        title: 'Data',
        xaxis: {
          title: 'x',
          showgrid: true,
          zeroline: true,
          showline: true,
        },
        yaxis: {
          title: 'y',
          showgrid: true,
          zeroline: true,
          showline: true,
        }
      };
  
      const config = { responsive: true, displaylogo: false };
      Plotly.newPlot(this.#chartContainer!, data, layout, config);
      // initial resize to panel before 'responsive' config kicks in
      Plotly.Plots.resize(this.#chartContainer!);
    }
  }

  render() {
    this.init();

    const xArray: number[] = [];
    const yArray: number[] = [];
    for (const xyPair of this.#data) {
      xArray.push(xyPair.x);
      yArray.push(xyPair.y);
    }

    const mode = xArray.length > 1 ? 'lines' : 'markers';

    const data: Plotly.Data[] = [{
      x: xArray,
      y: yArray,
      type: 'scatter',
      mode,
    }];

    Plotly.react(this.#chartContainer!, data);
  }

  handleDataChanged(message: Message) {
    console.log('DataChart handleDataChanged() called');
    const url = new URL(message.detail.url);
    const sameUrl = isSameUrl(this.#url, url);
    if (sameUrl && message.name === 'sf-tree-node-deselected') {
      this.#url = null;
      this.data = [];
    } else if (!sameUrl && message.name === 'sf-tree-node-selected') {
      this.#url = url;
      this.data = message.detail.data;
    }
  }

  connectedCallback() {
    console.log('DataChart connectedCallback() called');
    const handle0 = this.#channel.addListener('sf-tree-node-selected', this.handleDataChanged.bind(this));
    const handle1 = this.#channel.addListener('sf-tree-node-deselected', this.handleDataChanged.bind(this));
    this.#handles.push(handle0, handle1);
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
