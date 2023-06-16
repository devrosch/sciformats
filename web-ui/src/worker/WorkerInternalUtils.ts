import { extractFilename, extractUuid } from 'util/UrlUtils';

/**
 * Check if library module has been initialized.
 * @returns True if initialized, otherwise false.
 */
export const hasInitCompleted: () => boolean = () => !(
  // use @ts-expect-error instead of use @ts-ignore to make sure the issue occurs and
  // to avoid need to avoind @typescript-eslint/ban-ts-comment linter warning
  /* @ts-expect-error */
  typeof Module === 'undefined' || Module === null || typeof Module.Scanner === 'undefined');

  /**
   * Initialized the converter service.
   * @returns Initialized converter service.
   */
export const initConverterService = () => {
  if (!hasInitCompleted()) {
    throw Error('Cannot initialize converter service. Module initialization has not completed.');
  }
  /* @ts-expect-error */
  const jdxScanner = new Module.JdxScanner();
  /* @ts-expect-error */
  let scanners = new Module.vector$std$$shared_ptr$sciformats$$api$$Scanner$$();
  scanners.push_back(jdxScanner);
  /* @ts-expect-error */
  return new Module.ConverterService(scanners);
};

/**
 * Mounts a file in Emscripten's filesystem's "work" directory.
 * @param url The file URL from which UUID and file name are extracted.
 * @param blob A Blob containing the file data.
 * @param workingDir The working directory in Emscripten's file system.
 */
export const mountFile = (url: URL, blob: Blob, workingDir: string) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  /* @ts-expect-error */
  const filesystem = FS;
  const workingDirExists = filesystem.analyzePath(workingDir, false).exists;
  if (!workingDirExists) {
    filesystem.mkdir(workingDir);
  }
  const uuidDirPath = `${workingDir}/${uuid}`;
  const uuidDirExists = filesystem.analyzePath(uuidDirPath, false).exists;
  if (!uuidDirExists) {
    filesystem.mkdir(uuidDirPath);
  }
  /* @ts-expect-error */
  const workerFS = WORKERFS;
  filesystem.mount(workerFS, {
    blobs: [{ name: filename, data: blob }],
  }, uuidDirPath);
};

/**
 * Unmounts a file from the filesystem's "work" directory.
 * @param url The file URL from which UUID and file name are extracted.
 * @param workingDir The working directory in Emscripten's file system.
 */
export const unmountFile = (url: URL, workingDir: string) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  /* @ts-expect-error */
  const filesystem = FS;
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
 * Creates a new mapping parser for the URL.
 * @param url URL to read.
 * @param workingDir The working directory in Emscripten's file system.
 * @param scanner Scanner (e.g. ConverterService) to check if file is recognized.
 * @returns Mapping parser for URL.
 */
/* @ts-expect-error */
export const createConverter = (url: URL, workingDir: string, scanner: Module.Scanner) => {
  const uuid = extractUuid(url);
  const filename = extractFilename(url);
  const filePath = `${workingDir}/${uuid}/${filename}`;
  let converter = null;
  try {
    converter = scanner.getConverter(filePath);
    return converter;
  } catch (error) {
    if (converter !== null) {
      converter.delete();
    }
    throw error;
  }
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
    console.log(`root URL: ${rootUrl}`);
    console.log(`number of keys in openFiles: ${openFiles.keys.length}`);
    Object.keys(openFiles).forEach((key) => {
      console.log(`map key: ${key}`);
    });
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
export const nodeToJson = (url: URL, node: Module.Node) => {
  const json: any = {};

  json.url = url.toString();

  // // [[nodiscard]] virtual std::string getName() const;
  // json.name = node.name;

  // virtual std::vector<KeyValueParam> getParams();
  const params = node.parameters;
  const paramsSize = params.size();
  const jsonParameters: any = [];
  for (let index = 0; index < paramsSize; index += 1) {
    const keyValuePair = params.get(index);
    const key = keyValuePair.key;
    const value = keyValuePair.value;
    // jsonParameters[key] = value;
    jsonParameters.push({ key, value });
  }
  json.parameters = jsonParameters;
  params.delete();

  // virtual std::vector<Point2D> getData();
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

  // virtual std::vector<std::shared_ptr<Node>> getChildNodes() = 0;
  const childNodeNames = node.childNodeNames;
  const childNodesSize = childNodeNames.size();
  const jsonChildNodes = [];
  for (let index = 0; index < childNodesSize; index += 1) {
    const childNodeName = childNodeNames.get(index);
    jsonChildNodes.push(childNodeName);
  }
  json.children = jsonChildNodes;
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
  if (exception.message) {
    // JS Error
    return exception.message;
  }
  // something else
  return exception;
};
