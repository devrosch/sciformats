import NodeData from 'model/NodeData';
import Parser from 'model/Parser';

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
