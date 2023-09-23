import { postMessage } from 'util/WorkerUtils';
import WorkerResponse from 'worker/WorkerResponse';
import WorkerFileInfo from 'worker/WorkerFileInfo';
import LocalFileParser from './LocalFileParser';
import Parser from './Parser';
import ParserRepository from './ParserRepository';

export default class LocalParserRepository implements ParserRepository {
  #workers: Worker[];

  constructor(workers: Worker[]) {
    this.#workers = workers;
  }

  async findParser(file: File): Promise<Parser> {
    // generate URL of type file:///UUID/fileName#/
    const uuid = crypto.randomUUID();
    const urlSafefileName = encodeURIComponent(file.name);
    const url = new URL(`file:///${uuid}/${urlSafefileName}#/`);

    const payload: WorkerFileInfo = { url: url.toString(), blob: file };
    for (const worker of this.#workers) {
      const scanReply: WorkerResponse = await postMessage(worker, 'scan', payload) as any;
      if (scanReply.name === 'scanned' && (scanReply.detail as { recognized: boolean }).recognized === true) {
        const parser = new LocalFileParser(worker, url, file);
        return parser;
      }
    }
    throw new Error(`File not recognized: "${file.name}"`);
  }
}
