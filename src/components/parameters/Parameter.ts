const template = `
  <span></span>: <span></span>
`;

export default class Parameter extends HTMLElement {
  static get observedAttributes() { return ['key', 'value']; }

  #initialized = false;

  #key = null as string | null;

  #value = null as string | null;

  constructor() {
    super();
    console.log('Parameter constructor() called');
  }

  init() {
    if (!this.#initialized) {
      // noop
      this.#initialized = true;
    }
  }

  render() {
    this.innerHTML = template;
    const spans = this.querySelectorAll('span');
    const keySpan = spans[0];
    const valueSpan = spans[1];
    keySpan.textContent = this.#key;
    valueSpan.textContent = this.#value;
  }

  connectedCallback() {
    console.log('Parameter connectedCallback() called');
    this.init();
    const key = this.getAttribute('key');
    const value = this.getAttribute('value');
    this.#key = key;
    this.#value = value;
    this.render();
  }

  disconnectedCallback() {
    console.log('Parameter disconnectedCallback() called');
  }

  adoptedCallback() {
    console.log('Parameter adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Parameter attributeChangedCallback() called');
    this.init();
    if (name === 'key') {
      this.#key = newValue;
    } else if (name === 'value') {
      this.#value = newValue;
    }
    this.render();
  }
}

console.log('define "sf-parameter"');
customElements.define('sf-parameter', Parameter);
