import LocalFileParser from 'model/LocalFileParser';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Message from 'model/Message';
import Channel from 'model/Channel';
import TreeNode from './TreeNode';

const template = '';

export default class Tree extends HTMLElement {
  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #eventListeners: any[] = [];

  #children = [] as TreeNode[];

  #selectedNodeUrl: URL | null = null;

  constructor() {
    super();
    console.log('Tree constructor() called');
  }

  init() {
    if (this.#children.length === 0 && this.children.length > 0) {
      this.innerHTML = template;
    }
  }

  render() {
    this.init();
    const rootNodes = this.#children;
    const children = this.children;
    let i = 0;

    // TODO: do more generic set reconciliation
    // assumes that new root nodes are always appended, not inserted
    for (const rootNode of rootNodes) {
      if (i >= children.length) {
        this.append(rootNode);
        i += 1;
      } else {
        const rootNodeUrl = rootNode.getAttribute('url');
        while (i < children.length) {
          const childNode = children.item(i) as TreeNode;
          const childNodeUrl = childNode.getAttribute('url');
          if (rootNodeUrl === childNodeUrl) {
            // matches => noop
            i += 1;
            break;
          } else {
            // surplus child node => remove
            this.removeChild(children.item(i) as Element);
            // children gets updated => do not i++
          }
        }
      }
    }
    // more children than root nodes => remove extra children
    while (i < children.length) {
      this.removeChild(children.item(i) as Element);
    }
  }

  // #region user events

  handleFilesOpenRequested(message: Message) {
    const files = message.detail.files as File[];
    for (const file of files) {
      console.log(`Tree -> sf-file-open-requested received for: ${file.name}`);
      // generate URL of type file:///UUID/fileName#/
      const uuid = crypto.randomUUID();
      const url = new URL(`file:///${uuid}/${file.name}#/`);
      const repo = new LocalFileParser(url, file);
      const rootNode = new TreeNode(repo, url);
      this.#children.push(rootNode);
    }
    this.render();
  }

  handleFileCloseRequested() {
    console.log('handleFileCloseRequested()');
    if (!this.#selectedNodeUrl) {
      return;
    }
    const selectedUrl = this.#selectedNodeUrl.toString();
    for (const child of this.#children) {
      const childUrl = child.getAttribute('url');
      if (childUrl !== null && selectedUrl.startsWith(childUrl)) {
        const index = this.#children.indexOf(child);
        this.#children.splice(index, 1);
        this.render();
        return;
      }
    }
  }

  handleFileCloseAllRequested() {
    console.log('handleFileCloseAllRequested()');
    this.#children = [];
    this.render();
  }

  handleTreeNodeSelection(message: Message) {
    console.log(`handleTreeNodeSelection() -> ${message.name}: ${message.detail.url}`);
    const url = message.detail.url;
    if (message.name === 'sf-tree-node-selected') {
      this.#selectedNodeUrl = url;
    } else if (message.name === 'sf-tree-node-deselected') {
      if (this.#selectedNodeUrl === url) {
        this.#selectedNodeUrl = null;
      }
    }
  }

  // #endregion user events

  // #region lifecycle events

  connectedCallback() {
    console.log('Tree connectedCallback() called');
    const fileOpenHandle = this.#channel.addListener('sf-file-open-requested', this.handleFilesOpenRequested.bind(this));
    const fileCloseHandle = this.#channel.addListener('sf-file-close-requested', this.handleFileCloseRequested.bind(this));
    const fileCloseAllHandle = this.#channel.addListener('sf-file-close-all-requested', this.handleFileCloseAllRequested.bind(this));
    const selectedHandle = this.#channel.addListener('sf-tree-node-selected', this.handleTreeNodeSelection.bind(this));
    const deselectedHandle = this.#channel.addListener('sf-tree-node-deselected', this.handleTreeNodeSelection.bind(this));
    this.#eventListeners.push(fileOpenHandle);
    this.#eventListeners.push(fileCloseHandle);
    this.#eventListeners.push(fileCloseAllHandle);
    this.#eventListeners.push(selectedHandle);
    this.#eventListeners.push(deselectedHandle);
    this.render();
  }

  disconnectedCallback() {
    console.log('Tree disconnectedCallback() called');
    for (const handle of this.#eventListeners) {
      this.#channel.removeListener(handle);
    }
  }

  adoptedCallback() {
    console.log('Tree adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Tree attributeChangedCallback() called');
  }

  // #endregion lifecycle events
}

console.log('define "sf-tree"');
customElements.define('sf-tree', Tree);
