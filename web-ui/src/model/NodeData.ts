import PeakTable from './PeakTable';

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
   * A peak table.
   */
  peakTable: PeakTable,

  /**
   * Child nodes/fragments.
   */
  childNodeNames: string[],
};

export default NodeData;
