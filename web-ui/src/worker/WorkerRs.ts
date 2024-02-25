import * as sf_rs from 'sf_rs';
import WorkerRequest from './WorkerRequest';
import WorkerResponse from './WorkerResponse';
import WorkerStatus from './WorkerStatus';
import {
  onClose, onOpen, onRead, onScan,
} from './WorkerRsInternalUtils';

// quench warnings for using "self", alternatively "globalThis" could be used instead
/* eslint-disable no-restricted-globals */

const openFiles = new Map<string, sf_rs.Reader>();
const scanner = new sf_rs.ScannerRepository();

self.onmessage = (event) => {
  const request = event.data as WorkerRequest;
  switch (request.name) {
    case 'status': {
      // library has been imported already when onmessage gets defined
      self.postMessage(new WorkerResponse('status', request.correlationId, WorkerStatus.Initialized));
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
    case 'close': {
      self.postMessage(onClose(request, openFiles));
      break;
    }
    default:
      self.postMessage(
        new WorkerResponse('error', request.correlationId, `Unknown command: ${request.name}`),
      );
      break;
  }
};
