import { setElementAttribute, setElementTextContent } from 'util/RenderUtils';
import './Submenu.css';

/**
 * Max width for vertical menu.
 */
const maxWidth = 576;

export default class Submenu extends HTMLElement {
  static get observedAttributes() { return ['title', 'key', 'expand']; }

  #title: string | null = null;

  #key: string | null = null;

  #expand: boolean = false;

  constructor() {
    super();
    console.log('Submenu constructor() called');
  }

  init() {
    if (this.children.length < 1
      || !(this.children.item(0) instanceof HTMLAnchorElement)) {
      // add <a> at beginning
      const innerHtml = this.innerHTML;
      this.innerHTML = `
        <a href="#" key="${this.#key}">
          <span class="sf-expand-collapse-indicator"></span>&nbsp;<span id="sf-submenu-title">${this.#title}</span>
        </a>
        <div role="none">
          ${innerHtml}
        </div>
        `;
    }
  }

  render() {
    this.init();
    const a = this.getElementsByTagName('a').item(0) as HTMLAnchorElement;
    const aExpandCollapseSpan = a.querySelector('.sf-expand-collapse-indicator') as HTMLSpanElement;
    const aTitleSpan = a.querySelector('#sf-submenu-title') as HTMLSpanElement;
    const expandendChar = this.#expand ? '▾' : '▸';

    setElementAttribute(this, 'role', 'menu');
    setElementAttribute(a, 'key', this.#key);
    setElementAttribute(a, 'title', this.#title);
    setElementTextContent(aExpandCollapseSpan, expandendChar);
    setElementAttribute(aTitleSpan, 'key', this.#key);
    setElementTextContent(aTitleSpan, this.#title);
    if (this.#expand) {
      this.setAttribute('expand', 'true');
      this.classList.add('sf-submenu-expand');
    } else {
      this.setAttribute('expand', 'false');
      this.classList.remove('sf-submenu-expand');
      const subMenus = this.getElementsByClassName('sf-submenu-expand');
      for (const subMenu of subMenus) {
        if (subMenu.hasAttribute('expand')
          && subMenu.getAttribute('expand') !== 'false') {
          subMenu.setAttribute('expand', 'false');
        }
      }
    }
  }

  onMouseEnter = (e: Event) => {
    console.log(`onMouseEnter(): ${this.#key}`);
    // only take action if screen is small
    if (window.innerWidth > maxWidth) {
      e.stopPropagation();
      this.#expand = true;
      this.render();
    }
  };

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  onMouseLeave = (e: Event) => {
    console.log(`onMouseLeave(): ${this.#key}`);
    // only take action if screen is small
    if (window.innerWidth > maxWidth) {
      this.#expand = false;
      this.render();
    }
  };

  onClick = (e: MouseEvent) => {
    console.log(`onClick(): ${this.#key}`);
    if (!(e.target instanceof Element)) {
      return;
    }
    const key = e?.target?.getAttribute('key');
    if (key === this.#key && !(e.target instanceof Submenu)) {
      e.stopPropagation();
      e.preventDefault();
      this.#expand = !this.#expand;
      this.click();
      this.render();
    }
  };

  connectedCallback() {
    console.log('Submenu connectedCallback() called');
    this.#title = this.hasAttribute('title') ? this.getAttribute('title') : '';
    this.#key = this.hasAttribute('key') ? this.getAttribute('key') : '';
    this.#expand = this.hasAttribute('expand') ? this.getAttribute('expand') === 'true' : false;
    this.addEventListener('mouseenter', this.onMouseEnter);
    this.addEventListener('mouseleave', this.onMouseLeave);
    this.addEventListener('click', this.onClick);
    this.render();
  }

  disconnectedCallback() {
    console.log('Submenu disconnectedCallback() called');
    this.removeEventListener('mouseenter', this.onMouseEnter);
    this.removeEventListener('mouseleave', this.onMouseLeave);
    this.removeEventListener('click', this.onClick);
  }

  adoptedCallback() {
    console.log('Submenu adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Submenu attributeChangedCallback() called');
    if (name === 'title' && this.#title !== newValue) {
      this.#title = newValue;
      this.render();
    } else if (name === 'key' && this.#key !== newValue) {
      this.#key = newValue;
      this.render();
    } else if (name === 'expand' && (newValue === 'true') !== this.#expand) {
      this.#expand = newValue === 'true';
      this.render();
    }
  }
}

console.log('define "sf-submenu"');
customElements.define('sf-submenu', Submenu);
