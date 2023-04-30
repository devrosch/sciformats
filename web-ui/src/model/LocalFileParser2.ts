import WorkerNodeData from 'worker/WorkerNodeData';
import Parser from './Parser';
import { postMessage } from 'util/WorkerUtils';
import WorkerResponse from 'worker/WorkerResponse';

export default class LocalFileParser2 implements Parser {
  #worker: Worker;

  #rootUrl: URL;

  // #file: File;

  // constructor(worker: Worker, rootUrl: URL, file: File) {
  //   this.#worker = worker;
  //   this.#rootUrl = rootUrl;
  //   this.#file = file;
  // }
  constructor(worker: Worker, rootUrl: URL) {
    this.#worker = worker;
    this.#rootUrl = rootUrl;
  }

  get rootUrl(): URL {
    return this.#rootUrl;
  }

  async read(url: URL) {
    const urlString = url.toString();
    if (!urlString.startsWith(this.#rootUrl.toString())) {
      throw new Error(`Illegal URL for parser: ${url}`);
    }

    const response = await postMessage(this.#worker, 'read', { url: url.toString() }) as WorkerResponse;
    const json = response.detail as WorkerNodeData;

    // TODO: harmonize?
    return {
      url: new URL(json.url),
      data: json.data,
      parameters: json.parameters,
      children: json.children,
    };
  }
}
