import NodeData from 'model/NodeData';
import Parser from 'model/Parser';
import Table from 'model/Table';

// a StubParser really, but jest requires the name to start with "Mock"
// see: https://jestjs.io/docs/es6-class-mocks#calling-jestmock-with-the-module-factory-parameter
export default class MockParser implements Parser {
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
    const parameters: { key: string, value: string }[] = [];
    const data: { x: number, y: number }[] = [];
    const table: Table = {
      columnNames: [{ key: 'col0', value: 'Cloumn 0 Value' }],
      rows: [new Map([['col0', 'Cloumn 0 Value']])],
    };
    const childNodeNames: string[] = ['child1', 'child2'];
    const metadata = {};

    const nodeData: NodeData = {
      url,
      parameters,
      data,
      metadata,
      table,
      childNodeNames,
    };

    return new Promise((resolve) => { resolve(nodeData); });
  }

  // eslint-disable-next-line class-methods-use-this
  async close() {
    // noop
  }
}
