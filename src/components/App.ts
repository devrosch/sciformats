import 'components/menu/Navbar';
import 'components/tree/Tree';
import 'components/data/DataPanel';
import 'components/parameters/ParametersPanel';
import 'components/footer/Footer';
import './App.css';
import Channel from 'model/Channel';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';

const template = `
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
  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

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

  /* eslint-disable class-methods-use-this */
  onDragEnter(e: DragEvent) {
    // see https://www.quirksmode.org/blog/archives/2009/09/the_html5_drag.html for why this is necessary
    e.stopPropagation();
    e.preventDefault();
  }

  /* eslint-disable class-methods-use-this */
  onDragOver(e: DragEvent) {
    e.stopPropagation();
    e.preventDefault();
    if (e.dataTransfer !== null) {
      e.dataTransfer.dropEffect = 'copy';
    }
  }

  onFileDropped(e: DragEvent) {
    e.stopPropagation();
    e.preventDefault();
    if (e.dataTransfer === null) {
      return;
    }
    const selectedFiles = e.dataTransfer.files as FileList;
    // filter out directories, if possible, for now
    // this is only possible with a non-standardized function
    // see: https://stackoverflow.com/questions/25016442/how-to-distinguish-if-a-file-or-folder-is-being-dragged-prior-to-it-being-droppe
    // see: https://html5-demos.appspot.com/static/dnd/all_types_of_import.html
    // see: https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/webkitGetAsEntry
    const items = e.dataTransfer.items;
    let files: File[] = [];
    for (let i = 0; i < items.length; i += 1) {
      /* eslint-disable no-extra-boolean-cast */
      if (!!items[i].webkitGetAsEntry) {
        const entry = items[i].webkitGetAsEntry(); // non-standard
        if (entry && entry.isFile) {
          files.push(selectedFiles[i]);
        }
      } else {
        // non-standard webkitGetAsEntry() not available
        // => rely on error handling when trying to read the data
        files = Array.from(selectedFiles);
        break;
      }
    }
    this.#channel.dispatch('sf-file-open-requested', { files });
  }

  connectedCallback() {
    console.log('App connectedCallback() called');
    this.addEventListener('dragenter', this.onDragEnter.bind(this));
    this.addEventListener('dragover', this.onDragOver.bind(this));
    this.addEventListener('drop', this.onFileDropped.bind(this));
    this.render();
  }

  disconnectedCallback() {
    console.log('App disconnectedCallback() called');
    this.removeEventListener('dragenter', this.onDragEnter.bind(this));
    this.removeEventListener('dragover', this.onDragOver.bind(this));
    this.removeEventListener('drop', this.onFileDropped.bind(this));
  }
}

console.log('define "sf-app"');
customElements.define('sf-app', App);
