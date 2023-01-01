import Channel from 'model/Channel';
import Message from 'model/Message';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import { isSameUrl } from 'util/UrlUtils';
import './Footer.css';

const template = `
  <span></span>
`;

export default class Footer extends HTMLElement {
  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #handles: any = [];

  #url: URL | null = null;

  constructor() {
    super();
    console.log('Footer constructor() called');
  }

  init() {
    if (this.children.length !== 1
      || (this.children.item(0)?.nodeName !== 'SPAN')) {
      this.innerHTML = template;
    }
  }

  render() {
    this.init();

    const span = this.querySelector('span');
    if (this.#url === null && span !== null && span.textContent !== '') {
      span.textContent = '';
      span.removeAttribute('title');
    } else if (this.#url !== null && span !== null && !isSameUrl(this.#url, span.textContent)) {
      const url = this.#url.toString();
      span.textContent = url;
      span.setAttribute('title', url);
    }
  }

  handleUrlChanged(message: Message) {
    console.log('Footer handleParametersChanged() called');
    const url = new URL(message.detail.url);
    const sameUrl = isSameUrl(this.#url, url);
    if (sameUrl && message.name === 'sf-tree-node-deselected') {
      this.#url = null;
      this.render();
    } else if (!sameUrl && message.name === 'sf-tree-node-selected') {
      this.#url = url;
      this.render();
    }
  }

  connectedCallback() {
    console.log('Footer connectedCallback() called');
    const handle0 = this.#channel.addListener('sf-tree-node-selected', this.handleUrlChanged.bind(this));
    const handle1 = this.#channel.addListener('sf-tree-node-deselected', this.handleUrlChanged.bind(this));
    this.#handles.push(handle0, handle1);
    this.render();
  }

  disconnectedCallback() {
    console.log('Footer disconnectedCallback() called');
    for (const handle of this.#handles) {
      this.#channel.removeListener(handle);
    }
  }

  adoptedCallback() {
    console.log('Footer adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Footer attributeChangedCallback() called');
  }
}

console.log('define "sf-footer"');
customElements.define('sf-footer', Footer);
