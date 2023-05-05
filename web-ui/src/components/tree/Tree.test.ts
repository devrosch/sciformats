/* eslint-disable import/no-duplicates */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Message from 'model/Message';
import Parser from 'model/Parser';
import NodeData from 'model/NodeData';
import './Tree'; // for side effects
import Tree from './Tree';
import TreeNode from './TreeNode';

// a StubParser really, but jest requires the name to start with "Mock"
// see: https://jestjs.io/docs/es6-class-mocks#calling-jestmock-with-the-module-factory-parameter
class MockParser implements Parser {
  readonly prefix = 'file:///dummy/path/';

  rootUrl: URL;

  constructor(file: File) {
    this.rootUrl = new URL(`${this.prefix}${file.name}#/`);
  }

  // eslint-disable-next-line class-methods-use-this
  async open() {
    // noop
  }

  // eslint-disable-next-line class-methods-use-this
  read(url: URL): Promise<NodeData> {
    const data: { x: number, y: number }[] = [];
    const parameters: { key: string, value: string }[] = [];
    const children: string[] = ['child1', 'child2'];

    const nodeData = {
      url,
      data,
      parameters,
      children,
    };

    return new Promise((resolve) => { resolve(nodeData); });
  }

  // eslint-disable-next-line class-methods-use-this
  async close() {
    // noop
  }
}

jest.mock('model/ParserRepository', () => jest.fn().mockImplementation(
  () => ({ findParser: async (file: File) => new MockParser(file) }),
));

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
  for (const name of fileNames) {
    const file = new File([blob], name);
    files.push(file);
  }
  const message = new Message(fileOpenedEvent, { files });
  return message;
};

const waitForChildrenCount = async (el: HTMLElement, childrenCount: Number) => {
  // wait for DOM change
  while (el.children.length !== childrenCount) {
    // eslint-disable-next-line no-await-in-loop
    await new Promise((resolve) => { setTimeout(resolve, 1); });
  }
};

const prepareTreeStructure = async () => {
  // prepare structure
  // Tree
  //   |
  //   +- dummy.txt (selected)
  //   |     |
  //   |     +- child1
  //   |     |
  //   |     +- child2
  //   |
  //   +- dummy2.txt
  document.body.innerHTML = `<${element}/>`;
  const tree = document.body.querySelector(element) as Tree;
  expect(tree.children).toHaveLength(0);
  const message = prepareFileOpenMessage([fileName, fileName2]);
  tree.handleFilesOpenRequested(message);
  await waitForChildrenCount(tree, 2);
  expect(tree.children).toHaveLength(2);
  const root1 = tree.children[0] as TreeNode;
  root1.setExpand(true);
  const root1ChildNodes = root1.querySelectorAll('sf-tree-node');
  expect(root1ChildNodes).toHaveLength(2);
  await root1.setSelected(true);
  expect(root1.classList).toContain('selected');

  return {
    tree,
    root1,
    child1: root1ChildNodes[0] as TreeNode,
    child2: root1ChildNodes[1] as TreeNode,
    root2: tree.children[1] as TreeNode,
  };
};

const prepareActualKeyDownEvent = (key: string) => {
  const event = new KeyboardEvent(
    'keydown',
    // modifier keys for Linux, guaranteed to be used by mock at beginning of file
    {
      key, shiftKey: true, ctrlKey: false, altKey: true, metaKey: false, bubbles: true,
    },
  );
  return event;
};

const prepareStubKeyDownEvent = (key: String, target: Element) => {
  const event = {
    key,
    target,
    preventDefault: () => { },
  };
  return event as unknown as KeyboardEvent;
};

const prepareListener = (done: (value: unknown) => void) => {
  const listener = () => {
    try {
      done('success');
    } catch (error) {
      done(error);
    }
  };
  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.addListener('sf-tree-node-selected', listener);
  return listener;
};

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
  await waitForChildrenCount(tree, 1);

  expect(tree.children).toHaveLength(1);
  const treeNode = tree.querySelector(nodeElement) as TreeNode;
  expect(treeNode).toBeTruthy();
  expect(treeNode.hasAttribute(urlAttr)).toBeTruthy();
  expect(treeNode.getAttribute(urlAttr)).toMatch(urlRegex);

  channel.dispatch(fileOpenedEvent, { files: [file] });
  await waitForChildrenCount(tree, 2);

  expect(tree.children).toHaveLength(2);
});

test('sf-tree listenes to file close events', async () => {
  document.body.innerHTML = `<${element}/>`;

  const tree = document.body.querySelector(element) as Tree;
  expect(tree.children.length).toBe(0);

  const message = prepareFileOpenMessage([fileName, fileName2, fileName3]);
  tree.handleFilesOpenRequested(message);
  await waitForChildrenCount(tree, 3);

  expect(tree.children).toHaveLength(3);
  // no node selected => noop
  tree.handleFileCloseRequested();
  // allow for potential changes to take place
  await new Promise((resolve) => { setTimeout(resolve, 10); });

  expect(tree.children).toHaveLength(3);

  const child0 = tree.children.item(0) as TreeNode;
  const child1 = tree.children.item(1) as TreeNode;
  const child2 = tree.children.item(2) as TreeNode;
  child1.setSelected(true);
  tree.handleFileCloseRequested();
  await waitForChildrenCount(tree, 2);

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
  await waitForChildrenCount(tree, 3);

  expect(tree.children).toHaveLength(3);
  tree.handleFileCloseAllRequested();
  expect(tree.children).toHaveLength(0);
});

