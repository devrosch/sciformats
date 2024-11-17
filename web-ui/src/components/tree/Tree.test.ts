/* eslint-disable no-duplicate-imports */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import MockParser from 'model/__mocks__/MockParser';
import './Tree'; // for side effects
import Tree from './Tree';
import TreeNode from './TreeNode';

const element = 'sf-tree';
const nodeElement = 'sf-tree-node';
const fileContent = 'abc';
const fileName = 'dummy.txt';
const fileName2 = 'dummy2.txt';
const blob = new Blob([fileContent]);
const nodeInitText = 'Loading';

const waitForNodeExpansion = async (el: HTMLElement, childrenCount: number) => {
  // wait for DOM change
  while (el.querySelectorAll(nodeElement).length !== childrenCount) {
    /* eslint-disable-next-line no-await-in-loop */
    await new Promise((resolve) => {
      setTimeout(resolve, 1);
    });
  }
};

const waitForNodeInitialization = async (el: HTMLElement) => {
  // wait for DOM change
  while (!el.textContent?.includes(nodeInitText)) {
    /* eslint-disable-next-line no-await-in-loop */
    await new Promise((resolve) => {
      setTimeout(resolve, 1);
    });
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

  // prepareSimpleTree();
  document.body.innerHTML = `<${element}></${element}>`;
  const tree = document.body.querySelector(element) as Tree;
  expect(tree.children).toHaveLength(0);
  const mockParser0 = new MockParser(new File([blob], fileName));
  const mockParser1 = new MockParser(new File([blob], fileName2));
  tree.addRootNode(mockParser0);
  tree.addRootNode(mockParser1);
  // await waitForChildrenCount(tree, 2);
  expect(tree.children).toHaveLength(2);
  const root1 = tree.children[0] as TreeNode;
  root1.setExpand(true);
  await waitForNodeExpansion(root1, 2);
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
      key,
      shiftKey: true,
      ctrlKey: false,
      altKey: true,
      metaKey: false,
      bubbles: true,
    },
  );
  return event;
};

const prepareStubKeyDownEvent = (key: string, target: Element) => {
  const event = {
    key,
    target,
    preventDefault: () => {},
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
  // prepareSimpleTree();
  document.body.innerHTML = `<${element}></${element}>`;
  expect(document.body.innerHTML).toContain(element);
});

test('sf-tree observes key down events', async () => {
  // workaround for using "done" in async method
  // see: https://github.com/facebook/jest/issues/11404
  let done: (value: unknown) => void = () => {};

  const nodes = await prepareTreeStructure();
  const arrowDownEvent = prepareActualKeyDownEvent('ArrowDown');

  // root1
  const callback1Resolved = new Promise((resolve) => {
    done = resolve;
  });
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

  nodes.tree.onKeyDown(prepareStubKeyDownEvent('ArrowUp', nodes.root2));
  expect(nodes.root2.classList).not.toContain('selected');
  expect(nodes.child2.classList).toContain('selected');

  nodes.tree.onKeyDown(prepareStubKeyDownEvent('ArrowUp', nodes.child2));
  expect(nodes.child2.classList).not.toContain('selected');
  expect(nodes.child1.classList).toContain('selected');

  nodes.tree.onKeyDown(prepareStubKeyDownEvent('ArrowUp', nodes.child1));
  expect(nodes.child1.classList).not.toContain('selected');
  expect(nodes.root1.classList).toContain('selected');

  nodes.tree.onKeyDown(prepareStubKeyDownEvent('ArrowUp', nodes.root1));
  expect(nodes.root1.classList).toContain('selected');
});

