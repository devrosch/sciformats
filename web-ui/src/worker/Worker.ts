import WorkerRequest from './WorkerRequest';
import WorkerResponse from './WorkerResponse';
import {
  initConverterService,
  onMessageStatus,
  onMessageScan,
  onMessageOpen,
  onMessageRead,
  onMessageClose,
} from './WorkerInternalUtils';

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
      self.postMessage(
        onMessageStatus(converterService, request.correlationId),
      );
      break;
    }
    case 'scan': {
      self.postMessage(
        /* @ts-expect-error */
        onMessageScan(request, workingDir, converterService, FS, WORKERFS),
      );
      break;
    }
    case 'open': {
      self.postMessage(
        onMessageOpen(
          request,
          workingDir,
          openFiles,
          converterService,
          /* @ts-expect-error */
          FS,
          /* @ts-expect-error */
          WORKERFS,
          /* @ts-expect-error */
          Module,
        ),
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
        new WorkerResponse(
          'error',
          request.correlationId,
          `Unknown command: ${request.name}`,
        ),
      );
      break;
  }
};

converterService = await initConverterService(self);
