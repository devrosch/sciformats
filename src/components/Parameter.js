const template = `
  <span></span>: <span></span>
`

export default class Parameter extends HTMLElement {
  static get observedAttributes() { return ['key', 'value']; }

  #data = { key: null, value: null };

  constructor() {
    super();
    console.log('Parameter constructor() called');
  }

  render() {
    this.innerHTML = template;
    const spans = this.querySelectorAll('span');
    const keySpan = spans[0];
    const valueSpan = spans[1];
    keySpan.textContent = this.#data.key;
    valueSpan.textContent = this.#data.value;
  }

  connectedCallback() {
    console.log('Parameter connectedCallback() called');
    const key = this.getAttribute('key');
    const value = this.getAttribute('value');
    this.#data.key = key;
    this.#data.value = value;
    this.render();
  }

  disconnectedCallback() {
    console.log('Parameter disconnectedCallback() called');
  }

  adoptedCallback() {
    console.log('Parameter adoptedCallback() called');
  }

  attributeChangedCallback(name, oldValue, newValue) {
    console.log('Parameter attributeChangedCallback() called');
    if (name === 'key') {
      this.#data.key = newValue;
    }
    else if (name === 'value') {
      this.#data.value = newValue;
    }
    this.render();
  }

}
