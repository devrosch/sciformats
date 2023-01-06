import './AboutDialog.css';

const template = `
<dialog>
  <p>About dialog.</p>
  <form method="dialog">
    <button autofocus>OK</button>
  </form>
</dialog>
`;

export default class AboutDialog extends HTMLElement {
  #open: boolean = false;

  constructor() {
    super();
    console.log('AboutDialog constructor() called');
  }

  init() {
    if (this.children.length !== 1
      || !(this.children.item(0) instanceof HTMLDialogElement)) {
      this.innerHTML = template;
    }
  }

  render() {
    this.init();
    const dialog = this.getElementsByTagName('dialog').item(0) as HTMLDialogElement;
    if (this.#open) {
      if (!dialog.hasAttribute('open')) {
        dialog.showModal();
      }
      if (!this.hasAttribute('open')) {
        this.setAttribute('open', '');
      }
    } else {
      if (dialog.hasAttribute('open')) {
        dialog.close();
      }
      if (this.hasAttribute('open')) {
        this.removeAttribute('open');
      }
    }
  }

  showModal(show: boolean) {
    this.#open = show;
    this.render();
  }

  onClick = (e: MouseEvent) => {
    console.log('About dialog clicked.');
    if (!(e.target instanceof Element)) {
      return;
    }
    // make sure default action (e.g. close dialog) takes place
    e.stopPropagation();
    if (e.target.nodeName === 'DIALOG') {
      this.showModal(false);
    }
  };

  connectedCallback() {
    console.log('AboutDialog connectedCallback() called');
    this.#open = this.hasAttribute('open');
    this.addEventListener('click', this.onClick);
    this.render();
  }

  disconnectedCallback() {
    console.log('AboutDialog disconnectedCallback() called');
    this.removeEventListener('click', this.onClick);
  }

  adoptedCallback() {
    console.log('AboutDialog adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('AboutDialog attributeChangedCallback() called');
  }
}

console.log('define "sf-about-dialog"');
customElements.define('sf-about-dialog', AboutDialog);
