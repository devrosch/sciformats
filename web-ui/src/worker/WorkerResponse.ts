export default class WorkerResponse {
  name: string;

  detail: any;

  constructor(name: string, detail: any) {
    this.name = name;
    this.detail = detail;
  }
}
