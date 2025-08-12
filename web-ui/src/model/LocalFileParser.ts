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
import WorkerNodeData from 'worker/WorkerNodeData';
import WorkerResponse from 'worker/WorkerResponse';
import WorkerFileInfo from 'worker/WorkerFileInfo';
import WorkerFileUrl from 'worker/WorkerFileUrl';
import Parser from './Parser';
import WorkerExportInfo from 'worker/WorkerExportInfo';
import WorkerExport from 'worker/WorkerExport';

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
    const payload: WorkerFileInfo = {
      url: this.#rootUrl.toString(),
      blob: this.#file,
    };
    const openReply: WorkerResponse = (await postMessage(
      this.#worker,
      'open',
      payload,
    )) as any;
    if (openReply.name !== 'opened') {
      throw new Error(`Could not open file: "${this.#file.name}".`);
    }
  }

  async read(url: URL) {
    const urlString = url.toString();
    if (!urlString.startsWith(this.#rootUrl.toString())) {
      throw new Error(`Illegal URL for parser: ${url}`);
    }

    const payload: WorkerFileUrl = { url: url.toString() };
    const response = (await postMessage(
      this.#worker,
      'read',
      payload,
    )) as WorkerResponse;
    const json = response.detail as WorkerNodeData;

    // TODO: harmonize?
    return {
      url: new URL(json.url),
      parameters: json.parameters,
      data: json.data,
      metadata: json.metadata,
      table: json.table,
      childNodeNames: json.childNodeNames,
    };
  }

  async export(format: 'Json') {
    const payload: WorkerExportInfo = { url: this.#rootUrl.toString(), format };
    const response = (await postMessage(
      this.#worker,
      'export',
      payload,
    )) as WorkerResponse;
    const data = response.detail as WorkerExport;
    const blob = data.blob;
    return blob;
  }

  async close() {
    const payload: WorkerFileUrl = { url: this.#rootUrl.toString() };
    const closeReply: WorkerResponse = (await postMessage(
      this.#worker,
      'close',
      payload,
    )) as any;
    if (closeReply.name !== 'closed') {
      throw new Error(`Could not close file: "${this.#file.name}."`);
    }
  }
}
