export default class WorkerResponse {
  name: string;

  correlationId: string;

  detail: any;

  constructor(name: string, correlationId: string, detail: any) {
    this.name = name;
    this.correlationId = correlationId;
    this.detail = detail;
  }
}
