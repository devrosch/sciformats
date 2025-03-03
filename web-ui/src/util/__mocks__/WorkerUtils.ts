/* eslint-disable-next-line @typescript-eslint/no-unused-vars */
export const postMessage = (worker: Worker, name: string, payload: any) =>
  new Promise((resolve) => {
    resolve(null);
  });

export const initWorker = async () => {
  /* noop */
};
