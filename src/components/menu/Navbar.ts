/* eslint-disable import/no-duplicates */
import Channel from 'model/Channel';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import SysInfoProvider from 'util/SysInfoProvider';
import './Menu'; // for side effects
import Menu from './Menu';
import './AboutDialog'; // for side effects
import AboutDialog from './AboutDialog';
import './Navbar.css';

const isMacOs = SysInfoProvider.detectOS() === 'macOS';
const fileShortcutsModifierKeys = isMacOs ? '⇧ ⌃ ' : 'Alt-Shift-';
const fileOpenShortcutModifierKeys = isMacOs ? '⌃ ⌥ ' : fileShortcutsModifierKeys;
// const editShortcutsModifierKeys = isMacOs ? '⌃ ' : 'Ctrl-';

// no template with slots required/possible
// see: https://stackoverflow.com/a/67333433
// maybe use html-template-loader instead
// https://stackoverflow.com/questions/37818401/importing-html-files-with-es6-template-string-loader
// <div>s required for relative/absolute placement of child elements
const template = `
  <a href="#" class="sf-logo" key="sf-navbar-logo">Logo</a>
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
        <sf-menu-item
          key="sf-file-close-all"
          title="Close All"
          shortcut="${fileShortcutsModifierKeys}-Q">
        </sf-menu-item>
      </sf-submenu>
      <sf-menu-item key="sf-menu-item-2" title="Menu Item 2"></sf-menu-item>
      <sf-submenu key="sf-submenu-1" title="Submenu 1">
        <sf-menu-item key="sf-menu-item-3" title="Menu Item 3" shortcut="Ctrl-3"></sf-menu-item>
        <sf-submenu key="sf-submenu-2" title="Submenu 2">
          <sf-menu-item key="sf-menu-item-5" title="Menu Item 5" shortcut="Ctrl-5"></sf-menu-item>
          <sf-menu-item key="sf-menu-item-6" title="Menu Item 6" shortcut="Ctrl-6"></sf-menu-item>
          <sf-menu-item key="sf-menu-item-7" title="Menu Item 7" shortcut="Ctrl-7"></sf-menu-item>
        </sf-submenu>
        <sf-menu-item key="sf-menu-item-4" title="Menu Item 4" shortcut="Ctrl-4"></sf-menu-item>
      </sf-submenu>
      <sf-menu-item key="sf-about" title="About..."></sf-menu-item>
    </sf-menu>
  </nav>
  <sf-about-dialog/>
`;

const events = {
  fileCloseRequested: 'sf-file-close-requested',
  fileCloseAllRequested: 'sf-file-close-all-requested',
};

const mediaQuery = window.matchMedia('screen and (max-width: 576px)');

export default class Navbar extends HTMLElement {
  static get observedAttributes() { return ['app-selector']; }

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #app: HTMLElement | null = null;

  constructor() {
    super();
    console.log('Navbar constructor() called');
  }

  #showMenu: boolean = false;

  init() {
    if (this.children.length !== 4
      || !(this.children.item(0) instanceof HTMLAnchorElement)
      || !(this.children.item(1) instanceof HTMLAnchorElement)
      || this.children.item(2)?.nodeName !== 'NAV'
      || !(this.children.item(3) instanceof AboutDialog)) {
      // init
      this.innerHTML = template;
    }
  }

  render() {
    this.init();
    const menu = this.querySelector('sf-menu') as Menu;
    menu.showMenu(this.#showMenu);
  }

  showAboutDialog() {
    const aboutDialog = this.querySelector('sf-about-dialog') as AboutDialog;
    aboutDialog.showModal(true);
  }

  updateAppReference(selector: string | null) {
    if (selector !== null) {
      this.#app = document.querySelector(selector);
    }
  }

  // eslint-disable-next-line class-methods-use-this
  onClick = (e: MouseEvent) => {
    console.log('Navbar item clicked.');
    if (!(e.target instanceof Element)) {
      return;
    }
    e.preventDefault();
    e.stopPropagation();
    const key = e?.target?.getAttribute('key');
    console.log({ key });
    if (!key) {
      return;
    }

    switch (key) {
      case 'sf-navbar-hamburger':
        this.#showMenu = !this.#showMenu;
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
        this.showAboutDialog();
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
    const fileModifiersPressed = isMacOs
      ? e.shiftKey && e.ctrlKey && !e.altKey && !e.metaKey
      : e.shiftKey && e.altKey && !e.ctrlKey && !e.metaKey;
    // const fileModifiersPressed = e.shiftKey && e.altKey && !e.ctrlKey && !e.metaKey;
    // const editModifiersPressed = e.ctrlKey && !e.altKey && !e.shiftKey && !e.metaKey;

    // cannot use same mechanism for fileOpen() due to browser security limitations
    // => use "accessKey" property instead
    if (fileModifiersPressed && e.key.toLowerCase() === 'c') {
      this.#channel.dispatch(events.fileCloseRequested, null);
      this.#showMenu = false;
      this.render();
    } else if (fileModifiersPressed && e.key.toLowerCase() === 'q') {
      this.#channel.dispatch(events.fileCloseAllRequested, null);
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
    console.log('handleOutsideSelection() called');
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
  };

  connectedCallback() {
    console.log('Navbar connectedCallback() called');
    const appSelector = this.getAttribute('app-selector');
    this.updateAppReference(appSelector);
    this.addEventListener('click', this.onClick);
    // only document will reliably receive all keydown events
    document.addEventListener('keydown', this.handleShortcuts);
    mediaQuery.addEventListener('change', this.handleScreenChange);
    document.addEventListener('click', this.handleOutsideSelection);
    this.#app?.addEventListener('dragenter', this.onDragEnter);
    this.#app?.addEventListener('dragover', this.onDragOver);
    this.#app?.addEventListener('drop', this.onFileDropped);
    this.render();
  }

  disconnectedCallback() {
    console.log('Navbar disconnectedCallback() called');
    this.removeEventListener('click', this.onClick);
    document.removeEventListener('keydown', this.handleShortcuts);
    mediaQuery.removeEventListener('change', this.handleScreenChange);
    document.removeEventListener('click', this.handleOutsideSelection);
    this.#app?.removeEventListener('dragenter', this.onDragEnter);
    this.#app?.removeEventListener('dragover', this.onDragOver);
    this.#app?.removeEventListener('drop', this.onFileDropped);
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Navbar attributeChangedCallback() called');
    if (name === 'app-selector' && newValue !== oldValue) {
      this.updateAppReference(newValue);
      this.render();
    }
  }
}

console.log('define "sf-menu"');
customElements.define('sf-navbar', Navbar);
