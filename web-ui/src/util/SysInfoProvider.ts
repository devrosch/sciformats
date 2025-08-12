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

/* eslint-disable-next-line @typescript-eslint/no-extraneous-class */
class SysInfoProvider {
  /**
   * Detects the current Operating System.
   * @returns 'Windows' for Windows, 'Linux/Unix' for Linux/Unix,
   * 'macOS' for Mac OS, and 'unknown' otherwise.
   */
  static detectOS() {
    // for details, see: https://stackoverflow.com/questions/9514179/how-to-find-the-operating-system-details-using-javascript
    // also: https://stackoverflow.com/questions/38241480/detect-macos-ios-windows-android-and-linux-os-with-js
    // do not user navigator.oscpu as it's deprecated and missing in some browsers
    const userAgent = window?.navigator?.userAgent
      ? window.navigator.userAgent
      : '';
    const platform = window?.navigator?.platform
      ? window.navigator.platform
      : '';

    let os = 'unknown'; // output
    if (
      platform.includes('Win') ||
      platform.includes('WOW') ||
      userAgent.includes('Win') ||
      userAgent.includes('WOW')
    ) {
      os = 'Windows';
    } else if (
      platform.includes('Linux') ||
      platform.includes('X11') ||
      platform.includes('BSD') ||
      userAgent.includes('Linux') ||
      userAgent.includes('X11') ||
      userAgent.includes('BSD')
    ) {
      os = 'Linux/Unix';
    } else if (platform.includes('Mac') || userAgent.includes('Mac')) {
      os = 'macOS';
    }
    // return detected OS or default 'unknown'
    return os;
  }
}

export default SysInfoProvider;
