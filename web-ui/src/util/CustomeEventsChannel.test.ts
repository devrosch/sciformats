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

import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Channel from 'model/Channel';
import Message from 'model/Message';

const checker = (
  eventType: string,
  payload: { test: string },
  channel: Channel,
) => {
  const eventHandler = jest.fn((message: Message) => message.detail);

  expect(eventHandler).toHaveBeenCalledTimes(0);
  channel.dispatch(eventType, payload);
  expect(eventHandler).toHaveBeenCalledTimes(0);

  const handle = channel.addListener(eventType, eventHandler);
  channel.dispatch(eventType, payload);
  expect(eventHandler).toHaveBeenCalledTimes(1);
  expect(eventHandler).toHaveLastReturnedWith(payload);

  channel.removeListener(handle);
  channel.dispatch(eventType, payload);
  expect(eventHandler).toHaveBeenCalledTimes(1);
};

test('registering/unregistering listener and message dispatch succeeds for default channel', async () => {
  const eventType = 'sf-dispatch-event-test';
  const payload = { test: 'test payload' };
  const channel = CustomEventsMessageBus.getDefaultChannel();

  checker(eventType, payload, channel);
});

test('registering/unregistering listener and message dispatch succeeds for non default channel', async () => {
  const eventType = 'sf-dispatch-event-test';
  const channelName = 'sf-test-channel';
  const payload = { test: 'test payload' };
  const channel = CustomEventsMessageBus.getChannel(channelName);

  checker(eventType, payload, channel);
});

test('fails when tyring to remove a listener for a different channel', async () => {
  const eventType = 'sf-dispatch-event-test';
  const channelName0 = 'sf-test-channel0';
  const channelName1 = 'sf-test-channel1';
  const channel0 = CustomEventsMessageBus.getChannel(channelName0);
  const channel1 = CustomEventsMessageBus.getChannel(channelName1);

  const handle0 = channel0.addListener(eventType, () => {
    /* noop */
  });
  expect(() => channel1.removeListener(handle0)).toThrow();
});
