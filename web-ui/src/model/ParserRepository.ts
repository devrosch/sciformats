import { postMessage } from 'util/WorkerUtils';
import WorkerResponse from 'worker/WorkerResponse';
import Parser from './Parser';
import LocalFileParser from './LocalFileParser';
import WorkerFileInfo from 'worker/WorkerFileInfo';

export default class ParserRepository {
  #worker: Worker;

  constructor() {
    const worker = new Worker(new URL('worker/worker.ts', import.meta.url));
    this.#worker = worker;
  }

  async findParser(file: File): Promise<Parser> {
    // generate URL of type file:///UUID/fileName#/
    const uuid = crypto.randomUUID();
    const urlSafefileName = encodeURIComponent(file.name);
    const url = new URL(`file:///${uuid}/${urlSafefileName}#/`);

    const payload: WorkerFileInfo = { url: url.toString(), blob: file };
    const scanReply: WorkerResponse = await postMessage(this.#worker, 'scan', payload) as any;
    if (scanReply.name === 'scanned' && (scanReply.detail as { recognized: boolean }).recognized === true) {
      const parser = new LocalFileParser(this.#worker, url, file);
      return parser;
    }
    throw Error(`File not recognized: "${file.name}"`);
  }
}
