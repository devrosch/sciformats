import './Parameter';
import DataRepository from 'model/DataRepository';
import StubDataRepository from 'model/StubDataRepository';

const html = `
  <h1>Heading 1</h1>
  <ul></ul>
`;

export default class ParametersPanel extends HTMLElement {
  static get observedAttributes() { return ['title']; }

  #repository = new StubDataRepository() as DataRepository;

  #title = '';

  #url: URL | null = null;

  #data : { key: string, value: string }[] = [];

  constructor(repository: DataRepository | null) {
    super();
    console.log('ParametersPanel constructor() called');
    if (repository !== null && typeof repository !== 'undefined') {
      this.#repository = repository;
    }
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

  handleParametersChanged(e: Event) {
    console.log('ParametersPanel handleParametersChanged() called');
    const ce = e as CustomEvent;
    const url = new URL(ce.detail.url);
    const sameUrl = url.toString() === this.#url?.toString();
    if (sameUrl && e.type === 'sf-tree-node-unselected') {
      this.#url = null;
      this.#data = [];
      this.render();
    } else if (!sameUrl && e.type === 'sf-tree-node-selected') {
      const data = this.#repository.read(url);
      this.#url = url;
      this.#data = data.parameters;
      this.render();
    }
  }

  connectedCallback() {
    console.log('ParametersPanel connectedCallback() called');
    window.addEventListener('sf-tree-node-selected', this.handleParametersChanged.bind(this));
    window.addEventListener('sf-tree-node-unselected', this.handleParametersChanged.bind(this));
    const title = this.hasAttribute('title') ? this.getAttribute('title') : '';
    this.#title = title === null ? '' : title;
    this.render();
  }

  disconnectedCallback() {
    console.log('ParametersPanel disconnectedCallback() called');
    window.removeEventListener('sf-tree-node-selected', this.handleParametersChanged.bind(this));
    window.removeEventListener('sf-tree-node-unselected', this.handleParametersChanged.bind(this));
  }

  adoptedCallback() {
    console.log('ParametersPanel adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
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
