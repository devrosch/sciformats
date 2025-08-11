// Note: For clarity the below examples use simplified error handling.
// In a production application you should ensure that file descriptors get closed and readers get freed in all potential error cases.

// -----------------------------
// Initialize UI and web worker.
// -----------------------------

clearDisplay('fileInput');
const worker = new Worker(new URL('worker.js', import.meta.url));

// ------------------------------------------------
// Lazily load file in worker and display contents.
// ------------------------------------------------

window.onFileSelected = async function (input) {
  const selectedFiles = input.files;
  if (selectedFiles === null || typeof selectedFiles === 'undefined') {
    return;
  }
  clearDisplay();
  const file = selectedFiles[0];
  // Send file to worker for processing.
  worker.postMessage({ command: 'read', file });
};

// ------------
// Export data.
// ------------

window.onFileExport = async function (fileInputElementId) {
  const fileInput = document.getElementById(fileInputElementId);
  if (fileInput === null || typeof fileInput === 'undefined') {
    return;
  }
  const selectedFiles = fileInput.files;
  if (selectedFiles === null || typeof selectedFiles === 'undefined' || selectedFiles.length === 0) {
    return;
  }
  const file = selectedFiles[0];

  worker.postMessage({ command: 'export', file });
};

// ----------------------------
// Process data sent by worker.
// ----------------------------

worker.onmessage = (e) => {
  console.log(`Message received from worker: ${JSON.stringify(e.data)}`);

  const { command, data } = e.data;

  switch (command) {
    case 'showName':
      showName(data);
      break;
    case 'showNodeContent':
      const { path, node } = data;
      showNodeContent(path, node);
      break;
    case 'saveBlob':
      const { name, blob } = data;
      saveFile(name, blob);
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
  fileNameEl.value += `parameters: ${JSON.stringify(node.parameters, (_, v) => typeof v === 'bigint' ? v.toString() : v)}\n`;
  fileNameEl.value += `data: ${JSON.stringify(node.data, (_, v) => typeof v === 'bigint' ? v.toString() : v)}\n`;
  fileNameEl.value += `metadata: ${JSON.stringify(node.metadata, (_, v) => typeof v === 'bigint' ? v.toString() : v)}\n`;
  fileNameEl.value += `table: ${JSON.stringify(node.table, (_, v) => typeof v === 'bigint' ? v.toString() : v)}\n`;
  fileNameEl.value += `childNodeNames: ${JSON.stringify(node.childNodeNames)}\n\n`;
}

function saveFile(fileName, blob) {
  // save blob via anchor element with download attribute and object URL
  let a = document.createElement('a');
  a.href = URL.createObjectURL(blob);
  a.download = fileName;
  document.body.appendChild(a);
  a.click();
  // remove element
  setTimeout(() => {
    document.body.removeChild(a);
    window.URL.revokeObjectURL(a.href);
  }, 100);
};
