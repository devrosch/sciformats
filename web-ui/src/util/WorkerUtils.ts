import WorkerRequest from 'worker/WorkerRequest';
import WorkerResponse from 'worker/WorkerResponse';
import WorkerStatus from 'worker/WorkerStatus';

/**
 * Post message to web worker.
 * @param worker Web worker to send message to.
 * @param name Message name, i.e. type of message. Example: "scan".
 * @param payload Payload to send to web worker.
 * @returns A promise for the web worker response.
 */
/* eslint-disable-next-line import/prefer-default-export */
export const postMessage = (worker: Worker, name: string, payload: any) => {
  const correlationId = crypto.randomUUID();

  const promise = new Promise((resolve, reject) => {
    const listener = (event: MessageEvent<any>) => {
      const result = event.data as WorkerResponse;
      // console.log(`Promise received message from worker: ${JSON.stringify(result)}`);
      if (result.correlationId === correlationId) {
        // console.log(`Promise correlationId matched: ${result.correlationId}`);
        worker.removeEventListener('message', listener);
        if (result.name === 'error') {
          reject(result);
        } else {
          resolve(result);
        }
      }
    };
    // do not set worker.onmessage as this overwrites other such handlers
    worker.addEventListener('message', listener);
  });

  worker.postMessage(new WorkerRequest(name, correlationId, payload));

  return promise;
};

/**
 * Initialize web worker.
 * @returns Initialized web worker.
 */
export const initWorker = async () => {
  const worker = new Worker(new URL('worker/Worker.ts', import.meta.url));
  let isWorkerInitialized = false;
  while (!isWorkerInitialized) {
    // eslint-disable-next-line no-await-in-loop
    const scanReply: WorkerResponse = await postMessage(worker, 'status', null) as WorkerResponse;
    if (scanReply.detail === WorkerStatus.Initialized) {
      isWorkerInitialized = true;
    } else {
      // eslint-disable-next-line no-await-in-loop
      await new Promise((resolve) => { setTimeout(resolve, 500); });
    }
  }
  return worker;
};
