import WorkerNodeData from 'worker/WorkerNodeData';
import { postMessage } from 'util/WorkerUtils';
import WorkerResponse from 'worker/WorkerResponse';
import WorkerFileInfo from 'worker/WorkerFileInfo';
import WorkerFileUrl from 'worker/WorkerFileUrl';
import Parser from './Parser';

export default class LocalFileParser implements Parser {
  #worker: Worker;

  #rootUrl: URL;

  #file: File;

  constructor(worker: Worker, rootUrl: URL, file: File) {
    this.#worker = worker;
    this.#rootUrl = rootUrl;
    this.#file = file;
  }

  get rootUrl(): URL {
    return this.#rootUrl;
  }

  async open() {
    const payload: WorkerFileInfo = { url: this.#rootUrl.toString(), blob: this.#file };
    const openReply: WorkerResponse = await postMessage(this.#worker, 'open', payload) as any;
    if (openReply.name !== 'opened') {
      throw Error(`Could not open file: "${this.#file.name}."`);
    }
  }

  async read(url: URL) {
    const urlString = url.toString();
    if (!urlString.startsWith(this.#rootUrl.toString())) {
      throw new Error(`Illegal URL for parser: ${url}`);
    }

    const payload: WorkerFileUrl = { url: url.toString() };
    const response = await postMessage(this.#worker, 'read', payload) as WorkerResponse;
    const json = response.detail as WorkerNodeData;

    // TODO: harmonize?
    return {
      url: new URL(json.url),
      parameters: json.parameters,
      data: json.data,
      peakTable: json.peakTable,
      childNodeNames: json.childNodeNames,
    };
  }

  async close() {
    const payload: WorkerFileUrl = { url: this.#rootUrl.toString() };
    const closeReply: WorkerResponse = await postMessage(this.#worker, 'close', payload) as any;
    if (closeReply.name !== 'closed') {
      throw Error(`Could not close file: "${this.#file.name}."`);
    }
  }
}
