/**
 * Data representing a peak table.
 */
type PeakTable = {
  /**
   * The column names.
   * Each column has a technical name (key) and a display name (value).
   */
  columnNames: { key: string, value: string }[],

  /**
   * The peaks.
   * Each element (i.e., each map) of the array represents table row, i.e., a single peak.
   * The map holds the cells for the row, i.e., the peak data. Should only contain keys that are
   * also present as column name keys. May miss some column name key which the represents blank
   * cells.
   */
  peaks: { [key: string]: any }[],
};

export default PeakTable;
