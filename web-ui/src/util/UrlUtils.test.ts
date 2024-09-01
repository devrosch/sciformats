import {
  extractFilename,
  extractHashPath,
  extractUuid,
  isSameUrl,
} from 'util/UrlUtils';

test('isSameURL() is false if any parameter is null or undefined', async () => {
  const url = new URL('https://valid.url1');

  expect(isSameUrl(url, null)).toBe(false);
  expect(isSameUrl(null, url)).toBe(false);
  expect(isSameUrl(url, undefined)).toBe(false);
  expect(isSameUrl(undefined, url)).toBe(false);
  expect(isSameUrl(null, null)).toBe(false);
  expect(isSameUrl(null, undefined)).toBe(false);
  expect(isSameUrl(undefined, null)).toBe(false);
});

test('isSameURL() is true for matching URLs', async () => {
  const url0 = new URL('https://valid.url#frag ment');
  const url1 = new URL('https://valid.url#frag ment');
  const url2 = new URL('https://valid.url#frag%20ment');
  const url1String = 'https://valid.url#frag ment';
  const url2String = 'https://valid.url#frag%20ment';

  expect(isSameUrl(url0, url1)).toBe(true);
  expect(isSameUrl(url0, url2)).toBe(true);
  expect(isSameUrl(url0, url1String)).toBe(true);
  expect(isSameUrl(url0, url2String)).toBe(true);
  expect(isSameUrl(url1String, url0)).toBe(true);
  expect(isSameUrl(url2String, url0)).toBe(true);
  expect(isSameUrl(url1String, url2String)).toBe(true);
  expect(isSameUrl(url2String, url1String)).toBe(true);
});

test('isSameURL() is false for mismatching URLs', async () => {
  const url00 = new URL('https://valid.url0#fragment0');
  const url10 = new URL('https://valid.url1#fragment0');
  const url01 = new URL('https://valid.url0#fragment1');
  expect(isSameUrl(url00, url10)).toBe(false);
  expect(isSameUrl(url00, url01)).toBe(false);
});

test('isSameURL() is false when comparing illegal URLs', async () => {
  const url00 = new URL('https://valid.url0#fragment0');
  const urlFragment = '#fragment0';
  const urlBlank = '';
  expect(isSameUrl(url00, urlFragment)).toBe(false);
  expect(isSameUrl(url00, urlBlank)).toBe(false);
});

test('extractUuid() extracts UUID for well-formed file URL with slah before hash', async () => {
  const url = new URL(
    'file:///aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#',
  );
  const uuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ee';
  expect(extractUuid(url)).toBe(uuid);
});

test('extractUuid() extracts UUID for well-formed file URL without slah before hash', async () => {
  const url = new URL('file:///aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx#');
  const uuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ee';
  expect(extractUuid(url)).toBe(uuid);
});

test('extractUuid() throws for malformed file URL', async () => {
  // invalid uuid
  const url = new URL('file:///aaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#');
  expect(() => extractUuid(url)).toThrow(/Cannot extract UUID/i);
});

test('extractUuid() extracts UUID for well-formed http URL', async () => {
  const url = new URL('http://aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#');
  const uuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ee';
  expect(extractUuid(url)).toBe(uuid);
});

test('extractUuid() extracts UUID for well-formed https URL', async () => {
  const url = new URL(
    'https://aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#',
  );
  const uuid = 'aaaaaaaa-bbbb-cccc-dddd-1234567890ee';
  expect(extractUuid(url)).toBe(uuid);
});

test('extractFilename() extracts filename for well-formed file URL with slah before hash', async () => {
  const url = new URL(
    'file:///aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#',
  );
  const filename = 'test.jdx';
  expect(extractFilename(url)).toBe(filename);
});

test('extractFilename() extracts filename for well-formed file URL without slah before hash', async () => {
  const url = new URL('file:///aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx#');
  const filename = 'test.jdx';
  expect(extractFilename(url)).toBe(filename);
});

test('extractFilename() throws for malformed file URL', async () => {
  // invalid uuid
  const url = new URL('file:///aaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#');
  expect(() => extractFilename(url)).toThrow(/Cannot extract filename/i);
});

test('extractFilename() extracts UUID for well-formed http URL', async () => {
  const url = new URL('http://aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#');
  const filename = 'test.jdx';
  expect(extractFilename(url)).toBe(filename);
});

test('extractFilename() extracts UUID for well-formed https URL', async () => {
  const url = new URL(
    'https://aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#',
  );
  const filename = 'test.jdx';
  expect(extractFilename(url)).toBe(filename);
});

test('extractHashPath() extracts the path from regular hash', async () => {
  const url = new URL(
    'https://aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#/hash/path',
  );
  const path = '/hash/path';
  expect(extractHashPath(url)).toBe(path);
});

test('extractHashPath() extracts the path from hash ending with slash', async () => {
  const url = new URL(
    'https://aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#/hash/path/',
  );
  const path = '/hash/path/';
  expect(extractHashPath(url)).toBe(path);
});

test('extractHashPath() extracts the path from blank hash', async () => {
  const url = new URL(
    'https://aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#',
  );
  const path = '/';
  expect(extractHashPath(url)).toBe(path);
});

test('extractHashPath() extracts the path from slash only hash', async () => {
  const url = new URL(
    'https://aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#/',
  );
  const path = '/';
  expect(extractHashPath(url)).toBe(path);
});

test('extractHashPath() extracts the path for missing hash', async () => {
  const url = new URL('https://aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/');
  const path = '/';
  expect(extractHashPath(url)).toBe(path);
});

test('extractHashPath() throws for ill-formed non blank hash not starting with slash', async () => {
  const url = new URL(
    'https://aaaaaaaa-bbbb-cccc-dddd-1234567890ee/test.jdx/#abc/def',
  );
  expect(() => extractHashPath(url)).toThrow();
});
