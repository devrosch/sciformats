import './Parameter';

const html = `
  <h1>Heading 1</h1>
  <ul></ul>
`;

export default class ParametersPanel extends HTMLElement {
  static get observedAttributes() { return ['test-attr']; }

  #data : { key: string, value: string }[] = [{ key: 'key1', value: 'value1' }, { key: 'key2', value: 'value2' }, { key: 'key3', value: 'value3' }];

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
    if (heading === null) {
      throw new Error('Illegal state. No "h1" found in ParametersPanel.');
    }
    heading.textContent = text;

    const ul = this.querySelector('ul');
    if (ul === null) {
      throw new Error('Illegal state. No "ul" found in ParametersPanel.');
    }
    for (const param of this.data) {
      const li = document.createElement('li');
      const parameterEl = document.createElement('sf-parameter');
      parameterEl.setAttribute('key', param.key);
      parameterEl.setAttribute('value', param.value);
      li.append(parameterEl);
      ul.appendChild(li);
    }
  }

  connectedCallback() {
    console.log('ParametersPanel connectedCallback() called');
    this.render();
  }

  disconnectedCallback() {
    console.log('ParametersPanel disconnectedCallback() called');
  }

  adoptedCallback() {
    console.log('ParametersPanel adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('ParametersPanel attributeChangedCallback() called');
  }
}

console.log('define "sf-parameters-panel"');
customElements.define('sf-parameters-panel', ParametersPanel);
