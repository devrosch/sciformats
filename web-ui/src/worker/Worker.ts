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
let converterService: Module.ConverterService | null = null;
/* @ts-expect-error */
const openFiles = new Map<string, Module.Converter>();

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
      self.postMessage(onMessageRead(request, openFiles));
      break;
    }
    case 'close': {
      self.postMessage(onMessageClose(request, workingDir, openFiles));
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
