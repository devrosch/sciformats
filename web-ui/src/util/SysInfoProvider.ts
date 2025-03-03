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
