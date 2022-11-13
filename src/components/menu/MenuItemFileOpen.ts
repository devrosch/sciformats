import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'util/CustomEventsChannel';
import './MenuItemFileOpen.css';

const template = `
  <input id="sf-file-open-input" class="sf-file-open-input" key="sf-file-open-input" type="file" multiple="true"/>
  <label for="sf-file-open-input" class="sf-file-input-label" accessKey="o">
    <a href="#" key="sf-file-open-input-a"></a>
  </label>  
`;

export default class MenuItemFileOpen extends HTMLLIElement {
  static get observedAttributes() { return ['title', 'key']; }

  #title: string | null = null;

  #key: string | null = null;

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
    const input = this.getElementsByTagName('input').item(0) as HTMLInputElement;
    const inputKey = input.getAttribute('key') ? input.getAttribute('key') as string : '';
    if (inputKey !== `${this.#key}-input`) {
      input.setAttribute('key', inputKey);
    }

    const labelA = this.querySelector('label > a') as HTMLLabelElement;
    const labelText = labelA.textContent;
    if (labelText !== this.#title) {
      labelA.textContent = this.#title;
    }
  }

  onClick(e: MouseEvent) {
    console.log('MenuItemFileOpen clicked.');
    if (!(e.target instanceof Element)) {
      return;
    }
    // e.stopPropagation();
    const key = e?.target?.getAttribute('key');
    console.log({ key });
    if (!key) {
      return;
    }
    if (key === 'sf-file-open-input-a') {
      console.log('MenuItemFileOpen sf-file-open-input-a clicked.');
      e.stopPropagation();
      const input = this.getElementsByTagName('input').item(0) as HTMLInputElement;
      input.click();
    }
    if (key === 'sf-file-open-input') {
      console.log('MenuItemFileOpen sf-file-open-input clicked.');
      e.stopPropagation();
    }
  }

  onChange(e: Event) {
    console.log('MenuItemFileOpen onChange().');
    const input = e.target as HTMLInputElement;
    const selectedFiles = input.files;
    if (selectedFiles === null || typeof selectedFiles === 'undefined') {
      return;
    }
    const files = [];
    for (let i = 0; i < selectedFiles.length; i += 1) {
      files.push(selectedFiles[i]);
    }
    // reset file input, see https://stackoverflow.com/questions/20549241/how-to-reset-input-type-file
    // if not reset, opening the same file again does not fire on change event => cannot be opened
    input.value = '';
    // notify other application components
    this.#channel.dispatch('sf-files-opened', { files });
    // notify parents, so that menu might be closed
    this.click();
  }

  connectedCallback() {
    console.log('MenuItemFileOpen connectedCallback() called');
    this.#title = this.getAttribute('title');
    this.#key = this.getAttribute('key');
    this.addEventListener('click', this.onClick.bind(this));
    this.addEventListener('change', this.onChange.bind(this));
    this.render();
  }

  disconnectedCallback() {
    console.log('MenuItemFileOpen disconnectedCallback() called');
    this.removeEventListener('click', this.onClick.bind(this));
    this.removeEventListener('change', this.onChange.bind(this));
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
    }
  }
}

console.log('define "sf-menu-item-file-open"');
customElements.define('sf-menu-item-file-open', MenuItemFileOpen, { extends: 'li' });
