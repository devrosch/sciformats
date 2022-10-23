import { dispatchWindowCustomEvent } from './EventUtils';

test('dispatchWindowCustomEvent() dispatches', async () => {
  const eventType = 'sf-dispatch-event-test';
  const payload = { test: 'test payload' };

  let receivedPayload: any = null;
  const eventHandler = (e: Event) => {
    const ce = e as CustomEvent;
    receivedPayload = ce.detail;
  };
  window.addEventListener(eventType, eventHandler);
  dispatchWindowCustomEvent(eventType, payload);
  window.removeEventListener(eventType, eventHandler);

  expect(receivedPayload.test).toBe('test payload');
});
