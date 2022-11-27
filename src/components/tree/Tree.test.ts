/* eslint-disable import/no-duplicates */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import './Tree'; // for side effects
import Tree from './Tree';
import TreeNode from './TreeNode';

const element = 'sf-tree';
const nodeElement = 'sf-tree-node';
const fileOpenedEvent = 'sf-files-open-requested';
const fileContent = 'abc';
const fileName = 'dummy.txt';
const urlAttr = 'url';
const urlRegex = new RegExp(`file:///.*/${fileName}#/`);

beforeAll(() => {
  window.crypto.randomUUID = jest.fn(() => 'aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee');
});

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-tree renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  expect(document.body.innerHTML).toContain(element);
});

test('sf-tree listenes to file open events', async () => {
  document.body.innerHTML = `<${element}/>`;
  
  const tree = document.body.querySelector(element) as Tree;
  expect(tree.children.length).toBe(0);

  const blob = new Blob([fileContent]);
  const file = new File([blob], fileName);

  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.dispatch(fileOpenedEvent, { files: [file] });

  expect(tree.children).toHaveLength(1);
  const treeNode = tree.querySelector(nodeElement) as TreeNode;
  expect(treeNode).toBeTruthy();
  expect(treeNode.hasAttribute(urlAttr)).toBeTruthy();
  expect(treeNode.getAttribute(urlAttr)).toMatch(urlRegex)
  expect(treeNode.innerHTML).toContain(fileName);

  channel.dispatch(fileOpenedEvent, { files: [file] });
  expect(tree.children).toHaveLength(2);
});
