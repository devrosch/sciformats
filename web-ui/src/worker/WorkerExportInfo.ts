/**
 * URL representing a file.
 */
interface WorkerExportInfo {
  /**
   * The URL identifying this file.
   * The @type { string } type is used here as @type { URL }
   * are not serialized for messages between a web worker and the main thread.
   */
  url: string;

  /**
   * The format to export into. Currently "Json" is the only supported export format.
   */
  format: 'Json';
}

export default WorkerExportInfo;
