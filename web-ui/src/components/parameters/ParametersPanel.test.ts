/* eslint-disable import/no-duplicates */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import './ParametersPanel'; // for side effects
import ParametersPanel from './ParametersPanel';

const element = 'sf-parameters-panel';
const titleAttr = 'title';
const title = 'Test Title';
const title2 = 'Test Title2';
const parameters = [
  { key: 'testKey1', value: 'testValue1' },
  { key: 'testKey2', value: 'testValue2' },
];

const waitForChildrenCount = async (el: HTMLElement, childrenCount: number) => {
  // wait for DOM change
  while (el.children.length !== childrenCount) {
    /* eslint-disable-next-line no-await-in-loop */
    await new Promise((resolve) => { setTimeout(resolve, 1); });
  }
};

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-parameters-panel renders', async () => {
  document.body.innerHTML = `<${element} ${titleAttr}="${title}"/>`;
  expect(document.body.innerHTML).toContain(title);
  expect(document.body.innerHTML).not.toContain(parameters[0].key);
  expect(document.body.innerHTML).not.toContain(parameters[0].value);
  expect(document.body.innerHTML).not.toContain(parameters[1].key);
  expect(document.body.innerHTML).not.toContain(parameters[1].value);

  const panel = document.body.querySelector(element) as ParametersPanel;
  panel.data = parameters;
  const ul = panel.querySelector('ul');
  await waitForChildrenCount(ul!, 2);
  expect(document.body.innerHTML).toContain(parameters[0].key);
  expect(document.body.innerHTML).toContain(parameters[0].value);
  expect(document.body.innerHTML).toContain(parameters[1].key);
  expect(document.body.innerHTML).toContain(parameters[1].value);
  panel.setAttribute(titleAttr, title2);
  expect(document.body.innerHTML).toContain(title2);
});

test('sf-parameters-panel reacts to sf-tree-node-(de)selected events', async () => {
  const urlChild2 = new URL('file:///test/path/root.txt#/child 2');

  const panel = new ParametersPanel();
  document.body.append(panel);

  const channel = CustomEventsMessageBus.getDefaultChannel();

  expect(document.body.innerHTML).not.toContain(parameters[0].key);
  expect(document.body.innerHTML).not.toContain(parameters[0].value);
  expect(document.body.innerHTML).not.toContain(parameters[1].key);
  expect(document.body.innerHTML).not.toContain(parameters[1].value);

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data: null,
    parameters,
  });
  let ul = panel.querySelector('ul');
  await waitForChildrenCount(ul!, 2);
  expect(document.body.innerHTML).toContain(parameters[0].key);
  expect(document.body.innerHTML).toContain(parameters[0].value);
  expect(document.body.innerHTML).toContain(parameters[1].key);
  expect(document.body.innerHTML).toContain(parameters[1].value);

  channel.dispatch('sf-tree-node-deselected', { url: urlChild2 });
  ul = panel.querySelector('ul');
  await waitForChildrenCount(ul!, 0);
  expect(document.body.innerHTML).not.toContain(parameters[0].key);
  expect(document.body.innerHTML).not.toContain(parameters[0].value);
  expect(document.body.innerHTML).not.toContain(parameters[1].key);
  expect(document.body.innerHTML).not.toContain(parameters[1].value);
});
