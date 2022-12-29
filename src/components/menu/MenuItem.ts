const template = '<a href="#"></a>';

export default class MenuItem extends HTMLElement {
  static get observedAttributes() { return ['title', 'key']; }

  #title: string | null = null;

  #key: string | null = null;

  constructor() {
    super();
    console.log('MenuItem constructor() called');
  }

  init() {
    if (this.children.length !== 1
      || !(this.children.item(0) instanceof HTMLAnchorElement)) {
      // add <a>
      this.innerHTML = template;
    }
  }

  render() {
    this.init();
    const role = this.hasAttribute('role') ? this.getAttribute('role') : '';
    const a = this.getElementsByTagName('a').item(0) as HTMLAnchorElement;
    const aKey = a.hasAttribute('key') ? a.getAttribute('key') as string : '';
    const aTitle = a.hasAttribute('title') ? a.getAttribute('title') as string : '';
    const aRole = a.hasAttribute('role') ? a.getAttribute('role') as string : '';
    if (role !== 'none') {
      this.setAttribute('role', 'none');
    }
    if (aKey !== this.#key) {
      a.setAttribute('key', this.#key ? this.#key : '');
    }
    if (aTitle !== this.#title) {
      a.setAttribute('title', this.#title ? this.#title : '');
      a.textContent = this.#title;
    }
    if (aRole !== 'menuitem') {
      a.setAttribute('role', 'menuitem');
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
customElements.define('sf-menu-item', MenuItem);
