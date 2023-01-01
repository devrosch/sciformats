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
          <span class="sf-expand-collapse-indicator">▸</span>&nbsp;<span id="sf-submenu-title">${this.#title}</span>
        </a>
        <div role="none">
          ${innerHtml}
        </div>
        `;
    }
  }

  render() {
    this.init();
    const role = this.hasAttribute('role') ? this.getAttribute('role') as string : '';
    const a = this.getElementsByTagName('a').item(0) as HTMLAnchorElement;
    const aKey = a.hasAttribute('key') ? a.getAttribute('key') as string : '';
    const aTitle = a.hasAttribute('title') ? a.getAttribute('title') as string : '';
    const aExpandCollapseSpan = a.querySelector('.sf-expand-collapse-indicator') as HTMLSpanElement;
    const aTitleSpan = a.querySelector('#sf-submenu-title') as HTMLSpanElement;
    const aTitleSpanKey = aTitleSpan.hasAttribute('key') ? a.getAttribute('key') as string : '';
    const expandendChar = this.#expand ? '▼' : '▶';
    if (role !== 'menu') {
      this.setAttribute('role', 'menu');
    }
    if (aKey !== this.#key) {
      a.setAttribute('key', this.#key ? this.#key : '');
    }
    if (aTitle !== this.#title) {
      a.setAttribute('title', this.#title ? this.#title : '');
    }
    if (aExpandCollapseSpan.textContent !== expandendChar) {
      aExpandCollapseSpan.textContent = expandendChar;
    }
    if (aTitleSpanKey !== this.#key) {
      aTitleSpan.setAttribute('key', this.#key ? this.#key : '');
    }
    if (aTitleSpan.textContent !== this.#title) {
      aTitleSpan.textContent = this.#title;
    }
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

  onMouseEnter(e: Event) {
    console.log(`onMouseEnter(): ${this.#key}`);
    // only take action if screen is small
    if (window.innerWidth > maxWidth) {
      e.stopPropagation();
      this.#expand = true;
      this.render();
    }
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  onMouseLeave(e: Event) {
    console.log(`onMouseLeave(): ${this.#key}`);
    // only take action if screen is small
    if (window.innerWidth > maxWidth) {
      this.#expand = false;
      this.render();
    }
  }

  onClick(e: MouseEvent) {
    console.log(`onClick(): ${this.#key}`);
    if (!(e.target instanceof Element)) {
      return;
    }
    const key = e?.target?.getAttribute('key');
    if (key === this.#key) {
      e.stopPropagation();
      e.preventDefault();
      this.#expand = !this.#expand;
      this.render();
    }
  }

  connectedCallback() {
    console.log('Submenu connectedCallback() called');
    this.#title = this.hasAttribute('title') ? this.getAttribute('title') : '';
    this.#key = this.hasAttribute('key') ? this.getAttribute('key') : '';
    this.#expand = this.hasAttribute('expand') ? this.getAttribute('key') === 'true' : false;
    this.addEventListener('mouseenter', this.onMouseEnter.bind(this));
    this.addEventListener('mouseleave', this.onMouseLeave.bind(this));
    this.addEventListener('click', this.onClick.bind(this));
    this.render();
  }

  disconnectedCallback() {
    console.log('Submenu disconnectedCallback() called');
    this.removeEventListener('mouseenter', this.onMouseEnter.bind(this));
    this.removeEventListener('mouseleave', this.onMouseLeave.bind(this));
    this.removeEventListener('click', this.onClick.bind(this));
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
