import { AndiChromScanner, JsReader } from 'sf_rs';
import WorkerFileInfo from './WorkerFileInfo';
import * as WorkerRsInternalUtils from './WorkerRsInternalUtils';
import WorkerRequest from './WorkerRequest';

const uuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ee';
const filename = 'test.cdf';
const url = new URL(`file:///${uuid}/${filename}#`);
// const rootUrl = new URL(`file:///${uuid}/${filename}`);
// const filePath = `${workingDir}/${uuid}/${filename}`;

const mockReader: JsReader = {
  read: jest.fn(),
  free: jest.fn(),
};

jest.mock('sf_rs', () => ({
  AndiChromScanner: jest.fn(() => ({
    js_is_recognized: jest.fn(() => true),
    js_get_reader: jest.fn(() => mockReader),
    free: jest.fn(),
  })),
  FileWrapper: jest.fn(() => ({ free: jest.fn() })),
}));

const fileInfoStub: WorkerFileInfo = {
  url: url.toString(),
  blob: new Blob(),
};

afterEach(() => {
  jest.clearAllMocks();
});

test('onScan() uses Scanner to scan if a file could be parsed', async () => {
  const requestStub = new WorkerRequest('scan', '123', fileInfoStub);
  const mockScanner = new AndiChromScanner();

  const scanResponse = WorkerRsInternalUtils.onScan(
    requestStub,
    mockScanner,
  );

  expect(mockScanner.js_is_recognized).toHaveBeenCalledTimes(1);
  expect(mockScanner.js_get_reader).toHaveBeenCalledTimes(0);
  expect(mockScanner.free).toHaveBeenCalledTimes(0);

  expect(scanResponse.name).toBe('scanned');
  expect(scanResponse.correlationId).toBe(requestStub.correlationId);
  expect(scanResponse.detail).toHaveProperty('recognized');
  const responseDetail = scanResponse.detail as { recognized: true };
  expect(responseDetail.recognized).toBe(true);
});
