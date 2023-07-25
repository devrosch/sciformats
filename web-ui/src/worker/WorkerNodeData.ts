/**
 * Data representing a node/fragment in the data hierarchy as provided by a web worker.
 */
type WorkerNodeData = {
  /**
   * The URL identifying this data.
   * The @type { string } type is used here as @type { URL }
   * is not serializable for messages between a web worker and the main thread.
   */
  url: string,
  /**
   * XY data.
   */
  data: { x: number, y: number }[],
  /**
   * Meta data represented as key-value pairs.
   */
  parameters: { key: string, value: string }[],
  /**
   * Child nodes/fragments.
   */
  childNodeNames: string[],
};

export default WorkerNodeData;
