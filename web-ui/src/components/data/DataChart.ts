import { isSameUrl } from 'util/UrlUtils';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'model/Channel';
import Message from 'model/Message';
// import * as Plotly from 'plotly.js';
import * as Plotly from 'plotly.js-dist-min';
import './DataChart.css';

const template = '<div id="sf-data-chart-placeholder"/>';

const nodeSelectedEvent = 'sf-tree-node-selected';
const nodeDeselectedEvent = 'sf-tree-node-deselected';
const nodeDataUpdatedEvent = 'sf-tree-node-data-updated';

export default class DataChart extends HTMLElement {
  #initialized = false;

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
      line: {
        width: 1,
      },
    },
    layout: {
      // no title
      // smaller margins
      margin: {
        l: 80,
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
    const xyData = DataChart.fromXyArrays({ x, y });
    // xaxis/yaxis.title properties do not return a string but an object with a text property
    const xTitle = (this.#chartState.layout.xaxis.title as any).text as string;
    const yTitle = (this.#chartState.layout.yaxis.title as any).text as string;
    return {
      xyData,
      metadata: {
        xTitle,
        yTitle,
      },
    };
  }

  set data(
    data: {
      xyData: { x: number, y: number }[],
      metadata: { xTitle: string, yTitle: string },
    },
  ) {
    const xyData = DataChart.toXyArrays(data.xyData);
    this.#chartState.data.x = xyData.x;
    this.#chartState.data.y = xyData.y;
    this.#chartState.layout.xaxis.title = data.metadata.xTitle;
    this.#chartState.layout.yaxis.title = data.metadata.yTitle;
    this.render();
  }

  init() {
    if (!this.#initialized) {
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
      this.resize();

      this.#initialized = true;

      // unhide chart
      setTimeout(() => { this.classList.remove('init'); }, 100);
    }
  }

  resize() {
    Plotly.Plots.resize(this.#chartContainer!);
  }

  static toXyArrays(data: { x: number, y: number }[]) {
    const xArray: number[] = [];
    const yArray: number[] = [];
    if (data !== null && typeof data !== 'undefined') {
      for (const xyPair of data) {
        xArray.push(xyPair.x);
        yArray.push(xyPair.y);
      }
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
    const mode = this.#chartState.data.x.length > 1 ? 'lines' : 'markers';
    this.#chartState.data.mode = mode;
    Plotly.newPlot(
      this.#chartContainer!,
      [this.#chartState.data] as Plotly.Data[],
      this.#chartState.layout,
      this.#chartState.config,
    );
  }

  static #extractAxisTitles(metadata: { [key: string]: string }) {
    if (metadata === null || typeof metadata === 'undefined') {
      return { xTitle: '', yTitle: '' };
    }

    const xLabel = Object.prototype.hasOwnProperty.call(metadata, 'x.label')
      ? metadata['x.label'] : null;
    const xUnit = Object.prototype.hasOwnProperty.call(metadata, 'x.unit')
      ? metadata['x.unit'] : null;
    const yLabel = Object.prototype.hasOwnProperty.call(metadata, 'y.label')
      ? metadata['y.label'] : null;
    const yUnit = Object.prototype.hasOwnProperty.call(metadata, 'y.unit')
      ? metadata['y.unit'] : null;

    let xTitle = '';
    if (xLabel !== null && xUnit !== null) {
      xTitle = `${xLabel} / ${xUnit}`;
    } else if (xLabel === null && xUnit !== null) {
      xTitle = `${xUnit}`;
    } else if (xLabel !== null && xUnit === null) {
      xTitle = `${xLabel}`;
    }

    let yTitle = '';
    if (yLabel !== null && yUnit !== null) {
      yTitle = `${yLabel} / ${yUnit}`;
    } else if (yLabel === null && yUnit !== null) {
      yTitle = `${yUnit}`;
    } else if (yLabel !== null && yUnit === null) {
      yTitle = `${yLabel}`;
    }

    return { xTitle, yTitle };
  }

  handleDataChanged(message: Message) {
    console.log('DataChart handleDataChanged() called');
    const url = new URL(message.detail.url);
    const sameUrl = isSameUrl(this.#url, url);
    if ((!sameUrl && message.name === nodeSelectedEvent)
      || (sameUrl && message.name === nodeDataUpdatedEvent)) {
      this.#url = url;
      this.data = {
        xyData: message.detail.data,
        metadata: DataChart.#extractAxisTitles(message.detail.metadata),
      };
    } else if (sameUrl && message.name === nodeDeselectedEvent) {
      this.#url = null;
      this.data = {
        xyData: [],
        metadata: DataChart.#extractAxisTitles({}),
      };
    }
  }

  connectedCallback() {
    console.log('DataChart connectedCallback() called');
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
    this.init();
  }
}

console.log('define "sf-data-chart"');
customElements.define('sf-data-chart', DataChart);
