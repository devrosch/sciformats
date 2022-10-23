/* eslint-disable import/no-duplicates */
import './TreeNode'; // for side effects
import TreeNode from './TreeNode';

test('sf-tree-node renders', async () => {
  const element = 'sf-tree-node';
  const urlAttr = 'url';
  const urlRoot = new URL('file:///test/path/root.txt');
  const urlRootSlash = new URL('file:///test/path/root.txt/');
  const urlFragment = new URL('file:///test/path/root.txt#/fragment');

  document.body.innerHTML = `<${element}/>`;
  expect(document.body.innerHTML).toContain('dummy.txt');
  expect(document.body.innerHTML).not.toContain('root.txt');
  expect(document.body.innerHTML).not.toContain('fragment');

  const treeNode = document.body.querySelector(element) as TreeNode;

  treeNode.setAttribute(urlAttr, urlRoot.toString());
  expect(document.body.innerHTML).toContain('root.txt');
  expect(treeNode.textContent).toContain('root.txt');

  treeNode.setAttribute(urlAttr, urlRootSlash.toString());
  expect(treeNode.textContent).toContain('root.txt');

  treeNode.setAttribute(urlAttr, urlFragment.toString());
  expect(document.body.innerHTML).toContain('fragment');
  expect(treeNode.textContent).toContain('fragment');

  let plusMinusSpan = document.body.querySelector('.plusminus') as HTMLSpanElement;
  expect(plusMinusSpan?.innerHTML).toContain('⊞');
  expect(document.body.innerHTML).not.toContain('child 1');
  expect(document.body.innerHTML).not.toContain('child 2');
  expect(document.body.innerHTML).not.toContain('child 3');

  treeNode.onToggleCollapsed();
  // new span is created, so query it again
  plusMinusSpan = document.body.querySelector('.plusminus') as HTMLSpanElement;
  expect(plusMinusSpan?.innerHTML).toContain('⊟');
  expect(document.body.innerHTML).toContain('child 1');
  expect(document.body.innerHTML).toContain('child 2');
  expect(document.body.innerHTML).toContain('child 3');

  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-tree-node generates sf-tree-node-selected events', async () => {
  const element = 'sf-tree-node';

  document.body.innerHTML = `<${element}/>`;
  const treeNode = document.body.querySelector(element) as TreeNode;

  let called = 0;
  const eventHandler = () => {
    called += 1;
  };
  window.addEventListener('sf-tree-node-selected', eventHandler);
  treeNode.onSelected();
  expect(called).toBe(1);
  window.removeEventListener('sf-tree-node-selected', eventHandler);

  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-tree-node observes sf-tree-node-selected events', async () => {
  const element = 'sf-tree-node';

  document.body.innerHTML = `<${element}/>`;
  const treeNode = document.body.querySelector(element) as TreeNode;
  expect(treeNode.classList).not.toContain('selected');
  treeNode.onSelected();
  expect(treeNode.classList).toContain('selected');

  window.dispatchEvent(new CustomEvent('sf-tree-node-selected', {
    bubbles: true,
    cancelable: true,
    composed: true,
    detail: { url: new URL('https://dummy') },
  }));

  expect(treeNode.classList).not.toContain('selected');

  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});
