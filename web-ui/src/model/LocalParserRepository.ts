import { postMessage } from 'util/WorkerUtils';
import WorkerResponse from 'worker/WorkerResponse';
import WorkerFileInfo from 'worker/WorkerFileInfo';
import LocalFileParser from './LocalFileParser';
import Parser from './Parser';
import ParserRepository from './ParserRepository';

export default class LocalParserRepository implements ParserRepository {
  #worker: Worker;

  constructor(worker: Worker) {
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
    throw new Error(`File not recognized: "${file.name}"`);
  }
}
