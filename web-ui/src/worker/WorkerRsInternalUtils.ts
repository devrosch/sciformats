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

import * as sciformats_js from 'sciformats_js';
import { extractFilename, extractHashPath } from 'util/UrlUtils';
import WorkerFileInfo from './WorkerFileInfo';
import WorkerRequest from './WorkerRequest';
import WorkerResponse from './WorkerResponse';
import WorkerNodeData from './WorkerNodeData';
import WorkerFileUrl from './WorkerFileUrl';
import WorkerExportInfo from './WorkerExportInfo';
import WorkerExport from './WorkerExport';
import Table from 'model/Table';

const errorHandlingWrapper = (
  request: WorkerRequest,
  fn: () => WorkerResponse,
) => {
  try {
    return fn.apply(this);
  } catch (error) {
    return new WorkerResponse('error', request.correlationId, `${error}`);
  }
};

const extractFromRequest = (request: WorkerRequest) => {
  const fileInfo = request.detail as WorkerFileInfo;
  const url = new URL(fileInfo.url);
  const rootUrl = new URL(url.toString().split('#')[0]);
  const fileName = extractFilename(url);

  return {
    fileInfo,
    url,
    rootUrl,
    fileName,
  };
};

export const onScan = (
  request: WorkerRequest,
  scanner: sciformats_js.ScannerRepository,
) =>
  errorHandlingWrapper(request, () => {
    const { fileInfo, fileName } = extractFromRequest(request);
    const recognized = scanner.isRecognized(fileName, fileInfo.blob);
    return new WorkerResponse('scanned', request.correlationId, { recognized });
  });

export const onOpen = (
  request: WorkerRequest,
  scanner: sciformats_js.ScannerRepository,
  openFiles: Map<string, sciformats_js.Reader>,
) =>
  errorHandlingWrapper(request, () => {
    const { fileInfo, rootUrl, fileName } = extractFromRequest(request);
    const reader = scanner.getReader(fileName, fileInfo.blob);
    openFiles.set(rootUrl.toString(), reader);
    return new WorkerResponse('opened', request.correlationId, {
      url: rootUrl.toString(),
    });
  });

export const onRead = (
  request: WorkerRequest,
  openFiles: Map<string, sciformats_js.Reader>,
) =>
  errorHandlingWrapper(request, () => {
    const { url, rootUrl } = extractFromRequest(request);
    const reader = openFiles.get(rootUrl.toString());
    if (typeof reader === 'undefined') {
      throw new Error(`No open file found for ${url.toString()}`);
    }
    const hash = extractHashPath(url);
    const rawNode = reader.read(hash);
    const node: WorkerNodeData = {
      url: url.toString(),
      parameters: rawNode.parameters,
      data: rawNode.data,
      metadata: rawNode.metadata as Record<string, string>,
      table: rawNode.table as Table,
      childNodeNames: rawNode.childNodeNames,
    };
    rawNode.free();

    return new WorkerResponse('read', request.correlationId, node);
  });

export const onExport = (
  request: WorkerRequest,
  openFiles: Map<string, sciformats_js.Reader>,
) =>
  errorHandlingWrapper(request, () => {
    const fileUrl = request.detail as WorkerExportInfo;
    const url = fileUrl.url;
    const format = fileUrl.format;
    const rootUrl = new URL(url.toString().split('#')[0]);
    const reader = openFiles.get(rootUrl.toString());
    if (typeof reader === 'undefined') {
      throw new Error(`No open file found for ${url}`);
    }

    const blob = reader.exportToBlob(format);
    const data: WorkerExport = {
      blob,
    };

    return new WorkerResponse('exported', request.correlationId, data);
  });

export const onClose = (
  request: WorkerRequest,
  openFiles: Map<string, sciformats_js.Reader>,
) =>
  errorHandlingWrapper(request, () => {
    const fileUrl = request.detail as WorkerFileUrl;
    const url = new URL(fileUrl.url);
    const rootUrl = new URL(url.toString().split('#')[0]);
    if (openFiles.has(rootUrl.toString())) {
      const reader = openFiles.get(rootUrl.toString());
      openFiles.delete(rootUrl.toString());
      reader?.free();
    }
    return new WorkerResponse('closed', request.correlationId, {
      url: rootUrl.toString(),
    });
  });
