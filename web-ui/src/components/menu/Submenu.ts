import {
  setElementAttribute,
  setElementTextContent,
  updateStateAndRender,
} from 'util/RenderUtils';
import './Submenu.css';

/**
 * Max width for vertical menu.
 */
const maxWidth = 576;

export default class Submenu extends HTMLElement {
  static get observedAttributes() {
    return ['title', 'key', 'expand'];
  }

  #initialized = false;

  private _title: string | null = null;

  private _key: string | null = null;

  private _expand: boolean = false;

  constructor() {
    super();
    console.log('Submenu constructor() called');
  }

  init() {
    if (!this.#initialized) {
      this._title = this.getAttribute('title');
      this._key = this.getAttribute('key');
      this._expand = this.hasAttribute('expand')
        ? this.getAttribute('expand') === 'true'
        : false;

      // add <a> at beginning
      const innerHtml = this.innerHTML;
      this.innerHTML = `
        <a href="#" key="${this._key}">
          <span class="sf-expand-collapse-indicator">›</span>&nbsp;<span>${this._title}</span>
        </a>
        <div role="none">
          ${innerHtml}
        </div>
        `;

      this.#initialized = true;
    }
  }

  render() {
    const a = this.getElementsByTagName('a').item(0) as HTMLAnchorElement;
    const aIndicatorSpan = a.querySelector(':nth-child(1)') as HTMLSpanElement;
    const aTitleSpan = a.querySelector(':nth-child(2)') as HTMLSpanElement;

    setElementAttribute(this, 'role', 'menu');
    setElementAttribute(a, 'key', this._key);
    setElementAttribute(a, 'title', this._title);
    setElementAttribute(aIndicatorSpan, 'key', this._key);
    setElementAttribute(aTitleSpan, 'key', this._key);
    setElementTextContent(aTitleSpan, this._title);
    if (this._expand) {
      setElementAttribute(this, 'expand', 'true');
      this.classList.add('sf-submenu-expand');
    } else {
      setElementAttribute(this, 'expand', 'false');
      this.classList.remove('sf-submenu-expand');
      const subMenus = this.getElementsByClassName('sf-submenu-expand');
      for (const subMenu of subMenus) {
        if (
          subMenu.hasAttribute('expand') &&
          subMenu.getAttribute('expand') !== 'false'
        ) {
          subMenu.setAttribute('expand', 'false');
        }
      }
    }
  }

  onMouseEnter = (e: Event) => {
    console.log(`onMouseEnter(): ${this._key}`);
    // only take action if screen is small
    if (window.innerWidth > maxWidth) {
      e.stopPropagation();
      this._expand = true;
      this.render();
    }
  };

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  onMouseLeave = (e: Event) => {
    console.log(`onMouseLeave(): ${this._key}`);
    // only take action if screen is small
    if (window.innerWidth > maxWidth) {
      this._expand = false;
      this.render();
    }
  };

  onClick = (e: MouseEvent) => {
    console.log(`onClick(): ${this._key}`);
    const key = (e?.target as Element | null)?.getAttribute('key');
    if (key === this._key && !(e.target instanceof Submenu)) {
      e.stopPropagation();
      e.preventDefault();
      this._expand = !this._expand;
      this.click();
      this.render();
    }
  };

  connectedCallback() {
    console.log('Submenu connectedCallback() called');
    this.init();
    this._title = this.getAttribute('title');
    this._key = this.getAttribute('key');
    this._expand = this.getAttribute('expand') === 'true';
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

  /* eslint-disable-next-line class-methods-use-this */
  adoptedCallback() {
    console.log('Submenu adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log(
      'Submenu attributeChangedCallback() called',
      this._key,
      name,
      this._expand,
      oldValue,
      newValue,
      this.getAttribute('expand'),
    );
    this.init();
    updateStateAndRender(this, 'title', '_title', name, newValue);
    updateStateAndRender(this, 'key', '_key', name, newValue);
    updateStateAndRender(this, 'expand', '_expand', name, newValue === 'true');
  }
}

console.log('define "sf-submenu"');
customElements.define('sf-submenu', Submenu);
