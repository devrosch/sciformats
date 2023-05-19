// import * as Module from "./libsf";
// import "./libsf.wasm";
// import libsf from './libsf.js';
// import * as libsf from "./libsf";
// import libsf from './libsf.js';
// import './libsf';
// import('./libsf');

import { extractFilename, extractUuid } from 'util/UrlUtils';
import WorkerRequest from './WorkerRequest';
import WorkerResponse from './WorkerResponse';
import WorkerStatus from './WorkerStatus';
import WorkerFileInfo from './WorkerFileInfo';
import WorkerNodeData from './WorkerNodeData';
import WorkerFileUrl from './WorkerFileUrl';

// quench warnings for using "self", alternatively "globalThis" could be used instead
/* eslint-disable no-restricted-globals */

self.importScripts('libsf.js');

const workingDir = '/work';

/* @ts-expect-error */
const openFiles = new Map<string, Module.JdxDataMapper>();

// use @ts-expect-error instead of use @ts-ignore to make sure the issue occurs and
// to avoid need to avoind @typescript-eslint/ban-ts-comment linter warning
/* @ts-expect-error */
const hasInitCompleted: () => boolean = () => !(typeof Module === 'undefined' || Module === null || typeof Module.FileParser === 'undefined');

/**
 * Mounts a file in the filesystem's "work" directory.
 * @param url The file URL from which UUID and file name are extracted.
 * @param blob A Blob containing the file data.
 */
const mountFile = (url: URL, blob: Blob) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  /* @ts-expect-error */
  const filesystem = FS;
  const workingDirExists = filesystem.analyzePath(workingDir, false).exists;
  if (!workingDirExists) {
    filesystem.mkdir(workingDir);
  }
  const uuidDirPath = `${workingDir}/${uuid}`;
  const uuidDirExists = filesystem.analyzePath(uuidDirPath, false).exists;
  if (!uuidDirExists) {
    filesystem.mkdir(uuidDirPath);
  }
  /* @ts-expect-error */
  const workerFS = WORKERFS;
  filesystem.mount(workerFS, {
    blobs: [{ name: filename, data: blob }],
  }, uuidDirPath);
};

const storeMapper = (url: URL) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  const filePath = `${workingDir}/${uuid}/${filename}`;
  const rootUrl = new URL(url.toString().split('#')[0]);
  let mapper = null;
  try {
    /* @ts-expect-error */
    mapper = new Module.JdxDataMapper(filePath);
    openFiles.set(rootUrl.toString(), mapper);
  } catch (error) {
    if (mapper !== null) {
      mapper.delete();
    }
    throw error;
  }
};

/**
 * Unmounts a file from the filesystem's "work" directory.
 * @param url The file URL from which UUID and file name are extracted.
 */
const unmountFile = (url: URL) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  /* @ts-expect-error */
  const filesystem = FS;
  const workingDirExists = filesystem.analyzePath(workingDir, false).exists;
  if (!workingDirExists) {
    return;
  }
  const uuidDirPath = `${workingDir}/${uuid}`;
  const uuidDirExists = filesystem.analyzePath(uuidDirPath, false).exists;
  if (!uuidDirExists) {
    return;
  }
  const filePath = `${uuidDirPath}/${filename}`;
  const fileExists = filesystem.analyzePath(filePath, false).exists;
  if (fileExists) {
    filesystem.unmount(uuidDirPath);
  }
  filesystem.rmdir(uuidDirPath);
};

const isFileRecognized = (url: URL) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  /* @ts-expect-error */
  const scanner = new Module.JdxDataScanner();
  const filePath = `${workingDir}/${uuid}/${filename}`;
  const recognized = scanner.isRecognized(filePath);
  scanner.delete();
  return recognized;
};

/* @ts-expect-error */
const nodeToJson = (url: URL, node: Module.Node) => {
  const json: any = {};

  json.url = url.toString();

  // // [[nodiscard]] virtual std::string getName() const;
  // json.name = node.name;

  // virtual std::vector<KeyValueParam> getParams();
  const params = node.parameters;
  const paramsSize = params.size();
  const jsonParameters: any = [];
  for (let index = 0; index < paramsSize; index += 1) {
    const keyValuePair = params.get(index);
    const key = keyValuePair.key;
    const value = keyValuePair.value;
    // jsonParameters[key] = value;
    jsonParameters.push({ key, value });
  }
  json.parameters = jsonParameters;
  params.delete();

  // virtual std::vector<Point2D> getData();
  const data = node.data;
  const dataSize = data.size();
  const jsonData = [];
  for (let index = 0; index < dataSize; index += 1) {
    const point = data.get(index);
    const x = point.x;
    const y = point.y;
    jsonData.push({ x, y });
  }
  json.data = jsonData;
  data.delete();

  // virtual std::vector<std::shared_ptr<Node>> getChildNodes() = 0;
  const childNodeNames = node.childNodeNames;
  const childNodesSize = childNodeNames.size();
  const jsonChildNodes = [];
  for (let index = 0; index < childNodesSize; index += 1) {
    const childNodeName = childNodeNames.get(index);
    jsonChildNodes.push(childNodeName);
  }
  json.children = jsonChildNodes;
  childNodeNames.delete();

  return json;
};

