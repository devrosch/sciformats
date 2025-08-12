/**
 * Copyright (c) 2025 Robert Schiwon
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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
      // TODO: this could be turned into a promise all
      /* eslint-disable-next-line no-await-in-loop */
      const scanReply: WorkerResponse = await postMessage(
        worker,
        'scan',
        payload,
      );
      if (
        scanReply.name === 'scanned' &&
        (scanReply.detail as { recognized: boolean }).recognized === true
      ) {
        const parser = new LocalFileParser(worker, url, file);
        return parser;
      }
    }
    throw new Error(`File not recognized: "${file.name}"`);
  }
}
