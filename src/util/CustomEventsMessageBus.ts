import MessageBus from 'model/MessageBus';
import Message from 'model/Message';

export default class CustomEventsMessageBus implements MessageBus {
  #defaultChannelName: string = 'sf-default-channel';

  #messageBus: HTMLUnknownElement;

  constructor(channelName?: string) {
    let bus = null;
    if (channelName !== null && typeof channelName !== 'undefined') {
      bus = CustomEventsMessageBus.findOrCreateNode(channelName);
    } else {
      bus = CustomEventsMessageBus.findOrCreateNode(this.#defaultChannelName);
    }
    this.#messageBus = bus;
  }

  static findOrCreateNode(name: string): HTMLUnknownElement {
    let bus: HTMLUnknownElement | null = document.getElementById(name) as HTMLUnknownElement | null;
    if (typeof bus === 'undefined' || bus === null) {
      bus = document.createElement('sf-message-bus');
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

  dispatch(name: string, detail: any, channel?: string) {
    if (channel !== null && typeof channel !== 'undefined') {
      throw new Error('Non default channel not supported.');
    }
    this.#messageBus.dispatchEvent(new CustomEvent(name, {
      bubbles: true,
      cancelable: true,
      composed: true,
      detail,
    }));
  }

  addListener(name: string, listener: (message: Message) => void, channel?: string) {
    if (channel !== null && typeof channel !== 'undefined') {
      throw new Error('Non default channel not supported.');
    }

    const ceHandler = {
      meta: {
        name,
        channel,
      },
      customEventListener: (e: Event) => {
        const ce = e as CustomEvent;
        const message: Message = new Message(name, ce.detail);
        listener(message);
      },
    };
    this.#messageBus.addEventListener(name, ceHandler.customEventListener);
    return ceHandler;
  }

  removeListener(handle: any) {
    const channel = handle.meta.channel;
    if (channel !== null && typeof channel !== 'undefined') {
      throw new Error('Non default channel not supported.');
    }
    this.#messageBus.removeEventListener(handle.meta.name, handle.customEventListener);
  }
}
