/**
 * Data representing a table, e.g., a peak table.
 */
type Table = {
  /**
   * The column names.
   * Each column has a technical name (key) and a display name (value).
   */
  columnNames: { key: string; value: string }[];

  /**
   * The table rows.
   * Each element (i.e., each map) of the array represents table row, e.g., a single peak.
   * The map holds the cells for the row, e.g., the peak data. Should only contain keys that are
   * also present as column name keys. May miss some column name key which the represents blank
   * cells.
   */
  rows: { [key: string]: any }[];
};

export default Table;
