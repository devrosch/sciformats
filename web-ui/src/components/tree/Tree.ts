import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Message from 'model/Message';
import Channel from 'model/Channel';
import TreeNode from './TreeNode';
import './Tree.css';
import Parser from 'model/Parser';
import { isSameUrl } from 'util/UrlUtils';

const template = '';

export default class Tree extends HTMLElement {
  #initialized = false;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #eventListeners: any[] = [];

  #children = [] as TreeNode[];

  #selectedNodeUrl: URL | null = null;

  constructor() {
    super();
    console.log('Tree constructor() called');
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
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

  addRootNode(parser: Parser) {
    const rootNode = new TreeNode(parser, parser.rootUrl);
    this.#children.push(rootNode);
    this.render();
  }

  // #region user events

  removeSelectedNode(): URL | null {
    if (!this.#selectedNodeUrl) {
      return null;
    }
    const selectedUrl = this.#selectedNodeUrl.toString();
    for (const child of this.#children) {
      const childUrl = child.getAttribute('url');
      if (childUrl !== null && selectedUrl.startsWith(childUrl)) {
        const index = this.#children.indexOf(child);
        this.#children.splice(index, 1);
        // todo: return Parser and close in App?
        child.close().then(() => {
          console.log(`File closed: ${child.name}`);
        });
        this.render();
        return new URL(childUrl);
      }
    }

    throw new Error(
      `Illegal state. No node found for selectedNodeUrl: ${this.#selectedNodeUrl}`,
    );
  }

  removeAllNodes(): URL[] {
    const urls = [];
    for (const child of this.#children) {
      const childUrl = child.getAttribute('url');
      urls.push(new URL(childUrl!));
      child.close().then(() => {
        console.log(`File closed: ${child.name}`);
      });
    }
    this.#children = [];
    this.render();
    return urls;
  }

  #handleTreeNodeSelection(message: Message) {
    console.log(
      `handleTreeNodeSelection() -> ${message.name}: ${message.detail.url}`,
    );
    const url = message.detail.url;
    if (message.name === 'sf-tree-node-selected') {
      this.#selectedNodeUrl = url;
    } else if (message.name === 'sf-tree-node-deselected') {
      if (this.#selectedNodeUrl === url) {
        this.#selectedNodeUrl = null;
      }
    }
  }

