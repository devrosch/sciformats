export default interface DataRepository {
  /**
   * 
   * @param {URL} url URL to file including fragment.
   * @example
   * file:///local/path/to/file#/
   * file:///local/path/to/file#/path/to/fragment
   * https://host/path/to/file#/path/to/fragment
   * @returns {JSON} A JSON document representing the fragment.
   */
  read(url: URL): {
    url: URL,
    data: { x: number, y: number }[],
    parameters: { key: string, value: string }[],
    children: string[],
  };
}