/* eslint-disable no-duplicate-imports */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import './Footer'; // for side effects
import Footer from './Footer';

const element = 'sf-footer';
const data: { x: number; y: number }[] = [];
const urlChild2 = new URL('file:///test/path/root.txt#/child 2');

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-footer renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  const footer = document.body.querySelector(element);
  expect(footer).toBeTruthy();
  const span = footer?.querySelector('span');
  expect(span).toBeTruthy();
  expect(span?.textContent).toBe('');
});

test('sf-footer reacts to sf-tree-node-(de)selected events', async () => {
  const footer = new Footer();
  document.body.append(footer);
  const channel = CustomEventsMessageBus.getDefaultChannel();

  const span = footer.querySelector('span');
  expect(span).toBeTruthy();
  expect(span?.textContent).toBe('');

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data,
    parameters: null,
  });
  expect(span?.textContent).toBe(urlChild2.toString());

  channel.dispatch('sf-tree-node-deselected', { url: urlChild2 });
  expect(span?.textContent).toBe('');
});
