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
export const postMessage = (
  worker: Worker,
  name: string,
  payload: any,
): Promise<WorkerResponse> => {
  const correlationId = crypto.randomUUID();

  const promise = new Promise<WorkerResponse>((resolve, reject) => {
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
export const initWorker = async (worker: Worker) => {
  let isWorkerInitialized = false;
  while (!isWorkerInitialized) {
    const scanReply = postMessage(worker, 'status', null);
    const timeout = new Promise((resolve) => {
      setTimeout(resolve, 500);
    });
    /* eslint-disable-next-line no-await-in-loop */
    const response = (await Promise.any([scanReply, timeout])) as any;
    if (
      response !== null &&
      typeof response === 'object' &&
      Object.hasOwn(response, 'correlationId')
    ) {
      const statusResponse = response as WorkerResponse;
      if (statusResponse.detail === WorkerStatus.Initialized) {
        isWorkerInitialized = true;
      } else {
        /* eslint-disable-next-line no-await-in-loop */
        await new Promise((resolve) => {
          setTimeout(resolve, 500);
        });
      }
    }
  }
  return worker;
};

export const initWorkerCpp = async () => {
  // Webpack requires a string literal with the worker path
  const workerCpp = new Worker(new URL('worker/Worker.ts', import.meta.url));
  await initWorker(workerCpp);
  return workerCpp;
};

export const initWorkerRs = async () => {
  // Webpack requires a string literal with the worker path
  const workerRs = new Worker(new URL('worker/WorkerRs.ts', import.meta.url));
  await initWorker(workerRs);
  return workerRs;
};
