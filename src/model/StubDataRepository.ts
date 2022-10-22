import DataRepository from "./DataRepository";

export default class StubDataRepository implements DataRepository {
  read(url: URL) {
    const hash = decodeURIComponent(url.hash);
    let children: string[] = [];
    if (hash === '' || hash === '#' || hash === '/' || hash === '#/') {
      children = ['child 1', 'child 2', 'child 3'];
    }
    if (hash === '#/child 2') {
      children = ['child 4', 'child 5'];
    }

    return {
      url: url,
      data: [{ x: 1, y: 2 }, { x: 2, y: 4 }, { x: 3, y: 6 }],
      parameters: [{ key: 'key 1', value: 'value 1' }],
      children: children,
    }
  };
}