const template = `<a href="#"></a>`;

export default class MenuItem extends HTMLLIElement {
  static get observedAttributes() { return ['title']; }
  
  #title: string | null = null;

  constructor() {
    super();
    console.log('MenuItem constructor() called');
  }

  render() {
    this.innerHTML = template;
    const a = this.getElementsByTagName('a').item(0) as HTMLAnchorElement;
    const key: string = this.getAttribute('key') ? this.getAttribute('key') as string : '' as string;
    a.setAttribute('key', key);
    // this.textContent = 'Test Menu Item';
    a.textContent = this.#title;
    // this.textContent = this.#title;
  }

  connectedCallback() {
    console.log('MenuItem connectedCallback() called');
    this.#title = this.getAttribute('title');
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
    }
  }
}

console.log('define "sf-menu-item"');
customElements.define('sf-menu-item', MenuItem, { extends: 'li' });
