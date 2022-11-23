import DataRepository from 'model/DataRepository';
import LocalFileDataRepository from 'model/LocalFileDataRepository';
// import StubDataRepository from 'model/StubDataRepository';
// import { isSameUrl } from 'util/UrlUtils';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Message from 'model/Message';
import Channel from 'model/Channel';
import TreeNode from './TreeNode';

const template = '';

export default class Tree extends HTMLElement {
  // #repository = new StubDataRepository() as DataRepository;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #eventListener: any = null;

  #children = [] as TreeNode[];

  constructor(repository: DataRepository | null) {
    super();
    console.log('Tree constructor() called');
    // if (repository !== null && typeof repository !== 'undefined') {
    //   this.#repository = repository;
    // }
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

    for (const rootNode of rootNodes) {
      if (i < children.length) {
        const rootNodeUrl = rootNode.getAttribute('url');
        const childNode = children.item(i) as TreeNode;
        const childNodeUrl = childNode.getAttribute('url');
        if (rootNodeUrl !== childNodeUrl) {
          // missing child node => insert
          childNode.insertBefore(rootNode, childNode);
        }
      } else {
        // 
        this.append(rootNode);
      }
      i++;
    }
    // more children than root nodes => remove extra children
    while (i < children.length) {
      this.removeChild(children.item(i) as Element);
    }
  }

  // #region user events

  handleFilesOpened(message: Message) {
    const files = message.detail.files as File[];
    for (const file of files) {
      console.log('Tree -> sf-files-opened received for: ' + file.name);
      // generate URL of type file:///UUID/fileName#/
      const uuid = crypto.randomUUID();
      const url = new URL(`file:///${uuid}/${file.name}#/`);
      const repo = new LocalFileDataRepository(url, file);
      const rootNode = new TreeNode(repo, url);
      this.#children.push(rootNode);
    }
    this.render();
  }

  // #endregion user events

  // #region lifecycle events

  connectedCallback() {
    console.log('Tree connectedCallback() called');
    this.#eventListener = this.#channel.addListener('sf-files-opened', this.handleFilesOpened.bind(this));
    this.render();
  }

  disconnectedCallback() {
    console.log('Tree disconnectedCallback() called');
    this.#channel.removeListener(this.#eventListener);
  }

  adoptedCallback() {
    console.log('Tree adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Tree attributeChangedCallback() called');
  }

  // #endregion lifecycle events
}

console.log('define "sf-tree"');
customElements.define('sf-tree', Tree);
