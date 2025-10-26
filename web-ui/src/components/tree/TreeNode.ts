/**
 * Copyright (c) 2025 Robert Schiwon
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

import { isSameUrl } from 'util/UrlUtils';
import { setElementAttribute, setElementTextContent } from 'util/RenderUtils';
import 'components/common/DancingDots';
import './TreeNode.css';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Parser from 'model/Parser';
import Message from 'model/Message';
import Channel from 'model/Channel';
import NodeData from 'model/NodeData';

const nodeSelectedEvent = 'sf-tree-node-selected';
const nodeDeselectedEvent = 'sf-tree-node-deselected';
const nodeDataUpdatedEvent = 'sf-tree-node-data-updated';

const template = `
  <span class="plusminus"></span>
  <span class="node-error">🅧</span>  
  <span class="node-name" tabindex="0"></span>
`;

export default class TreeNode extends HTMLElement {
  #initialized = false;

  #channel: Channel = CustomEventsMessageBus.getDefaultChannel();

  #eventListener: any = null;

  #parser: Parser;

  #url: URL;

  #nodeData: NodeData | null = null;

  #expand = false;

  #selected = false;

  #error: string | null = null;

  constructor(parser: Parser, url: URL) {
    super();
    this.#parser = parser;
    this.#url = url;
  }

  init() {
    if (!this.#initialized) {
      this.innerHTML = template;
      this.#initialized = true;
    }
  }

  render() {
    const plusMinusSpan = this.querySelector('.plusminus') as HTMLSpanElement;
    const errorSpan = this.querySelector('.node-error') as HTMLSpanElement;
    const nameSpan = this.querySelector('.node-name') as HTMLSpanElement;

    setElementAttribute(this, 'url', this.#url.toString());
    // do not bind 'this' as that results in a new callable and thus multiple listeners
    nameSpan.addEventListener('click', this.onSelected);

    if (this.#nodeData === null) {
      // while loading data ...
      plusMinusSpan.style.display = 'none';
      errorSpan.style.display = 'none';
      nameSpan.innerHTML = 'Loading<sf-dancing-dots></sf-dancing-dots>';
      return;
    }

    // after data has been loaded ...
    // span is focusable and thus is keyboard event target and requires URL
    setElementAttribute(nameSpan, 'url', this.#url.toString());
    // render something as name even if name is blank
    const renderName =
      this.name === '' || this.name === null ? '""' : this.name;
    setElementTextContent(nameSpan, renderName);
    if (this.#selected) {
      nameSpan.classList.add('selected');
    }

    // show error if any
    if (this.#error !== null) {
      errorSpan.style.display = '';
      setElementTextContent(nameSpan, `ERROR: ${this.name}`);
      nameSpan.setAttribute('title', this.#error);
    } else {
      errorSpan.style.display = 'none';
      nameSpan.removeAttribute('title');
    }

    const numChildNodes = this.#nodeData.childNodeNames.length;
    const hasChildren = numChildNodes > 0;
    if (hasChildren) {
      this.setAttribute('expand', `${this.#expand}`);
      plusMinusSpan.style.display = '';
      plusMinusSpan.textContent = this.#expand ? '⊟' : '⊞';
      plusMinusSpan.addEventListener('click', this.onToggleCollapsed);
    }

    if (hasChildren) {
      if (this.#expand) {
        // add child nodes
        for (let i = 0; i < this.#nodeData.childNodeNames.length; i += 1) {
          const childUrl = new URL(this.#url);
          if (!this.#url.hash.endsWith('/')) {
            childUrl.hash += '/';
          }
          const childNodeName = this.#nodeData.childNodeNames[i];
          childUrl.hash += `${i}-${encodeURIComponent(childNodeName)}`;
          const childNode = new TreeNode(this.#parser, childUrl);
          this.appendChild(childNode);
        }
      } else {
        // remove child nodes
        const numChildren = this.children.length;
        for (let i = numChildren; i >= 0; i -= 1) {
          const child = this.children[i];
          if (child instanceof TreeNode) {
            this.removeChild(child);
          }
        }
      }
    }
  }

  async #retrieveNodeData() {
    try {
      const data = await this.#parser.read(this.#url);
      this.#nodeData = data;
    } catch (error: any) {
      const detail = error.detail ? error.detail : error;
      const errorMessage = `Error reading node: "${this.#url}". ${detail}`;
      this.#error = errorMessage;
      this.#nodeData = {
        url: this.#url,
        parameters: [{ key: 'Error', value: errorMessage }],
        data: [],
        table: { columnNames: [], rows: [] },
        childNodeNames: [],
        metadata: {},
      };
      this.#channel.dispatch('sf-error', errorMessage);
      console.error(errorMessage);
    }
    this.#channel.dispatch(nodeDataUpdatedEvent, this.#nodeData);
    this.render();
  }

  static #extractName(path: string): string {
    const segments: string[] = path.split('/');
    if (segments.length === 0) {
      return '';
    }
    let name = segments.pop()!.trim();
    if (name === '' || name === undefined) {
      name = segments.length > 0 ? segments.pop()!.trim() : '';
    }
    return decodeURIComponent(name);
  }

  get name() {
    const hash = this.#url.hash.trim();
    if (hash === '' || hash === '#' || hash === '#/') {
      return TreeNode.#extractName(this.#url.pathname);
    }
    const prefixedName = TreeNode.#extractName(hash);
    const hyphenIndex = prefixedName.indexOf('-');
    // display name without prefixed index
    return prefixedName.substring(hyphenIndex + 1);
  }

  get parser() {
    return this.#parser;
  }

  setSelected(selected: boolean) {
    this.#selected = selected;
    if (selected) {
      this.classList.add('selected');
      let nodeData = this.#nodeData;
      if (nodeData == null) {
        nodeData = {
          url: this.#url,
          parameters: [],
          data: [],
          metadata: {},
          table: { columnNames: [], rows: [] },
          childNodeNames: [],
        };
      }
      const nameElement = this.querySelector('.node-name') as HTMLElement;
      // focus on node name, but do not show outline
      nameElement.focus({ focusVisible: false } as FocusOptions);
      this.#channel.dispatch(nodeSelectedEvent, nodeData);
    } else {
      this.classList.remove('selected');
      this.#channel.dispatch(nodeDeselectedEvent, { url: this.#url });
    }
  }

  setExpand(expand: boolean) {
    if (this.#expand === expand) {
      return;
    }
    this.#expand = expand;
    this.render();
  }

  async close() {
    try {
      await this.#parser.close();
    } catch (error) {
      console.warn(`Error closing file "${this.#url}": ${error?.toString()}`);
    }
  }

  // #region user events

  onToggleCollapsed = () => {
    this.#expand = !this.#expand;
    this.render();
  };

  onSelected = () => {
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
    this.init();
    this.#eventListener = this.#channel.addListener(
      'sf-tree-node-selected',
      this.handleTreeNodeSelected.bind(this),
    );
    this.#retrieveNodeData();
    this.render();
  }

  disconnectedCallback() {
    // do not call close() as this may not be a root node
    // close() is handled by Tree
    if (this.#selected) {
      this.setSelected(false);
    }
    this.#channel.removeListener(this.#eventListener);
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    this.init();
  }

  // #endregion lifecycle events
}

customElements.define('sf-tree-node', TreeNode);
