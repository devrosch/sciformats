import WorkerRequest from './WorkerRequest';
import WorkerResponse from './WorkerResponse';
import * as sf_rs from 'sf_rs';
import WorkerStatus from './WorkerStatus';
import WorkerFileInfo from './WorkerFileInfo';
import { extractFilename } from 'util/UrlUtils';

// quench warnings for using "self", alternatively "globalThis" could be used instead
/* eslint-disable no-restricted-globals */

// /* @ts-expect-error */
// const openFiles = new Map<string, Module.Converter>();
// /* @ts-expect-error */
// let converterService: Module.ConverterService | null = null;

let converterService = new sf_rs.AndiChromScanner();

self.onmessage = (event) => {
  const request = event.data as WorkerRequest;
  switch (request.name) {
    case 'status': {
      self.postMessage(new WorkerResponse('status', request.correlationId, WorkerStatus.Initialized));
      break;
    }
    case 'scan': {
      const fileInfo = request.detail as WorkerFileInfo;
      const url = new URL(fileInfo.url);
      const fileName = extractFilename(url);
      const file = fileInfo.blob as File;
      const fileWrapper = new sf_rs.FileWrapper(file);
      
      console.log('Worker: JS: file name: ' + fileName);
      console.log('Worker: JS: file: ' + file);
      console.log('Worker: JS: file wrapper: ' + fileWrapper);

      const recognized = converterService.js_is_recognized(fileName, fileWrapper);
      fileWrapper.free();
      self.postMessage(new WorkerResponse('scanned', request.correlationId, { recognized }));
      break;
    }
    case 'open': {
      self.postMessage(
        /* @ts-expect-error */
        onMessageOpen(request, workingDir, openFiles, converterService, FS, WORKERFS, Module),
      );
      break;
    }
    case 'read': {
      /* @ts-expect-error */
      self.postMessage(onMessageRead(request, openFiles, Module));
      break;
    }
    case 'close': {
      /* @ts-expect-error */
      self.postMessage(onMessageClose(request, workingDir, openFiles, FS));
      break;
    }
    default:
      self.postMessage(
        new WorkerResponse('error', request.correlationId, `Unknown command: ${request.name}`),
      );
      break;
  }
};

console.log('JS: sf_rs worker loaded');
