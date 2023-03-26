import NodeData from './NodeData';

export default interface Parser {
  /**
   * @returns {URL} URL to file root.
   * @example
   * file:///local/path/to/file
   * https://host/path/to/file
  */
  readonly rootUrl: URL;

  /**
   * @param {URL} url URL to file including fragment. Should start with root URL.
   * @example
   * file:///local/path/to/file#/
   * file:///local/path/to/file#/path/to/fragment
   * https://host/path/to/file#/path/to/fragment
   * @returns {NodeData} An object representing the fragment.
   */
  read(url: URL): Promise<NodeData>;
}
