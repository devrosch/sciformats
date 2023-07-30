/* eslint-disable import/no-duplicates */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Message from 'model/Message';
import NodeData from 'model/NodeData';
import Parser from 'model/Parser';
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

  // eslint-disable-next-line class-methods-use-this
  async open() {
    // noop
  }

  /* eslint-disable class-methods-use-this */
  async read(url: URL): Promise<NodeData> {
    const hash = decodeURIComponent(url.hash);

    const parameters = [{ key: 'key 1', value: 'value 1' }];

    let data: { x: number, y: number }[] = [];
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

    const peakTable: {
      columnNames: { key: string, value: string }[],
      peaks: Map<string, string>[],
    } = { columnNames: [], peaks: [] };
    if (hash.endsWith('/child 2')) {
      peakTable.columnNames.push({ key: 'col0', value: 'Cloumn 0' });
      peakTable.columnNames.push({ key: 'col1', value: 'Cloumn 1' });
      const peak0 = new Map<string, string>();
      peak0.set('col0', 'peak0col0value');
      peak0.set('col1', 'peak0col1value');
      peakTable.peaks.push(peak0);
      const peak1 = new Map<string, string>();
      peak1.set('col0', 'peak1col0value');
      peak1.set('col1', 'peak1col1value');
      peakTable.peaks.push(peak1);
    }

    let childNodeNames: string[] = [];
    if (hash === '' || hash === '#' || hash === '/' || hash === '#/') {
      childNodeNames = ['child 1', 'child 2', 'child 3'];
    }
    if (hash.endsWith('/child 2')) {
      childNodeNames = ['child 1', 'child 2'];
    }

    return {
      url,
      parameters,
      data,
      peakTable,
      childNodeNames,
    };
  }

  // eslint-disable-next-line class-methods-use-this
  async close() {
    // noop
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
    } catch (err) {
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

test('sf-tree-node generates sf-tree-node-data-updated events', (done) => {
  const dataReadEvent = 'sf-tree-node-data-updated';

  const channel = CustomEventsMessageBus.getDefaultChannel();
  let handle: any;
  const listener = (message: Message) => {
    try {
      channel.removeListener(handle);
      expect(message.name).toBe(dataReadEvent);
      done();
    } catch (error) {
      done(error);
    }
  };
  handle = channel.addListener(dataReadEvent, listener);

  treeNode = new TreeNode(parser, parser.rootUrl);
  document.body.append(treeNode);
});

test('sf-tree-node generates sf-error events in case of data loading error', (done) => {
  const errorEvent = 'sf-error';
  const mockParser = {
    rootUrl: new URL('https://dummy#/'),
    open: () => new Promise<void>(() => { }),
    read: () => { throw new Error('Test Error.'); },
    close: () => new Promise<void>(() => { }),
  } as Parser;
  treeNode = new TreeNode(mockParser, mockParser.rootUrl);

  const channel = CustomEventsMessageBus.getDefaultChannel();
  let handle: any;
  const listener = (message: Message) => {
    try {
      channel.removeListener(handle);
      expect(message.name).toBe(errorEvent);
      done();
    } catch (error) {
      done(error);
    }
  };
  handle = channel.addListener(errorEvent, listener);

  document.body.append(treeNode);
});

test('sf-tree-node displays error in case of data loading error', (done) => {
  const mockParser = {
    rootUrl: new URL('https://dummy#/'),
    open: () => new Promise<void>(() => { }),
    read: () => { throw new Error('Test Error.'); },
    close: () => new Promise<void>(() => { }),
  } as Parser;
  treeNode = new TreeNode(mockParser, mockParser.rootUrl);
  document.body.append(treeNode);

  // wait for async parser.read() to execute
  process.nextTick(() => {
    try {
      expect(treeNode.textContent?.toLowerCase()).toContain('error');
      done();
    } catch (err) {
      done(err);
    }
  });
});
