import { openSync, closeSync, readFileSync } from 'node:fs';
import { ScannerRepository } from 'sf_js';

const fileNameAndi = '../_resources/andi_chrom_valid.cdf';
const fileNameJdx = '../_resources/CompoundFile.jdx';

// Load file into memory and then parse and print contents.
// This is efficient and suitable for files if they are of moderate size in relation to the available memory.
loadIntoMemoryAndPrintContents(fileNameAndi);
loadIntoMemoryAndPrintContents(fileNameJdx);

// Load file data lazily, parse and print contents.
// This suitable for large files to conserve memory.
loadLazilyAndPrintContents(fileNameAndi);
loadLazilyAndPrintContents(fileNameJdx);

function loadIntoMemoryAndPrintContents(fileName) {
  try {
    const scannerRepository = new ScannerRepository();
    // Read contents fully into memory.
    const buffer = readFileSync(fileName);
    // A Node.js buffer is derived from Uint8Array, so it can be used directly.
    const isRecognized = scannerRepository.isRecognized(fileName, buffer);
    if (!isRecognized) {
      console.log(`Unrecognized file format: : ${fileName}`);
      return;
    }
    const reader = scannerRepository.getReader(fileName, buffer);
    // Iterate through all nodes depth first starting with the root node '' and print contents
    readNodes(reader, '');
  } catch (err) {
    console.error(err);
  }
}

function loadLazilyAndPrintContents(fileName) {
  // file descriptor
  let fd = null;

  try {
    const scannerRepository = new ScannerRepository();
    // Open file desecriptor.
    fd = openSync(fileName);
    if (fd < 0) {
      console.log(`Error opening file: ${fileName}`);
      return;
    }
    console.log(`Successfully opened file: ${fileName}`);
    console.log(`File descriptor: ${fd}`);

    // A file descriptor is an integer and can be used directly.
    const isRecognized = scannerRepository.isRecognized(fileName, fd);
    if (!isRecognized) {
      console.log(`Unrecognized file format: : ${fileName}`);
      return;
    }
    console.log(`Recognized file format: ${fileName}\n`);

    const reader = scannerRepository.getReader(fileName, fd);
    // Read and print contents starting with the root node ''.
    readNodes(reader, '');
  } catch (err) {
    console.error(err);
  }

  // Close file descriptor to avoid leaking resources.
  if (fd !== null && fd >= 0) {
    closeSync(fd)
  }
}

// Iterate through all nodes depth first.
function readNodes(reader, searchPath) {
  const path = searchPath || '';
  const node = reader.read(path);

  printNodeContent(path, node);

  // Read child nodes.
  for (let i = 0; i < node.childNodeNames.length; i += 1) {
    const childPath = path + `/${i}`;
    readNodes(reader, childPath);
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
