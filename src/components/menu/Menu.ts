import 'components/menu/MenuItem';
import 'components/menu/Submenu';
import './Menu.css';

// no template with slots required/possible
// see: https://stackoverflow.com/a/67333433
// const template = ``;

// maybe use html-template-loader instead
// https://stackoverflow.com/questions/37818401/importing-html-files-with-es6-template-string-loader
// <ul> required because of: https://stackoverflow.com/a/20550925
const template = `
  <li is="sf-menu-item" key="sf-menu-item-1" title="Menu Item 1"></li>
  <li is="sf-menu-item" key="sf-menu-item-2" title="Menu Item 2"></li>
  <li is="sf-submenu" key="sf-submenu-1" title="Submenu 1">
    <ul>
      <li is="sf-menu-item" key="sf-menu-item-3" title="Menu Item 3"></li>
      <li is="sf-submenu" key="sf-submenu-2" title="Submenu 2">
        <ul>
          <li is="sf-menu-item" key="sf-menu-item-5" title="Menu Item 5"></li>
          <li is="sf-menu-item" key="sf-menu-item-6" title="Menu Item 6"></li>
          <li is="sf-menu-item" key="sf-menu-item-7" title="Menu Item 7"></li>
        </ul>
      <li is="sf-menu-item" key="sf-menu-item-4" title="Menu Item 4"></li>
    </ul>
  </li>
`;

export default class Menu extends HTMLUListElement {
  constructor() {
    super();
    this.render();
    console.log('Menu constructor() called');
  }

  render() {
    this.innerHTML = template;
    // this.innerHTML = template;
    // for (let index = 0; index < 5; index++) {
    //   const li = document.createElement('li', { is: 'sf-menu-item' });
    //   this.append(li);
    // }
  }

  onClick(e: MouseEvent) {
    console.log('Menu item clicked.');
    if (!(e.target instanceof Element)) {
      return;
    }
    e.preventDefault();
    const key = e?.target?.getAttribute('key');
    console.log({key});
    if (!key) {
      return;
    }

    switch (key) {
      case 'sf-file-open':
        console.log('TODO: file open...');
        break;
      case 'sf-file-close':
        console.log('TODO: file close...');
        break;
      default:
        break;
    }
  }

  connectedCallback() {
    console.log('Menu connectedCallback() called');
    this.addEventListener('click', this.onClick.bind(this));
    this.render();
  }

  disconnectedCallback() {
    console.log('Menu disconnectedCallback() called');
    this.removeEventListener('click', this.onClick.bind(this));
  }

  adoptedCallback() {
    console.log('Menu adoptedCallback() called');
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log('Menu attributeChangedCallback() called');
  }
}

console.log('define "sf-menu"');
customElements.define('sf-menu', Menu, { extends: 'ul' });