const readNodeData = (url: URL) => {
  const rootUrl = new URL(url.toString().split('#')[0]);
  if (!openFiles.has(rootUrl.toString())) {
    console.log(`root URL: ${rootUrl}`);
    console.log(`number of keys in openFiles: ${openFiles.keys.length}`);
    Object.keys(openFiles).forEach((key) => {
      console.log(`map key: ${key}`);
    });
    throw new Error(`File not found: ${rootUrl}`);
  }

  let hash = url.hash;
  if (hash.length > 0 && !hash.startsWith('#/')) {
    throw new Error(`Unexpected URL hash: ${hash}`);
  }

  const mapper = openFiles.get(rootUrl.toString());

  // '', '#', '#/' all denote the root node
  // splitting by '/' results in:
  // '' => ['']
  // '/' => ['', '']
  // TODO: implement util function for this
  if (hash.startsWith('#')) {
    hash = hash.substring(1);
  }
  if (hash.length === 0) {
    hash = '/';
  }

  // node is of type Node2 and bound as a value object, hence it has no delete() method
  const node = mapper.read(hash);
  const json = nodeToJson(url, node);
  return json;
};

const getExceptionMessage = (exception: any) => {
  if (typeof exception === 'number') {
    // JS wrapped C++/WASM exception
    /* @ts-expect-error */
    const msg = Module.getCppExceptionMessage(exception);
    return msg;
  }
  /* @ts-expect-error */
  if (exception instanceof WebAssembly.Exception) {
    // native C++/WASM exception
    // see: https://github.com/emscripten-core/emscripten/issues/16033
    // https://github.com/emscripten-core/emscripten/pull/17219
    /* @ts-expect-error */
    const msgArray = Module.getExceptionMessage(exception);
    const msg = Array.isArray(msgArray) ? msgArray.join(', ') : msgArray;
    return msg;
  }
  if (exception.message) {
    // JS Error
    return exception.message;
  }
  // something else
  return exception;
};

self.onmessage = (event) => {
  const request = event.data as WorkerRequest;
  const correlationId = request.correlationId;
  switch (request.name) {
    case 'status': {
      const initCompleted = hasInitCompleted() ? WorkerStatus.Initialized
        : WorkerStatus.Initializing;
      const result = new WorkerResponse('status', correlationId, initCompleted);
      self.postMessage(result);
      break;
    }
    case 'scan': {
      const fileInfo = request.detail as WorkerFileInfo;
      const url = new URL(fileInfo.url);
      const blob = fileInfo.blob;
      mountFile(url, blob);
      const recognized = isFileRecognized(url);
      unmountFile(url);
      self.postMessage(new WorkerResponse('scanned', correlationId, { recognized }));
      break;
    }
    case 'open': {
      const fileInfo = request.detail as WorkerFileInfo;
      const url = new URL(fileInfo.url);
      const blob = fileInfo.blob;
      const rootUrl = new URL(url.toString().split('#')[0]);
      if (!openFiles.has(rootUrl.toString())) {
        try {
          mountFile(url, blob);
          storeMapper(url);
        } catch (error: any) {
          const message = getExceptionMessage(error);
          self.postMessage(new WorkerResponse('error', correlationId, message));
          break;
        }
      }
      self.postMessage(new WorkerResponse('opened', correlationId, { url: rootUrl.toString() }));
      break;
    }
    case 'read': {
      const fileUrl = request.detail as WorkerFileUrl;
      const url = new URL(fileUrl.url);
      try {
        const nodeData: WorkerNodeData = readNodeData(url);
        self.postMessage(new WorkerResponse('read', correlationId, nodeData));
        break;
      } catch (error: any) {
        const message = getExceptionMessage(error);
        self.postMessage(new WorkerResponse('error', correlationId, message));
        break;
      }
    }
    case 'close': {
      const fileUrl = request.detail as WorkerFileUrl;
      const url = new URL(fileUrl.url);
      const rootUrl = new URL(url.toString().split('#')[0]);
      if (openFiles.has(rootUrl.toString())) {
        const node = openFiles.get(rootUrl.toString());
        openFiles.delete(rootUrl.toString());
        node.delete();
      }
      unmountFile(url);
      self.postMessage(new WorkerResponse('closed', correlationId, { url: url.toString() }));
      break;
    }
    default:
      self.postMessage(new WorkerResponse('error', correlationId, `Unknown command: ${request.name}`));
      break;
  }
};
