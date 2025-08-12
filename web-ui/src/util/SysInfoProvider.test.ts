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

import SysInfoProvider from './SysInfoProvider';

let navigatorSpy: jest.SpyInstance | null = null;

// for details, see: https://stackoverflow.com/questions/41885841/how-can-i-mock-the-javascript-window-object-using-jest
beforeEach(() => {
  navigatorSpy = jest.spyOn(window, 'navigator', 'get');
});

afterEach(() => {
  jest.restoreAllMocks();
});

it('returns one of the defined strings for the OS', () => {
  const os = SysInfoProvider.detectOS();

  expect(
    os === 'unknown' ||
      os === 'Windows' ||
      os === 'Linux/Unix' ||
      os === 'macOS',
  ).toBeTruthy();
});

it('detects Mac OS with Firefox', () => {
  navigatorSpy?.mockImplementation(() => ({
    oscpu: 'Intel Mac OS X 10.15',
    platform: 'MacIntel',
    userAgent:
      'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:103.0) Gecko/20100101 Firefox/103.0',
  }));

  const os = SysInfoProvider.detectOS();

  expect(os).toBe('macOS');
});

it('detects Mac OS with Safari', () => {
  navigatorSpy?.mockImplementation(() => ({
    // no oscpu entry
    platform: 'MacIntel',
    userAgent:
      'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Safari/605.1.15',
  }));

  const os = SysInfoProvider.detectOS();

  expect(os).toBe('macOS');
});

it('detects Mac OS with Chrome', () => {
  navigatorSpy?.mockImplementation(() => ({
    // no oscpu entry
    platform: 'MacIntel',
    userAgent:
      'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.79 Safari/537.36',
  }));

  const os = SysInfoProvider.detectOS();

  expect(os).toBe('macOS');
});

it('detects Windows with Chrome', () => {
  navigatorSpy?.mockImplementation(() => ({
    // no oscpu entry
    platform: 'Win32',
    userAgent:
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36',
  }));

  const os = SysInfoProvider.detectOS();

  expect(os).toBe('Windows');
});

it('detects Windows with Edge', () => {
  navigatorSpy?.mockImplementation(() => ({
    // no oscpu entry
    platform: 'Win32',
    userAgent:
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.36 Safari/537.36 Edg/104.0.1293.54',
  }));

  const os = SysInfoProvider.detectOS();

  expect(os).toBe('Windows');
});

it('detects Windows with Firefox', () => {
  navigatorSpy?.mockImplementation(() => ({
    oscpu: 'Windows NT 6.1; Win64; x64',
    platform: 'Win32',
    userAgent:
      'Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:90.0) Gecko/20100101 Firefox/90.0',
  }));

  const os = SysInfoProvider.detectOS();

  expect(os).toBe('Windows');
});

it('detects Linux/Unix with Firefox', () => {
  navigatorSpy?.mockImplementation(() => ({
    oscpu: 'Linux x86_64',
    platform: 'Linux x86_64',
    userAgent:
      'Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv: 79.0) Gecko/20100101 Firefox/79.0',
  }));

  const os = SysInfoProvider.detectOS();

  expect(os).toBe('Linux/Unix');
});

it('detects Linux/Unix with Chromium', () => {
  navigatorSpy?.mockImplementation(() => ({
    platform: 'Linux x86_64',
    userAgent:
      'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Ubuntu Chromium/79.0.3945.130 Chrome/79.0.3945.130 Safari/537.36',
  }));

  const os = SysInfoProvider.detectOS();

  expect(os).toBe('Linux/Unix');
});

it('detects Linux/Unix with Chrome', () => {
  navigatorSpy?.mockImplementation(() => ({
    platform: 'Linux x86_64', // assumption, no actually checked
    userAgent:
      'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.121 Safari/537.36',
  }));

  const os = SysInfoProvider.detectOS();

  expect(os).toBe('Linux/Unix');
});
