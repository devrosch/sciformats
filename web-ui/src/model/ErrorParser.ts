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
    throw Error(this.#error);
  }

  async read() {
    throw Error(this.#error);
    // somehow required so TS is satisfied
    return {} as NodeData;
  }

  async close() {
    // noop
  }
}
