/**
 * Copyright (c) 2025 Robert Schiwon
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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
      if (result.correlationId === correlationId) {
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

export const initWorkerRs = async () => {
  // Webpack requires a string literal with the worker path
  const workerRs = new Worker(new URL('worker/WorkerRs.ts', import.meta.url));
  await initWorker(workerRs);
  return workerRs;
};
