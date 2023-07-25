/**
 * Data representing a node/fragment in the data hierarchy.
 */
type NodeData = {
  /**
   * The URL identifying this data.
   */
  url: URL,
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

export default NodeData;
