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
  #initialized = false;

  constructor() {
    super();
    console.log('App constructor() called');
  }

  init() {
    if (!this.#initialized) {
      // init
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  // eslint-disable-next-line class-methods-use-this
  render() {
    // noop
  }

  /* eslint-disable class-methods-use-this */
  onDragStart = (e: DragEvent) => {
    // prevent UI elements from being draggable
    e.preventDefault();
    return false;
  };

  connectedCallback() {
    console.log('App connectedCallback() called');
    this.init();
    this.addEventListener('dragstart', this.onDragStart);
    this.render();
  }

  disconnectedCallback() {
    console.log('App disconnectedCallback() called');
    this.removeEventListener('dragstart', this.onDragStart);
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('TreeNode attributeChangedCallback() called');
    this.init();
  }
}

console.log('define "sf-app"');
customElements.define('sf-app', App);
