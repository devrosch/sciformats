import Channel from 'model/Channel';
import Message from 'model/Message';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';

const checker = (eventType: string, payload: { test: string }, channel: Channel) => {
  let numCalls = 0;
  let receivedPayload: any = null;
  const eventHandler = (message: Message) => {
    numCalls += 1;
    receivedPayload = message.detail;
  };

  expect(numCalls).toBe(0);
  channel.dispatch(eventType, payload);
  expect(numCalls).toBe(0);

  const handle = channel.addListener(eventType, eventHandler);
  channel.dispatch(eventType, payload);
  expect(numCalls).toBe(1);
  expect(receivedPayload.test).toBe(payload.test);

  channel.removeListener(handle);
  channel.dispatch(eventType, payload);
  expect(numCalls).toBe(1);
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

  const handle0 = channel0.addListener(eventType, () => {});
  expect(() => channel1.removeListener(handle0)).toThrow();
});
