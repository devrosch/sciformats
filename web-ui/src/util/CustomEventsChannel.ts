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

import Message from 'model/Message';
import Channel from 'model/Channel';

const elementName = 'sf-message-bus';

export default class CustomEventsChannel implements Channel {
  #name: string;

  #domElement: HTMLUnknownElement;

  constructor(channelName: string) {
    const bus = CustomEventsChannel.findOrCreateNode(channelName);
    this.#name = channelName;
    this.#domElement = bus;
  }

  static findOrCreateNode(name: string): HTMLUnknownElement {
    let bus: HTMLUnknownElement | null = document.getElementById(
      name,
    ) as HTMLUnknownElement | null;
    if (typeof bus === 'undefined' || bus === null) {
      bus = document.createElement(elementName);
      bus.id = name;
      bus.style.display = 'hidden';
      const firstChild = document.body.firstChild;
      if (firstChild) {
        document.body.insertBefore(bus, firstChild);
      } else {
        document.body.appendChild(bus);
      }
    }
    return bus;
  }

  get name() {
    return this.#name;
  }

  dispatch(name: string, detail: any) {
    this.#domElement.dispatchEvent(
      new CustomEvent(name, {
        bubbles: true,
        cancelable: true,
        composed: true,
        detail,
      }),
    );
  }

  addListener(name: string, listener: (message: Message) => void) {
    const ceHandler = {
      meta: {
        name,
        channel: this.#name,
      },
      customEventListener: (e: Event) => {
        const ce = e as CustomEvent;
        const message: Message = new Message(name, ce.detail);
        listener(message);
      },
    };
    this.#domElement.addEventListener(name, ceHandler.customEventListener);
    return ceHandler;
  }

  removeListener(handle: any) {
    const channelName = handle?.meta?.channel;
    if (channelName !== this.#name) {
      throw new Error('Illegal listener for removal from this channel.');
    }
    this.#domElement.removeEventListener(
      handle.meta.name,
      handle.customEventListener,
    );
  }
}
