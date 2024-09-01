import {
  setElementAttribute,
  setElementTextContent,
  updateStateAndRender,
} from 'util/RenderUtils';
import './MenuItem.css';

const template = `
  <a href="#">
    <span class="menu-item-name"></span>
    <span class="menu-item-shortcut"></span>
  </a>
`;

export default class MenuItem extends HTMLElement {
  static get observedAttributes() {
    return ['title', 'shortcut'];
  }

  #initialized = false;

  private _title: string | null = null;

  private _shortcut: string | null = null;

  constructor() {
    super();
    console.log('MenuItem constructor() called');
  }

  init() {
    if (!this.#initialized) {
      // add <a>
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    const a = this.getElementsByTagName('a').item(0) as HTMLAnchorElement;
    const nameSpan = a.children.item(0) as HTMLSpanElement;
    const shortcutSpan = a.children.item(1) as HTMLSpanElement;
    setElementAttribute(this, 'role', 'none');
    setElementAttribute(a, 'role', 'menuitem');
    setElementTextContent(nameSpan, this._title);
    setElementTextContent(shortcutSpan, this._shortcut);
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
    this.init();
    this._title = this.getAttribute('title');
    this._shortcut = this.getAttribute('shortcut');
    this.addEventListener('click', this.onClick);
    this.render();
  }

  disconnectedCallback() {
    console.log('MenuItem disconnectedCallback() called');
    this.removeEventListener('click', this.onClick);
  }

  /* eslint-disable-next-line class-methods-use-this */
  adoptedCallback() {
    console.log('MenuItem adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('MenuItem attributeChangedCallback() called');
    this.init();
    updateStateAndRender(this, 'title', '_title', name, newValue);
    updateStateAndRender(this, 'shortcut', '_shortcut', name, newValue);
  }
}

console.log('define "sf-menu-item"');
customElements.define('sf-menu-item', MenuItem);
