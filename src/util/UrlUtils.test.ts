import { isSameUrl } from 'util/UrlUtils';

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
