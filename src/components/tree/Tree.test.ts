/* eslint-disable import/no-duplicates */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Message from 'model/Message';
import './Tree'; // for side effects
import Tree from './Tree';
import TreeNode from './TreeNode';

const element = 'sf-tree';
const nodeElement = 'sf-tree-node';
const fileOpenedEvent = 'sf-file-open-requested';
const fileContent = 'abc';
const fileName = 'dummy.txt';
const fileName2 = 'dummy2.txt';
const fileName3 = 'dummy3.txt';
const urlAttr = 'url';
const urlRegex = new RegExp(`file:///.*/${fileName}#/`);

const prepareFileOpenMessage = (fileNames: string[]) => {  
  const blob = new Blob([fileContent]);
  const files = [];
  for (const fileName of fileNames) {
    const file = new File([blob], fileName);  
    files.push(file);
  }
  const message = new Message(fileOpenedEvent, { files });
  return message;
};

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

test('sf-tree listenes to file close events', async () => {
  document.body.innerHTML = `<${element}/>`;
  
  const tree = document.body.querySelector(element) as Tree;
  expect(tree.children.length).toBe(0);

  const message = prepareFileOpenMessage([fileName, fileName2, fileName3]);
  tree.handleFilesOpenRequested(message);

  expect(tree.children).toHaveLength(3);
  // no node selected => noop
  tree.handleFileCloseRequested();

  expect(tree.children).toHaveLength(3);
  
  const child0 = tree.children.item(0) as TreeNode;
  const child1 = tree.children.item(1) as TreeNode;
  const child2 = tree.children.item(2) as TreeNode;
  child1.setSelected(true);
  tree.handleFileCloseRequested();

  expect(tree.children).toHaveLength(2);
  expect(tree.children.item(0)).toBe(child0);
  // child2 moved to position 1
  expect(tree.children.item(1)).toBe(child2);
});

test('sf-tree listenes to file close all events', async () => {
  document.body.innerHTML = `<${element}/>`;
  
  const tree = document.body.querySelector(element) as Tree;
  expect(tree.children.length).toBe(0);

  const message = prepareFileOpenMessage([fileName, fileName2, fileName3]);
  tree.handleFilesOpenRequested(message);

  expect(tree.children).toHaveLength(3);
  tree.handleFileCloseAllRequested();
  expect(tree.children).toHaveLength(0);
});