test('sf-tree ArrowDown keyboard navigation moves to next node below', async () => {
  const nodes = await prepareTreeStructure();
  expect(nodes.root1.classList).toContain('selected');

  nodes.tree.onKeyDown(prepareStubKeyDownEvent('ArrowDown', nodes.root1));
  expect(nodes.root1.classList).not.toContain('selected');
  expect(nodes.child1.classList).toContain('selected');

  nodes.tree.onKeyDown(prepareStubKeyDownEvent('ArrowDown', nodes.child1));
  expect(nodes.child1.classList).not.toContain('selected');
  expect(nodes.child2.classList).toContain('selected');

  nodes.tree.onKeyDown(prepareStubKeyDownEvent('ArrowDown', nodes.child2));
  expect(nodes.child2.classList).not.toContain('selected');
  expect(nodes.root2.classList).toContain('selected');

  nodes.tree.onKeyDown(prepareStubKeyDownEvent('ArrowDown', nodes.root2));
  expect(nodes.root2.classList).toContain('selected');
});

test('sf-tree ArrowRight keyboard navigation expands node', async () => {
  const nodes = await prepareTreeStructure();
  await nodes.root2.setSelected(true);
  expect(nodes.root2.classList).toContain('selected');
  expect(nodes.root2.getAttribute('expand')).toBe('false');

  nodes.tree.onKeyDown(prepareStubKeyDownEvent('ArrowRight', nodes.root2));
  expect(nodes.root2.getAttribute('expand')).toBe('true');
});

test('sf-tree ArrowLeft keyboard navigation collapses node', async () => {
  const nodes = await prepareTreeStructure();
  expect(nodes.root1.classList).toContain('selected');
  expect(nodes.root1.getAttribute('expand')).toBe('true');

  nodes.tree.onKeyDown(prepareStubKeyDownEvent('ArrowLeft', nodes.root1));
  expect(nodes.root1.getAttribute('expand')).toBe('false');
});

test('sf-tree Enter keyboard navigation toggles node collapse', async () => {
  const nodes = await prepareTreeStructure();
  expect(nodes.root1.classList).toContain('selected');
  expect(nodes.root1.getAttribute('expand')).toBe('true');

  nodes.tree.onKeyDown(prepareStubKeyDownEvent('Enter', nodes.root1));
  expect(nodes.root1.getAttribute('expand')).toBe('false');
  nodes.tree.onKeyDown(prepareStubKeyDownEvent('Enter', nodes.root1));
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

test('sf-tree addRootNode() appends root node', async () => {
  const nodes = await prepareTreeStructure();

  expect(nodes.tree.children.length).toEqual(2);
  expect(nodes.root1.classList).toContain('selected');

  const mockParser = new MockParser(new File([blob], fileName));
  nodes.tree.addRootNode(mockParser);

  expect(nodes.root1.classList).toContain('selected');
  expect(nodes.tree.children.length).toEqual(3);
  const addedNode = nodes.tree.children[2] as TreeNode;
  await waitForNodeInitialization(addedNode);
  expect(addedNode.textContent).toContain(fileName);
});

test('sf-tree removeSelectedNode() removes root node', async () => {
  const nodes = await prepareTreeStructure();

  expect(nodes.tree.children.length).toEqual(2);
  expect(nodes.root1.classList).toContain('selected');

  const url = nodes.tree.removeSelectedNode();

  expect(nodes.tree.children.length).toEqual(1);
  expect(url?.toString().includes(fileName)).toBeTruthy();
  const remainingNode = nodes.tree.children[0];
  expect(remainingNode).toBe(nodes.root2);
  expect(remainingNode).not.toContain('selected');
});

test('sf-tree removeAllNodes() removes all root nodes', async () => {
  const nodes = await prepareTreeStructure();

  expect(nodes.tree.children.length).toEqual(2);

  nodes.tree.removeAllNodes();

  expect(nodes.tree.children.length).toEqual(0);
});

test('sf-tree getSelectedNodeParser() returns the parser for the selected node', async () => {
  const nodes = await prepareTreeStructure();

  const parser = nodes.tree.getSelectedNodeParser();

  expect(parser).not.toBeFalsy();
});
