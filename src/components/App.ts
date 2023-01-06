import 'components/menu/Navbar';
import 'components/tree/Tree';
import 'components/data/DataPanel';
import 'components/parameters/ParametersPanel';
import 'components/footer/Footer';
import './App.css';

const template = `
  <div class="header">
    <sf-navbar app-selector="sf-app"></sf-navbar>
  </div>
  <div class="content">
    <div class="tree">
      <sf-tree></sf-tree>
    </div>
    <div class="node-content">
      <div class="data">
        <sf-data-panel></sf-data-panel>
      </div>
      <div class="params">
        <sf-parameters-panel title="Parameters"></sf-parameters-panel>
      </div>
    </div>
  </div>
  <div class="footer">
    <sf-footer></sf-footer>
  </div>
`;

export default class App extends HTMLElement {
  constructor() {
    super();
    console.log('App constructor() called');
  }

  init() {
    if (this.children.length !== 3
      || !(this.children.item(0) instanceof HTMLDivElement)
      || !(this.children.item(1) instanceof HTMLDivElement)
      || !(this.children.item(2) instanceof HTMLDivElement)) {
      // init
      this.innerHTML = template;
    }
  }

  render() {
    this.init();
  }

  connectedCallback() {
    console.log('App connectedCallback() called');
    this.render();
  }

  disconnectedCallback() {
    console.log('App disconnectedCallback() called');
  }
}

console.log('define "sf-app"');
customElements.define('sf-app', App);
