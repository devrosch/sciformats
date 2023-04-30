import WorkerFileUrl from './WorkerFileUrl';
import WorkerNodeData from './WorkerNodeData';
import WorkerStatus from './WorkerStatus';

export default class WorkerResponse {
  name: string;

  correlationId: string;

  detail: WorkerStatus | { recognized: boolean } | WorkerFileUrl | WorkerNodeData | string;

  constructor(
    name: string,
    correlationId: string,
    detail: WorkerStatus | { recognized: boolean } | WorkerFileUrl | WorkerNodeData | string,
  ) {
    this.name = name;
    this.correlationId = correlationId;
    this.detail = detail;
  }
}
