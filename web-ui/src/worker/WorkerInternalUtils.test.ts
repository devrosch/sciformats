import * as WorkerInternalUtils from './WorkerInternalUtils';

const uuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ee';
const filename = 'test.jdx';
const url = new URL(`file:///${uuid}/${filename}/#`);
const workingDir = '/work';
const filePath = `${workingDir}/${uuid}/${filename}`;

test('initConverterService() waits for Module init and initializes ConverterService', async () => {
  const pushBackMock = jest.fn();
  const workerNamespaceMock = {
    Module: {
      Scanner: jest.fn(),
      JdxScanner: jest.fn(),
      vector$std$$shared_ptr$sciformats$$api$$Scanner$$: jest.fn(
        () => ({ push_back: pushBackMock }),
      ),
      ConverterService: jest.fn(),
    },
  };

  await WorkerInternalUtils.initConverterService(workerNamespaceMock);

  expect(workerNamespaceMock.Module.JdxScanner).toHaveBeenCalledTimes(1);
  expect(workerNamespaceMock.Module.vector$std$$shared_ptr$sciformats$$api$$Scanner$$)
    .toHaveBeenCalledTimes(1);
  expect(pushBackMock).toHaveBeenCalledTimes(1);
  expect(workerNamespaceMock.Module.ConverterService).toHaveBeenCalledTimes(1);
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
