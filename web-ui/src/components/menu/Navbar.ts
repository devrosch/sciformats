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
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import SysInfoProvider from 'util/SysInfoProvider';
import Channel from 'model/Channel';
import Logo from 'assets/sf-ui.svg';
import './Menu'; // for side effects
import Menu from './Menu';
import './Navbar.css';
import MenuItemFileOpen from './MenuItemFileOpen';

const isMacOs = SysInfoProvider.detectOS() === 'macOS';
const fileShortcutsModifierKeys = isMacOs ? '⇧ ⌃ ' : 'Alt-Shift';
const fileOpenShortcutModifierKeys = isMacOs
  ? '⌃ ⌥ '
  : fileShortcutsModifierKeys;
// const editShortcutsModifierKeys = isMacOs ? '⌃ ' : 'Ctrl-';

// no template with slots required/possible
// see: https://stackoverflow.com/a/67333433
// maybe use html-template-loader instead
// https://stackoverflow.com/questions/37818401/importing-html-files-with-es6-template-string-loader
// <div>s required for relative/absolute placement of child elements
const template = `
  <img src="${Logo}" class="sf-logo" alt="Logo">
  <a href="#" class="sf-hamburger" key="sf-navbar-hamburger">☰</a>
  <nav>
    <sf-menu>
      <sf-submenu key="sf-submenu-file" title="File">
        <sf-menu-item-file-open
          key="sf-file-open"
          title="Open..."
          shortcut="${fileOpenShortcutModifierKeys}-O">
        </sf-menu-item-file-open>
        <sf-menu-item
          key="sf-file-close"
          title="Close"
          shortcut="${fileShortcutsModifierKeys}-C">
        </sf-menu-item>
        <sf-submenu key="sf-export" title="Export...">
          <sf-menu-item key="sf-export-json" title="JSON" shortcut="${fileShortcutsModifierKeys}-J"></sf-menu-item>
        </sf-submenu>
        <sf-menu-item
          key="sf-file-close-all"
          title="Close All"
          shortcut="${fileShortcutsModifierKeys}-Q">
        </sf-menu-item>
      </sf-submenu>
      <sf-menu-item key="sf-about" title="About..."></sf-menu-item>
    </sf-menu>
  </nav>
`;

const events = {
  fileExportRequested: 'sf-file-export-requested',
  fileCloseRequested: 'sf-file-close-requested',
  fileCloseAllRequested: 'sf-file-close-all-requested',
  showAboutRequested: 'sf-show-about-requested',
};

const mediaQuery = window.matchMedia('screen and (max-width: 576px)');

export default class Navbar extends HTMLElement {
  #initialized = false;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #app: HTMLElement | null = null;

  #showMenu = false;

  #shortcutsActive = false;

