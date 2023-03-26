import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'util/CustomEventsChannel';
import { setElementAttribute, updateStateAndRender } from 'util/RenderUtils';
import MenuItem from './MenuItem';
import './MenuItemFileOpen.css';

const menuItemKeyPostfix = 'input-menu-item';

const template = `
  <input id="sf-file-open-input" class="sf-file-open-input" tabindex="-1" key="sf-file-open-input" type="file" multiple="true"/>
  <label for="sf-file-open-input" class="sf-file-input-label">
    <sf-menu-item key="sf-file-open-${menuItemKeyPostfix}" accesskey="o"></sf-menu-item>
  </label>  
`;

export default class MenuItemFileOpen extends HTMLElement {
  static get observedAttributes() { return ['title', 'key', 'shortcut']; }

  #initialized = false;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  private _title: string | null = null;

  private _key: string | null = null;

  private _shortcut: string | null = null;

  constructor() {
    super();
    console.log('MenuItemFileOpen constructor() called');
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    setElementAttribute(this, 'role', 'none');

    const input = this.getElementsByTagName('input').item(0) as HTMLInputElement;
    setElementAttribute(input, 'key', `${this._key}-input`);

    const label = this.querySelector('label') as HTMLLabelElement;
    setElementAttribute(label, 'key', `${this._key}-input-label`);

    const menuItem = this.querySelector('label > sf-menu-item') as MenuItem;
    setElementAttribute(menuItem, 'key', `${this._key}-${menuItemKeyPostfix}`);
    setElementAttribute(menuItem, 'title', this._title);
    setElementAttribute(menuItem, 'shortcut', this._shortcut);
  }

  onClick = (e: MouseEvent) => {
    console.log('MenuItemFileOpen clicked.');
    const key = (e?.target as Element | null)?.getAttribute('key');
    console.log({ key });
    if (!key) {
      return;
    }
    if (key === `${this._key}-${menuItemKeyPostfix}`) {
      console.log('MenuItemFileOpen sf-file-open-input-menu-item clicked.');
      e.stopPropagation();
      e.preventDefault();
      const input = this.getElementsByTagName('input').item(0) as HTMLInputElement;
      input.click();
    }
    if (key === `${this._key}-input`) {
      console.log('MenuItemFileOpen sf-file-open-input clicked.');
      e.stopPropagation();
    }
  };

  onChange = (e: Event) => {
    console.log('MenuItemFileOpen onChange().');
    const input = e.target as HTMLInputElement;
    const selectedFiles = input.files;
    if (selectedFiles === null || typeof selectedFiles === 'undefined') {
      return;
    }
    const files = Array.from(selectedFiles);
    // reset file input, see https://stackoverflow.com/questions/20549241/how-to-reset-input-type-file
    // if not reset, opening the same file again does not fire on change event => cannot be opened
    input.value = '';
    // notify other application components
    this.#channel.dispatch('sf-file-open-requested', { files });
    // notify parents, so that menu might be closed
    this.click();
  };

  connectedCallback() {
    console.log('MenuItemFileOpen connectedCallback() called');
    this.init();
    this._title = this.getAttribute('title');
    this._key = this.getAttribute('key');
    this._shortcut = this.getAttribute('shortcut');
    this.addEventListener('click', this.onClick);
    this.addEventListener('change', this.onChange);
    this.render();
  }

  disconnectedCallback() {
    console.log('MenuItemFileOpen disconnectedCallback() called');
    this.removeEventListener('click', this.onClick);
    this.removeEventListener('change', this.onChange);
  }

  adoptedCallback() {
    console.log('MenuItemFileOpen adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('MenuItemFileOpen attributeChangedCallback() called');
    this.init();
    updateStateAndRender(this, 'title', '_title', name, newValue);
    updateStateAndRender(this, 'key', '_key', name, newValue);
    updateStateAndRender(this, 'shortcut', '_shortcut', name, newValue);
  }
}

console.log('define "sf-menu-item-file-open"');
customElements.define('sf-menu-item-file-open', MenuItemFileOpen);
