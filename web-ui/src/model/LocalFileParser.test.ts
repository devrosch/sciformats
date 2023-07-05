import WorkerFileInfo from 'worker/WorkerFileInfo';
import WorkerResponse from 'worker/WorkerResponse';
import LocalFileParser from './LocalFileParser';

const uuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ee';
const wrongUuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ff';
const filename = 'test.jdx';
const url = new URL(`file:///${uuid}/${filename}#/`);
const mockErrorUrl = `file:///${uuid}/error.txt#/`;
const rootUrl = new URL(`file:///${uuid}/${filename}`);
const file = new File(['dummy'], 'test.txt');
const worker = {} as Worker;
const workerNodeData = {
  url: url.toString(),
  data: [{ x: 1, y: 2 }, { x: 3, y: 4 }],
  parameters: [{ key: 'param 1', value: 'param value 1' }],
  children: ['child 1', 'child 2'],
};

const mockOpenedResponse = new WorkerResponse('opened', '123', { url: rootUrl.toString() });
const mockClosedResponse = new WorkerResponse('closed', '123', { url: rootUrl.toString() });
const mockReadResponse = new WorkerResponse('read', '123', workerNodeData);
const mockErrorResponse = new WorkerResponse('error', '123', 'error message');

jest.mock('util/WorkerUtils', () => ({
  postMessage: jest.fn((workerParam: Worker, name: string, payload: any) => {
    if ((payload as WorkerFileInfo).url === mockErrorUrl) {
      return mockErrorResponse;
    }
    let response: WorkerResponse | null = null;
    switch (name) {
      case 'open':
        response = mockOpenedResponse;
        break;
      case 'close':
        response = mockClosedResponse;
        break;
      case 'read':
        response = mockReadResponse;
        break;
      default:
        break;
    }
    return response;
  }),
}));

test('instatiating LocalFileParser succeeds', async () => {
  const parser = new LocalFileParser(worker, rootUrl, file);

  expect(parser.rootUrl).toBe(rootUrl);
});

test('opening a local file succeeds', async () => {
  const parser = new LocalFileParser(worker, rootUrl, file);

  await parser.open();
});

test('throw when error occurs while opening a local file', async () => {
  const parser = new LocalFileParser(worker, new URL(mockErrorUrl), file);

  // see: https://stackoverflow.com/a/47887098 for how to test throw of async function
  await expect(parser.open()).rejects.toThrowError(/open file/);
});

test('closing a local file succeeds', async () => {
  const parser = new LocalFileParser(worker, rootUrl, file);

  await parser.close();
});

test('throw when error occurs while closing a local file', async () => {
  const parser = new LocalFileParser(worker, new URL(mockErrorUrl), file);

  // see: https://stackoverflow.com/a/47887098 for how to test throw of async function
  await expect(parser.close()).rejects.toThrowError(/close file/);
});

test('reading a local file succeeds', async () => {
  const parser = new LocalFileParser(worker, rootUrl, file);
  const node = await parser.read(url);

  expect(node.url).toEqual(new URL(workerNodeData.url));
  expect(node.data).toBe(workerNodeData.data);
  expect(node.parameters).toBe(workerNodeData.parameters);
  expect(node.children).toBe(workerNodeData.children);
});

test('reading an illegal URL throws', async () => {
  const parser = new LocalFileParser(worker, rootUrl, file);

  // see: https://stackoverflow.com/a/47887098 for how to test throw of async function
  await expect(parser.read(new URL(`file:///${wrongUuid}/error.txt#/`))).rejects.toThrowError(/URL/);
});
