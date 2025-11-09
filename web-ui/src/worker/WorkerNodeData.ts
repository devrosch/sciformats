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

import Table from 'model/Table';

/**
 * Data representing a node/fragment in the data hierarchy as provided by a web worker.
 */
interface WorkerNodeData {
  /**
   * The URL identifying this data.
   * The @type { string } type is used here as @type { URL }
   * is not serializable for messages between a web worker and the main thread.
   */
  url: string;

  /**
   * Meta data represented as key-value pairs.
   */
  parameters: { key?: string; value: string | boolean | number | bigint }[];

  /**
   * XY data.
   */
  data: { x: number; y: number }[];

  /**
   * Metadata key/value pairs.
   */
  metadata: Record<string, string>;

  /**
   * A table, e.g., a peak table.
   */
  table: Table;

  /**
   * Child nodes/fragments.
   */
  childNodeNames: string[];
}

export default WorkerNodeData;
