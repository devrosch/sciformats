import init, { ScannerRepository } from './pkg/sf_js.js';

// ------------------------------
// Initialize library and worker.
// ------------------------------

// Load and initialize package.
await init({ url: './pkg/sf_js_bg.wasm' });
console.log('Worker: initialized sf_rs');

const repo = new ScannerRepository();

// -----------------
// Process messages.
// -----------------

onmessage = (e) => {
  console.log(`Worker: received from main script: ${JSON.stringify(e.data)}`);

  const { command, data: { fileName, blob } } = e.data;
  switch (command) {
    case 'isRecognized':
      isRecognized(fileName, blob);
      break;
    case 'read':
      read(fileName, blob);
      break;
    default:
      postMessage({ command, data: 'Error: command not recognized.' });
  }
};

const isRecognized = (fileName, blob) => {
  const isRecognized = repo.isRecognized(fileName, blob);
  postMessage({ command: 'isRecognized', data: { isRecognized, fileName, blob } });
}

const read = (fileName, blob) => {
  // Get the suitable reader for reading file contents.
  const reader = repo.getReader(fileName, blob);
  // Read contents starting with the root node ''.
  readNodes(reader, '');
}

// Iterate through all nodes depth first.
function readNodes(reader, searchPath) {
  const path = searchPath || '';
  const node = reader.read(path);
  console.log(`Worker: node read: ${JSON.stringify(node)}`);

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
  postMessage({ command: 'read', data: { path, node: nodeCopy } });

  // Read child nodes.
  for (let i = 0; i < node.childNodeNames.length; i += 1) {
    const childPath = path + `/${i}`;
    readNodes(reader, childPath);
  }
}
