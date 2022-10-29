import CustomEventsMessageBus from 'util/CustomEventsMessageBus';

test('getting default channel succeeds', async () => {
  const channel = CustomEventsMessageBus.getDefaultChannel();
  expect(channel).toBeDefined();
  expect(channel).not.toBeNull();
});

test('getting named channel succeeds', async () => {
  const channelName = 'sf-test-name';
  const channel = CustomEventsMessageBus.getChannel(channelName);
  expect(channel).toBeDefined();
  expect(channel).not.toBeNull();
  expect(channel.name).toBe(channelName);
});
