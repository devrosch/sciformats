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
  const correlationId = request.correlationId;
  switch (request.name) {
    case 'status': {
      self.postMessage(onMessageStatus(converterService, correlationId));
      break;
    }
    case 'scan': {
      self.postMessage(onMessageScan(request, workingDir, converterService, correlationId));
      break;
    }
    case 'open': {
      self.postMessage(
        onMessageOpen(request, workingDir, openFiles, converterService, correlationId),
      );
      break;
    }
    case 'read': {
      self.postMessage(onMessageRead(request, openFiles, correlationId));
      break;
    }
    case 'close': {
      self.postMessage(onMessageClose(request, workingDir, openFiles, correlationId));
      break;
    }
    default:
      self.postMessage(
        new WorkerResponse('error', correlationId, `Unknown command: ${request.name}`),
      );
      break;
  }
};

converterService = await initConverterService(self);
