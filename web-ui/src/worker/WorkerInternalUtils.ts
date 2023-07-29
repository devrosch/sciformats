import { extractFilename, extractUuid } from 'util/UrlUtils';
import PeakTable from 'model/PeakTable';
import WorkerStatus from './WorkerStatus';
import WorkerResponse from './WorkerResponse';
import WorkerFileInfo from './WorkerFileInfo';
import WorkerNodeData from './WorkerNodeData';
import WorkerRequest from './WorkerRequest';
import WorkerFileUrl from './WorkerFileUrl';

/**
 * Check if library module has been initialized.
 * @returns True if initialized, otherwise false.
 */
const hasModuleInitCompleted: (workerSelf: any) => boolean = (workerSelf: any) => !(
  typeof workerSelf.Module === 'undefined' || workerSelf.Module === null || typeof workerSelf.Module.Scanner === 'undefined'
);

/**
 * Initialize the converter service.
 * @returns Initialized converter service.
 */
export const initConverterService = async (workerSelf: any) => {
  while (!hasModuleInitCompleted(workerSelf)) {
    /* eslint-disable-next-line no-await-in-loop */
    await new Promise((resolve) => { setTimeout(resolve, 100); });
  }
  let jdxScanner = null;
  let scanners = null;
  try {
    jdxScanner = new workerSelf.Module.JdxScanner();
    /* eslint-disable-next-line new-cap */
    scanners = new workerSelf.Module.vector$std$$shared_ptr$sciformats$$api$$Scanner$$();
    scanners.push_back(jdxScanner);
    jdxScanner.delete();
  } catch (error) {
    if (jdxScanner !== null) {
      jdxScanner.delete();
    }
    if (scanners !== null) {
      scanners.delete();
    }
    throw error;
  }
  return new workerSelf.Module.ConverterService(scanners);
};

/**
 * Mounts a file in Emscripten's filesystem's "work" directory.
 * @param url The file URL from which UUID and file name are extracted.
 * @param blob A Blob containing the file data.
 * @param workingDir The working directory in Emscripten's file system.
 * @param filesystem Emscripten's file system (FS).
 * @param workerFS Emscripten's WORKERFS.
 */
export const mountFile = (
  url: URL,
  blob: Blob,
  workingDir: string,
  /* @ts-expect-error */
  filesystem: FS,
  /* @ts-expect-error */
  workerFS: WORKERFS,
) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  const workingDirExists = filesystem.analyzePath(workingDir, false).exists;
  if (!workingDirExists) {
    filesystem.mkdir(workingDir);
  }
  const uuidDirPath = `${workingDir}/${uuid}`;
  const uuidDirExists = filesystem.analyzePath(uuidDirPath, false).exists;
  if (!uuidDirExists) {
    filesystem.mkdir(uuidDirPath);
  }
  filesystem.mount(workerFS, {
    blobs: [{ name: filename, data: blob }],
  }, uuidDirPath);
};

/**
 * Unmounts a file from the filesystem's "work" directory.
 * @param url The file URL from which UUID and file name are extracted.
 * @param workingDir The working directory in Emscripten's file system.
 * @param filesystem Emscripten's file system (FS).
 */
/* @ts-expect-error */
export const unmountFile = (url: URL, workingDir: string, filesystem: FS) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  const workingDirExists = filesystem.analyzePath(workingDir, false).exists;
  if (!workingDirExists) {
    return;
  }
  const uuidDirPath = `${workingDir}/${uuid}`;
  const uuidDirExists = filesystem.analyzePath(uuidDirPath, false).exists;
  if (!uuidDirExists) {
    return;
  }
  const filePath = `${uuidDirPath}/${filename}`;
  const fileExists = filesystem.analyzePath(filePath, false).exists;
  if (fileExists) {
    filesystem.unmount(uuidDirPath);
  }
  filesystem.rmdir(uuidDirPath);
};

/**
 * Performs a shallow check if any file parser can parse the file.
 * @param url The file URL from which UUID and file name are extracted.
 * @param workingDir The working directory in Emscripten's file system.
 * @param scanner Scanner (e.g. ConverterService) to check if file is recognized.
 * @returns True if a file parser exists, false otherwise.
 */
/* @ts-expect-error */
export const isFileRecognized = (url: URL, workingDir: string, scanner: Module.Scanner) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  const filePath = `${workingDir}/${uuid}/${filename}`;
  const recognized = scanner.isRecognized(filePath);
  return recognized;
};

