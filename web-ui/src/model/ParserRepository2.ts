import { postMessage } from 'util/WorkerUtils';
import WorkerResponse from 'worker/WorkerResponse';
import Parser from './Parser';
import LocalFileParser2 from './LocalFileParser2';

export default class ParserRepository {
  #worker: Worker;

  constructor(worker: Worker) {
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
        // const parser = new LocalFileParser2(this.#worker, url, file);
        const parser = new LocalFileParser2(this.#worker, url);
        return parser;
      }
      throw Error(`Could not open file: ${file.name}`);
    }
    throw Error(`File not recognized: ${file.name}`);
  }
}
