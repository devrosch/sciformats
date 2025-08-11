// Note: For clarity the below examples use simplified error handling.
// In a production application you should ensure that file descriptors get closed and readers get freed in all potential error cases.

// ------------------------------
// Initialize library and worker.
// ------------------------------

import init, { ScannerRepository } from './pkg/sciformats_js.js';

// Load and initialize package.
await init({ url: './pkg/sciformats_js_bg.wasm' });
console.log('Worker: initialized sciformats_js');

const scannerRepository = new ScannerRepository();

// -----------------
// Process messages.
// -----------------

// Process data sent by main thread.
onmessage = (e) => {
  const { command, file } = e.data;
  console.log(`Worker: received from main script: ${JSON.stringify(e.data)}`);

  switch (command) {
    case 'read':
      read(file)
      break;
    case 'export':
      exportFile(file);
      break;
    default:
      console.log(`Error, command not recognized: ${JSON.stringify(command)}. Data: ${JSON.stringify(data)}.`);
  }
};

const read = async (file) => {
  const fileName = file.name;
  console.log(`file name: ${fileName}`);
  const reader = getReader(fileName, file);
  if (!reader) {
    return;
  }

  // Send file name to main thread for displaying.
  postMessage({ command: 'showName', data: fileName });
  // Read contents starting with the root node ''.
  readNodes(reader, '');
  reader.free();
}

const exportFile = async (file) => {
  const fileName = file.name;
  console.log(`file name: ${fileName}`);
  const reader = getReader(fileName, file);
  if (!reader) {
    return;
  }

  const exportFileName = fileName + '.json';
  const blob = reader.exportToBlob('Json');

  // Send file name to main thread for displaying.
  postMessage({ command: 'saveBlob', data: { name: exportFileName, blob } });
  reader.free();
}

// Iterate through all nodes depth first.
function readNodes(reader, searchPath) {
  const path = searchPath || '';
  const node = reader.read(path);

  // The attributes are functions really, so data has to be explicitly read before posting.
  const nodeCopy = {
    name: node.name,
    parameters: node.parameters,
    data: node.data,
    metadata: node.metadata,
    table: node.table,
    childNodeNames: node.childNodeNames
  }

  // Send node data to main thread for displaying.
  postMessage({ command: 'showNodeContent', data: { path, node: nodeCopy } });

  // Read child nodes.
  for (let i = 0; i < node.childNodeNames.length; i += 1) {
    const childPath = path + `/${i}`;
    readNodes(reader, childPath);
  }
}

function getReader(fileName, file) {
  // A File is a Blob and can be used directly.
  // Processing a Blob is only possible in a worker, not in the main thread.
  const isRecognized = scannerRepository.isRecognized(fileName, file);
  if (!isRecognized) {
    console.log(`Unrecognized file format: : ${fileName}`);
    return;
  }
  console.log(`Recognized file format: ${fileName}`);

  // Get the suitable reader for reading file contents.
  const reader = scannerRepository.getReader(fileName, file);
  return reader;
}