/**
 * Creates a new converter for the URL.
 * @param url URL to read.
 * @param workingDir The working directory in Emscripten's file system.
 * @param scanner Scanner (e.g. ConverterService) to check if file is recognized.
 * @returns Converter for URL.
 */
/* @ts-expect-error */
export const createConverter = (url: URL, workingDir: string, scanner: Module.Scanner) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  const filePath = `${workingDir}/${uuid}/${filename}`;
  return scanner.getConverter(filePath);
};

/**
 * Reads data from the URL representing a node in an opened file.
 * @param url URL to read.
 * @param openFiles Map of root URLs and corresponding converters.
 * @returns The node corresponding to the URL.
 */
/* @ts-expect-error */
export const readNode = (url: URL, openFiles: Map<string, Module.Converter>) => {
  const rootUrl = new URL(url.toString().split('#')[0]);

  if (!openFiles.has(rootUrl.toString())) {
    throw new Error(`File not found: ${rootUrl}`);
  }
  let hash = url.hash;
  if (hash.length > 0 && !hash.startsWith('#/')) {
    throw new Error(`Unexpected URL hash: ${hash}`);
  }

  const converter = openFiles.get(rootUrl.toString());

  // '', '#', '#/' all denote the root node
  // splitting by '/' results in:
  // '' => ['']
  // '/' => ['', '']
  // TODO: implement util function for this
  if (hash.startsWith('#')) {
    hash = hash.substring(1);
  }
  if (hash.length === 0) {
    hash = '/';
  }

  // node is of type Node and bound as a value object, hence it has no delete() method
  const node = converter.read(hash);
  return node;
};

/**
 * Maps node data to WorkerNodeData pure JSON.
 * @param url The URL representing a node.
 * @param node The node corresponding to the URL.
 * @returns WorkerNodeData JSON.
 */
/* @ts-expect-error */
export const nodeToJson = (url: URL, node: Module.Node): WorkerNodeData => {
  const json: WorkerNodeData = {} as WorkerNodeData;

  json.url = url.toString();

  // unused: json.name = node.name;

  // parameters
  const params = node.parameters;
  const paramsSize = params.size();
  const jsonParameters: any = [];
  for (let index = 0; index < paramsSize; index += 1) {
    const keyValuePair = params.get(index);
    const key = keyValuePair.key;
    const value = keyValuePair.value;
    jsonParameters.push({ key, value });
  }
  json.parameters = jsonParameters;
  params.delete();

  // data
  const data = node.data;
  const dataSize = data.size();
  const jsonData = [];
  for (let index = 0; index < dataSize; index += 1) {
    const point = data.get(index);
    const x = point.x;
    const y = point.y;
    jsonData.push({ x, y });
  }
  json.data = jsonData;
  data.delete();

  // peak table
  const jsonPeakTable: PeakTable = { columnNames: [], peaks: [] };

  const peakTable = node.peakTable;
  const peakTableColumnNames = peakTable.columnNames;
  const peakColumnCount = peakTableColumnNames.size();
  for (let index = 0; index < peakColumnCount; index += 1) {
    const columnKeyValuePair = peakTableColumnNames.get(index);
    jsonPeakTable.columnNames.push(
      { key: columnKeyValuePair.first, value: columnKeyValuePair.second },
    );
    // columnKeyValuePair is value object => no delete()
  }

  const peaks = peakTable.peaks;
  const peakCount = peaks.size();
  for (let peakIndex = 0; peakIndex < peakCount; peakIndex += 1) {
    const jsonPeak = new Map<string, string>();
    const peak = peaks.get(peakIndex);
    for (const column of jsonPeakTable.columnNames) {
      const columnKey = column.key;
      // TODO: what happens if no value for key?
      const peakValue = peak.get(columnKey);
      jsonPeak.set(columnKey, peakValue);
    }
    jsonPeakTable.peaks.push(jsonPeak);
    peak.delete();
  }
  json.peakTable = jsonPeakTable;
  peaks.delete();

  console.log('jsonPeakTable: ');
  console.log({ jsonPeakTable });

  // child node names
  const childNodeNames = node.childNodeNames;
  const childNodesSize = childNodeNames.size();
  const jsonChildNodes = [];
  for (let index = 0; index < childNodesSize; index += 1) {
    const childNodeName = childNodeNames.get(index);
    jsonChildNodes.push(childNodeName);
  }
  json.childNodeNames = jsonChildNodes;
  childNodeNames.delete();

  return json;
};

/**
 * Retrieves exception message from exception object.
 * @param exception Exception object.
 * @param module Emscripten Module.
 * @returns The exception message.
 * @description
 * For JS wrapped Emscripten C++/WASM exception, expects getCppExceptionMessage() to be present in
 * module. For native C++/WASM exception, expects getExceptionMessage() to be present in module.
 */
