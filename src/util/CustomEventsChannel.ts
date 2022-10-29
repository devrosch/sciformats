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
    let bus: HTMLUnknownElement | null = document.getElementById(name) as HTMLUnknownElement | null;
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
    this.#domElement.dispatchEvent(new CustomEvent(name, {
      bubbles: true,
      cancelable: true,
      composed: true,
      detail,
    }));
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
      throw new Error('Illegal listener for for removal from this channel.');
    }
    this.#domElement.removeEventListener(handle.meta.name, handle.customEventListener);
  }
}
