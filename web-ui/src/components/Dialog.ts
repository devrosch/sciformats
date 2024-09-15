import { setElementAttribute } from 'util/RenderUtils';
import './Dialog.css';

export default class Dialog extends HTMLElement {
  #initialized = false;
  #open: boolean = false;
  #message: string = '';

  constructor() {
    super();
    console.log('Dialog constructor() called');
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = `
        <dialog>
          <p></p>
          <form method="dialog">
            <button autofocus>OK</button>
          </form>
        </dialog>
        `;
      this.#initialized = true;
    }
  }

  render() {
    const dialog = this.getElementsByTagName('dialog').item(
      0,
    ) as HTMLDialogElement;
    const p = dialog.querySelector('p')!;
    p.textContent = this.#message;

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

  showMessage(message: string) {
    this.#message = message;
    this.showModal(true);
  }

  // eslint-disable-next-line class-methods-use-this
  onCancel = (e: Event) => {
    console.log('Cancel dialog clicked.');
    // prevent default action of close dialog
    e.stopPropagation();
    e.preventDefault();
  };

  handleOutsideSelection = (e: MouseEvent) => {
    console.log('handleOutsideSelection() called');
    const node = e.target as Node;
    if (node === this.querySelector('dialog')) {
      // close whenever click ouside window occured
      this.#message = '';
      this.showModal(false);
    }
  };

  connectedCallback() {
    console.log('Splash connectedCallback() called');
    this.init();
    this.#open = this.hasAttribute('open');
    const dialog = this.querySelector('dialog') as HTMLDialogElement;
    dialog.addEventListener('cancel', this.onCancel);
    document.addEventListener('click', this.handleOutsideSelection);
    this.render();
  }

  disconnectedCallback() {
    console.log('Dialog disconnectedCallback() called');
    const dialog = this.querySelector('dialog') as HTMLDialogElement;
    dialog.removeEventListener('cancel', this.onCancel);
    document.removeEventListener('click', this.handleOutsideSelection);
  }

  /* eslint-disable-next-line class-methods-use-this */
  adoptedCallback() {
    console.log('Dialog adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Dialog attributeChangedCallback() called');
    this.init();
  }
}

console.log('define "sf-dialog');
customElements.define('sf-dialog', Dialog);
