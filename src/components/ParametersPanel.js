import {} from './Parameter';

const html = `
  <h1>Heading 1</h1>
  <ul></ul>
`

export default class ParametersPanel extends HTMLElement {
  static get observedAttributes() { return ['test-attr']; }

  #data = [{ key: 'key1', value: 'value1' }, { key: 'key2', value: 'value2' }, { key: 'key3', value: 'value3' }];

  constructor() {
    super();
    console.log('ParametersPanel constructor() called');
  }

  get data() {
    return this.#data;
  }

  set data(data) {
    this.#data = data;
    this.render();
  }

  render() {
    this.innerHTML = html;

    const text = this.hasAttribute('title') ? this.getAttribute('title') : '';
    const heading = document.querySelector('h1');
    heading.textContent = text;

    const ul = document.querySelector('ul');
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

  attributeChangedCallback(name, oldValue, newValue) {
    console.log('ParametersPanel attributeChangedCallback() called');
  }

}

customElements.define("sf-parameters-panel", ParametersPanel);
