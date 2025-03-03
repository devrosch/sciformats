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
