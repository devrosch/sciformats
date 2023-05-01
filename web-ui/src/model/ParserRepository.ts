import { postMessage } from 'util/WorkerUtils';
import WorkerResponse from 'worker/WorkerResponse';
import Parser from './Parser';
import LocalFileParser from './LocalFileParser';

export default class ParserRepository {
  #worker: Worker;

  constructor() {
    const worker = new Worker(new URL('worker/worker.ts', import.meta.url));
    this.#worker = worker;
  }

  // eslint-disable-next-line class-methods-use-this
  async findParser(file: File): Promise<Parser> {
    // generate URL of type file:///UUID/fileName#/
    const uuid = crypto.randomUUID();
    const urlSafefileName = encodeURIComponent(file.name);
    const url = new URL(`file:///${uuid}/${urlSafefileName}#/`);

    const scanReply: WorkerResponse = await postMessage(this.#worker, 'scan', { url: url.toString(), file }) as any;
    if (scanReply.name === 'scanned' && (scanReply.detail as { recognized: boolean }).recognized === true) {
      const openReply: WorkerResponse = await postMessage(this.#worker, 'open', { url: url.toString(), file }) as any;
      if (openReply.name === 'opened') {
        const parser = new LocalFileParser(this.#worker, url);
        return parser;
      }
      throw Error(`Could not open file: "${file.name}"`);
    }
    throw Error(`File not recognized: "${file.name}"`);
  }
}
