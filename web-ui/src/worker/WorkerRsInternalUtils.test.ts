import { AndiScanner, Reader } from 'sf_rs';
import WorkerFileInfo from './WorkerFileInfo';
import * as WorkerRsInternalUtils from './WorkerRsInternalUtils';
import WorkerRequest from './WorkerRequest';
import WorkerFileUrl from './WorkerFileUrl';
import WorkerNodeData from './WorkerNodeData';

const uuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ee';
const filename = 'test.cdf';
const rootUrl = new URL(`file:///${uuid}/${filename}`);
const nodeName = 'x';
const url = new URL(`file:///${uuid}/${filename}#/${nodeName}`);

const mockReader: Reader = {
  read: jest.fn(() => ({
    name: nodeName,
    parameters: [],
    data: [],
    metadata: {},
    table: {},
    childNodeNames: [],
    free: jest.fn(),
  })),
  getExportFormats: jest.fn(() => []),
  export: jest.fn(() => {
    throw new Error('Unsupported');
  }),
  free: jest.fn(),
};

jest.mock('sf_rs', () => ({
  AndiScanner: jest.fn(() => ({
    isRecognized: jest.fn(() => true),
    getReader: jest.fn(() => mockReader),
    free: jest.fn(),
  })),
}));

const fileInfoStub: WorkerFileInfo = {
  url: rootUrl.toString(),
  blob: new Blob(),
};

const workerFileUrlStub: WorkerFileUrl = {
  url: url.toString(),
};

afterEach(() => {
  jest.clearAllMocks();
});

test('onScan() uses Scanner to scan if a file could be parsed', async () => {
  const requestStub = new WorkerRequest('scan', '123', fileInfoStub);
  const mockScanner = new AndiScanner();

  const response = WorkerRsInternalUtils.onScan(requestStub, mockScanner);

  expect(mockScanner.isRecognized).toHaveBeenCalledTimes(1);
  expect(mockScanner.getReader).toHaveBeenCalledTimes(0);
  expect(mockScanner.free).toHaveBeenCalledTimes(0);

  expect(response.name).toBe('scanned');
  expect(response.correlationId).toBe(requestStub.correlationId);
  expect(response.detail).toHaveProperty('recognized');
  const responseDetail = response.detail as { recognized: boolean };
  expect(responseDetail.recognized).toBe(true);
});

test('onScan() returns error for illegal input', async () => {
  const illegalFileInfoStub: WorkerFileInfo = {
    url: 'notavalidurl',
    blob: new Blob(),
  };
  const requestStub = new WorkerRequest('scan', '123', illegalFileInfoStub);
  const mockScanner = new AndiScanner();

  const response = WorkerRsInternalUtils.onScan(requestStub, mockScanner);

  expect(mockScanner.isRecognized).toHaveBeenCalledTimes(0);
  expect(response.name).toBe('error');
});

test('onOpen() uses Scanner to populate openFiles map', async () => {
  const requestStub = new WorkerRequest('open', '123', fileInfoStub);
  const mockScanner = new AndiScanner();
  const openFiles = new Map<string, Reader>();

  const response = WorkerRsInternalUtils.onOpen(
    requestStub,
    mockScanner,
    openFiles,
  );

  expect(mockScanner.isRecognized).toHaveBeenCalledTimes(0);
  expect(mockScanner.getReader).toHaveBeenCalledTimes(1);
  expect(mockScanner.free).toHaveBeenCalledTimes(0);

  expect(response.name).toBe('opened');
  expect(response.correlationId).toBe(requestStub.correlationId);
  expect(response.detail).toHaveProperty('url');
  const responseDetail = response.detail as WorkerFileUrl;
  expect(responseDetail.url).toEqual(rootUrl.toString());
  expect(openFiles.size).toBe(1);
  expect(openFiles.keys().next().value).toEqual(rootUrl.toString());
});

test('onOpen() returns error if exception occurs', async () => {
  const requestStub = new WorkerRequest('open', '123', fileInfoStub);
  const mockScanner = {
    isRecognized: jest.fn(() => true),
    getReader: jest.fn(() => {
      throw new Error('getReader() error');
    }),
    free: jest.fn(),
  };
  const openFiles = new Map<string, Reader>();

  const response = WorkerRsInternalUtils.onOpen(
    requestStub,
    mockScanner,
    openFiles,
  );

  expect(mockScanner.getReader).toHaveBeenCalledTimes(1);
  expect(response.name).toBe('error');
});

test('onRead() uses openFiles to read node', async () => {
  const requestStub = new WorkerRequest('read', '123', workerFileUrlStub);
  const openFiles = new Map<string, Reader>();
  openFiles.set(rootUrl.toString(), mockReader);

  const response = WorkerRsInternalUtils.onRead(requestStub, openFiles);

  expect(response.name).toBe('read');
  expect(response.correlationId).toBe(requestStub.correlationId);
  const responseDetail = response.detail as WorkerNodeData;
  expect(responseDetail.url).toEqual(url.toString());
  expect(mockReader.read).toHaveBeenCalledTimes(1);
});

test('onRead() returns error if file not open', async () => {
  const requestStub = new WorkerRequest('read', '123', workerFileUrlStub);
  const openFiles = new Map<string, Reader>();

  const response = WorkerRsInternalUtils.onRead(requestStub, openFiles);

  expect(response.name).toBe('error');
  expect(response.correlationId).toBe(requestStub.correlationId);
});

test('onClose() removes file from openFiles map', async () => {
  const requestStub = new WorkerRequest('close', '123', workerFileUrlStub);
  const openFiles = new Map<string, Reader>();
  openFiles.set(rootUrl.toString(), mockReader);

  const response = WorkerRsInternalUtils.onClose(requestStub, openFiles);

  expect(mockReader.read).toHaveBeenCalledTimes(0);
  expect(mockReader.free).toHaveBeenCalledTimes(1);

  expect(response.name).toBe('closed');
  expect(response.correlationId).toBe(requestStub.correlationId);
  expect(response.detail).toHaveProperty('url');
  const responseDetail = response.detail as WorkerFileUrl;
  expect(responseDetail.url).toEqual(rootUrl.toString());
  expect(openFiles.size).toBe(0);
});

test('onClose() also succeeds if file not open', async () => {
  const requestStub = new WorkerRequest('close', '123', workerFileUrlStub);
  const openFiles = new Map<string, Reader>();

  const response = WorkerRsInternalUtils.onClose(requestStub, openFiles);

  expect(response.name).toBe('closed');
});
