import WorkerFileUrl from './WorkerFileUrl';

/**
 * Data representing file content.
 */
type WorkerFileInfo = WorkerFileUrl & {
  /**
   * File data.
   */
  blob: Blob;
};

export default WorkerFileInfo;
