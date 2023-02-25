import { setElementAttribute } from 'util/RenderUtils';
import './MenuItem.css';

const template = `
  <a href="#">
    <span class="menu-item-name"></span>
    <span class="menu-item-shortcut"></span>
  </a>
`;

export default class MenuItem extends HTMLElement {
  static get observedAttributes() { return ['title', 'key', 'shortcut']; }

  #title: string | null = null;

  #key: string | null = null;

  #shortcut: string | null = null;

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
    const a = this.getElementsByTagName('a').item(0) as HTMLAnchorElement;
    const nameSpan = a.children.item(0) as HTMLSpanElement;
    const shortcutSpan = a.children.item(1) as HTMLSpanElement;
    setElementAttribute(this, 'role', 'none');
    setElementAttribute(a, 'role', 'menuitem');
    if (nameSpan.textContent !== this.#title) {
      nameSpan.textContent = this.#title;
    }
    if (shortcutSpan.textContent !== this.#shortcut) {
      shortcutSpan.textContent = this.#shortcut;
    }
  }

  onClick = (e: Event) => {
    if (e.target !== this) {
      // click event from child element
      e.stopPropagation();
      this.click();
    }
  };

  connectedCallback() {
    console.log('MenuItem connectedCallback() called');
    this.#title = this.getAttribute('title');
    this.#key = this.getAttribute('key');
    this.#shortcut = this.getAttribute('shortcut');
    this.addEventListener('click', this.onClick);
    this.render();
  }

  disconnectedCallback() {
    console.log('MenuItem disconnectedCallback() called');
    this.removeEventListener('click', this.onClick);
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
    } else if (name === 'shortcut' && this.#shortcut !== newValue) {
      this.#shortcut = newValue;
      this.render();
    }
  }
}

console.log('define "sf-menu-item"');
customElements.define('sf-menu-item', MenuItem);
