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
