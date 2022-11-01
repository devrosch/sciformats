// const template = `<a href="#"></a>`;

export default class Submenu extends HTMLLIElement {
  static get observedAttributes() { return ['title', 'key']; }
  
  #title: string | null = null;
  #key: string | null = null;

  constructor() {
    super();
    console.log('Submenu constructor() called');
  }

  render() {
    // this.innerHTML = template;
    const key: string = this.getAttribute('key') ? this.getAttribute('key') as string : '' as string;
    if (this.children.length === 2
      && this.children.item(0) instanceof HTMLAnchorElement
      && this.children.item(1) instanceof HTMLUListElement) {
        // no elements to render
    } else {
      // add <a> and wrap children in <ul>
      const innerHtml = this.innerHTML;
      this.innerHTML = `
        <a href="#" key="${key}">
          <span>▸</span>&nbsp;${this.title}
        </a>
        ${innerHtml}`;
    }
    const a = this.getElementsByTagName('a').item(0) as HTMLAnchorElement;
    a.setAttribute('key', key);
    // this.textContent = 'Test Menu Item';
    a.textContent = '▸ ' + this.#title;
    // this.textContent = this.#title;
  }

  connectedCallback() {
    console.log('Submenu connectedCallback() called');
    this.#title = this.getAttribute('title');
    this.#key = this.getAttribute('key');
    this.render();
  }

  disconnectedCallback() {
    console.log('Submenu disconnectedCallback() called');
  }

  adoptedCallback() {
    console.log('Submenu adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Submenu attributeChangedCallback() called');
    if (name === 'title' && this.#title !== newValue) {
      this.#title = newValue;
      this.render();
    } else  if (name === 'key' && this.#key !== newValue) {
      this.#key = newValue;
      this.render();
    }
  }
}

console.log('define "sf-submenu"');
customElements.define('sf-submenu', Submenu, { extends: 'li' });
