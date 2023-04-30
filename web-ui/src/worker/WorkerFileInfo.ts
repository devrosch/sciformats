import WorkerFileUrl from './WorkerFileUrl';

/**
 * Data representing a file.
 */
type WorkerFileInfo = WorkerFileUrl & {
  /**
   * File data.
   */
  file: Blob,
};

export default WorkerFileInfo;
