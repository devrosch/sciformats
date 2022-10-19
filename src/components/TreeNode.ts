import './TreeNode.css';

const template = '';

export default class TreeNode extends HTMLElement {
  static get observedAttributes() { return ['name']; }

  #name = '' as string;

  #collapsed = true as boolean;

  #dataModel = (path: string) =>
    path === 'root' ? ['child 1', 'child 2', 'child 3'] :
    path === 'child 2' ? ['child 4', 'child 5'] : [];

  constructor(dataModel : ((path: string) => string[]) | null) {
    super();
    console.log('TreeNode constructor() called');
    if (dataModel !== null && typeof dataModel !== 'undefined') {
      this.#dataModel = dataModel;
    }
  }

  onClick = () => {
    console.log('onClick() called');
    this.#collapsed = !this.#collapsed;
    this.render();
  }

  get name() {
    return this.#name;
  }

  render() {
    this.innerHTML = template;
    const numChildNodes = this.#dataModel(this.#name).length;
    const hasChildren = numChildNodes > 0;
    if (hasChildren) {
      const plusMinusSpan = document.createElement('span');
      plusMinusSpan.classList.add('plusminus');
      plusMinusSpan.textContent = hasChildren ? this.#collapsed ? '⊞' : '⊟' : '';
      plusMinusSpan.addEventListener('click', () => this.onClick());
      this.append(plusMinusSpan);
    }
    const nameSpan = document.createElement('span');
    nameSpan.classList.add('plusminus');
    nameSpan.textContent = this.#name;
    this.append(nameSpan);

    if (hasChildren && !this.#collapsed) {
      const childNodeNames = this.#dataModel(this.#name);
      for (const childNodeName of childNodeNames) {
        const childNode = new TreeNode(this.#dataModel);
        childNode.setAttribute('name', childNodeName);
        this.appendChild(childNode);
      }
    }
  }

  connectedCallback() {
    console.log('TreeNode connectedCallback() called');
    this.render();
  }

  disconnectedCallback() {
    console.log('TreeNode disconnectedCallback() called');
  }

  adoptedCallback() {
    console.log('TreeNode adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('TreeNode attributeChangedCallback() called');
    if (name === 'name') {
      this.#name = newValue;
    }
    this.render();
  }
}

console.log('define "sf-tree-node"');
customElements.define('sf-tree-node', TreeNode);
