/**
 * Copyright (c) 2025 Robert Schiwon
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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
    const parameters: {
      key: string;
      value: string | boolean | number | bigint;
    }[] = [];
    const data: { x: number; y: number }[] = [];
    const table: Table = {
      columnNames: [{ key: 'col0', name: 'Column 0 Value' }],
      rows: [{ col0: 'Column 0 Value' }],
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

    return new Promise((resolve) => {
      resolve(nodeData);
    });
  }

  /* eslint-disable-next-line @typescript-eslint/no-unused-vars, class-methods-use-this */
  async export(format: string): Promise<Blob> {
    throw new Error('Export not implemented.');
  }

  /* eslint-disable-next-line class-methods-use-this */
  async close() {
    // noop
  }
}
