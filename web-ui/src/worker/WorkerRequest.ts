export default class WorkerRequest {
  name: string;

  detail: any;

  constructor(name: string, detail: any) {
    this.name = name;
    this.detail = detail;
  }
}
