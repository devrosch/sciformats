import init, { ScannerRepository } from './pkg/sf_js.js';

// -----------------------------
// Initialize UI and web worker.
// -----------------------------

// 'module' type option is required so that the worker can import libraries.
const worker = new Worker(new URL("worker.js", import.meta.url), { type: 'module' });

clearDisplay('fileInput');

// --------------------------------------------
// Lazily load file in worker display contents.
// --------------------------------------------

window.onFileSelected = async function (input) {
  const selectedFiles = input.files;
  if (selectedFiles === null || typeof selectedFiles === 'undefined') {
    return;
  }
  clearDisplay();
  const file = selectedFiles[0];
  worker.postMessage({ command: 'isRecognized', data: { fileName: file.name, blob: file } });
};

worker.onmessage = (e) => {
  console.log(`Message received from worker: ${JSON.stringify(e.data)}`);

  const { command, data } = e.data;

  switch (command) {
    case 'isRecognized':
      if (data.isRecognized === true) {
        showName(data.fileName);
        worker.postMessage({ command: 'read', data: { fileName: data.fileName, blob: data.blob } });
      } else {
        console.log(`Unrecognized file format: : ${fileName}`);
      }
      break;
    case 'read':
      const { path, node } = data;
      showNodeContent(path, node);
      break;
    default:
      console.log(`Error, command not recognized: ${JSON.stringify(command)}. Data: ${JSON.stringify(data)}.`);
  }
};

// ------------------
// Utility functions.
// ------------------

function clearDisplay(...ids) {
  // Clear file inputs.
  for (let id of ids) {
    const fileinputEl = document.getElementById(id);
    fileinputEl.value = '';
  }
  // Clear file name.
  const fileContentEl = document.getElementById('fileContent');
  fileContentEl.value = '';
  // Clear textarea.
  const fileNameEl = document.getElementById('fileName');
  fileNameEl.textContent = '';
}

function showName(fileName) {
  const fileNameEl = document.getElementById('fileName');
  fileNameEl.textContent = fileName;
}

function showNodeContent(path, node) {
  const fileNameEl = document.getElementById('fileContent');
  fileNameEl.value += `Node path: "${path}"\n`;
  fileNameEl.value += `name: ${JSON.stringify(node.name)}\n`;
  fileNameEl.value += `parameters: ${JSON.stringify(node.parameters)}\n`;
  fileNameEl.value += `data: ${JSON.stringify(node.data)}\n`;
  fileNameEl.value += `metadata: ${JSON.stringify(node.metadata)}\n`;
  fileNameEl.value += `table: ${JSON.stringify(node.table)}\n`;
  fileNameEl.value += `childNodeNames: ${JSON.stringify(node.childNodeNames)}\n\n`;
}
