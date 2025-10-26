import { setElementAttribute } from 'util/RenderUtils';
import 'components/common/DancingDots';
import './Splash.css';
import Logo from 'assets/sf-ui.svg';

const APP_NAME = process.env.APP_NAME;
const APP_VERSION = process.env.APP_VERSION;

const template = `
<dialog>
  <div class="sf-title">
    <img src="${Logo}" class="sf-logo" alt="Logo">
    <div>
      <p><span>${APP_NAME}</span><span class="sf-version">${APP_VERSION}</span></p>
      <p>Copyright © 2025 Robert Schiwon</p>
    </div>
  </div>
  <p class="sf-initializing">
    <span>Initializing</span>
    <sf-dancing-dots></sf-dancing-dots>
  </p>
</dialog>
`;

export default class Splash extends HTMLElement {
  #initialized = false;

  #open = false;

  constructor() {
    super();
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    const dialog = this.getElementsByTagName('dialog').item(
      0,
    ) as HTMLDialogElement;
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

  /* eslint-disable-next-line class-methods-use-this */
  onCancel = (e: Event) => {
    // prevent default action of close dialog
    e.stopPropagation();
    e.preventDefault();
  };

  connectedCallback() {
    this.init();
    this.#open = this.hasAttribute('open');
    const dialog = this.querySelector('dialog') as HTMLDialogElement;
    dialog.addEventListener('cancel', this.onCancel);
    this.render();
  }

  disconnectedCallback() {
    const dialog = this.querySelector('dialog') as HTMLDialogElement;
    dialog.removeEventListener('cancel', this.onCancel);
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
  }
}

customElements.define('sf-splash', Splash);
