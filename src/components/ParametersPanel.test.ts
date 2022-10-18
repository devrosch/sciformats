import './ParametersPanel'; // for side effects
import ParametersPanel from './ParametersPanel';

test('sf-parameters-panel renders', async () => {
  const element = 'sf-parameters-panel';
  const titleAttr = 'title';
  const title = "Test Title";
  const data = [
    { key: 'testKey1', value: 'testValue1' },
    { key: 'testKey2', value: 'testValue2' },
  ];

  document.body.innerHTML = `<${element} ${titleAttr}="${title}"/>`;
  expect(document.body.innerHTML).toContain(title);
  expect(document.body.innerHTML).not.toContain(data[0].key);
  expect(document.body.innerHTML).not.toContain(data[0].value);
  expect(document.body.innerHTML).not.toContain(data[1].key);
  expect(document.body.innerHTML).not.toContain(data[1].value);

  let panel = document.body.querySelector(element) as ParametersPanel;
  panel.data = data;
  expect(document.body.innerHTML).toContain(data[0].key);
  expect(document.body.innerHTML).toContain(data[0].value);
  expect(document.body.innerHTML).toContain(data[1].key);
  expect(document.body.innerHTML).toContain(data[1].value);

  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = ``;
})