import Parser from 'model/Parser';
import { isSameUrl } from 'util/UrlUtils';
import './TreeNode.css';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Message from 'model/Message';
import Channel from 'model/Channel';
import NodeData from 'model/NodeData';

const nodeSelectedEvent = 'sf-tree-node-selected';
const nodeDeselectedEvent = 'sf-tree-node-deselected';
const nodeDataReadEvent = 'sf-tree-node-data-read';

const template = '';

export default class TreeNode extends HTMLElement {
  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #eventListener: any = null;

  #parser: Parser;

  #url: URL;

  #nodeData: NodeData | null = null;

  #collapsed: boolean = true;

  #selected: boolean = false;

  constructor(parser: Parser, url: URL) {
    super();
    console.log('TreeNode constructor() called');
    this.#parser = parser;
    this.#url = url;
  }

  render() {
    this.innerHTML = template;
    const urlAttr = this.getAttribute('url');
    if (urlAttr !== this.#url.toString()) {
      this.setAttribute('url', this.#url.toString());
    }
    if (this.#nodeData === null) {
      const nameSpan = document.createElement('span');
      nameSpan.classList.add('node-name');
      nameSpan.textContent = 'Loading...';
      this.append(nameSpan);
      return;
    }

    const numChildNodes = this.#nodeData.children.length;
    const hasChildren = numChildNodes > 0;
    if (hasChildren) {
      if (hasChildren) {
        const plusMinusSpan = document.createElement('span');
        plusMinusSpan.classList.add('plusminus');
        plusMinusSpan.textContent = this.#collapsed ? '⊞' : '⊟';
        plusMinusSpan.addEventListener('click', this.onToggleCollapsed);
        this.append(plusMinusSpan);
      }
    }
    const nameSpan = document.createElement('span');
    nameSpan.classList.add('node-name');
    nameSpan.addEventListener('click', this.onSelected);
    nameSpan.textContent = this.name;
    if (this.#selected) {
      nameSpan.classList.add('selected');
    }
    this.append(nameSpan);

    if (hasChildren && !this.#collapsed) {
      for (const childNodeName of this.#nodeData.children) {
        const childUrl = new URL(this.#url);
        if (!this.#url.hash.endsWith('/')) {
          childUrl.hash += ('/');
        }
        childUrl.hash += childNodeName;
        const childNode = new TreeNode(this.#parser, childUrl);
        childNode.setAttribute('url', childUrl.toString());
        this.appendChild(childNode);
      }
    }
  }

  async #retrieveNodeData() {
    const data = await this.#parser.read(this.#url);
    this.#nodeData = data;
    this.#channel.dispatch(nodeDataReadEvent, this.#nodeData);
    this.render();
  }

  static #extractName(path: string): string {
    const segments: string[] = path.split('/');
    if (segments.length === 0) {
      return '';
    }
    let name = segments.pop()!.trim();
    if (name === '' || typeof name === undefined) {
      name = segments.length > 0 ? segments.pop()!.trim() : '';
    }
    return decodeURIComponent(name);
  }

  get name() {
    const hash = this.#url.hash.trim();
    if (hash === '' || hash === '#' || hash === '#/') {
      return TreeNode.#extractName(this.#url.pathname);
    }
    return TreeNode.#extractName(hash);
  }

  async setSelected(selected: boolean) {
    this.#selected = selected;
    if (selected) {
      this.classList.add('selected');
      let nodeData = this.#nodeData;
      if (nodeData == null) {
        nodeData = {
          url: this.#url, data: [], parameters: [], children: [],
        };
      }
      this.#channel.dispatch(nodeSelectedEvent, nodeData);
    } else {
      this.classList.remove('selected');
      this.#channel.dispatch(nodeDeselectedEvent, { url: this.#url });
    }
  }

  // #region user events

  onToggleCollapsed = () => {
    console.log('onClickPlusMinus() called');
    this.#collapsed = !this.#collapsed;
    this.render();
  };

  onSelected = () => {
    console.log('onSelected() called');
    this.setSelected(true);
  };

  handleTreeNodeSelected(message: Message) {
    const url = message.detail.url;
    if (!isSameUrl(this.#url, url) && this.#selected) {
      this.setSelected(false);
    }
  }

  // #endregion user events

  // #region lifecycle events

  connectedCallback() {
    console.log('TreeNode connectedCallback() called');
    this.#eventListener = this.#channel.addListener('sf-tree-node-selected', this.handleTreeNodeSelected.bind(this));
    this.#retrieveNodeData();
    this.render();
  }

  disconnectedCallback() {
    console.log('TreeNode disconnectedCallback() called');
    if (this.#selected) {
      this.setSelected(false);
    }
    this.#channel.removeListener(this.#eventListener);
  }

  adoptedCallback() {
    console.log('TreeNode adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('TreeNode attributeChangedCallback() called');
  }

  // #endregion lifecycle events
}

console.log('define "sf-tree-node"');
customElements.define('sf-tree-node', TreeNode);
