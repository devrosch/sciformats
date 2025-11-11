/**
 * Copyright (c) 2025 Robert Schiwon
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/* eslint-disable no-duplicate-imports */
import { initWorkerRs } from 'util/WorkerUtils';
import 'components/dialogs/Splash';
import Splash from 'components/dialogs/Splash';
import 'components/menu/Navbar';
import Navbar from 'components/menu/Navbar';
import 'components/tree/Tree';
import Tree from 'components/tree/Tree';
import 'components/data/DataPanel';
import 'components/parameters/ParametersPanel';
import 'components/footer/Footer';
import 'components/dialogs/Dialog';
import Dialog from 'components/dialogs/Dialog';
import LocalParserRepository from 'model/LocalParserRepository';
import './App.css';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'model/Channel';
import Message from 'model/Message';
import ParserRepository from 'model/ParserRepository';
import ErrorParser from 'model/ErrorParser';
import 'components/dialogs/AboutDialog'; // for side effects
import AboutDialog from 'components/dialogs/AboutDialog';
import { saveFile } from 'util/FileUtils';
import { extractFilename } from 'util/UrlUtils';

const template = `
  <sf-splash open></sf-splash>
  <sf-dialog></sf-dialog>
  <sf-about-dialog></sf-about-dialog>
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

  #channelEventListeners: any[] = [];

  #parserRepository: ParserRepository | null = null;

  init() {
    if (!this.#initialized) {
      // init
      this.innerHTML = template;
      this.#initialized = true;
      this.initWorker();
    }
  }

  async initWorker() {
    const workerRs = await initWorkerRs();
    const parserRepository = new LocalParserRepository([workerRs]);
    this.#parserRepository = parserRepository;
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

  async handleFilesOpenRequested(message: Message) {
    const files = message.detail.files as File[];
    // find parsers
    for (const file of files) {
      // find parser
      let parser = null;
      try {
        /* eslint-disable-next-line no-await-in-loop */
        parser = await this.#parserRepository!.findParser(file);
      } catch (error: any) {
        const detail = error.detail ? error.detail : error;
        const warningMessage = `No parser found for file: "${file.name}". ${detail}`;
        this.#channel.dispatch('sf-warning', warningMessage);
        console.warn(warningMessage);
      }

      if (parser !== null) {
        // open file
        const tree = this.querySelector('.content .tree sf-tree') as Tree;
        try {
          /* eslint-disable-next-line no-await-in-loop */
          await parser.open();
          tree.addRootNode(parser);
          this.#channel.dispatch('sf-file-opened', { url: parser.rootUrl });
        } catch (error: any) {
          const detail = error.detail ? error.detail : error;
          const errorMessage = `Error opening file "${file.name}": ${detail}`;
          this.#channel.dispatch('sf-error', errorMessage);
          console.error(errorMessage);
          // show node with error in tree
          const errorParser = new ErrorParser(parser.rootUrl, errorMessage);
          tree.addRootNode(errorParser);
        }
      }
    }
  }

  async handleFileExportRequested(/* message: Message */) {
    const tree = this.querySelector('.content .tree sf-tree') as Tree;
    const parser = tree.getSelectedNodeParser();
    const rootUrl = parser?.rootUrl;
    const fileName = rootUrl ? extractFilename(rootUrl) : '';
    if (parser === null) {
      const dialog = this.querySelector('sf-dialog') as Dialog;
      dialog.showMessage('No node selected.');
      return;
    }

    try {
      const blob = await parser.export('Json');
      // for export file replace extension with ".json" or add ".json" if no extension
      const originalFileName = extractFilename(parser.rootUrl);
      const pos = originalFileName.lastIndexOf('.');
      const exportFileName =
        originalFileName.substring(0, pos < 0 ? originalFileName.length : pos) +
        '.json';
      // save/download export
      saveFile(exportFileName, blob);
      this.#channel.dispatch('sf-file-exported', { url: rootUrl });
    } catch (error: any) {
      const detail = error.detail ? error.detail : error;
      const errorMessage = `Error exporting file "${fileName}": ${detail}`;
      this.#channel.dispatch('sf-error', errorMessage);
      const dialog = this.querySelector('sf-dialog') as Dialog;
      dialog.showMessage(errorMessage);
    }
  }

  handleFileCloseRequested() {
    const tree = this.querySelector('.content .tree sf-tree') as Tree;
    const url = tree.removeSelectedNode();
    if (url !== null) {
      this.#channel.dispatch('sf-file-closed', { url });
    }
  }

  handleFileCloseAllRequested() {
    const tree = this.querySelector('.content .tree sf-tree') as Tree;
    const urls = tree.removeAllNodes();
    for (const url of urls) {
      this.#channel.dispatch('sf-file-closed', { url });
    }
  }

  handleShowAboutDialog() {
    const aboutDialog = this.querySelector('sf-about-dialog') as AboutDialog;
    aboutDialog.showModal(true);
  }

  handleEscape(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      const isBody = document.activeElement === document.body;
      if (!isBody && document.activeElement instanceof HTMLElement) {
        const activeElement = document.activeElement as HTMLElement;
        activeElement.blur();
      }
    }
  }

  connectedCallback() {
    this.init();
    this.addEventListener('dragstart', this.onDragStart);
    this.addEventListener('keydown', this.handleEscape);
    const fileOpenHandle = this.#channel.addListener(
      'sf-file-open-requested',
      this.handleFilesOpenRequested.bind(this),
    );
    const fileCloseHandle = this.#channel.addListener(
      'sf-file-close-requested',
      this.handleFileCloseRequested.bind(this),
    );
    const fileCloseAllHandle = this.#channel.addListener(
      'sf-file-close-all-requested',
      this.handleFileCloseAllRequested.bind(this),
    );
    const fileExportHandle = this.#channel.addListener(
      'sf-file-export-requested',
      this.handleFileExportRequested.bind(this),
    );
    const showAboutHandle = this.#channel.addListener(
      'sf-show-about-requested',
      this.handleShowAboutDialog.bind(this),
    );
    this.#channelEventListeners.push(fileOpenHandle);
    this.#channelEventListeners.push(fileCloseHandle);
    this.#channelEventListeners.push(fileCloseAllHandle);
    this.#channelEventListeners.push(fileExportHandle);
    this.#channelEventListeners.push(showAboutHandle);

    this.render();
  }

  disconnectedCallback() {
    this.removeEventListener('dragstart', this.onDragStart);
    this.removeEventListener('keydown', this.handleEscape);
    for (const handle of this.#channelEventListeners) {
      this.#channel.removeListener(handle);
    }
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
  }
}

customElements.define('sf-app', App);
