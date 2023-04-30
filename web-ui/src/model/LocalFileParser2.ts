import WorkerNodeData from 'worker/WorkerNodeData';
import { postMessage } from 'util/WorkerUtils';
import WorkerResponse from 'worker/WorkerResponse';
import Parser from './Parser';

export default class LocalFileParser2 implements Parser {
  #worker: Worker;

  #rootUrl: URL;

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
