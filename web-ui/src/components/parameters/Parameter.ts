import { updateStateAndRender } from 'util/RenderUtils';

const keyValueTemplate = '<span></span>: <span></span>';
const valueOnlyTemplate = '<span></span>';

export default class Parameter extends HTMLElement {
  static get observedAttributes() { return ['key', 'value']; }

  #initialized = false;

  private _key = null as string | null;

  private _value = null as string | null;

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
    if (this._key === '') {
      this.innerHTML = valueOnlyTemplate;
      const valueSpan = this.querySelector('span');
      valueSpan!.textContent = this._value;
    } else {
      this.innerHTML = keyValueTemplate;
      const spans = this.querySelectorAll('span');
      const keySpan = spans[0];
      const valueSpan = spans[1];
      keySpan.textContent = this._key === null ? '' : this._key;
      valueSpan.textContent = this._value === null ? '' : this._value;
    }
  }

  connectedCallback() {
    console.log('Parameter connectedCallback() called');
    this.init();
    const key = this.getAttribute('key');
    const value = this.getAttribute('value');
    this._key = key;
    this._value = value;
    this.render();
  }

  /* eslint-disable-next-line class-methods-use-this */
  disconnectedCallback() {
    console.log('Parameter disconnectedCallback() called');
  }

  /* eslint-disable-next-line class-methods-use-this */
  adoptedCallback() {
    console.log('Parameter adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Parameter attributeChangedCallback() called');
    this.init();
    updateStateAndRender(this, 'key', '_key', name, newValue);
    updateStateAndRender(this, 'value', '_value', name, newValue);
    this.render();
  }
}

console.log('define "sf-parameter"');
customElements.define('sf-parameter', Parameter);
