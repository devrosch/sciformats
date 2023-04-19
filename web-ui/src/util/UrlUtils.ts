/* disable rule for now until more helpers are added */
/* eslint-disable import/prefer-default-export */

/**
 * Test if two URLs are the same.
 * @param url0 A URL.
 * @param url1 Another URL.
 * @returns True if the URLs are valid and their normalized string representations match.
 * False if either or both URLs are null, undefined or invalid
 * or their normalized string representations do not match.
 */
/* eslint-disable-next-line function-paren-newline */
export const isSameUrl = (
  url0: URL | string | null | undefined, url1: URL | string | null | undefined) => {
  if (typeof url0 === 'undefined'
    || typeof url1 === 'undefined'
    || url0 === null
    || url1 === null) {
    return false;
  }

  try {
    const urlLhs = typeof url0 === 'string' ? new URL(url0).toString() : url0!.toString();
    const urlRhs = typeof url1 === 'string' ? new URL(url1).toString() : url1!.toString();
    return urlLhs === urlRhs;
  } catch (error) {
    return false;
  }
};

export const extractGroup = (url: URL | string, groupIndex: number, errorMessage: string) => {
  const urlString = url instanceof URL ? url.toString() : url;
  const regex = /^(file:\/\/\/|https?:\/\/)([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-xfA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})\/([^/]*)/g;
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
export const extractUuid = (url: URL | string) => extractGroup(url, 2, 'Cannot extract UUID from URL. URL does not match patterns "file:///<uuid>/<filename>/#..." or "http(s)://<uuid>/<filename>/#..."');

/**
 * Extract a filename from a URL matching the patterns "file:///<uuid>/<filename>/#..." or "http(s)://<uuid>/<filename>/#...".
 * @param url A URL.
 * @returns The <filename> from the URL.
 * @example For "file:///aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee/test.jdx/#" this function returns "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee".
 */
export const extractFilename = (url: URL | string) => extractGroup(url, 3, 'Cannot extract filename from URL. URL does not match patterns "file:///<uuid>/<filename>/#..." or "http(s)://<uuid>/<filename>/#..."');
