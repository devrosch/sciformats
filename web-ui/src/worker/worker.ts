// import * as Module from "./libsf";
// import "./libsf.wasm";
// import libsf from './libsf.js';
// import * as libsf from "./libsf";
// import libsf from './libsf.js';
// import './libsf';
// import('./libsf');

self.importScripts('libsf.js');

let answer: number = 42;

self.onmessage = ({ data: { question } }) => {
  console.log('Module:');
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  console.log(Module);
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  let sfr = new Module.StubFileParser();
  console.log('StubFileParser: ' + sfr);
  sfr.delete();

  self.postMessage({
    answer: `${question} -> ${answer++}`,
  });
};

// self.importScripts('libsf.js');
// self.importScripts(libsf);
