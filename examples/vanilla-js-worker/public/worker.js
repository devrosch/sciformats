// ------------------------------
// Initialize library and worker.
// ------------------------------

import init, { ScannerRepository } from './pkg/sf_js.js';

// Load and initialize package.
await init({ url: './pkg/sf_js_bg.wasm' });
console.log('Worker: initialized sf_js');

const scannerRepository = new ScannerRepository();

// -----------------
// Process messages.
// -----------------

// Process data sent by main thread.
onmessage = (e) => {
  console.log(`Worker: received from main script: ${JSON.stringify(e.data)}`);
  const file = e.data;
  read(file)
};

const read = async (file) => {
  // Read contents fully into memory.
  const fileName = file.name;
  console.log(`file name: ${fileName}`);
  console.log(`"${fileName}" content size: ${file.size}`);

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

  // Send file name to main thread for displaying.
  postMessage({ command: 'showName', data: fileName });
  // Read contents starting with the root node ''.
  readNodes(reader, '');
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
