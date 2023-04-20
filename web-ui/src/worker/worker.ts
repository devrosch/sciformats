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
 * Mounts the file in the filesystem's "work" directory.
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
  // filesystem.mount(workerFS, { files: [file] }, dir);
  filesystem.mount(workerFS, {
    blobs: [{ name: filename, data: blob }],
  }, uuidDirPath);
};

/* eslint-disable-next-line @typescript-eslint/no-unused-vars */
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

// let answer: number = 41;

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
      self.postMessage(new WorkerResponse('recognized', isFileRecognized(url)));
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
