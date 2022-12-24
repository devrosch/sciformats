/* eslint-disable import/no-duplicates */
import Message from 'model/Message';
import Parser from 'model/Parser';
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import './TreeNode'; // for side effects
import TreeNode from './TreeNode';

const child1 = encodeURIComponent('child 1');
const child2 = encodeURIComponent('child 2');
const child3 = encodeURIComponent('child 3');

class StubParser implements Parser {
  #url: URL = new URL('file:///dummy.txt#/');

  get rootUrl(): URL {
    return this.#url;
  }

  async read(url: URL) {
    const hash = decodeURIComponent(url.hash);
    let children: string[] = [];
    let data: { x: number, y: number }[] = [];
    if (hash === '' || hash === '#' || hash === '/' || hash === '#/') {
      children = ['child 1', 'child 2', 'child 3'];
    }
    if (hash.endsWith('/child 2')) {
      children = ['child 1', 'child 2'];
    }

    const parameters = [{ key: 'key 1', value: 'value 1' }];
    data = [{ x: 1, y: 2 }];
    if (hash.endsWith('/child 2')) {
      data = [{ x: 1, y: 2 }, { x: 2, y: 4 }];
      parameters.push({ key: 'key 2', value: 'value 2' });
    }
    if (hash.endsWith('/child 3')) {
      data = [{ x: 1, y: 2 }, { x: 2, y: 4 }, { x: 3, y: 6 }];
      parameters.push({ key: 'key 2', value: 'value 2' });
      parameters.push({ key: 'key 3', value: 'value 3' });
    }

    return {
      url,
      data,
      parameters,
      children,
    };
  }
}

let parser: Parser;
let treeNode: TreeNode;

beforeEach(() => {
  parser = new StubParser();
  treeNode = new TreeNode(parser, parser.rootUrl);
  document.body.append(treeNode);
});

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-tree-node renders', (done) => {
  expect(document.body.innerHTML).toContain(parser.rootUrl.toString());

  // wait for async parser.read() to execute
  process.nextTick(() => {
    try {
      expect(treeNode.textContent).toContain('dummy.txt');

      let plusMinusSpan = document.body.querySelector('.plusminus') as HTMLSpanElement;
      expect(plusMinusSpan?.innerHTML).toContain('⊞');
      expect(document.body.innerHTML).not.toContain(child1);
      expect(document.body.innerHTML).not.toContain(child2);
      expect(document.body.innerHTML).not.toContain(child3);

      treeNode.onToggleCollapsed();
      // new span is created, so query it again
      plusMinusSpan = document.body.querySelector('.plusminus') as HTMLSpanElement;
      expect(plusMinusSpan?.innerHTML).toContain('⊟');
      expect(document.body.innerHTML).toContain(child1);
      expect(document.body.innerHTML).toContain(child2);
      expect(document.body.innerHTML).toContain(child3);

      done();
    }
    catch(err) {
      done(err);
    }
  });
});

test('sf-tree-node generates sf-tree-node-selected events', async () => {
  const channel = CustomEventsMessageBus.getDefaultChannel();
  const eventHandler = jest.fn();
  const handle = channel.addListener('sf-tree-node-selected', eventHandler);
  treeNode.onSelected();
  channel.removeListener(handle);
  expect(eventHandler).toHaveBeenCalledTimes(1);
});

test('sf-tree-node observes sf-tree-node-selected events', async () => {
  expect(treeNode.classList).not.toContain('selected');
  treeNode.onSelected();
  expect(treeNode.classList).toContain('selected');

  const channel = CustomEventsMessageBus.getDefaultChannel();
  channel.dispatch('sf-tree-node-selected', { url: new URL('https://dummy') });

  expect(treeNode.classList).not.toContain('selected');
});

test('sf-tree-node generates sf-tree-node-data-read events', (done) => {
  const dataReadEvent = 'sf-tree-node-data-read';

  const channel = CustomEventsMessageBus.getDefaultChannel();
  let handle: any;
  const listener = (message: Message) => {
    try {
      expect(message.name).toBe(dataReadEvent);
      channel.removeListener(handle);
      done();
    } catch (error) {
      done(error);
    }
  };
  handle = channel.addListener(dataReadEvent, listener);
  
  treeNode = new TreeNode(parser, parser.rootUrl);
  document.body.append(treeNode);
});