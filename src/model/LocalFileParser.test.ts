import Parser from './Parser';
import LocalFileParser from './LocalFileParser';

let url: URL;
let parser: Parser;

beforeEach(() => {
  url = new URL('file:///test/only.txt');
  const file = new File([], 'dummy');
  parser = new LocalFileParser(url, file);
});

test('root URL matches the one passed into the constructor', async () => {
  expect(parser.rootUrl).toBe(url);
});

test('reading node data succeeds', async () => {
  // TODO: this whole test just checks the mock implementation
  // needs to be re-written for the actual feature
  const rootNodeData = await parser.read(url);

  expect(rootNodeData.url).toBe(url);
  expect(rootNodeData.data).toHaveLength(1);
  expect(rootNodeData.parameters).toHaveLength(1);
  expect(rootNodeData.children).toHaveLength(3);

  const child2Url = new URL(url);
  child2Url.hash = '/child 2';

  const child2Data = await parser.read(child2Url);
  expect(child2Data.url).toBe(child2Url);
  expect(child2Data.data).toHaveLength(2);
  expect(child2Data.parameters).toHaveLength(2);
  expect(child2Data.children).toHaveLength(2);

  const child3Url = new URL(url);
  child3Url.hash = '/child 3';

  const child3Data = await parser.read(child3Url);
  expect(child3Data.url).toBe(child3Url);
  expect(child3Data.data).toHaveLength(3);
  expect(child3Data.parameters).toHaveLength(3);
  expect(child3Data.children).toHaveLength(0);
});
