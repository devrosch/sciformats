import WorkerFileInfo from './WorkerFileInfo';
import WorkerFileUrl from './WorkerFileUrl';

export default class WorkerRequest {
  name: string;

  correlationId: string;

  detail: null | WorkerFileUrl | WorkerFileInfo;

  constructor(name: string, correlationId: string, detail: null | WorkerFileUrl | WorkerFileInfo) {
    this.name = name;
    this.correlationId = correlationId;
    this.detail = detail;
  }
}
