import { log } from 'node:console';
import { openSync, closeSync, readFileSync } from 'node:fs';
import { ScannerRepository } from 'sf_js';

const resourcesDir = '../_resources/';
const exportDir = './exports/';
const fileNameAndi = 'andi_chrom_valid.cdf';
const fileNameJdx = 'CompoundFile.jdx';

// Note: For clarity the below examples use simplified error handling.
// In a production application you should ensure that file descriptors get closed and readers get freed in all potential error cases.

// ----------------------------------------------------------------------------------------------------------
// Load file into memory and then parse and print contents.
// This is efficient and suitable for files if they are of moderate size in relation to the available memory.
// ----------------------------------------------------------------------------------------------------------

// AnDI
const memoryReaderAndi = openEager(resourcesDir + fileNameAndi);
printNodes(memoryReaderAndi, '');
memoryReaderAndi.free();
// JCAMP-DX
const memoryReaderJdx = openEager(resourcesDir + fileNameJdx);
printNodes(memoryReaderJdx, '');
memoryReaderJdx.free();

// -------------------------------------------------
// Load file data lazily, parse and print contents.
// This suitable for large files to conserve memory.
// -------------------------------------------------

// AnDI
try {
  const { fd, reader } = openLazy(resourcesDir + fileNameAndi);
  printNodes(reader, '');
  closeSync(fd);
  reader.free();
} catch (err) {
  console.error(err);
}

// JCAMP-DX
try {
  const { fd, reader } = openLazy(resourcesDir + fileNameJdx);
  printNodes(reader, '');

  closeSync(fd);
  reader.free();
} catch (err) {
  console.error(err);
}

// ------------------
// Export.
// ------------------

// AnDI
try {
  const reader = openEager(resourcesDir + fileNameAndi);
  const exportFileName = exportDir + fileNameAndi + '.json';

  const fdExport = openSync(exportFileName, 'w');
  reader.exportToFile('Json', fdExport);
  // Alternatively, export to a Blob and process that.
  // const blob = reader.exportToBlob('Json');

  closeSync(fdExport);
  reader.free();
} catch (err) {
  console.error(err);
}

// JCAMP-DX
try {
  const { fd, reader } = openLazy(resourcesDir + fileNameJdx);
  const exportFileName = exportDir + fileNameJdx + '.json';

  const fdExport = openSync(exportFileName, 'w');
  reader.exportToFile('Json', fdExport);
  // Alternatively, export to a Blob and process that.
  // const blob = reader.exportToBlob('Json');

  closeSync(fdExport);
  closeSync(fd);
  reader.free();
} catch (err) {
  console.error(err);
}

// ------------------
// Utility functions.
// ------------------

function openEager(path) {
  try {
    const scannerRepository = new ScannerRepository();
    // Read contents fully into memory.
    const buffer = readFileSync(path);
    // A Node.js buffer is derived from Uint8Array, so it can be used directly.
    const isRecognized = scannerRepository.isRecognized(path, buffer);
    if (!isRecognized) {
      console.log(`Unrecognized file format: : ${path}`);
      return;
    }
    const reader = scannerRepository.getReader(path, buffer);
    return reader;
  } catch (err) {
    console.error(err);
  }
  return null;
}

function openLazy(path) {
  const scannerRepository = new ScannerRepository();
  // Open file desecriptor.
  const fd = openSync(path);
  if (fd < 0) {
    console.log(`Error opening file: ${path}`);
    return { fd, reader: null };
  }

  // A file descriptor is an integer and can be used directly.
  const isRecognized = scannerRepository.isRecognized(path, fd);
  if (!isRecognized) {
    console.log(`Unrecognized file format: : ${path}`);
    return { fd, reader: null };
  }
  const reader = scannerRepository.getReader(path, fd);

  return { fd, reader };
}

// Iterate through all nodes depth first and print node contents.
function printNodes(reader, searchPath) {
  const path = searchPath || '';
  const node = reader.read(path);

  printNodeContent(path, node);

  // Read child nodes.
  for (let i = 0; i < node.childNodeNames.length; i += 1) {
    const childPath = path + `/${i}`;
    printNodes(reader, childPath);
  }
}

function printNodeContent(path, node) {
  console.log(`Node path: ${path}`);
  console.log(`name: ${JSON.stringify(node.name)}`);
  console.log(`parameters: ${JSON.stringify(node.parameters)}`);
  console.log(`data: ${JSON.stringify(node.data)}`);
  console.log(`metadata: ${JSON.stringify(node.metadata)}`);
  console.log(`table: ${JSON.stringify(node.table)}`);
  console.log(`childNodeNames: ${JSON.stringify(node.childNodeNames)}`);
  console.log();
}
