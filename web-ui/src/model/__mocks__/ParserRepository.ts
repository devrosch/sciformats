import LocalFileParser from './LocalFileParser';
import Parser from '../Parser';

export default class ParserRepository {
  // eslint-disable-next-line class-methods-use-this
  async findParser(file: File): Promise<Parser> {
    // generate URL of type file:///UUID/fileName#/
    const uuid = crypto.randomUUID();
    const url = new URL(`file:///${uuid}/${file.name}#/`);
    const parser = new LocalFileParser(url, file);
    return parser;
  }
}
