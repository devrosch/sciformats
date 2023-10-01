import * as sf_rs from 'sf_rs';
import { extractFilename, extractHashPath } from 'util/UrlUtils';
import WorkerFileInfo from './WorkerFileInfo';
import WorkerRequest from './WorkerRequest';
import WorkerResponse from './WorkerResponse';
import WorkerNodeData from './WorkerNodeData';
import WorkerFileUrl from './WorkerFileUrl';

const errorHandlingWrapper = (request: WorkerRequest, fn: () => WorkerResponse) => {
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

export const onScan = (request: WorkerRequest, scanner: sf_rs.AndiChromScanner) => {
  let fileWrapper = null;
  try {
    const { fileInfo, fileName } = extractFromRequest(request);
    const file = fileInfo.blob as File;
    fileWrapper = new sf_rs.FileWrapper(file);
    const recognized = scanner.js_is_recognized(fileName, fileWrapper);
    fileWrapper.free();
    return new WorkerResponse('scanned', request.correlationId, { recognized });
  } catch (error) {
    if (fileWrapper !== null) {
      fileWrapper.free();
    }
    return new WorkerResponse('error', request.correlationId, `${error}`);
  }
};

export const onOpen = (
  request: WorkerRequest,
  scanner: sf_rs.AndiChromScanner,
  openFiles: Map<string, sf_rs.JsReader>,
) => {
  let fileWrapper = null;
  try {
    const { fileInfo, rootUrl, fileName } = extractFromRequest(request);
    const file = fileInfo.blob as File;
    fileWrapper = new sf_rs.FileWrapper(file);
    const reader = scanner.js_get_reader(fileName, fileWrapper);
    openFiles.set(rootUrl.toString(), reader);
    return new WorkerResponse('opened', request.correlationId, { url: rootUrl.toString() });
  } catch (error) {
    if (fileWrapper !== null) {
      fileWrapper.free();
    }
    return new WorkerResponse('error', request.correlationId, `${error}`);
  }
};

export const onRead = (
  request: WorkerRequest,
  openFiles: Map<string, sf_rs.JsReader>,
) => errorHandlingWrapper(request, () => {
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
    metadata: rawNode.metadata as { [key: string]: string },
    table: rawNode.table as { columnNames: [], rows: [] },
    childNodeNames: rawNode.child_node_names,
  };
  rawNode.free();

  return new WorkerResponse('read', request.correlationId, node);
});

export const onClose = (
  request: WorkerRequest,
  openFiles: Map<string, sf_rs.JsReader>,
) => errorHandlingWrapper(request, () => {
  const fileUrl = request.detail as WorkerFileUrl;
  const url = new URL(fileUrl.url);
  const rootUrl = new URL(url.toString().split('#')[0]);
  if (openFiles.has(rootUrl.toString())) {
    const reader = openFiles.get(rootUrl.toString());
    openFiles.delete(rootUrl.toString());
    reader?.free();
  }
  return new WorkerResponse('closed', request.correlationId, { url: rootUrl.toString() });
});
