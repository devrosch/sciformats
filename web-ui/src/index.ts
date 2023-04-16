import 'components/App';
import './style.css';

// TODO: example code => remove
const worker = new Worker(new URL('worker/worker.ts', import.meta.url));
worker.onmessage = ({ data: { answer } }) => {
  console.log(answer);
};
// worker.postMessage({
//   question: 'bla',
// });
setTimeout(() => {
  worker.postMessage({ question: 'bla2' });
}, 5000);

console.log('index.ts executed');
