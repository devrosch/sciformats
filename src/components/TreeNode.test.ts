/* eslint-disable import/no-duplicates */
import './TreeNode'; // for side effects
import TreeNode from './TreeNode';

test('sf-tree-node renders', async () => {
  const element = 'sf-tree-node';
  const urlAttr = 'url';
  const urlRoot = new URL('file:///test/path/root.txt');
  const urlFragment = new URL('file:///test/path/root.txt#/fragment');

  document.body.innerHTML = `<${element}/>`;
  expect(document.body.innerHTML).toContain('dummy.txt');
  expect(document.body.innerHTML).not.toContain('root.txt');
  expect(document.body.innerHTML).not.toContain('fragment');

  const treeNode = document.body.querySelector(element) as TreeNode;

  treeNode.setAttribute(urlAttr, urlRoot.toString());
  expect(document.body.innerHTML).toContain('root.txt');

  treeNode.setAttribute(urlAttr, urlFragment.toString());
  expect(document.body.innerHTML).toContain('fragment');

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
