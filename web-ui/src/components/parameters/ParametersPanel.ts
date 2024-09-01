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

  #data: { key: string; value: string }[] = [];

  constructor() {
    super();
    console.log('ParametersPanel constructor() called');
  }

  get data() {
    return this.#data;
  }

  set data(data: { key: string; value: string }[]) {
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

    const ulOld = this.querySelector('ul')!;
    const ulNew = document.createElement('ul');
    const div = ulOld!.parentElement;
    div!.replaceChild(ulNew, ulOld);
    // don't await the async function as it would block the main thread
    ParametersPanel.renderAsync(ulNew!, this.data);
    console.log('ParametersPanel render() completed');
  }

  static async renderAsync(
    ul: HTMLUListElement,
    parameters: { key: string; value: string }[],
  ) {
    let index = 0;
    for (const param of parameters) {
      if (ul.parentElement === null || !ul.isConnected) {
        // the ul is no longer attached to the DOM
        console.log(
          'Stopping async population of ParametersPanel as ul is detached',
        );
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
      parameterEl.setAttribute('key', param.key);
      parameterEl.setAttribute('value', param.value);
      li.append(parameterEl);
      ul.appendChild(li);
      index += 1;
    }
    console.log('ParametersPanel renderAsync() completed');
  }

  handleParametersChanged(message: Message) {
    console.log('ParametersPanel handleParametersChanged() called');
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
    console.log('ParametersPanel connectedCallback() called');
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
    console.log('ParametersPanel disconnectedCallback() called');
    for (const handle of this.#handles) {
      this.#channel.removeListener(handle);
    }
  }

  /* eslint-disable-next-line class-methods-use-this */
  adoptedCallback() {
    console.log('ParametersPanel adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('ParametersPanel attributeChangedCallback() called');
    this.init();
    updateStateAndRender(this, 'title', '_title', name, newValue);
  }
}

console.log('define "sf-parameters-panel"');
customElements.define('sf-parameters-panel', ParametersPanel);
