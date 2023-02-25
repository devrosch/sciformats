import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'util/CustomEventsChannel';
import { setElementAttribute } from 'util/RenderUtils';
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

  #title: string | null = null;

  #key: string | null = null;

  #shortcut: string | null = null;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  constructor() {
    super();
    console.log('MenuItemFileOpen constructor() called');
  }

  init() {
    if (this.children.length !== 2
      || !(this.children.item(0) instanceof HTMLInputElement)
      || !(this.children.item(1) instanceof HTMLLabelElement)) {
      this.innerHTML = template;
    }
  }

  render() {
    this.init();
    setElementAttribute(this, 'role', 'none');

    const input = this.getElementsByTagName('input').item(0) as HTMLInputElement;
    setElementAttribute(input, 'key', `${this.#key}-input`);

    const label = this.querySelector('label') as HTMLLabelElement;
    setElementAttribute(label, 'key', `${this.#key}-input-label`);

    const menuItem = this.querySelector('label > sf-menu-item') as MenuItem;
    setElementAttribute(menuItem, 'key', `${this.#key}-${menuItemKeyPostfix}`);
    setElementAttribute(menuItem, 'title', this.#title);
    const shortcut = this.getAttribute('shortcut');
    setElementAttribute(menuItem, 'shortcut', shortcut);
  }

  onClick = (e: MouseEvent) => {
    console.log('MenuItemFileOpen clicked.');
    if (!(e.target instanceof Element)) {
      return;
    }
    const key = e?.target?.getAttribute('key');
    console.log({ key });
    if (!key) {
      return;
    }
    if (key === `${this.#key}-${menuItemKeyPostfix}`) {
      console.log('MenuItemFileOpen sf-file-open-input-menu-item clicked.');
      e.stopPropagation();
      e.preventDefault();
      const input = this.getElementsByTagName('input').item(0) as HTMLInputElement;
      input.click();
    }
    if (key === `${this.#key}-input`) {
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
    this.#title = this.getAttribute('title');
    this.#key = this.getAttribute('key');
    this.#shortcut = this.getAttribute('shortcut');
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

console.log('define "sf-menu-item-file-open"');
customElements.define('sf-menu-item-file-open', MenuItemFileOpen);
