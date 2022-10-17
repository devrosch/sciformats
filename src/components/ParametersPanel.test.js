import {} from './ParametersPanel';

test('sf-parameters-panel renders', async () => {
  const title = "Test Title";
  document.body.innerHTML = `<sf-parameters-panel title="${title}"></sf-parameters-panel>`;
  expect(document.body.innerHTML).toContain(title);

  const data = [
    { key: 'testKey1', value: 'testValue1' },
    { key: 'testKey2', value: 'testValue2' },
  ];
  expect(document.body.innerHTML).not.toContain(data[0].key);
  expect(document.body.innerHTML).not.toContain(data[0].value);
  expect(document.body.innerHTML).not.toContain(data[1].key);
  expect(document.body.innerHTML).not.toContain(data[1].value);

  const panel = document.body.querySelector('sf-parameters-panel');
  panel.data = data;
  expect(document.body.innerHTML).toContain(data[0].key);
  expect(document.body.innerHTML).toContain(data[0].value);
  expect(document.body.innerHTML).toContain(data[1].key);
  expect(document.body.innerHTML).toContain(data[1].value);

  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = ``;
})