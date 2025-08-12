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

/**
 * Test if two URLs are the same.
 * @param url0 A URL.
 * @param url1 Another URL.
 * @returns True if the URLs are valid and their normalized string representations match.
 * False if either or both URLs are null, undefined or invalid
 * or their normalized string representations do not match.
 */
export const isSameUrl = (
  url0: URL | string | null | undefined,
  url1: URL | string | null | undefined,
) => {
  if (
    typeof url0 === 'undefined' ||
    typeof url1 === 'undefined' ||
    url0 === null ||
    url1 === null
  ) {
    return false;
  }

  try {
    const urlLhs =
      typeof url0 === 'string' ? new URL(url0).toString() : url0!.toString();
    const urlRhs =
      typeof url1 === 'string' ? new URL(url1).toString() : url1!.toString();
    return urlLhs === urlRhs;
    /* eslint-disable @typescript-eslint/no-unused-vars */
  } catch (error) {
    return false;
  }
};

export const extractGroup = (
  url: URL | string,
  groupIndex: number,
  errorMessage: string,
) => {
  const urlString = url instanceof URL ? url.toString() : url;
  const regex =
    /^(file:\/\/\/|https?:\/\/)([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-xfA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})\/([^#/]*)/g;
  const matches = urlString.matchAll(regex);
  const uuid = Array.from(matches, (match) => match[groupIndex]);
  if (uuid !== null && uuid.length === 1) {
    return uuid[0];
  }
  throw new Error(errorMessage);
};

/**
 * Extract a UUID from a URL matching the patterns "file:///<uuid>/<filename>/#..." or "http(s)://<uuid>/<filename>/#...".
 * @param url A URL.
 * @returns The <uuid> from the URL.
 * @example For "file:///aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee/test.jdx/#" this function returns "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee".
 */
export const extractUuid = (url: URL | string) =>
  extractGroup(
    url,
    2,
    'Cannot extract UUID from URL. URL does not match patterns "file:///<uuid>/<filename>/#..." or "http(s)://<uuid>/<filename>/#..."',
  );

/**
 * Extract a filename from a URL matching the patterns "file:///<uuid>/<filename>/#..." or "http(s)://<uuid>/<filename>/#...".
 * @param url A URL.
 * @returns The <filename> from the URL.
 * @example For "file:///aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee/test.jdx/#" this function returns "test.jdx".
 */
export const extractFilename = (url: URL | string) =>
  extractGroup(
    url,
    3,
    'Cannot extract filename from URL. URL does not match patterns "file:///<uuid>/<filename>/#..." or "http(s)://<uuid>/<filename>/#..."',
  );

/**
 * Extracts the hash part of a URL as path string.
 * @param url A URL that may have a hash.
 * @returns A path extracted from the hash.
 * @example For "file:///some/url#/hash/path" this function returns "/hash/path".
 * @example For "file:///some/url#/hash/path/" this function returns "/hash/path/".
 * @example For "file:///some/url#/" this function returns "/".
 * @example For "file:///some/url#" this function returns "/".
 * @example For "file:///some/url" this function returns "/".
 * @throws if the passed value is not a URL or if the hash exists but does not statrt with '/'.
 */
export const extractHashPath = (url: URL | string) => {
  const urlObject = url instanceof URL ? url : new URL(url);

  // for an empty hash .../# hash is '', not '#'
  let hash = urlObject.hash;
  if (hash.length > 0 && !hash.startsWith('#/')) {
    throw new Error(`Unexpected URL hash: ${hash}`);
  }

  // '', '#', '#/' all denote the root node
  if (hash.startsWith('#')) {
    hash = hash.substring(1);
  }
  if (hash.length === 0) {
    hash = '/';
  }

  return hash;
};
