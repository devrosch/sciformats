// disable one generally applicable eslint error for this stub
/* eslint-disable class-methods-use-this */
import DataRepository from './DataRepository';

export default class StubDataRepository implements DataRepository {
  read(url: URL) {
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
