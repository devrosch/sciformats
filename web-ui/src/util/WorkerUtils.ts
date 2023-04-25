import WorkerRequest from "worker/WorkerRequest";
import WorkerResponse from "worker/WorkerResponse";

/**
 * Post message to web worker.
 * @param worker Web worker to send message to.
 * @param name Message name, i.e. type of message. Example: "scan".
 * @param payload Payload to send to web worker.
 * @returns A promise for the web worker response.
 */
export const postMessage = (worker: Worker, name: string, payload: any) => {
  const correlationId = crypto.randomUUID();

  const promise = new Promise((resolve, reject) => {
    const listener = (event: MessageEvent<any>) => {
      const result = event.data as WorkerResponse;
      console.log(`Promise received message from worker: ${result}`);
      if (result.correlationId === correlationId) {
        console.log(`Promise correlationId matched: ${result.correlationId}`);
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
