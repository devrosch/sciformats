/**
 * Copyright (c) 2025 Robert Schiwon
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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
