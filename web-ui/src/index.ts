import 'components/App';
import './style.css';
import WorkerResponse from 'worker/WorkerResponse';
import WorkerRequest from 'worker/WorkerRequest';
import { postMessage } from 'util/WorkerUtils';

// TODO: example code => remove
const exampleJdx = `##TITLE= Data XYDATA (PAC) Block
##JCAMP-DX= 4.24
##DATA TYPE= INFRARED SPECTRUM
##XUNITS= 1/CM
##YUNITS= ABSORBANCE
##XFACTOR= 1.0
##YFACTOR= 1.0
##FIRSTX= 450
##LASTX= 451
##NPOINTS= 2
##FIRSTY= 10
##XYDATA= (X++(Y..Y))
+450+10
+451+11
##END=
`;

const worker = new Worker(new URL('worker/worker.ts', import.meta.url));
worker.onmessage = (event) => {
  const result = event.data as WorkerResponse;
  if (result.name === 'opened') {
    console.log(`worker response - ${result.name}: ${JSON.stringify(result.detail)} (correlation ID: ${result.correlationId})`);
  } else {
    console.log(`worker response - ${result.name}: ${result.detail} (correlation ID: ${result.correlationId})`);
  }
};

worker.postMessage(new WorkerRequest('status', crypto.randomUUID(), null));
setTimeout(() => {
  worker.postMessage(new WorkerRequest('status', crypto.randomUUID(), null));
}, 2500);

setTimeout(() => {
  const url = new URL('file:///aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee/test.jdx/#');
  const file = new File([exampleJdx], 'test.jdx');
  worker.postMessage(new WorkerRequest('scan', crypto.randomUUID(), { url: url.toString(), file }));
}, 5000);

setTimeout(() => {
  const url = new URL('file:///aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee/test.jdx/#');
  const file = new File([exampleJdx], 'test.jdx');
  // const file = new File(['##TITLE= '], 'test.jdx');
  worker.postMessage(new WorkerRequest('open', crypto.randomUUID(), { url: url.toString(), file }));
}, 7500);

setTimeout(() => {
  const url = new URL('file:///aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee/test.jdx/#');
  worker.postMessage(new WorkerRequest('close', crypto.randomUUID(), { url: url.toString() }));
}, 10000);

setTimeout(async () => {
  console.log('------ Async Worker messaging through Promises ------');
  
  const url = new URL('file:///aaaaaaa1-bbb2-ccc3-ddd4-eeeeeeeeeee5/test.jdx/#');
  const file = new File([exampleJdx], 'test.jdx');
  const payload = { url: url.toString(), file };

  const promise = postMessage(worker, 'scan', payload);
  const result = await promise;
  console.log(`result from promise: ${JSON.stringify(result)}`);

  const promise2 = postMessage(worker, 'scan', payload);
  const result2 = await promise2;
  console.log(`result2 from promise2: ${JSON.stringify(result2)}`);
}, 12500);

console.log('index.ts executed');
