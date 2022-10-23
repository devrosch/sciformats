import DataRepository from 'model/DataRepository';
import StubDataRepository from 'model/StubDataRepository';
import { isSameUrl } from 'util/UrlUtils';
import { dispatchWindowCustomEvent } from 'util/EventUtils';
import 'components/TreeNode.css';

const template = '';

export default class TreeNode extends HTMLElement {
  static get observedAttributes() { return ['url']; }

  #repository = new StubDataRepository() as DataRepository;

  #url = new URL('file:///dummy.txt#/') as URL;

  #children = [] as string[];

  #collapsed = true as boolean;

  #selected = false as boolean;

  constructor(repository: DataRepository | null, url: URL | null) {
    super();
    console.log('TreeNode constructor() called');
    if (repository !== null && typeof repository !== 'undefined') {
      this.#repository = repository;
    }
    if (url !== null && typeof url !== 'undefined') {
      this.#url = url;
    }
    const data = this.#repository.read(this.#url);
    this.#children = data.children;
  }

  render() {
    this.innerHTML = template;
    const numChildNodes = this.#children.length;
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
      for (const childNodeName of this.#children) {
        const childUrl = new URL(this.#url);
        if (!this.#url.hash.endsWith('/')) {
          childUrl.hash += ('/');
        }
        childUrl.hash += childNodeName;
        const childNode = new TreeNode(this.#repository, childUrl);
        childNode.setAttribute('url', childUrl.toString());
        this.appendChild(childNode);
      }
    }
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

  setSelected(selected: boolean) {
    this.#selected = selected;
    if (selected) {
      this.classList.add('selected');
      dispatchWindowCustomEvent('sf-tree-node-selected', { url: this.#url });
    } else {
      this.classList.remove('selected');
      dispatchWindowCustomEvent('sf-tree-node-unselected', { url: this.#url });
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

  handleTreeNodeSelected(e: Event) {
    const ce = e as CustomEvent;
    const url = ce.detail.url;
    if (!isSameUrl(this.#url, url) && this.#selected) {
      this.setSelected(false);
    }
  }

  // #endregion user events

  // #region lifecycle events

  connectedCallback() {
    console.log('TreeNode connectedCallback() called');
    window.addEventListener('sf-tree-node-selected', this.handleTreeNodeSelected.bind(this));
    this.render();
  }

  disconnectedCallback() {
    console.log('TreeNode disconnectedCallback() called');
    if (this.#selected) {
      this.setSelected(false);
    }
    window.removeEventListener('sf-tree-node-selected', this.handleTreeNodeSelected.bind(this));
  }

  adoptedCallback() {
    console.log('TreeNode adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('TreeNode attributeChangedCallback() called');
    if (name === 'url') {
      if (this.#url.toString() !== newValue) {
        console.log('TreeNode url attribute value changed');
        this.#url = new URL(newValue);
        this.render();
      }
    }
  }

  // #endregion lifecycle events
}

console.log('define "sf-tree-node"');
customElements.define('sf-tree-node', TreeNode);
