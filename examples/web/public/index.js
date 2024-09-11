import init, { ScannerRepository } from './pkg/sf_js.js';

// ------------------------------
// Initialize library and worker.
// ------------------------------

// 'module' type option is required so that the worker can import libraries.
const worker = new Worker(new URL("worker.js", import.meta.url), { type: 'module' });

async function run() {
  clearDisplay('fileInputEager', 'fileInputLazy');
  // Load and initialize package.
  await init({ url: './pkg/sf_js_bg.wasm' });
  console.log('Main thread: initialized sf_rs');
}

run();

// ---------------------------------------
// Eagerly load file and display contents.
// ---------------------------------------

window.onFileSelectedEager = async function (input) {
  const selectedFiles = input.files;
  if (selectedFiles === null || typeof selectedFiles === 'undefined') {
    return;
  }
  clearDisplay('fileInputLazy');
  const file = selectedFiles[0];
  read(file);
};

// Iterate through all nodes depth first.
function readNodes(reader, searchPath) {
  const path = searchPath || '';
  const node = reader.read(path);
  showNodeContent(path, node);

  // Read child nodes.
  for (let i = 0; i < node.childNodeNames.length; i += 1) {
    const childPath = path + `/${i}`;
    readNodes(reader, childPath);
  }
}

async function read(file) {
  // Read contents fully into memory.
  const fileName = file.name;
  const buffer = await file.arrayBuffer();
  console.log(`file name: ${fileName}`);
  console.log(`"${fileName}" content size: ${buffer.byteLength}`);

  const scannerRepository = new ScannerRepository();
  // As a Uint8Array is expected, an ArrayBuffer cannot be used directly.
  var uint8Array = new Uint8Array(buffer);
  const isRecognized = scannerRepository.isRecognized(fileName, uint8Array);
  if (!isRecognized) {
    console.log(`Unrecognized file format: : ${fileName}`);
    return;
  }
  console.log(`Recognized file format: ${fileName}`);

  // Get the suitable reader for reading file contents.
  const reader = scannerRepository.getReader(fileName, uint8Array);
  showName(fileName);
  // Read contents starting with the root node ''.
  readNodes(reader, '');
}

// --------------------------------------------
// Lazily load file in worker display contents.
// --------------------------------------------

window.onFileSelectedLazy = async function (input) {
  const selectedFiles = input.files;
  if (selectedFiles === null || typeof selectedFiles === 'undefined') {
    return;
  }
  clearDisplay('fileInputEager');
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
