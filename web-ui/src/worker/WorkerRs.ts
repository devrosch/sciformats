import * as sf_js from 'sf_js';
import WorkerRequest from './WorkerRequest';
import WorkerResponse from './WorkerResponse';
import WorkerStatus from './WorkerStatus';
import {
  onClose,
  onExport,
  onOpen,
  onRead,
  onScan,
} from './WorkerRsInternalUtils';

const openFiles = new Map<string, sf_js.Reader>();
const scanner = new sf_js.ScannerRepository();

self.onmessage = (event) => {
  const request = event.data as WorkerRequest;
  switch (request.name) {
    case 'status': {
      // library has been imported already when onmessage gets defined
      self.postMessage(
        new WorkerResponse(
          'status',
          request.correlationId,
          WorkerStatus.Initialized,
        ),
      );
      break;
    }
    case 'scan': {
      self.postMessage(onScan(request, scanner));
      break;
    }
    case 'open': {
      self.postMessage(onOpen(request, scanner, openFiles));
      break;
    }
    case 'read': {
      self.postMessage(onRead(request, openFiles));
      break;
    }
    case 'export': {
      self.postMessage(onExport(request, openFiles));
      break;
    }
    case 'close': {
      self.postMessage(onClose(request, openFiles));
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