  init() {
    if (!this.#initialized) {
      // init
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    const menu = this.querySelector('sf-menu') as Menu;
    menu.showMenu(this.#showMenu);
  }

  #removeDragAndDropListeners() {
    if (this.#app) {
      this.#app.removeEventListener('dragenter', this.onDragEnter);
      this.#app.removeEventListener('dragover', this.onDragOver);
      this.#app.removeEventListener('drop', this.onFileDropped);
    }
  }

  activateDragAndDrop(app: HTMLElement) {
    this.#removeDragAndDropListeners();
    this.#app = app;
    this.#app.addEventListener('dragenter', this.onDragEnter);
    this.#app.addEventListener('dragover', this.onDragOver);
    this.#app.addEventListener('drop', this.onFileDropped);
  }

  activateShortcuts() {
    this.#shortcutsActive = true;
    const fileOpenMenuItems = this.querySelectorAll('sf-menu-item-file-open');
    fileOpenMenuItems.forEach((item) => {
      (item as MenuItemFileOpen).activateShortcut();
    });
  }

  onClick = (e: MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    const key = (e?.target as Element | null)?.getAttribute('key');
    if (!key) {
      return;
    }

    switch (key) {
      case 'sf-navbar-hamburger':
        this.#showMenu = !this.#showMenu;
        this.render();
        break;
      case 'sf-export-json':
        this.#channel.dispatch(events.fileExportRequested, 'json');
        this.#showMenu = false;
        this.render();
        break;
      case 'sf-file-close':
        this.#channel.dispatch(events.fileCloseRequested, null);
        this.#showMenu = false;
        this.render();
        break;
      case 'sf-file-close-all':
        this.#channel.dispatch(events.fileCloseAllRequested, null);
        this.#showMenu = false;
        this.render();
        break;
      case 'sf-about':
        this.#channel.dispatch(events.showAboutRequested, null);
        this.#showMenu = false;
        this.render();
        break;
      default:
        this.#showMenu = false;
        this.render();
        break;
    }
  };

  handleShortcuts = (e: KeyboardEvent) => {
    if (!this.#shortcutsActive) {
      return;
    }

    const fileModifiersPressed = isMacOs
      ? e.shiftKey && e.ctrlKey && !e.altKey && !e.metaKey
      : e.shiftKey && e.altKey && !e.ctrlKey && !e.metaKey;
    // const fileModifiersPressed = e.shiftKey && e.altKey && !e.ctrlKey && !e.metaKey;
    // const editModifiersPressed = e.ctrlKey && !e.altKey && !e.shiftKey && !e.metaKey;

    // cannot use same mechanism for fileOpen() due to browser security limitations
    // => use "accessKey" property instead
    if (fileModifiersPressed && e.key.toLowerCase() === 'j') {
      this.#channel.dispatch(events.fileExportRequested, 'json');
      this.#showMenu = false;
      this.render();
    } else if (fileModifiersPressed && e.key.toLowerCase() === 'c') {
      this.#channel.dispatch(events.fileCloseRequested, null);
      this.#showMenu = false;
      this.render();
    } else if (fileModifiersPressed && e.key.toLowerCase() === 'q') {
      this.#channel.dispatch(events.fileCloseAllRequested, null);
      this.#showMenu = false;
      this.render();
    } else if (e.key === 'Escape') {
      this.#showMenu = false;
      this.render();
    }
  };

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  handleScreenChange = (e: MediaQueryListEvent) => {
    // close menu including submenus whenever screen layout crosses threshold
    this.#showMenu = false;
    this.render();
  };

  handleOutsideSelection = (e: MouseEvent) => {
    const node = e.target as Node;
    if (!this.contains(node)) {
      // close menu including submenus whenever click ouside navbar occured
      this.#showMenu = false;
      this.render();
    }
  };

  /* eslint-disable class-methods-use-this */
  onDragEnter = (e: DragEvent) => {
    // see https://www.quirksmode.org/blog/archives/2009/09/the_html5_drag.html for why this is necessary
    e.stopPropagation();
    e.preventDefault();
  };

  /* eslint-disable class-methods-use-this */
  onDragOver = (e: DragEvent) => {
    e.stopPropagation();
    e.preventDefault();
    if (e.dataTransfer !== null) {
      e.dataTransfer.dropEffect = 'copy';
    }
  };

  onFileDropped = (e: DragEvent) => {
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
      if (items[i].webkitGetAsEntry !== undefined) {
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
  };

  connectedCallback() {
    this.init();
    this.addEventListener('click', this.onClick);
    // only document will reliably receive all keydown events
    document.addEventListener('keydown', this.handleShortcuts);
    mediaQuery.addEventListener('change', this.handleScreenChange);
    document.addEventListener('click', this.handleOutsideSelection);
    this.render();
  }

  disconnectedCallback() {
    this.removeEventListener('click', this.onClick);
    document.removeEventListener('keydown', this.handleShortcuts);
    mediaQuery.removeEventListener('change', this.handleScreenChange);
    document.removeEventListener('click', this.handleOutsideSelection);
    this.#removeDragAndDropListeners();
  }

  /* eslint-disable-next-line @typescript-eslint/no-unused-vars */
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
  }
}

customElements.define('sf-navbar', Navbar);
