import './DataTable';
import './DataPanel.css';

const template = `
  <div id="sf-data-tabs" class="tabs">
    <button id="sf-data-chart-link" class="tab-link">Chart</button>
    <button id="sf-data-table-link" class="tab-link">Table</button>
  </div>

  <div id="sf-data-chart-panel" class="tab-content">
    <h3>Chart</h3>
  </div>
  <div id="sf-data-table-panel" class="tab-content">
    <sf-data-table/>
  </div>
`;

export default class DataPanel extends HTMLElement {
  constructor() {
    super();
    console.log('DataPanel constructor() called');
  }

  #active = 'chart';

  init() {
    if (this.children.length !== 3
      || (this.children.item(0)?.id !== 'sf-data-tabs')
      || (this.children.item(1)?.id !== 'sf-data-chart-panel')
      || (this.children.item(2)?.id !== 'sf-data-table-panel')) {
      this.innerHTML = template;
    }
  }

  render() {
    this.init();
    
    const tabLinks = this.querySelectorAll('#sf-data-tabs > button');
    for (const link of tabLinks) {
      if (link.id === `sf-data-${this.#active}-link`) {
        link.classList.add('active');
      } else {
        link.classList.remove('active');
      }
    }

    const panels = this.querySelectorAll('.tab-content');
    for (const panel of panels) {
      if (panel.id === `sf-data-${this.#active}-panel`) {
        panel.classList.add('active');
      } else {
        panel.classList.remove('active');
      }
    }
  }

  onClick(e: MouseEvent) {
    console.log('DataPanel item clicked.');
    if (!(e.target instanceof Element)) {
      return;
    }
    e.preventDefault();
    e.stopPropagation();
    const id = e?.target?.getAttribute('id');
    console.log({ id });

    switch (id) {
      case 'sf-data-chart-link':
        this.#active = 'chart';
        this.render();
        break;
      case 'sf-data-table-link':
        this.#active = 'table';
        this.render();
        break;
      default:
        // noop
    }
  }

  connectedCallback() {
    console.log('DataPanel connectedCallback() called');
    this.addEventListener('click', this.onClick.bind(this));
    this.render();
  }

  disconnectedCallback() {
    console.log('DataPanel disconnectedCallback() called');
    this.removeEventListener('click', this.onClick.bind(this));
  }

  adoptedCallback() {
    console.log('DataPanel adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('DataPanel attributeChangedCallback() called');
  }
}

console.log('define "sf-data-panel"');
customElements.define('sf-data-panel', DataPanel);
