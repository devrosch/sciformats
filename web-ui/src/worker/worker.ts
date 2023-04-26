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

// quench warnings for using "self", alternatively "globalThis" could be used instead
/* eslint-disable no-restricted-globals */

self.importScripts('libsf.js');

const workingDir = '/work';

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
  const parser = new Module.JdxFileParser();
  console.log(`Parser: ${parser}`);
  const filePath = `${workingDir}/${uuid}/${filename}`;
  const recognized = parser.isRecognized(filePath);
  parser.delete();
  return recognized;
};

const parse = (url: URL) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  /* @ts-expect-error */
  const parser = new Module.JdxFileParser();
  const filePath = `${workingDir}/${uuid}/${filename}`;
  try {
    const node = parser.parse(filePath);
    parser.delete();
    return node;
  } catch (error) {
    parser.delete();
    throw error;
  }
};

/* @ts-expect-error */
const nodeToJson = (node: Module.Node) => {
  const json: any = {};

  // [[nodiscard]] virtual std::string getName() const;
  json.name = node.name;

  // virtual std::vector<KeyValueParam> getParams();
  const params = node.getParams();
  const paramsSize = params.size();
  const jsonParameters: any = {};
  for (let index = 0; index < paramsSize; index += 1) {
    const keyValuePair = params.get(index);
    const key = keyValuePair.key;
    const value = keyValuePair.value;
    jsonParameters[key] = value;
  }
  json.parameters = jsonParameters;
  params.delete();

  // virtual std::vector<Point2D> getData();
  const data = node.getData();
  const dataSize = data.size();
  const jsonData = [];
  for (let index = 0; index < dataSize; index += 1) {
    const point = data.get(index);
    const x = point.x;
    const y = point.y;
    jsonData.push({ x, y });
  }
  json.data = jsonData;

  // virtual std::vector<std::shared_ptr<Node>> getChildNodes() = 0;
  const childNodes = node.getChildNodes();
  const childNodesSize = childNodes.size();
  const jsonChildNodes = [];
  for (let index = 0; index < childNodesSize; index += 1) {
    const childNode = childNodes.get(index);
    const childNodeName = childNode.name;
    jsonChildNodes.push(childNodeName);
    childNode.delete();
  }
  json.children = jsonChildNodes;
  childNodes.delete();

  return json;
};

/* @ts-expect-error */
const openFiles = new Map<URL, Module.Node>();

self.onmessage = (event) => {
  const request = event.data as WorkerRequest;
  const correlationId = request.correlationId;
  switch (request.name) {
    case 'status': {
      const initCompleted = hasInitCompleted() ? 'initialized' : 'initializing';
      const result = new WorkerResponse('state', correlationId, initCompleted);
      self.postMessage(result);
      break;
    }
    case 'scan': {
      const url = new URL(request.detail.url);
      const file = request.detail.file;
      mountFile(url, file);
      const recognized = isFileRecognized(url);
      unmountFile(url);
      self.postMessage(new WorkerResponse('recognized', correlationId, recognized));
      break;
    }
    case 'open': {
      const url = new URL(request.detail.url);
      const file = request.detail.file;
      const rootUrl = new URL(url.toString().split('#')[0]);
      if (!openFiles.has(rootUrl)) {
        mountFile(url, file);
        try {
          const node = parse(url);
          openFiles.set(rootUrl, node);
        } catch (error: any) {
          const message = error.message;
          unmountFile(url);
          self.postMessage(new WorkerResponse('error', correlationId, message));
          break;
        }
      }
      const node = openFiles.get(rootUrl);
      const json = nodeToJson(node);
      self.postMessage(new WorkerResponse('opened', correlationId, json));
      break;
    }
    case 'close': {
      const url = new URL(request.detail.url);
      const rootUrl = new URL(url.toString().split('#')[0]);
      if (openFiles.has(rootUrl)) {
        const node = openFiles.get(rootUrl);
        openFiles.delete(rootUrl);
        node.delete();
      }
      unmountFile(url);
      self.postMessage(new WorkerResponse('closed', correlationId, url.toString()));
      break;
    }
    default:
      self.postMessage(new WorkerResponse('error', correlationId, `Unknown command: ${request.name}`));
      break;
  }
};
