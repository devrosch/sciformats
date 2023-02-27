import { setElementAttribute } from 'util/RenderUtils';
import './AboutDialog.css';

const template = `
<dialog>
  <p>SciFormats</p>
  <p>Copyright © 2022, 2023 Robert Schiwon</p>
  <p>SciFormats is free software according to the terms of the GNU General Public License Version 3
    (license: <a href="https://gitlab.com/devrosch/sf-ui/blob/master/COPYING">GPL</a>, source code:
    <a href="https://gitlab.com/devrosch/sf-ui">GitLab</a>)
    and makes use of the following third-party package that comes with its own license terms.
  </p>
  <ul>
    <li>
      <a
        href="https://www.npmjs.com/package/plotly.js-dist-min">
        plotly.js-dist-min</a>
      (license:
      <a
        href="https://github.com/plotly/plotly.js/blob/master/LICENSE">MIT</a>,
      source code:
      <a
        href="https://github.com/plotly/plotly.js">
        GitHub</a>)
    </li>
  </ul>
  <p>
    See <a href="https://gitlab.com/devrosch/sf-ui/blob/master/package.json">package.json</a>
    for details on additional dependencies for development.
  </p>
  <form method="dialog">
    <button autofocus>OK</button>
  </form>
</dialog>
`;

export default class AboutDialog extends HTMLElement {
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
    this.init();
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
    this.init();
  }
}

console.log('define "sf-about-dialog"');
customElements.define('sf-about-dialog', AboutDialog);
