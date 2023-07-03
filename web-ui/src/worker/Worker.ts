import WorkerRequest from './WorkerRequest';
import WorkerResponse from './WorkerResponse';
import {
  initConverterService, onMessageStatus, onMessageScan, onMessageOpen, onMessageRead,
  onMessageClose,
} from './WorkerInternalUtils';

// quench warnings for using "self", alternatively "globalThis" could be used instead
/* eslint-disable no-restricted-globals */

self.importScripts('libsf.js');

const workingDir = '/work';
/* @ts-expect-error */
const openFiles = new Map<string, Module.Converter>();
/* @ts-expect-error */
let converterService: Module.ConverterService | null = null;

self.onmessage = (event) => {
  const request = event.data as WorkerRequest;
  switch (request.name) {
    case 'status': {
      self.postMessage(onMessageStatus(converterService, request.correlationId));
      break;
    }
    case 'scan': {
      /* @ts-expect-error */
      self.postMessage(onMessageScan(request, workingDir, converterService, FS, WORKERFS));
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

converterService = await initConverterService(self);
