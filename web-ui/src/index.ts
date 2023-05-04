import 'components/App';
import './style.css';
// import { postMessage } from 'util/WorkerUtils';
// import WorkerStatus from 'worker/WorkerStatus';
// import WorkerResponse from 'worker/WorkerResponse';
// import WorkerNodeData from 'worker/WorkerNodeData';

// // TODO: example code => remove
// const exampleJdx = `##TITLE= Data XYDATA (PAC) Block
// ##JCAMP-DX= 4.24
// ##DATA TYPE= INFRARED SPECTRUM
// ##XUNITS= 1/CM
// ##YUNITS= ABSORBANCE
// ##XFACTOR= 1.0
// ##YFACTOR= 1.0
// ##FIRSTX= 450
// ##LASTX= 451
// ##NPOINTS= 2
// ##FIRSTY= 10
// ##XYDATA= (X++(Y..Y))
// +450+10
// +451+11
// ##END=
// `;

// const worker = new Worker(new URL('worker/worker.ts', import.meta.url));

// setTimeout(async () => {
//   console.log('------ Async Worker messaging through Promises ------');

//   let initialized = false;
//   while (!initialized) {
//     /* eslint-disable-next-line no-await-in-loop */
//     const statusResponse = await postMessage(worker, 'status', null) as WorkerResponse;
//     const status = statusResponse.detail as WorkerStatus;
//     console.log(`status after 1s: ${WorkerStatus[status]}`);
//     initialized = status === WorkerStatus.Initialized;
//     if (!initialized) {
//       /* eslint-disable-next-line no-await-in-loop */
//       await new Promise((resolve) => { setTimeout(resolve, 100); });
//     }
//   }

//   const url = new URL('file:///aaaaaaa1-bbb2-ccc3-ddd4-eeeeeeeeeee5/test.jdx#/');
//   const blob = new Blob([exampleJdx]);

//   const scanResult = await postMessage(worker, 'scan', { url: url.toString(), file: blob });
//   console.log(`scan result: ${JSON.stringify(scanResult)}`);

//   const openResult = await postMessage(worker, 'open', { url: url.toString(), file: blob });
//   console.log(`open result: ${JSON.stringify(openResult)}`);

//   const readResult = await postMessage(worker, 'read', { url: url.toString() }) as WorkerResponse;
//   const rootNodeData = readResult.detail as WorkerNodeData;
//   console.log(`root node read result: ${JSON.stringify(rootNodeData)}`);

//   const subNodeUrl = new URL('file:///aaaaaaa1-bbb2-ccc3-ddd4-eeeeeeeeeee5/test.jdx#/XYDATA');
//   const subNodeReadResult = await postMessage(worker, 'read', { url: subNodeUrl.toString() }) as WorkerResponse;
//   const subNodeData = subNodeReadResult.detail as WorkerNodeData;
//   console.log(`sub node read result: ${JSON.stringify(subNodeData)}`);

//   const closeResult = await postMessage(worker, 'close', { url: url.toString() }) as WorkerResponse;
//   console.log(`close result: ${JSON.stringify(closeResult)}`);
// }, 0);

console.log('index.ts executed');
