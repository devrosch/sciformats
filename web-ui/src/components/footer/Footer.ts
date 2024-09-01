import { isSameUrl } from 'util/UrlUtils';
import { setElementAttribute, setElementTextContent } from 'util/RenderUtils';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'model/Channel';
import Message from 'model/Message';
import './Footer.css';

const template = `
  <span></span>
`;

export default class Footer extends HTMLElement {
  #initialized = false;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #handles: any = [];

  #url: URL | null = null;

  constructor() {
    super();
    console.log('Footer constructor() called');
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    const span = this.querySelector('span') as HTMLSpanElement;
    const url = this.#url === null ? null : this.#url.toString();
    setElementAttribute(span, 'title', url);
    setElementTextContent(span, url);
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
    this.init();
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

  /* eslint-disable-next-line class-methods-use-this */
  adoptedCallback() {
    console.log('Footer adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
    console.log('Footer attributeChangedCallback() called');
  }
}

console.log('define "sf-footer"');
customElements.define('sf-footer', Footer);
