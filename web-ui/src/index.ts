import 'components/App';
import './style.css';
import WorkerResponse from 'worker/WorkerResponse';
import WorkerRequest from 'worker/WorkerRequest';

// TODO: example code => remove
const worker = new Worker(new URL('worker/worker.ts', import.meta.url));
worker.onmessage = (event) => {
  const result = event.data as WorkerResponse;
  console.log(`worker - ${result.name}: ${result.detail}`);
};

worker.postMessage(new WorkerRequest('status', null));
setTimeout(() => {
  worker.postMessage(new WorkerRequest('status', null));
}, 2500);

setTimeout(() => {
  const url = new URL('file:///aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee/test.jdx/#');
  const file = new File(['##TITLE= '], 'test.jdx');
  worker.postMessage(new WorkerRequest('scan', { url: url.toString(), file }));
}, 5000);

console.log('index.ts executed');
