import 'components/App';
import './style.css';
import WorkerResponse from 'worker/WorkerResponse';
import WorkerRequest from 'worker/WorkerRequest';

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
  const url = new URL('file:///aaaaaaa1-bbb2-ccc3-ddd4-eeeeeeeeeee5/test.jdx/#');
  const file = new File([exampleJdx], 'test.jdx');
  const correlationId = crypto.randomUUID();
  const promise = new Promise((resolve, reject) => {
    // do not set worker.onmessage as this overwrites other such handlers
    worker.addEventListener("message", (event) => {
      const result = event.data as WorkerResponse;
      console.log(`Promise received message from worker: ${result}`);
      if (result.correlationId === correlationId) {
        if (result.name === 'error') {
          reject(result);
        } else {
          resolve(result);
        }
      }
    });
  });

  worker.postMessage(new WorkerRequest('scan', correlationId, { url: url.toString(), file }));
  const result = await promise;  
  console.log(`result from promise: ${JSON.stringify(result)}`);
  
}, 12500);

console.log('index.ts executed');
