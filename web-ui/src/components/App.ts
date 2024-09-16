/* eslint-disable no-duplicate-imports */
import { initWorkerCpp, initWorkerRs } from 'util/WorkerUtils';
import 'components/Splash';
import Splash from 'components/Splash';
import 'components/menu/Navbar';
import Navbar from 'components/menu/Navbar';
import 'components/tree/Tree';
import Tree from 'components/tree/Tree';
import 'components/data/DataPanel';
import 'components/parameters/ParametersPanel';
import 'components/footer/Footer';
import 'components/Dialog';
import Dialog from 'components/Dialog';
import LocalParserRepository from 'model/LocalParserRepository';
import './App.css';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'model/Channel';
import Message from 'model/Message';

const template = `
  <sf-splash open></sf-splash>
  <sf-dialog></sf-dialog>
  <div class="header">
    <sf-navbar></sf-navbar>
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

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #eventListeners: any[] = [];

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
    const workerCpp = await initWorkerCpp();
    const workerRs = await initWorkerRs();
    const parserRepository = new LocalParserRepository([workerCpp, workerRs]);
    const tree = this.querySelector('sf-tree') as Tree;
    tree.setParserRepository(parserRepository);
    const splash = this.querySelector('sf-splash') as Splash;
    splash.showModal(false);
    const navbar = this.querySelector('sf-navbar') as Navbar;
    navbar.activateDragAndDrop(this);
    navbar.activateShortcuts();
  }

  /* eslint-disable-next-line class-methods-use-this */
  render() {
    // noop
  }

  /* eslint-disable class-methods-use-this */
  onDragStart = (e: DragEvent) => {
    // prevent UI elements from being draggable
    e.preventDefault();
    return false;
  };

  handleFileExportRequested(message: Message) {
    console.log(
      `App::handleFileExportRequested() -> ${message.name}: ${message.detail}`,
    );
    console.log('File export currently not supported.');
    const dialog = this.querySelector('sf-dialog') as Dialog;
    dialog.showMessage('File export currently not supported.');
  }

  connectedCallback() {
    console.log('App connectedCallback() called');
    this.init();
    this.addEventListener('dragstart', this.onDragStart);

    const fileExportHandle = this.#channel.addListener(
      'sf-file-export-requested',
      this.handleFileExportRequested.bind(this),
    );
    this.#eventListeners.push(fileExportHandle);

    this.render();
  }

  disconnectedCallback() {
    console.log('App disconnectedCallback() called');
    this.removeEventListener('dragstart', this.onDragStart);
    for (const handle of this.#eventListeners) {
      this.#channel.removeListener(handle);
    }
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('TreeNode attributeChangedCallback() called');
    this.init();
  }
}

console.log('define "sf-app"');
customElements.define('sf-app', App);
