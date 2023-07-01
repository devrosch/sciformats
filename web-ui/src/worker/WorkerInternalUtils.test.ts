import WorkerFileInfo from './WorkerFileInfo';
import * as WorkerInternalUtils from './WorkerInternalUtils';
import WorkerRequest from './WorkerRequest';
import WorkerStatus from './WorkerStatus';

const uuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ee';
const filename = 'test.jdx';
const url = new URL(`file:///${uuid}/${filename}/#`);
const workingDir = '/work';
const filePath = `${workingDir}/${uuid}/${filename}`;

test('initConverterService() waits for Module init and initializes ConverterService', async () => {
  const pushBackMock = jest.fn();
  const workerSelfMock = {
    Module: {
      Scanner: jest.fn(),
      JdxScanner: jest.fn(),
      vector$std$$shared_ptr$sciformats$$api$$Scanner$$: jest.fn(
        () => ({ push_back: pushBackMock }),
      ),
      ConverterService: jest.fn(),
    },
  };

  await WorkerInternalUtils.initConverterService(workerSelfMock);

  expect(workerSelfMock.Module.JdxScanner).toHaveBeenCalledTimes(1);
  expect(workerSelfMock.Module.vector$std$$shared_ptr$sciformats$$api$$Scanner$$)
    .toHaveBeenCalledTimes(1);
  expect(pushBackMock).toHaveBeenCalledTimes(1);
  expect(workerSelfMock.Module.ConverterService).toHaveBeenCalledTimes(1);
});

test('mountFile() creates workingDir and UUID directories and mounts WORKERFS', async () => {
  const blob = new Blob();
  const uuidPath = `${workingDir}/${uuid}`;
  const analyzePath = jest.fn(() => ({ exists: false }));
  const mkdir = jest.fn();
  const mount = jest.fn();
  const filesystemMock = {
    analyzePath,
    mkdir,
    mount,
  };
  const workerFs = {};

  WorkerInternalUtils.mountFile(url, blob, workingDir, filesystemMock, workerFs);

  expect(analyzePath).toHaveBeenCalledTimes(2);
  expect(analyzePath).toHaveBeenCalledWith(workingDir, false);
  expect(analyzePath).toHaveBeenCalledWith(uuidPath, false);
  expect(mkdir).toHaveBeenCalledTimes(2);
  expect(mkdir).toHaveBeenCalledWith(workingDir);
  expect(mkdir).toHaveBeenCalledWith(uuidPath);
  expect(mount).toHaveBeenCalledTimes(1);
  expect(mount).toHaveBeenCalledWith(
    workerFs,
    { blobs: [{ name: filename, data: blob }] },
    uuidPath,
  );
});

test('unmountFile() unmounts WORKERFS and deletes UUID directory', async () => {
  const analyzePath = jest.fn(() => ({ exists: true }));
  const rmdir = jest.fn();
  const unmount = jest.fn();
  const filesystem = {
    analyzePath,
    rmdir,
    unmount,
  };

  WorkerInternalUtils.unmountFile(url, workingDir, filesystem);

  expect(analyzePath).toHaveBeenCalledTimes(3);
  expect(unmount).toBeCalledTimes(1);
  expect(unmount).toHaveBeenCalledWith(`${workingDir}/${uuid}`);
  expect(rmdir).toBeCalledTimes(1);
  expect(rmdir).toHaveBeenCalledWith(`${workingDir}/${uuid}`);
});

test('isRecognized() scans file', async () => {
  const isRecognized = jest.fn(() => true);
  const scanner = {
    isRecognized,
  };

  const result = WorkerInternalUtils.isFileRecognized(url, workingDir, scanner);

  expect(isRecognized).toHaveBeenCalledTimes(1);
  expect(isRecognized).toHaveBeenCalledWith(filePath);
  expect(result).toBe(true);
});

test('createConverter() uses scanner to retrieve converter', async () => {
  const mockConverter = { prop: 'dummy' };
  const getConverter = jest.fn(() => mockConverter);
  const scanner = {
    getConverter,
  };

  const converter = WorkerInternalUtils.createConverter(url, workingDir, scanner);

  expect(getConverter).toHaveBeenCalledTimes(1);
  expect(getConverter).toHaveBeenCalledWith(filePath);
  expect(converter).toBe(mockConverter);
});

test('readNode() uses converter to read node data', async () => {
  const rootUrl = new URL(`file:///${uuid}/${filename}/`);
  const nodeStub = {
    name: 'abc',
    data: [],
    parameters: [],
    childNodeNames: ['def'],
  };
  const readMock = jest.fn(() => nodeStub);
  const converterMock = { read: readMock };
  const converterMapStub = new Map([[rootUrl.toString(), converterMock]]);

  const node = WorkerInternalUtils.readNode(url, converterMapStub);

  expect(readMock).toHaveBeenCalledTimes(1);
  expect(readMock).toHaveBeenCalledWith('/');
  expect(node).toBe(nodeStub);
});

