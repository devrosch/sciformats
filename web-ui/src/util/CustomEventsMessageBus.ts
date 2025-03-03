import CustomEventsChannel from 'util/CustomEventsChannel';

/**
 * A message bus for sending/receiving application messages.
 * Based on custom events.
 */
/* eslint-disable-next-line @typescript-eslint/no-extraneous-class */
export default class CustomEventsMessageBus {
  static #defaultChannelName = 'sf-default-channel';

  /**
   * Returns the default channel.
   */
  static getDefaultChannel() {
    return new CustomEventsChannel(CustomEventsMessageBus.#defaultChannelName);
  }

  /**
   * Returns a named channel.
   * @param channelName The channel name.
   */
  static getChannel(name: string) {
    return new CustomEventsChannel(name);
  }
}
