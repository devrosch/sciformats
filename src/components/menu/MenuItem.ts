const template = '<a href="#"></a>';

export default class MenuItem extends HTMLLIElement {
  static get observedAttributes() { return ['title', 'key']; }

  #title: string | null = null;

  #key: string | null = null;

  constructor() {
    super();
    console.log('MenuItem constructor() called');
  }

  render() {
    this.innerHTML = template;
    const a = this.getElementsByTagName('a').item(0) as HTMLAnchorElement;
    const aKey = a.getAttribute('key') ? a.getAttribute('key') as string : '';
    const aTitle = a.getAttribute('title') ? a.getAttribute('title') as string : '';
    if (aKey !== this.#key) {
      a.setAttribute('key', this.#key ? this.#key : '');
    }
    if (aTitle !== this.#title) {
      a.setAttribute('title', this.#title ? this.#title : '');
      a.textContent = this.#title;
    }
  }

  connectedCallback() {
    console.log('MenuItem connectedCallback() called');
    this.#title = this.getAttribute('title');
    this.#key = this.getAttribute('key');
    this.render();
  }

  disconnectedCallback() {
    console.log('MenuItem disconnectedCallback() called');
  }

  adoptedCallback() {
    console.log('MenuItem adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('MenuItem attributeChangedCallback() called');
    if (name === 'title' && this.#title !== newValue) {
      this.#title = newValue;
      this.render();
    } else if (name === 'key' && this.#key !== newValue) {
      this.#key = newValue;
      this.render();
    }
  }
}

console.log('define "sf-menu-item"');
customElements.define('sf-menu-item', MenuItem, { extends: 'li' });
