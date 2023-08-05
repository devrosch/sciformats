import Table from './Table';

/**
 * Data representing a node/fragment in the data hierarchy.
 */
type NodeData = {
  /**
   * The URL identifying this data.
   */
  url: URL,

  /**
   * Meta data represented as key-value pairs.
   */
  parameters: { key: string, value: string }[],

  /**
   * XY data.
   */
  data: { x: number, y: number }[],

  /**
   * Metadata key/value pairs.
   */
  metadata: { [key: string]: string },

  /**
   * A peak table.
   */
  table: Table,

  /**
   * Child nodes/fragments.
   */
  childNodeNames: string[],
};

export default NodeData;