/* @ts-expect-error */
export const getExceptionMessage = (exception: any, module: Module) => {
  if (exception.message || exception instanceof Error) {
    // JS Error
    return exception.message;
  }
  if (typeof exception === 'number') {
    // JS wrapped C++/WASM exception
    const msg = module.getCppExceptionMessage(exception);
    return msg;
  }
  /* @ts-expect-error */
  if (exception instanceof WebAssembly.Exception) {
    // native C++/WASM exception
    // see: https://github.com/emscripten-core/emscripten/issues/16033
    // https://github.com/emscripten-core/emscripten/pull/17219
    const msgArray = module.getExceptionMessage(exception);
    const msg = Array.isArray(msgArray) ? msgArray.join(', ') : msgArray;
    return msg;
  }
  // something else
  return exception;
};

export const onMessageStatus = (
  /* @ts-expect-error */
  converterService: Module.ConverterService,
  correlationId: string,
) => {
  const moduleInitCompleted = converterService === null ? WorkerStatus.Initializing
    : WorkerStatus.Initialized;
  return new WorkerResponse('status', correlationId, moduleInitCompleted);
};

export const onMessageScan = (
  request: WorkerRequest,
  workingDir: string,
  /* @ts-expect-error */
  converterService: Module.ConverterService,
  /* @ts-expect-error */
  filesystem: FS,
  /* @ts-expect-error */
  workerFs: WORKERFS,
) => {
  const fileInfo = request.detail as WorkerFileInfo;
  const url = new URL(fileInfo.url);
  const blob = fileInfo.blob;
  mountFile(url, blob, workingDir, filesystem, workerFs);
  const recognized = isFileRecognized(url, workingDir, converterService);
  unmountFile(url, workingDir, filesystem);
  return new WorkerResponse('scanned', request.correlationId, { recognized });
};

export const onMessageOpen = (
  request: WorkerRequest,
  workingDir: string,
  /* @ts-expect-error */
  openFiles: Map<string, Module.Converter>,
  /* @ts-expect-error */
  converterService: Module.ConverterService,
  /* @ts-expect-error */
  filesystem: FS,
  /* @ts-expect-error */
  workerFs: WORKERFS,
  /* @ts-expect-error */
  module: Module,
) => {
  const correlationId = request.correlationId;
  const fileInfo = request.detail as WorkerFileInfo;
  const url = new URL(fileInfo.url);
  const blob = fileInfo.blob;
  const rootUrl = new URL(url.toString().split('#')[0]);
  if (!openFiles.has(rootUrl.toString())) {
    try {
      mountFile(url, blob, workingDir, filesystem, workerFs);
      const converter = createConverter(url, workingDir, converterService);
      openFiles.set(rootUrl.toString(), converter);
    } catch (error: any) {
      const message = getExceptionMessage(error, module);
      return new WorkerResponse('error', correlationId, message);
    }
  }
  return new WorkerResponse('opened', correlationId, { url: rootUrl.toString() });
};

export const onMessageRead = (
  request: WorkerRequest,
  /* @ts-expect-error */
  openFiles: Map<string, Module.Converter>,
  /* @ts-expect-error */
  module: Module,
) => {
  const correlationId = request.correlationId;
  const fileUrl = request.detail as WorkerFileUrl;
  const url = new URL(fileUrl.url);
  try {
    const node = readNode(url, openFiles);
    const nodeData: WorkerNodeData = nodeToJson(url, node);
    return new WorkerResponse('read', correlationId, nodeData);
  } catch (error: any) {
    const message = getExceptionMessage(error, module);
    return new WorkerResponse('error', correlationId, message);
  }
};

export const onMessageClose = (
  request: WorkerRequest,
  workingDir: string,
  /* @ts-expect-error */
  openFiles: Map<string, Module.Converter>,
  /* @ts-expect-error */
  filesystem: FS,
) => {
  const correlationId = request.correlationId;
  const fileUrl = request.detail as WorkerFileUrl;
  const url = new URL(fileUrl.url);
  const rootUrl = new URL(url.toString().split('#')[0]);
  if (openFiles.has(rootUrl.toString())) {
    const node = openFiles.get(rootUrl.toString());
    openFiles.delete(rootUrl.toString());
    node.delete();
  }
  unmountFile(url, workingDir, filesystem);
  return new WorkerResponse('closed', correlationId, { url: rootUrl.toString() });
};
