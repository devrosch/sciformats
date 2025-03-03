import ErrorParser from './ErrorParser';

const uuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ee';
const filename = 'test.jdx';
const rootUrl = new URL(`file:///${uuid}/${filename}`);
const errorMessage = 'error!';

test('instatiating succeeds', async () => {
  const parser = new ErrorParser(rootUrl, errorMessage);
  expect(parser.rootUrl).toBe(rootUrl);
});

test('opening throws with passed error message', async () => {
  const parser = new ErrorParser(rootUrl, errorMessage);

  // see: https://stackoverflow.com/a/47887098 for how to test throw of async function
  await expect(parser.open()).rejects.toThrow(errorMessage);
});

test('exporting throws with passed error message', async () => {
  const parser = new ErrorParser(rootUrl, errorMessage);

  // see: https://stackoverflow.com/a/47887098 for how to test throw of async function
  await expect(parser.export()).rejects.toThrow(errorMessage);
});

test('reading throws with passed error message', async () => {
  const parser = new ErrorParser(rootUrl, errorMessage);

  // see: https://stackoverflow.com/a/47887098 for how to test throw of async function
  await expect(parser.read()).rejects.toThrow(errorMessage);
});

test('closing does not throw', async () => {
  const parser = new ErrorParser(rootUrl, errorMessage);

  // see: https://stackoverflow.com/a/47887098 for how to test throw of async function
  await expect(parser.close()).resolves.not.toThrow();
});
