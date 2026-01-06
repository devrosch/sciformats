import { setElementAttribute } from 'util/RenderUtils';
import './AboutDialog.css';
import Logo from 'assets/sf-ui.svg';

const APP_NAME = process.env.APP_NAME;
const APP_VERSION = process.env.APP_VERSION;

const template = `
<dialog>
  <p><img src="${Logo}" class="sf-logo" alt="Logo"><span>${APP_NAME}</span><span class="sf-version">v${APP_VERSION}</span></p>
  <p>An HTML/CSS/JS web UI for visualizing scientific data read with sciformats, a library for reading scientific data formats.</p>
  <p>Copyright © 2025 Robert Schiwon</p>
  <p>Currently, the following formats are supported:
    <ul>
      <li>AnDI/AIA for Chromatographic Data (<a href="https://www.astm.org/e1947-98r22.html">ASTM E1947-98(2022)</a>, <a href="https://www.astm.org/e1948-98r22.html">ASTM E1948-98(2022)</a>)</li>
      <li>AnDI/AIA for Mass Spectrometric Data (<a href="https://www.astm.org/e2077-00r16.html">ASTM E2077-00(2016)</a>, <a href="https://www.astm.org/e2078-00r16.html">ASTM E2078-00(2016)</a>)</li>
      <li>Generalized Analytical Markup Language (<a href="https://www.gaml.org/">GAML</a>)</li>
      <li>JCAMP-DX (<a href="http://www.jcamp-dx.org/">JCAMP-DX</a>)</li>
    </ul>
  </p>
  <p>sciformats and sciformats_web are made available under the terms of the MIT license
    (license: <a href="https://github.com/devrosch/sciformats/blob/main/LICENSE.txt">MIT</a>, source code:
    <a href="https://github.com/devrosch/sciformats">GitHub</a>)
    and make use of the following third-party packages that are provided by their copyright owners under their own license terms.
  </p>
  <ul>
    <li>
      <a href="https://www.npmjs.com/package/plotly.js-dist-min">plotly.js-dist-min</a>,
      license: <a href="https://github.com/plotly/plotly.js/blob/master/LICENSE">MIT</a>,
      source code: <a href="https://github.com/plotly/plotly.js">plotly.js</a>
    </li>
    <li>
      <a href="https://crates.io/crates/wasm-bindgen">wasm-bindgen</a>,
      license: <a href="https://github.com/wasm-bindgen/wasm-bindgen/blob/main/LICENSE-MIT">MIT</a>
      or <a href="https://github.com/wasm-bindgen/wasm-bindgen/blob/main/APACHE-MIT">Apache 2.0</a>,
      source code: <a href="https://github.com/wasm-bindgen/wasm-bindgen">wasm-bindgen</a>
    </li>
    <li>
      <a href="https://crates.io/crates/web-sys">web-sys</a>,
      license: <a href="https://github.com/wasm-bindgen/wasm-bindgen/blob/main/LICENSE-MIT">MIT</a>
      or <a href="https://github.com/wasm-bindgen/wasm-bindgen/blob/main/APACHE-MIT">Apache 2.0</a>,
      source code: <a href="https://github.com/wasm-bindgen/wasm-bindgen/tree/main/crates/web-sys">web-sys</a>
    </li>
    <li>
      <a href="https://crates.io/crates/js-sys">js-sys</a>,
      license: <a href="https://github.com/wasm-bindgen/wasm-bindgen/blob/main/LICENSE-MIT">MIT</a>
      or <a href="https://github.com/wasm-bindgen/wasm-bindgen/blob/main/APACHE-MIT">Apache 2.0</a>,
      source code: <a href="https://github.com/wasm-bindgen/wasm-bindgen/tree/main/crates/js-sys">js-sys</a>
    </li>
    <li>
      <a href="https://crates.io/crates/netcdf3">netcdf3</a>,
      license: <a href="https://github.com/julienbt/netcdf3/blob/main/LICENSE-MIT">MIT</a>
      or <a href="https://github.com/julienbt/netcdf3/blob/main/LICENSE-APACHE">Apache 2.0</a>,
      source code: <a href="https://github.com/julienbt/netcdf3">netcdf3</a>
    </li>
    <li>
      <a href="https://crates.io/crates/strum">strum</a>,
      license: <a href="https://github.com/Peternator7/strum/blob/master/LICENSE">MIT</a>,
      source code: <a href="https://github.com/Peternator7/strum">strum</a>
    </li>
    <li>
      <a href="https://crates.io/crates/chrono">chrono</a>,
      license: <a href="https://github.com/chronotope/chrono/blob/main/LICENSE.txt">MIT or Apache 2.0</a>,
      source code: <a href="https://github.com/chronotope/chrono">chrono</a>
    </li>
    <li>
      <a href="https://crates.io/crates/quick-xml">quick-xml</a>,
      license: <a href="https://github.com/tafia/quick-xml/blob/master/LICENSE-MIT.md">MIT</a>,
      source code: <a href="https://github.com/tafia/quick-xml">quick-xml</a>
    </li>
    <li>
      <a href="https://crates.io/crates/base64">base64</a>,
      license: <a href="https://github.com/marshallpierce/rust-base64/blob/master/LICENSE-MIT">MIT</a>
      or <a href="https://github.com/marshallpierce/rust-base64/blob/master/LICENSE-APACHE">Apache 2.0</a>,
      source code: <a href="https://github.com/marshallpierce/rust-base64">rust-base64</a>
    </li>
    <li>
      <a href="https://crates.io/crates/regex">regex</a>,
      license: <a href="https://github.com/rust-lang/regex/blob/master/LICENSE-MIT">MIT</a>
      or <a href="https://github.com/rust-lang/regex/blob/master/LICENSE-APACHE">Apache 2.0</a>,
      source code: <a href="https://github.com/rust-lang/regex">regex</a>
    </li>
    <li>
      <a href="https://crates.io/crates/serde">serde</a>,
      license: <a href="https://github.com/serde-rs/serde/blob/master/LICENSE-MIT">MIT</a>
      or <a href="https://github.com/serde-rs/serde/blob/master/LICENSE-APACHE">Apache 2.0</a>,
      source code: <a href="https://github.com/serde-rs/serde">serde</a>
    </li>
    <li>
      <a href="https://github.com/devrosch/sciformats_serde_json">sciformats_serde_json</a>,
      license: <a href="https://github.com/devrosch/sciformats_serde_json/blob/master/LICENSE-MIT">MIT</a>
      or <a href="https://github.com/devrosch/sciformats_serde_json/blob/master/LICENSE-APACHE">Apache 2.0</a>,
      source code: <a href="https://github.com/devrosch/sciformats_serde_json">sciformats_serde_json</a>
    </li>
  </ul>
  <p>
    See the above links for details.
    Also see <a href="https://github.com/devrosch/sciformats/web-ui/package.json">package.json</a>,
    <a href="https://github.com/devrosch/sciformats/lib-js/Cargo.toml">Cargo.toml</a>, and
    <a href="https://github.com/devrosch/sciformats/lib-rs/Cargo.toml">Cargo.toml</a>
    for the above dependencies and additional dependencies for development.
  </p>
  <form method="dialog">
    <button autofocus>OK</button>
  </form>
</dialog>
`;

export default class AboutDialog extends HTMLElement {
  #initialized = false;

  #open = false;

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    // console.log(`APP_NAME: ${process.env.APP_NAME}`);
    // console.log(`APP_VERSION: ${process.env.APP_VERSION}`);
    // console.log(`process.env: ${process.env}`);
    // console.log(`npm_package_version: ${process.env.npm_package_version}`);
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

  onClick = (e: MouseEvent) => {
    // make sure default action (e.g. close dialog) takes place
    e.stopPropagation();
    if ((e?.target as Element | null)?.nodeName === 'DIALOG') {
      this.showModal(false);
    }
  };

  connectedCallback() {
    this.init();
    this.#open = this.hasAttribute('open');
    this.addEventListener('click', this.onClick);
    this.render();
  }

  disconnectedCallback() {
    this.removeEventListener('click', this.onClick);
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
  }
}

customElements.define('sf-about-dialog', AboutDialog);
