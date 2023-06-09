import ParserRepository from 'model/ParserRepository';
import Parser from 'model/Parser';
import MockParser from './MockParser';

export default class MockParserRepository implements ParserRepository {
  /* eslint-disable-next-line class-methods-use-this */
  findParser(file: File): Promise<Parser> {
    return new Promise((resolve) => { resolve(new MockParser(file)); });
  }
}
