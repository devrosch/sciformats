import { setElementAttribute } from 'util/RenderUtils';
import './Splash.css';
import Logo from 'assets/sf-ui.svg';

const template = `
<dialog>
  <div class="sf-title">
    <img src="${Logo}" class="sf-logo" alt="Logo">
    <div>
      <p>SciFormats</p>
      <p>Copyright © 2022, 2023 Robert Schiwon</p>
    </div>
  </div>
  <p class="sf-initializing">
    <span>Initializing</span>
    <span class="sf-dots">
      <span class="sf-dot">.</span>
      <span class="sf-dot">.</span>
      <span class="sf-dot">.</span>
    </span>
  </p>
</dialog>
`;

export default class Splash extends HTMLElement {
  #initialized = false;

  #open: boolean = false;

  constructor() {
    super();
    console.log('AboutDialog constructor() called');
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    const dialog = this.getElementsByTagName('dialog').item(0) as HTMLDialogElement;
    if (this.#open) {
      if (!dialog.hasAttribute('open')) {
        dialog.showModal();
      }
      setElementAttribute(this, 'open', '');
    } else {
      if (dialog.hasAttribute('open')) {
        dialog.close();
      }
      setElementAttribute(this, 'open', null);
    }
  }

  showModal(show: boolean) {
    this.#open = show;
    this.render();
  }

  // eslint-disable-next-line class-methods-use-this
  onCancel = (e: Event) => {
    console.log('Cancel dialog clicked.');
    // prevent default action of close dialog
    e.stopPropagation();
    e.preventDefault();
  };

  connectedCallback() {
    console.log('Splash connectedCallback() called');
    this.init();
    this.#open = this.hasAttribute('open');
    const dialog = this.querySelector('dialog') as HTMLDialogElement;
    dialog.addEventListener('cancel', this.onCancel);
    this.render();
  }

  disconnectedCallback() {
    console.log('AboutDialog disconnectedCallback() called');
    const dialog = this.querySelector('dialog') as HTMLDialogElement;
    dialog.removeEventListener('cancel', this.onCancel);
  }

  adoptedCallback() {
    console.log('AboutDialog adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('AboutDialog attributeChangedCallback() called');
    this.init();
  }
}

console.log('define "sf-splash');
customElements.define('sf-splash', Splash);