test('sf-tree observes key down events', async () => {
  // workaround for using "done" in async method
  // see: https://github.com/facebook/jest/issues/11404
  let done: (value: unknown) => void = () => { };

  const nodes = await prepareTreeStructure();
  const arrowDownEvent = prepareActualKeyDownEvent('ArrowDown');

  // root1
  const callback1Resolved = new Promise((resolve) => { done = resolve; });
  prepareListener(done);
  nodes.root1.dispatchEvent(arrowDownEvent);
  await callback1Resolved;
  expect(nodes.root1.classList).not.toContain('selected');
  expect(nodes.child1.classList).toContain('selected');
});

test('sf-tree ArrowUp keyboard navigation moves to previous node above', async () => {
  const nodes = await prepareTreeStructure();
  await nodes.root2.setSelected(true);
  expect(nodes.root2.classList).toContain('selected');

  Tree.onKeyDown(prepareStubKeyDownEvent('ArrowUp', nodes.root2));
  expect(nodes.root2.classList).not.toContain('selected');
  expect(nodes.child2.classList).toContain('selected');

  Tree.onKeyDown(prepareStubKeyDownEvent('ArrowUp', nodes.child2));
  expect(nodes.child2.classList).not.toContain('selected');
  expect(nodes.child1.classList).toContain('selected');

  Tree.onKeyDown(prepareStubKeyDownEvent('ArrowUp', nodes.child1));
  expect(nodes.child1.classList).not.toContain('selected');
  expect(nodes.root1.classList).toContain('selected');

  Tree.onKeyDown(prepareStubKeyDownEvent('ArrowUp', nodes.root1));
  expect(nodes.root1.classList).toContain('selected');
});

test('sf-tree ArrowDown keyboard navigation moves to next node below', async () => {
  const nodes = await prepareTreeStructure();
  expect(nodes.root1.classList).toContain('selected');

  Tree.onKeyDown(prepareStubKeyDownEvent('ArrowDown', nodes.root1));
  expect(nodes.root1.classList).not.toContain('selected');
  expect(nodes.child1.classList).toContain('selected');

  Tree.onKeyDown(prepareStubKeyDownEvent('ArrowDown', nodes.child1));
  expect(nodes.child1.classList).not.toContain('selected');
  expect(nodes.child2.classList).toContain('selected');

  Tree.onKeyDown(prepareStubKeyDownEvent('ArrowDown', nodes.child2));
  expect(nodes.child2.classList).not.toContain('selected');
  expect(nodes.root2.classList).toContain('selected');

  Tree.onKeyDown(prepareStubKeyDownEvent('ArrowDown', nodes.root2));
  expect(nodes.root2.classList).toContain('selected');
});

test('sf-tree ArrowRight keyboard navigation expands node', async () => {
  const nodes = await prepareTreeStructure();
  await nodes.root2.setSelected(true);
  expect(nodes.root2.classList).toContain('selected');
  expect(nodes.root2.getAttribute('expand')).toBe('false');

  Tree.onKeyDown(prepareStubKeyDownEvent('ArrowRight', nodes.root2));
  expect(nodes.root2.getAttribute('expand')).toBe('true');
});

test('sf-tree ArrowLeft keyboard navigation collapses node', async () => {
  const nodes = await prepareTreeStructure();
  expect(nodes.root1.classList).toContain('selected');
  expect(nodes.root1.getAttribute('expand')).toBe('true');

  Tree.onKeyDown(prepareStubKeyDownEvent('ArrowLeft', nodes.root1));
  expect(nodes.root1.getAttribute('expand')).toBe('false');
});

test('sf-tree Enter keyboard navigation toggles node collapse', async () => {
  const nodes = await prepareTreeStructure();
  expect(nodes.root1.classList).toContain('selected');
  expect(nodes.root1.getAttribute('expand')).toBe('true');

  Tree.onKeyDown(prepareStubKeyDownEvent('Enter', nodes.root1));
  expect(nodes.root1.getAttribute('expand')).toBe('false');
  Tree.onKeyDown(prepareStubKeyDownEvent('Enter', nodes.root1));
  expect(nodes.root1.getAttribute('expand')).toBe('true');
});

test('sf-tree tree click events result in selected tree node to receive focus', async () => {
  const nodes = await prepareTreeStructure();
  const nameSpan = nodes.root1.querySelector('.node-name') as HTMLSpanElement;
  nameSpan.focus();
  expect(document.activeElement).toBe(nameSpan);

  nameSpan.blur();
  expect(document.activeElement).not.toBe(nameSpan);

  nodes.tree.click();
  expect(document.activeElement).toBe(nameSpan);
});
