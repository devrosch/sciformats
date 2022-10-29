import Message from 'model/Message';

/**
 * A message bus for sending/receiving application events.
 */
export default interface MessageBus {
  /**
   * Dispatches a message.
   * @param name The message name.
   * @param detail Message details.
   * @param channel The channel to dispatch the message to. If not specified, uses default channel.
   */
  dispatch(name: string, detail: any, channel?: string): void;

  /**
   * Registers a listener for messages.
   * @param name The message name to listen to.
   * @param listener The listener function.
   * @param channel The channel to listen to. If not specified, uses default channel.
   * @return A handle for the listener. Required to unregister.
   */
  addListener(name: string, listener: (message: Message) => void, channel?: string): any;

  /**
   * Unregisters a listener.
   * @param handle The handle for the listener to unregister as returned by addListener.
   */
  removeListener(handle: any): void;
}
