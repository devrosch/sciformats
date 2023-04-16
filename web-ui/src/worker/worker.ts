// import * as Module from "./libsf";
// import "./libsf.wasm";
// import libsf from './libsf.js';
// import * as libsf from "./libsf";
// import libsf from './libsf.js';
// import './libsf';
// import('./libsf');

// eslint-disable-next-line no-restricted-globals
self.importScripts('libsf.js');

let answer: number = 41;

// eslint-disable-next-line no-restricted-globals
self.onmessage = ({ data: { question } }) => {
  console.log('Module:');
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  console.log(Module);
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  const sfr = new Module.StubFileParser();
  console.log(`StubFileParser: ${sfr}`);
  sfr.delete();

  answer += 1;
  // eslint-disable-next-line no-restricted-globals
  self.postMessage({
    answer: `${question} -> ${answer}`,
  });
};

// self.importScripts('libsf.js');
// self.importScripts(libsf);
