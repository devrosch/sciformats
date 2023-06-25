import WorkerRequest from './WorkerRequest';
import WorkerResponse from './WorkerResponse';
import WorkerStatus from './WorkerStatus';
import WorkerFileInfo from './WorkerFileInfo';
import WorkerNodeData from './WorkerNodeData';
import WorkerFileUrl from './WorkerFileUrl';
import {
  mountFile, unmountFile, isFileRecognized, readNode, nodeToJson,
  getExceptionMessage, createConverter, initConverterService,
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
      const moduleInitCompleted = converterService === null ? WorkerStatus.Initialized
        : WorkerStatus.Initializing;
      const result = new WorkerResponse('status', correlationId, moduleInitCompleted);
      self.postMessage(result);
      break;
    }
    case 'scan': {
      const fileInfo = request.detail as WorkerFileInfo;
      const url = new URL(fileInfo.url);
      const blob = fileInfo.blob;
      /* @ts-expect-error */
      mountFile(url, blob, workingDir, FS, WORKERFS);
      const recognized = isFileRecognized(url, workingDir, converterService);
      /* @ts-expect-error */
      unmountFile(url, workingDir, FS);
      self.postMessage(new WorkerResponse('scanned', correlationId, { recognized }));
      break;
    }
    case 'open': {
      const fileInfo = request.detail as WorkerFileInfo;
      const url = new URL(fileInfo.url);
      const blob = fileInfo.blob;
      const rootUrl = new URL(url.toString().split('#')[0]);
      if (!openFiles.has(rootUrl.toString())) {
        try {
          /* @ts-expect-error */
          mountFile(url, blob, workingDir, FS, WORKERFS);
          const mappingParser = createConverter(url, workingDir, converterService);
          openFiles.set(rootUrl.toString(), mappingParser);
        } catch (error: any) {
          /* @ts-expect-error */
          const message = getExceptionMessage(error, Module);
          self.postMessage(new WorkerResponse('error', correlationId, message));
          break;
        }
      }
      self.postMessage(new WorkerResponse('opened', correlationId, { url: rootUrl.toString() }));
      break;
    }
    case 'read': {
      const fileUrl = request.detail as WorkerFileUrl;
      const url = new URL(fileUrl.url);
      try {
        const node = readNode(url, openFiles);
        const nodeData: WorkerNodeData = nodeToJson(url, node);
        self.postMessage(new WorkerResponse('read', correlationId, nodeData));
        break;
      } catch (error: any) {
        /* @ts-expect-error */
        const message = getExceptionMessage(error, Module);
        self.postMessage(new WorkerResponse('error', correlationId, message));
        break;
      }
    }
    case 'close': {
      const fileUrl = request.detail as WorkerFileUrl;
      const url = new URL(fileUrl.url);
      const rootUrl = new URL(url.toString().split('#')[0]);
      if (openFiles.has(rootUrl.toString())) {
        const node = openFiles.get(rootUrl.toString());
        openFiles.delete(rootUrl.toString());
        node.delete();
      }
      /* @ts-expect-error */
      unmountFile(url, workingDir, FS);
      self.postMessage(new WorkerResponse('closed', correlationId, { url: url.toString() }));
      break;
    }
    default:
      self.postMessage(new WorkerResponse('error', correlationId, `Unknown command: ${request.name}`));
      break;
  }
};

converterService = await initConverterService(self);
