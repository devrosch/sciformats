import init, { ScannerRepository } from './pkg/sf_js.js';

async function run() {
  clearDisplay(true);
  // Load and initialize package.
  await init();
  console.log('initialized sf_rs');
}

window.onFileSelected = async function (input) {
  console.log('File selected.');
  const selectedFiles = input.files;
  if (selectedFiles === null || typeof selectedFiles === 'undefined') {
    return;
  }
  const file = selectedFiles[0];
  read(file);
};

function clearDisplay(clearFileInput) {
  if (clearFileInput) {
    // Clear file input.
    const fileinputEl = document.getElementById('fileInput');
    fileinputEl.value = '';
  }
  // Clear file name.
  const fileContentEl = document.getElementById('fileContent');
  fileContentEl.value = '';
  // Clear textarea.
  const fileNameEl = document.getElementById('fileName');
  fileNameEl.textContent = '';
}

async function read(file) {
  clearDisplay();

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

  const reader = scannerRepository.getReader(fileName, uint8Array);
  showName(fileName);
  // Iterate through all nodes depth first starting with the root node '' and print contents
  showContent(reader, '');
}

function showName(fileName) {
  const fileNameEl = document.getElementById('fileName');
  fileNameEl.textContent = fileName;
}

function showContent(reader, searchPath) {
  const path = searchPath || '';
  const node = reader.read(path);
  showNodeContent(path, node);

  for (let i = 0; i < node.childNodeNames.length; i += 1) {
    const childPath = path + `/${i}`;
    showContent(reader, childPath);
  }
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

run();
