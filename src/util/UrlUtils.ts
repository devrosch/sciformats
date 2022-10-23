/* disable rule for now until more helpers are added */
/* eslint-disable import/prefer-default-export */

/**
 * Test if two URLs are the same.
 * @param url0 A URL.
 * @param url1 Another URL.
 * @returns True if the URLs their normalized string representations match.
 * False if either or both URLs are null or undefined
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

  const urlLhs = typeof url0 === 'string' ? new URL(url0).toString() : url0!.toString();
  const urlRhs = typeof url1 === 'string' ? new URL(url1).toString() : url1!.toString();

  return urlLhs === urlRhs;
};
