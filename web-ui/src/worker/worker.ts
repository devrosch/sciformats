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
  let json: any = {};
  json['name'] = node.name;
  // TODO:
  // [[nodiscard]] virtual std::string getName() const = 0;
  // virtual std::vector<KeyValueParam> getParams() = 0;
  // virtual std::optional<std::vector<Point2D>> getData() = 0;
  // virtual std::vector<std::shared_ptr<Node>> getChildNodes() = 0;
  return json;
}

// let answer: number = 41;

/* @ts-expect-error */
const openFiles = new Map<URL, Module.Node>();

self.onmessage = (event) => {
  const request = event.data as WorkerRequest;
  switch (request.name) {
    case 'status': {
      const initCompleted = hasInitCompleted() ? 'initialized' : 'initializing';
      const result = new WorkerResponse('state', initCompleted);
      self.postMessage(result);
      break;
    }
    case 'scan': {
      const url = new URL(request.detail.url);
      const file = request.detail.file;
      mountFile(url, file);
      const recognized = isFileRecognized(url);
      unmountFile(url);
      self.postMessage(new WorkerResponse('recognized', recognized));
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
          self.postMessage(new WorkerResponse('error', message));
          break;
        }
      }
      const node = openFiles.get(rootUrl);
      const json = nodeToJson(node);
      self.postMessage(new WorkerResponse('opened', json));
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
      self.postMessage(new WorkerResponse('closed', url.toString()));
      break;
    }
    default:
      self.postMessage(new WorkerResponse('error', `Unknown command: ${request.name}`));
      break;
  }

  // console.log('Module:');
  // /* eslint-disable-next-line @typescript-eslint/ban-ts-comment */
  // /* @ts-ignore */
  // console.log(Module);
  /* eslint-disable-next-line @typescript-eslint/ban-ts-comment */
  /* @ts-ignore */
  // const sfr = new Module.StubFileParser();
  // console.log(`StubFileParser: ${sfr}`);
  // sfr.delete();

  // answer += 1;
  // /* eslint-disable-next-line no-restricted-globals */
  // self.postMessage({
  //   answer: `${command} -> ${answer}`,
  // });
};

// self.importScripts('libsf.js');
// self.importScripts(libsf);
