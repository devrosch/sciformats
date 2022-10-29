import Message from 'model/Message';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';

test('registering/unregistering listener and message dispatch succeeds', async () => {
  const eventType = 'sf-dispatch-event-test';
  const payload = { test: 'test payload' };

  const bus = new CustomEventsMessageBus();

  let numCalls = 0;
  let receivedPayload: any = null;
  const eventHandler = (message: Message) => {
    numCalls += 1;
    receivedPayload = message.detail;
  };

  expect(numCalls).toBe(0);
  bus.dispatch(eventType, payload);
  expect(numCalls).toBe(0);

  const handle = bus.addListener(eventType, eventHandler);
  bus.dispatch(eventType, payload);
  expect(numCalls).toBe(1);
  expect(receivedPayload.test).toBe('test payload');

  bus.removeListener(handle);
  bus.dispatch(eventType, payload);
  expect(numCalls).toBe(1);
});

test('using non default channels currently fails', async () => {
  const eventType = 'sf-dispatch-event-test';
  const channel = 'sf-non-default-channel';

  const bus = new CustomEventsMessageBus();
  const eventHandler = () => {};

  expect(() => bus.addListener(eventType, eventHandler, channel)).toThrow(Error);
});
