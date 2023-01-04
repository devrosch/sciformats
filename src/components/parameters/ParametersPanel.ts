import './Parameter';
import Channel from 'model/Channel';
import Message from 'model/Message';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import { isSameUrl } from 'util/UrlUtils';
import './ParametersPanel.css';

const html = `
  <h1>Heading 1</h1>
  <ul></ul>
`;

const nodeSelectedEvent = 'sf-tree-node-selected';
const nodeDeselectedEvent = 'sf-tree-node-deselected';
const nodeDataUpdatedEvent = 'sf-tree-node-data-updated';

export default class ParametersPanel extends HTMLElement {
  static get observedAttributes() { return ['title']; }

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #handles: any = [];

  #title = '';

  #url: URL | null = null;

  #data : { key: string, value: string }[] = [];

  constructor() {
    super();
    console.log('ParametersPanel constructor() called');
  }

  get data() {
    return this.#data;
  }

  set data(data: { key: string, value: string }[]) {
    this.#data = data;
    this.render();
  }

  render() {
    this.innerHTML = html;

    const text = this.hasAttribute('title') ? this.getAttribute('title') : '';
    const heading = this.querySelector('h1');
    heading!.textContent = text;

    const ul = this.querySelector('ul');
    for (const param of this.data) {
      const li = document.createElement('li');
      const parameterEl = document.createElement('sf-parameter');
      parameterEl.setAttribute('key', param.key);
      parameterEl.setAttribute('value', param.value);
      li.append(parameterEl);
      ul!.appendChild(li);
    }
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
    const title = this.hasAttribute('title') ? this.getAttribute('title') : '';
    this.#title = title === null ? '' : title;
    this.render();
  }

  disconnectedCallback() {
    console.log('ParametersPanel disconnectedCallback() called');
    for (const handle of this.#handles) {
      this.#channel.removeListener(handle);
    }
  }

  adoptedCallback() {
    console.log('ParametersPanel adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('ParametersPanel attributeChangedCallback() called');
    if (name === 'title' && this.#title !== newValue) {
      this.#title = newValue;
      this.render();
    }
  }
}

console.log('define "sf-parameters-panel"');
customElements.define('sf-parameters-panel', ParametersPanel);