test('nodeToJson() maps C++ node data to WorkerNode', async () => {
  const parameters = {
    size: jest.fn(() => 2),
    get: jest.fn(() => ({ key: 'key value', value: 'value value' })),
    delete: jest.fn(),
  };
  const data = {
    size: jest.fn(() => 3),
    get: jest.fn(() => ({ x: 1, y: 2 })),
    delete: jest.fn(),
  };
  const childNodeNames = {
    size: jest.fn(() => 1),
    get: jest.fn(() => 'child node name'),
    delete: jest.fn(),
  };
  const nodeDataMock = {
    name: 'name value',
    parameters,
    data,
    childNodeNames,
  };

  const workerNode = WorkerInternalUtils.nodeToJson(url, nodeDataMock);

  expect(parameters.size).toHaveBeenCalledTimes(1);
  expect(parameters.get).toHaveBeenCalledTimes(2);
  expect(parameters.get).toHaveBeenCalledWith(0);
  expect(parameters.get).toHaveBeenCalledWith(1);
  expect(parameters.delete).toHaveBeenCalledTimes(1);

  expect(data.size).toHaveBeenCalledTimes(1);
  expect(data.get).toHaveBeenCalledTimes(3);
  expect(data.get).toHaveBeenCalledWith(0);
  expect(data.get).toHaveBeenCalledWith(1);
  expect(data.get).toHaveBeenCalledWith(2);
  expect(data.delete).toHaveBeenCalledTimes(1);

  expect(childNodeNames.size).toHaveBeenCalledTimes(1);
  expect(childNodeNames.get).toHaveBeenCalledTimes(1);
  expect(childNodeNames.get).toHaveBeenCalledWith(0);
  expect(childNodeNames.delete).toHaveBeenCalledTimes(1);

  expect(workerNode.url).toBe(url.toString());
  expect(workerNode.parameters).toHaveLength(2);
  expect(workerNode.parameters[0]).toEqual({ key: 'key value', value: 'value value' });
  expect(workerNode.data).toHaveLength(3);
  expect(workerNode.data[0]).toEqual({ x: 1, y: 2 });
  expect(workerNode.children).toHaveLength(1);
  expect(workerNode.children[0]).toEqual('child node name');
});

test('getExceptionMessage() reads exception message from Module', async () => {
  const cppExceptionMessage = 'C++ Exception Message';
  const wasmExceptionMessage = 'WASM Exception Message';
  // const jsExceptionMessage = 'JS Exception Message';
  const module = {
    getCppExceptionMessage: jest.fn(() => cppExceptionMessage),
    getExceptionMessage: jest.fn(() => ['type', wasmExceptionMessage]),
  };

  const cppResult = WorkerInternalUtils.getExceptionMessage(12345, module);
  // TODO: when WebAssembly.Exception gets added to TS, the commented out code should work
  // const wasmResult = WorkerInternalUtils.getExceptionMessage(
  //   {} as WebAssembly.Exception, module
  // );
  // const jsResult = WorkerInternalUtils.getExceptionMessage(
  //   { message: jsExceptionMessage }, module
  // );

  expect(cppResult).toBe(cppExceptionMessage);
  expect(module.getCppExceptionMessage).toHaveBeenCalledTimes(1);
  // expect(wasmResult).toBe(wasmExceptionMessage);
  // expect(module.getExceptionMessage).toHaveBeenCalledTimes(1);
  // expect(jsResult).toBe(jsExceptionMessage);
});

test('onMessageStatus() checks for the existence of ConverterService for returning the status', async () => {
  const initializingResponse = WorkerInternalUtils.onMessageStatus(null, '123');
  expect(initializingResponse.name).toBe('status');
  expect(initializingResponse.correlationId).toBe('123');
  expect(initializingResponse.detail).toBe(WorkerStatus.Initializing);

  const initializedResponse = WorkerInternalUtils.onMessageStatus({}, '123');
  expect(initializedResponse.name).toBe('status');
  expect(initializedResponse.correlationId).toBe('123');
  expect(initializedResponse.detail).toBe(WorkerStatus.Initialized);
});

test('onMessageStatus() checks for the existence of ConverterService for returning the status', async () => {
  const fileInfo: WorkerFileInfo = {
    url: url.toString(),
    blob: new Blob(),
  };
  const filesystemFileExistsMock = {
    analyzePath: jest.fn(() => ({ exists: true })),
    mkdir: jest.fn(),
    rmdir: jest.fn(),
    mount: jest.fn(),
    unmount: jest.fn(),
  };
  const workerFs = {};
  const converterService = {
    isRecognized: jest.fn(() => true),
  };
  const requestStub = new WorkerRequest('scan', '123', fileInfo);

  const scanResponse = WorkerInternalUtils.onMessageScan(
    requestStub,
    workingDir,
    converterService,
    filesystemFileExistsMock,
    workerFs,
  );
  expect(scanResponse.name).toBe('scanned');
  expect(scanResponse.correlationId).toBe(requestStub.correlationId);
  expect(scanResponse.detail).toHaveProperty('recognized');
  const responseDetail = scanResponse.detail as { recognized: true };
  expect(responseDetail.recognized).toBe(true);
});
