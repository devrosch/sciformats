// TODO: disable rule for now
/* eslint-disable class-methods-use-this */
import MessageBus from 'model/MessageBus';
import Message from 'model/Message';

export default class CustomEventsMessageBus implements MessageBus {
  dispatch(name: string, detail: any, channel?: string) {
    if (channel !== null && typeof channel !== 'undefined') {
      throw new Error('Non default channel not supported.');
    }
    window.dispatchEvent(new CustomEvent(name, {
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
      listener: (e: Event) => {
        const ce = e as CustomEvent;
        const message: Message = new Message(name, ce.detail);
        listener(message);
      },
    };
    window.addEventListener(name, ceHandler.listener);
    return ceHandler;
  }

  removeListener(handle: any) {
    const channel = handle.meta.channel;
    if (channel !== null && typeof channel !== 'undefined') {
      throw new Error('Non default channel not supported.');
    }

    window.removeEventListener(handle.meta.name, handle.listener);
  }
}
