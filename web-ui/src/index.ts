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
    console.log(`worker - ${result.name}: ${JSON.stringify(result.detail)}`);  
  } else {
    console.log(`worker - ${result.name}: ${result.detail}`);
  }
};

worker.postMessage(new WorkerRequest('status', null));
setTimeout(() => {
  worker.postMessage(new WorkerRequest('status', null));
}, 2500);

setTimeout(() => {
  const url = new URL('file:///aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee/test.jdx/#');
  const file = new File([exampleJdx], 'test.jdx');
  worker.postMessage(new WorkerRequest('scan', { url: url.toString(), file }));
}, 5000);

setTimeout(() => {
  const url = new URL('file:///aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee/test.jdx/#');
  const file = new File([exampleJdx], 'test.jdx');
  // const file = new File(['##TITLE= '], 'test.jdx');
  worker.postMessage(new WorkerRequest('open', { url: url.toString(), file }));
}, 7500);

setTimeout(() => {
  const url = new URL('file:///aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee/test.jdx/#');
  worker.postMessage(new WorkerRequest('close', { url: url.toString() }));
}, 10000);

console.log('index.ts executed');
