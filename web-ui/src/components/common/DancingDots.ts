import './DancingDots.css';

const template = `
<span class="sf-dots">
  <span class="sf-dot">.</span>
  <span class="sf-dot">.</span>
  <span class="sf-dot">.</span>
</span>
`;

export default class DancingDots extends HTMLElement {
  #initialized = false;

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  /* eslint-disable-next-line class-methods-use-this */
  render() {
    // noop
  }

  connectedCallback() {
    this.init();
    this.render();
  }
}

console.log('define "sf-dancing-dots');
customElements.define('sf-dancing-dots', DancingDots);
