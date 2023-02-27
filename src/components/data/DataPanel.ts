import './DataChart';
import './DataTable';
import './DataPanel.css';

const template = `
  <div id="sf-data-tabs" class="tabs">
    <button id="sf-data-chart-link" class="tab-link">Chart</button>
    <button id="sf-data-table-link" class="tab-link">Table</button>
  </div>

  <div id="sf-data-chart-panel" class="tab-content">
    <sf-data-chart/>
  </div>
  <div id="sf-data-table-panel" class="tab-content">
    <sf-data-table/>
  </div>
`;

export default class DataPanel extends HTMLElement {
  #initialized = false;

  #active = 'chart';

  constructor() {
    super();
    console.log('DataPanel constructor() called');
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
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

  onClick = (e: MouseEvent) => {
    console.log('DataPanel item clicked.');
    if (!(e.target instanceof Element)) {
      return;
    }
    e.preventDefault();
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
  };

  connectedCallback() {
    console.log('DataPanel connectedCallback() called');
    this.init();
    this.addEventListener('click', this.onClick);
    this.render();
  }

  disconnectedCallback() {
    console.log('DataPanel disconnectedCallback() called');
    this.removeEventListener('click', this.onClick);
  }

  adoptedCallback() {
    console.log('DataPanel adoptedCallback() called');
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('DataPanel attributeChangedCallback() called');
    this.init();
  }
}

console.log('define "sf-data-panel"');
customElements.define('sf-data-panel', DataPanel);
