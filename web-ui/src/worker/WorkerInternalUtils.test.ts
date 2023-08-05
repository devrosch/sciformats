import WorkerFileInfo from './WorkerFileInfo';
import WorkerFileUrl from './WorkerFileUrl';
import * as WorkerInternalUtils from './WorkerInternalUtils';
import WorkerNodeData from './WorkerNodeData';
import WorkerRequest from './WorkerRequest';
import WorkerStatus from './WorkerStatus';

const uuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ee';
const filename = 'test.jdx';
const url = new URL(`file:///${uuid}/${filename}#`);
const rootUrl = new URL(`file:///${uuid}/${filename}`);
const workingDir = '/work';
const filePath = `${workingDir}/${uuid}/${filename}`;

const fileInfoStub: WorkerFileInfo = {
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
const table = {
  columnNames: {
    size: jest.fn(() => 2),
    get: jest.fn(() => ({
      first: 'col0',
      second: 'Column 0 Name',
      // value object, no delete()
    })),
    delete: jest.fn(),
  },
  rows: {
    size: jest.fn(() => 2),
    get: jest.fn(() => ({
      get: jest.fn((columnKey) => `${columnKey} Value`),
      delete: jest.fn(),
    })),
    delete: jest.fn(),
  },
  delete: jest.fn(),
};
const childNodeNames = {
  size: jest.fn(() => 1),
  get: jest.fn(() => 'child node name'),
  delete: jest.fn(),
};
const metadata = {
  size: jest.fn(() => 1),
  keys: jest.fn(() => ({
    size: jest.fn(() => 1),
    get: jest.fn(() => 'x.unit'),
    delete: jest.fn(),
  })),
  get: jest.fn(() => '1/cm'),
  delete: jest.fn(),
};
const nodeDataMock = {
  name: 'name value',
  parameters,
  data,
  metadata,
  table,
  childNodeNames,
};
const converterMock = {
  read: jest.fn(() => nodeDataMock),
  delete: jest.fn(),
};
const converterServiceMock = {
  isRecognized: jest.fn(() => true),
  getConverter: jest.fn(() => converterMock),
};
const workerFsStub = {};
const module = {};

const checkWorkerNodeData = (workerNode: WorkerNodeData) => {
  expect(workerNode.url).toBe(url.toString());
  expect(workerNode.parameters).toHaveLength(2);
  expect(workerNode.parameters[0]).toEqual({ key: 'key value', value: 'value value' });
  expect(workerNode.data).toHaveLength(3);
  expect(workerNode.data[0]).toEqual({ x: 1, y: 2 });
  expect(workerNode.childNodeNames).toHaveLength(1);
  expect(workerNode.childNodeNames[0]).toEqual('child node name');
};

afterEach(() => {
  jest.clearAllMocks();
});

test('initConverterService() waits for Module init and initializes ConverterService', async () => {
  const pushBackMock = jest.fn();
  const workerSelfMock = {
    Module: {
      Scanner: jest.fn(),
      JdxScanner: jest.fn(() => ({ delete: jest.fn() })),
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

test('initConverterService() cleans up resources if init fails', async () => {
  const pushBackMock = jest.fn(() => { throw new Error(); });
  const jdxScannerDeleteMock = jest.fn();
  const vectorDeleteMock = jest.fn();
  const workerSelfMock = {
    Module: {
      Scanner: jest.fn(),
      JdxScanner: jest.fn(() => ({ delete: jdxScannerDeleteMock })),
      vector$std$$shared_ptr$sciformats$$api$$Scanner$$: jest.fn(
        () => ({ push_back: pushBackMock, delete: vectorDeleteMock }),
      ),
      ConverterService: jest.fn(),
    },
  };

  await expect(async () => { await WorkerInternalUtils.initConverterService(workerSelfMock); })
    .rejects.toThrowError();

  expect(jdxScannerDeleteMock).toHaveBeenCalledTimes(1);
  expect(vectorDeleteMock).toHaveBeenCalledTimes(1);
});

test('mountFile() creates workingDir and UUID directories and mounts WORKERFS', async () => {
  const blob = new Blob();
  const uuidPath = `${workingDir}/${uuid}`;
  const filesystemMock = {
    analyzePath: jest.fn(() => ({ exists: false })),
    mkdir: jest.fn(),
    mount: jest.fn(),
  };

  WorkerInternalUtils.mountFile(url, blob, workingDir, filesystemMock, workerFsStub);

  expect(filesystemMock.analyzePath).toHaveBeenCalledTimes(2);
  expect(filesystemMock.analyzePath).toHaveBeenCalledWith(workingDir, false);
  expect(filesystemMock.analyzePath).toHaveBeenCalledWith(uuidPath, false);
  expect(filesystemMock.mkdir).toHaveBeenCalledTimes(2);
  expect(filesystemMock.mkdir).toHaveBeenCalledWith(workingDir);
  expect(filesystemMock.mkdir).toHaveBeenCalledWith(uuidPath);
  expect(filesystemMock.mount).toHaveBeenCalledTimes(1);
  expect(filesystemMock.mount).toHaveBeenCalledWith(
    workerFsStub,
    { blobs: [{ name: filename, data: blob }] },
    uuidPath,
  );
});

test('unmountFile() unmounts WORKERFS and deletes UUID directory', async () => {
  WorkerInternalUtils.unmountFile(url, workingDir, filesystemFileExistsMock);

  expect(filesystemFileExistsMock.analyzePath).toHaveBeenCalledTimes(3);
  expect(filesystemFileExistsMock.unmount).toBeCalledTimes(1);
  expect(filesystemFileExistsMock.unmount).toHaveBeenCalledWith(`${workingDir}/${uuid}`);
  expect(filesystemFileExistsMock.rmdir).toBeCalledTimes(1);
  expect(filesystemFileExistsMock.rmdir).toHaveBeenCalledWith(`${workingDir}/${uuid}`);

  const filesystemNoDirMock = {
    analyzePath: jest.fn(() => ({ exists: false })),
    rmdir: jest.fn(),
    unmount: jest.fn(),
  };

  WorkerInternalUtils.unmountFile(url, workingDir, filesystemNoDirMock);

  expect(filesystemNoDirMock.analyzePath).toHaveBeenCalledTimes(1);
  expect(filesystemNoDirMock.unmount).toBeCalledTimes(0);
  expect(filesystemNoDirMock.rmdir).toBeCalledTimes(0);

  const filesystemNoUuidDirMock = {
    analyzePath: jest
      .fn()
      .mockReturnValueOnce({ exists: true })
      .mockReturnValue({ exists: false }),
    rmdir: jest.fn(),
    unmount: jest.fn(),
  };

  WorkerInternalUtils.unmountFile(url, workingDir, filesystemNoUuidDirMock);

  expect(filesystemNoUuidDirMock.analyzePath).toHaveBeenCalledTimes(2);
  expect(filesystemNoUuidDirMock.unmount).toBeCalledTimes(0);
  expect(filesystemNoUuidDirMock.rmdir).toBeCalledTimes(0);
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
  const scanner = {
    getConverter: jest.fn(() => mockConverter),
  };

  const converter = WorkerInternalUtils.createConverter(url, workingDir, scanner);

  expect(scanner.getConverter).toHaveBeenCalledTimes(1);
  expect(scanner.getConverter).toHaveBeenCalledWith(filePath);
  expect(converter).toBe(mockConverter);
});

test('readNode() uses converter to read node data', async () => {
  const converterMapStub = new Map([[rootUrl.toString(), converterMock]]);

  const node = WorkerInternalUtils.readNode(url, converterMapStub);

  expect(converterMock.read).toHaveBeenCalledTimes(1);
  expect(converterMock.read).toHaveBeenCalledWith('/');
  expect(node).toBe(nodeDataMock);
});

test('readNode() throws if there is no converter for the URL', async () => {
  const converterMapStub = new Map();

  expect(() => WorkerInternalUtils.readNode(url, converterMapStub)).toThrow();
});

test('readNode() throws for illegal URL hash', async () => {
  const converterMapStub = new Map([[rootUrl.toString(), converterMock]]);
  const urlIllegalHash = new URL(`${rootUrl.toString()}#illegal`);

  expect(() => WorkerInternalUtils.readNode(urlIllegalHash, converterMapStub)).toThrow();
});

test('nodeToJson() maps C++ node data to WorkerNode', async () => {
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

  checkWorkerNodeData(workerNode);
});

test('getExceptionMessage() reads exception message from Module', async () => {
  const cppExceptionMessage = 'C++ Exception Message';
  const wasmExceptionMessage = 'WASM Exception Message';
  const jsExceptionMessage = 'JS Exception Message';
  const moduleMock = {
    getCppExceptionMessage: jest.fn(() => cppExceptionMessage),
    getExceptionMessage: jest.fn(() => ['type', wasmExceptionMessage]),
  };

  const cppResult = WorkerInternalUtils.getExceptionMessage(12345, moduleMock);
  // TODO: when WebAssembly.Exception gets added to TS, the commented out code should work
  // const wasmResult = WorkerInternalUtils.getExceptionMessage(
  //   {} as WebAssembly.Exception, module
  // );
  const jsResult = WorkerInternalUtils.getExceptionMessage(
    { message: jsExceptionMessage },
    moduleMock,
  );

  expect(cppResult).toBe(cppExceptionMessage);
  expect(moduleMock.getCppExceptionMessage).toHaveBeenCalledTimes(1);
  // expect(wasmResult).toBe(wasmExceptionMessage);
  // expect(module.getExceptionMessage).toHaveBeenCalledTimes(1);
  expect(jsResult).toBe(jsExceptionMessage);
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

test('onMessageScan() uses ConverterService to scan if a file could be parsed', async () => {
  const requestStub = new WorkerRequest('scan', '123', fileInfoStub);

  const scanResponse = WorkerInternalUtils.onMessageScan(
    requestStub,
    workingDir,
    converterServiceMock,
    filesystemFileExistsMock,
    workerFsStub,
  );

  expect(scanResponse.name).toBe('scanned');
  expect(scanResponse.correlationId).toBe(requestStub.correlationId);
  expect(scanResponse.detail).toHaveProperty('recognized');
  const responseDetail = scanResponse.detail as { recognized: true };
  expect(responseDetail.recognized).toBe(true);
});

test('onMessageOpen() uses existing converter to parse data', async () => {
  const requestStub = new WorkerRequest('open', '123', fileInfoStub);
  /* @ts-expect-error */
  const openFiles = new Map<string, Module.Converter>([[rootUrl.toString(), converterMock]]);

  const openResponse = WorkerInternalUtils.onMessageOpen(
    requestStub,
    workingDir,
    openFiles,
    converterServiceMock,
    filesystemFileExistsMock,
    workerFsStub,
    module,
  );

  expect(openResponse.name).toBe('opened');
  expect(openResponse.correlationId).toBe(requestStub.correlationId);
  expect(openResponse.detail).toHaveProperty('url');
  const responseDetail = openResponse.detail as WorkerFileUrl;
  expect(responseDetail.url).toBe(rootUrl.toString());
});

test('onMessageOpen() creates new converter to parse data', async () => {
  const requestStub = new WorkerRequest('open', '123', fileInfoStub);
  /* @ts-expect-error */
  const openFiles = new Map<string, Module.Converter>();

  const openResponse = WorkerInternalUtils.onMessageOpen(
    requestStub,
    workingDir,
    openFiles,
    converterServiceMock,
    filesystemFileExistsMock,
    workerFsStub,
    module,
  );

  expect(openResponse.name).toBe('opened');
  expect(openResponse.correlationId).toBe(requestStub.correlationId);
  expect(openResponse.detail).toHaveProperty('url');
  const responseDetail = openResponse.detail as { url: string };
  expect(responseDetail.url).toBe(rootUrl.toString());
  expect(openFiles.size).toBe(1);
});

test('onMessageOpen() returns error response in case of error', async () => {
  const requestStub = new WorkerRequest('open', '123', fileInfoStub);
  /* @ts-expect-error */
  const openFiles = new Map<string, Module.Converter>();
  const errorConverterServiceMock = {
    isRecognized: jest.fn(() => true),
    getConverter: jest.fn(() => { throw new Error('open error message'); }),
  };

  const openResponse = WorkerInternalUtils.onMessageOpen(
    requestStub,
    workingDir,
    openFiles,
    errorConverterServiceMock,
    filesystemFileExistsMock,
    workerFsStub,
    module,
  );

  expect(openResponse.name).toBe('error');
  expect(openResponse.correlationId).toBe(requestStub.correlationId);
  expect(openResponse.detail).toBe('open error message');
});

test('onMessageRead() reads node data', async () => {
  const requestStub = new WorkerRequest('read', '123', fileInfoStub);
  /* @ts-expect-error */
  const openFiles = new Map<string, Module.Converter>([[rootUrl.toString(), converterMock]]);

  const readResponse = WorkerInternalUtils.onMessageRead(
    requestStub,
    openFiles,
    module,
  );

  expect(readResponse.name).toBe('read');
  expect(readResponse.correlationId).toBe(requestStub.correlationId);
  const workerNode = readResponse.detail as WorkerNodeData;
  checkWorkerNodeData(workerNode);
});

test('onMessageRead() returns error response in case of error', async () => {
  const requestStub = new WorkerRequest('read', '123', fileInfoStub);
  const errorConverterMock = {
    read: jest.fn(() => { throw new Error('read error message'); }),
  };
  /* @ts-expect-error */
  const openFiles = new Map<string, Module.Converter>([[rootUrl.toString(), errorConverterMock]]);

  const readResponse = WorkerInternalUtils.onMessageRead(
    requestStub,
    openFiles,
    module,
  );

  expect(readResponse.name).toBe('error');
  expect(readResponse.correlationId).toBe(requestStub.correlationId);
  expect(readResponse.detail).toBe('read error message');
});

test('onMessageClose() removes converter from open files map and deletes file in filesystem', async () => {
  const requestStub = new WorkerRequest('close', '123', fileInfoStub);
  /* @ts-expect-error */
  const openFiles = new Map<string, Module.Converter>([[rootUrl.toString(), converterMock]]);

  const closeResponse = WorkerInternalUtils.onMessageClose(
    requestStub,
    workingDir,
    openFiles,
    filesystemFileExistsMock,
  );

  expect(closeResponse.name).toBe('closed');
  expect(closeResponse.correlationId).toBe(requestStub.correlationId);
  expect(closeResponse.detail).toHaveProperty('url');
  const responseDetail = closeResponse.detail as WorkerFileUrl;
  expect(responseDetail.url).toBe(rootUrl.toString());
});
