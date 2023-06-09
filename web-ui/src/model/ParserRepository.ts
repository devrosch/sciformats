import Parser from './Parser';

/**
 * A repository of parsers.
 */
export default interface ParserRepository {
  /**
   * Find a parser for the file.
   * @param file The file to parse.
   */
  findParser(file: File): Promise<Parser>;
}
