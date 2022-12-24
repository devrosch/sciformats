// disable one generally applicable eslint error for this until full implementation is done
/* eslint-disable class-methods-use-this */
import Parser from './Parser';

export default class LocalFileParser implements Parser {
  #url: URL;

  // #file: File;

  // TODO: actually implement
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  constructor(url: URL, file: File) {
    this.#url = url;
    // this.#file = file;
  }

  get rootUrl(): URL {
    return this.#url;
  }

  async read(url: URL) {
    const baseUrlString = this.#url.toString();
    const urlString = url.toString();
    if (!urlString.startsWith(baseUrlString)) {
      throw new Error(`Illegal URL for parser: ${url}`);
    }

    // TODO: dummy
    const delay = (ms: number) => new Promise((resolve) => { setTimeout(resolve, ms); });
    await delay(500);

    const hash = decodeURIComponent(url.hash);
    let children: string[] = [];
    let data: { x: number, y: number }[] = [];
    if (hash === '' || hash === '#' || hash === '/' || hash === '#/') {
      children = ['child 1', 'child 2', 'child 3'];
    }
    if (hash.endsWith('/child 2')) {
      children = ['child 1', 'child 2'];
    }

    const parameters = [{ key: 'key 1', value: 'value 1' }];
    data = [{ x: 1, y: 2 }];
    if (hash.endsWith('/child 2')) {
      data = [{ x: 1, y: 2 }, { x: 2, y: 4 }];
      parameters.push({ key: 'key 2', value: 'value 2' });
    }
    if (hash.endsWith('/child 3')) {
      data = [{ x: 1, y: 2 }, { x: 2, y: 4 }, { x: 3, y: 6 }];
      parameters.push({ key: 'key 2', value: 'value 2' });
      parameters.push({ key: 'key 3', value: 'value 3' });
    }

    return {
      url,
      data,
      parameters,
      children,
    };
  }
}
