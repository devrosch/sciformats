import * as sf_rs from 'sf_rs';
import { extractFilename } from 'util/UrlUtils';
import WorkerRequest from './WorkerRequest';
import WorkerResponse from './WorkerResponse';
import WorkerStatus from './WorkerStatus';
import WorkerFileInfo from './WorkerFileInfo';
import WorkerNodeData from './WorkerNodeData';
import WorkerFileUrl from './WorkerFileUrl';

// quench warnings for using "self", alternatively "globalThis" could be used instead
/* eslint-disable no-restricted-globals */

const openFiles = new Map<string, sf_rs.JsReader>();
const converterService = new sf_rs.AndiChromScanner();

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
      const recognized = converterService.js_is_recognized(fileName, fileWrapper);
      fileWrapper.free();
      self.postMessage(new WorkerResponse('scanned', request.correlationId, { recognized }));
      break;
    }
    case 'open': {
      const fileInfo = request.detail as WorkerFileInfo;
      const url = new URL(fileInfo.url);
      const rootUrl = new URL(url.toString().split('#')[0]);
      const fileName = extractFilename(url);
      const file = fileInfo.blob as File;
      const fileWrapper = new sf_rs.FileWrapper(file);

      try {
        const reader = converterService.js_get_reader(fileName, fileWrapper);
        openFiles.set(rootUrl.toString(), reader);
        self.postMessage(
          new WorkerResponse('opened', request.correlationId, { url: rootUrl.toString() }),
        );
      } catch (error) {
        self.postMessage(
          new WorkerResponse('error', request.correlationId, `${error}`),
        );
      }
      break;
    }
    case 'read': {
      const fileInfo = request.detail as WorkerFileInfo;
      const url = new URL(fileInfo.url);
      const rootUrl = new URL(url.toString().split('#')[0]);
      try {
        const reader = openFiles.get(rootUrl.toString());
        if (typeof reader === 'undefined') {
          throw new Error(`No open file found for ${url.toString()}`);
        }
        let hash = url.hash;
        if (hash.length > 0 && !hash.startsWith('#/')) {
          throw new Error(`Unexpected URL hash: ${hash}`);
        }

        // '', '#', '#/' all denote the root node
        // splitting by '/' results in:
        // '' => ['']
        // '/' => ['', '']
        if (hash.startsWith('#')) {
          hash = hash.substring(1);
        }
        if (hash.length === 0) {
          hash = '/';
        }

        const rawNode = reader.read(hash);

        const node: WorkerNodeData = {
          url: url.toString(),
          parameters: rawNode.parameters,
          data: rawNode.data,
          metadata: rawNode.metadata as { [key: string]: string },
          table: rawNode.table as { columnNames: [], rows: [] },
          childNodeNames: rawNode.child_node_names,
        };

        rawNode.free();

        self.postMessage(
          new WorkerResponse('read', request.correlationId, node),
        );
      } catch (error) {
        self.postMessage(
          new WorkerResponse('error', request.correlationId, `${error}`),
        );
      }
      break;
    }
    case 'close': {
      const correlationId = request.correlationId;
      const fileUrl = request.detail as WorkerFileUrl;
      const url = new URL(fileUrl.url);
      const rootUrl = new URL(url.toString().split('#')[0]);
      if (openFiles.has(rootUrl.toString())) {
        const reader = openFiles.get(rootUrl.toString());
        openFiles.delete(rootUrl.toString());
        reader?.free();
      }
      self.postMessage(new WorkerResponse('closed', correlationId, { url: rootUrl.toString() }));
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
