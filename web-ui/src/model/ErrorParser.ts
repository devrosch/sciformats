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

import NodeData from './NodeData';
import Parser from './Parser';

/**
 * A parser implementation to indicate an error occured during file open.
 */
export default class ErrorParser implements Parser {
  #rootUrl: URL;

  #error: string;

  constructor(rootUrl: URL, error: string) {
    this.#rootUrl = rootUrl;
    this.#error = error;
  }

  get rootUrl(): URL {
    return this.#rootUrl;
  }

  async open() {
    throw new Error(this.#error);
  }

  async read(): Promise<NodeData> {
    throw new Error(this.#error);
  }

  async export(): Promise<Blob> {
    throw new Error(this.#error);
  }

  /* eslint-disable-next-line class-methods-use-this */
  async close() {
    // noop
  }
}
