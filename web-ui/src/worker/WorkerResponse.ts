import WorkerExport from './WorkerExport';
import WorkerFileUrl from './WorkerFileUrl';
import WorkerNodeData from './WorkerNodeData';
import WorkerStatus from './WorkerStatus';

export default class WorkerResponse {
  name: string;

  correlationId: string;

  detail:
    | WorkerStatus
    | { recognized: boolean }
    | WorkerFileUrl
    | WorkerNodeData
    | WorkerExport
    | string;

  constructor(
    name: string,
    correlationId: string,
    detail:
      | WorkerStatus
      | { recognized: boolean }
      | WorkerFileUrl
      | WorkerNodeData
      | WorkerExport
      | string,
  ) {
    this.name = name;
    this.correlationId = correlationId;
    this.detail = detail;
  }
}
