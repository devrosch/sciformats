/**
 * Copyright (c) 2025 Robert Schiwon
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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
  export(format: 'Json'): Promise<Blob>;

  /**
   * Closes the data set.
   * After closing it cannot be re-opened and no more reads are possible.
   * @returns {void}
   */
  close(): Promise<void>;
}
