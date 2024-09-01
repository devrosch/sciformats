/**
 * URL representing a file.
 */
type WorkerFileUrl = {
  /**
   * The URL identifying this file.
   * The @type { string } type is used here as @type { URL }
   * are not serialized for messages between a web worker and the main thread.
   */
  url: string;
};

export default WorkerFileUrl;
