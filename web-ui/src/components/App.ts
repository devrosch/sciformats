/* eslint-disable import/no-duplicates */
import 'components/Splash';
import Splash from 'components/Splash';
import 'components/menu/Navbar';
import 'components/tree/Tree';
import Tree from 'components/tree/Tree';
import 'components/data/DataPanel';
import 'components/parameters/ParametersPanel';
import 'components/footer/Footer';
import './App.css';
import WorkerResponse from 'worker/WorkerResponse';
import { postMessage } from 'util/WorkerUtils';
import WorkerStatus from 'worker/WorkerStatus';
import ParserRepository from 'model/ParserRepository';

const template = `
  <sf-splash open></sf-splash>
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
      this.initWorker();
    }
  }

  async initWorker() {
    const worker = new Worker(new URL('worker/worker.ts', import.meta.url));
    let isWorkerInitialized = false;
    while (!isWorkerInitialized) {
      // eslint-disable-next-line no-await-in-loop
      const scanReply: WorkerResponse = await postMessage(worker, 'status', null) as WorkerResponse;
      if (scanReply.detail === WorkerStatus.Initialized) {
        isWorkerInitialized = true;
      } else {
        // eslint-disable-next-line no-await-in-loop
        await new Promise((resolve) => { setTimeout(resolve, 500); });
      }
    }
    const parserRepository = new ParserRepository(worker);
    const tree = this.querySelector('sf-tree') as Tree;
    tree.setParserRepository(parserRepository);
    const splash = this.querySelector('sf-splash') as Splash;
    splash.showModal(false);
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
