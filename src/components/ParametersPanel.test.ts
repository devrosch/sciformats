/* eslint-disable import/no-duplicates */
import StubDataRepository from 'model/StubDataRepository';
import './ParametersPanel'; // for side effects
import ParametersPanel from './ParametersPanel';

test('sf-parameters-panel renders', async () => {
  const element = 'sf-parameters-panel';
  const titleAttr = 'title';
  const title = 'Test Title';
  const title2 = 'Test Title2';
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

  const panel = document.body.querySelector(element) as ParametersPanel;
  panel.data = data;
  expect(document.body.innerHTML).toContain(data[0].key);
  expect(document.body.innerHTML).toContain(data[0].value);
  expect(document.body.innerHTML).toContain(data[1].key);
  expect(document.body.innerHTML).toContain(data[1].value);
  panel.setAttribute(titleAttr, title2);
  expect(document.body.innerHTML).toContain(title2);

  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-parameters-panel reacts to sf-tree-node-changed events', async () => {
  const urlChild2 = new URL('file:///test/path/root.txt#/child 2');

  const repo = new StubDataRepository();
  const panel = new ParametersPanel(repo);
  document.body.append(panel);

  expect(document.body.innerHTML).not.toContain('key 1');
  expect(document.body.innerHTML).not.toContain('value 1');
  expect(document.body.innerHTML).not.toContain('key 2');
  expect(document.body.innerHTML).not.toContain('value 2');
  expect(document.body.innerHTML).not.toContain('key 3');
  expect(document.body.innerHTML).not.toContain('value 3');

  window.dispatchEvent(new CustomEvent('sf-tree-node-selected', {
    bubbles: true,
    cancelable: true,
    composed: true,
    detail: { url: urlChild2 },
  }));

  expect(document.body.innerHTML).toContain('key 1');
  expect(document.body.innerHTML).toContain('value 1');
  expect(document.body.innerHTML).toContain('key 2');
  expect(document.body.innerHTML).toContain('value 2');
  expect(document.body.innerHTML).not.toContain('key 3');
  expect(document.body.innerHTML).not.toContain('value 3');

  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});
