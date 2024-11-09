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
   * Open the data set for reading.
   * This is a prerequisite for reading.
   * @returns {void}
   */
  open(): Promise<void>;

  /**
   * Read the contents of the node at the given URL.
   * @param {URL} url URL to file including fragment. Should start with root URL.
   * @example
   * file:///local/path/to/file#/
   * file:///local/path/to/file#/path/to/fragment
   * https://host/path/to/file#/path/to/fragment
   * @returns {NodeData} An object representing the fragment.
   */
  read(url: URL): Promise<NodeData>;

  /**
   * Exports the contents of the file in the format provided.
   * @param {string} format The format to export the data to.
   * Currently "Json" is the only supported export format.
   * @returns {Blob} A blob containing the export.
   */
  export(format: string): Promise<Blob>;

  /**
   * Closes the data set.
   * After closing it cannot be re-opened and no more reads are possible.
   * @returns {void}
   */
  close(): Promise<void>;
}
