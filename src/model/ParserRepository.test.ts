import ParserRepository from './ParserRepository';

test('returns local file parser', async () => {
  const repo = new ParserRepository();
  const file = new File([], 'dummy');

  const parser = await repo.findParser(file);
  const rootUrl = parser.rootUrl.toString();
  const regex = new RegExp(`file:///.*/${file.name}#/`);
  expect(rootUrl).toMatch(regex);
});
