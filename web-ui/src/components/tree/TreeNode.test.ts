/* eslint-disable no-duplicate-imports */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Message from 'model/Message';
import NodeData from 'model/NodeData';
import Parser from 'model/Parser';
import Table from 'model/Table';
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

    const parameters: {
      key: string;
      value: string | boolean | number | bigint;
    }[] = [{ key: 'key 1', value: 'value 1' }];

    let data: { x: number; y: number }[] = [];
    data = [{ x: 1, y: 2 }];
    if (hash.endsWith('/child 2')) {
      data = [
        { x: 1, y: 2 },
        { x: 2, y: 4 },
      ];
      parameters.push({ key: 'key 2', value: true });
    }
    if (hash.endsWith('/child 3')) {
      data = [
        { x: 1, y: 2 },
        { x: 2, y: 4 },
        { x: 3, y: 6 },
      ];
      parameters.push({ key: 'key 2', value: false });
      parameters.push({ key: 'key 3', value: 123.456 });
      parameters.push({ key: 'key 4', value: BigInt(123456) });
    }

    const table: Table = { columnNames: [], rows: [] };
    if (hash.endsWith('/child 2')) {
      table.columnNames.push({ key: 'col0', name: 'Column 0' });
      table.columnNames.push({ key: 'col1', name: 'Column 1' });
      const peak0 = {
        col0: 'peak0col0value',
        col1: 'peak0col1value',
      };
      table.rows.push(peak0);
      const peak1 = {
        col0: 'peak1col0value',
        col1: 'peak1col1value',
      };
      table.rows.push(peak1);
    }

    let childNodeNames: string[] = [];
    if (hash === '' || hash === '#' || hash === '/' || hash === '#/') {
      childNodeNames = ['child 1', 'child 2', 'child 3'];
    }
    if (hash.endsWith('/child 2')) {
      childNodeNames = ['child 1', 'child 2'];
    }

    let metadata = {};
    if (hash.endsWith('/child 2')) {
      metadata = { 'x.units': '1/cm', 'y.units': 'a.u.' };
    }

    return {
      url,
      parameters,
      data,
      metadata,
      table,
      childNodeNames,
    };
  }

  /* eslint-disable-next-line @typescript-eslint/no-unused-vars */
  async export(format: string): Promise<Blob> {
    throw new Error('Export not implemented.');
  }

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

      let plusMinusSpan = document.body.querySelector(
        '.plusminus',
      ) as HTMLSpanElement;
      expect(plusMinusSpan?.innerHTML).toContain('⊞');
      expect(document.body.innerHTML).not.toContain(child1);
      expect(document.body.innerHTML).not.toContain(child2);
      expect(document.body.innerHTML).not.toContain(child3);

      treeNode.onToggleCollapsed();
      // new span is created, so query it again
      plusMinusSpan = document.body.querySelector(
        '.plusminus',
      ) as HTMLSpanElement;
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
  let handle: any = null;
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
    open: () =>
      new Promise<void>(() => {
        /* noop */
      }),
    read: () => {
      throw new Error('Test Error.');
    },
    /* eslint-disable-next-line @typescript-eslint/no-unused-vars */
    export: (format: string): Promise<Blob> => {
      throw new Error('Export not implemented.');
    },
    close: () =>
      new Promise<void>(() => {
        /* noop */
      }),
  } as Parser;
  treeNode = new TreeNode(mockParser, mockParser.rootUrl);

  const channel = CustomEventsMessageBus.getDefaultChannel();
  let handle: any = null;
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
    open: () =>
      new Promise<void>(() => {
        /* noop */
      }),
    read: () => {
      throw new Error('Test Error.');
    },
    /* eslint-disable-next-line @typescript-eslint/no-unused-vars */
    export: (format: string): Promise<Blob> => {
      throw new Error('Export not implemented.');
    },
    close: () =>
      new Promise<void>(() => {
        /* noop */
      }),
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
