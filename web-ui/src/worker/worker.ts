// import * as Module from "./libsf";
// import "./libsf.wasm";
// import libsf from './libsf.js';
// import * as libsf from "./libsf";
// import libsf from './libsf.js';
// import './libsf';
// import('./libsf');

import WorkerCommand from './WorkerCommand';
import WorkerResult from './WorkerResult';

// quench warnings for using "self", alternatively "globalThis" could be used instead
/* eslint-disable no-restricted-globals */

self.importScripts('libsf.js');

const hasInitCompleted: () => boolean = () => {
  // use @ts-expect-error instead of use @ts-ignore to make sure the issue occurs and
  // to avoid need to avoind @typescript-eslint/ban-ts-comment linter warning
  /* @ts-expect-error */
  return !(typeof Module === 'undefined' || Module === null || typeof Module.FileParser === 'undefined');
};

const saveFileToFilesystem = (url: URL, file: File) => {
  const dir = '/work';
  /* @ts-expect-error */
  const filesystem = FS;
  const dirExists = filesystem.analyzePath(dir, false).exists;
  if (!dirExists) {
      filesystem.mkdir(dir);
  }
  /* @ts-expect-error */
  const workerFS = WORKERFS;
  filesystem.mount(workerFS, { files: [file] }, dir);
};

const isFileRecognized = (name: string, url: URL) => {
  /* @ts-expect-error */
  const parser = new Module.JdxFileParser();
  console.log(`Parser: ${parser}`);
  const isFileRecognized = parser.isRecognized('/work/' + name);
  parser.delete();
  return isFileRecognized;
};

// let answer: number = 41;

self.onmessage = (event) => {
  const command = event.data as WorkerCommand;
  switch (command.name) {
    case 'status':
      const initCompleted = hasInitCompleted() ? 'initialized' : 'initializing';
      const result = new WorkerResult('state', initCompleted);
      self.postMessage(result);
      break;
    case 'scan':
      const url = new URL(command.detail.url);
      const file = command.detail.file;
      saveFileToFilesystem(url, file);
      self.postMessage(new WorkerResult('recognized', isFileRecognized(file.name, url)));
      break;
    default:
      self.postMessage(new WorkerResult('error', `Unknown command: ${command.name}`));
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
