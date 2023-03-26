import Message from 'model/Message';

/**
 * A message bus for sending/receiving application events.
 */
export default interface Channel {
  /**
   * The channel name.
   */
  readonly name: string;

  /**
   * Dispatches a message.
   * @param name The message name.
   * @param detail Message details.
   */
  dispatch(name: string, detail: any): void;

  /**
   * Registers a listener for messages.
   * @param name The message name to listen to.
   * @param listener The listener function.
   * @return A handle for the listener. Required to unregister.
   */
  addListener(name: string, listener: (message: Message) => void): any;

  /**
   * Unregisters a listener.
   * @param handle The handle for the listener to unregister as returned by addListener.
   */
  removeListener(handle: any): void;
}