  static #findLastLeafNode(node: Element): TreeNode {
    // recursively find last TreeNode
    let currentNode = node;
    while (
      currentNode.hasAttribute('expand') &&
      currentNode.getAttribute('expand') === 'true'
    ) {
      currentNode = currentNode.lastChild as Element;
    }
    return currentNode as TreeNode;
  }

  static #findPreviousTreeNode(element: Element): TreeNode | null {
    let prev = element.previousSibling;
    if (prev === null || !(prev instanceof TreeNode)) {
      prev = element.parentElement;
      if (!(prev instanceof TreeNode)) {
        prev = null;
      }
    } else if (
      prev !== null &&
      prev.hasAttribute('expand') &&
      prev.getAttribute('expand') === 'true'
    ) {
      prev = Tree.#findLastLeafNode(prev);
    }
    return prev as TreeNode;
  }

  static #findParentNextSibling(node: Element): TreeNode | null {
    // recursively move up tree to find parent's next sibling
    let parent = node.parentElement;
    while (parent instanceof TreeNode || parent instanceof Tree) {
      const nextSibling = parent.nextSibling;
      if (nextSibling instanceof TreeNode) {
        return nextSibling;
      }
      parent = parent.parentElement;
    }
    return null;
  }

  static #findNextTreeNode(element: Element): TreeNode | null {
    let next = null;
    if (
      element.hasAttribute('expand') &&
      element.getAttribute('expand') === 'true'
    ) {
      // find first child node
      next = element.querySelector('sf-tree-node');
    } else {
      // find next sibling
      next = element.nextSibling;
      if (next === null || !(next instanceof TreeNode)) {
        next = Tree.#findParentNextSibling(element);
      }
    }
    return next as TreeNode | null;
  }

  #findSelectedNode(): TreeNode | null {
    const node = this.querySelector(
      `sf-tree-node[url="${this.#selectedNodeUrl}"]`,
    );
    return node === null ? null : (node as TreeNode);
  }

  getSelectedNodeParser(): Parser | null {
    const node = this.#findSelectedNode();
    if (node instanceof TreeNode) {
      return node.parser;
    }
    return null;
  }

  static #findEventNode(e: KeyboardEvent): TreeNode | null {
    let node = e.target;
    while (node !== null && node !== undefined && !(node instanceof TreeNode)) {
      node = (node as Element | null)?.parentElement as TreeNode | null;
    }
    return node == null || node == undefined ? null : node;
  }

  onKeyDown(e: KeyboardEvent) {
    console.log('onKeyDown()');
    const key = e.key;
    console.log(key);

    if (key === 'Enter') {
      // event may originate from element within TreeNode => find TreeNode parentElement
      const treeNode = Tree.#findEventNode(e);
      if (treeNode === null) {
        return;
      }

      let isSelectedNode = false;
      if (isSameUrl(treeNode.getAttribute('url'), this.#selectedNodeUrl)) {
        isSelectedNode = true;
      }

      // only toggle expanded state if already selected
      if (isSelectedNode && treeNode.hasAttribute('expand')) {
        const expanded = treeNode.getAttribute('expand') === 'true';
        treeNode.setExpand(!expanded);
      }

      treeNode.setSelected(true);
      return;
    }

    const selectedNode = this.#findSelectedNode();
    if (selectedNode === null) {
      return;
    }
    switch (key) {
      case 'ArrowUp': {
        const prev = Tree.#findPreviousTreeNode(selectedNode);
        prev?.setSelected(true);
        // don't scroll
        e.preventDefault();
        break;
      }
      case 'ArrowDown': {
        const next = Tree.#findNextTreeNode(selectedNode);
        next?.setSelected(true);
        // don't scroll
        e.preventDefault();
        break;
      }
      case 'ArrowLeft':
        selectedNode.setExpand(false);
        selectedNode.setSelected(true);
        // do not scroll view
        e.preventDefault();
        break;
      case 'ArrowRight':
        selectedNode.setExpand(true);
        selectedNode.setSelected(true);
        // do not scroll view
        e.preventDefault();
        break;
      default:
        break;
    }
  }

  onClick = (e: Event) => {
    if (e.target instanceof TreeNode || e.target instanceof Tree) {
      // TODO: refactor to avoid code duplication and knowledge of TreeNode implementation
      const nameElement = this.querySelector(
        `span[url="${this.#selectedNodeUrl}"]`,
      ) as HTMLSpanElement | null;
      nameElement?.focus({ focusVisible: false } as FocusOptions);
    }
  };

  // #endregion user events

  // #region lifecycle events

  connectedCallback() {
    console.log('Tree connectedCallback() called');
    this.init();
    this.addEventListener('keydown', this.onKeyDown);
    this.addEventListener('click', this.onClick);
    const selectedHandle = this.#channel.addListener(
      'sf-tree-node-selected',
      this.#handleTreeNodeSelection.bind(this),
    );
    const deselectedHandle = this.#channel.addListener(
      'sf-tree-node-deselected',
      this.#handleTreeNodeSelection.bind(this),
    );
    this.#eventListeners.push(selectedHandle);
    this.#eventListeners.push(deselectedHandle);
    this.render();
  }

  disconnectedCallback() {
    console.log('Tree disconnectedCallback() called');
    this.removeEventListener('keydown', this.onKeyDown);
    this.removeEventListener('click', this.onClick);
    for (const handle of this.#eventListeners) {
      this.#channel.removeListener(handle);
    }
  }

  /* eslint-disable-next-line class-methods-use-this */
  adoptedCallback() {
    console.log('Tree adoptedCallback() called');
  }

  /* eslint-disable-next-line @typescript-eslint/no-unused-vars */
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Tree attributeChangedCallback() called');
    this.init();
  }

  // #endregion lifecycle events
}

console.log('define "sf-tree"');
customElements.define('sf-tree', Tree);
