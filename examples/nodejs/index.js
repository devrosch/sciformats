import { openSync, closeSync, readSync, readFileSync } from 'node:fs';
import { ScannerRepository } from 'sf_js';

const fileNameAndi = 'resources/andi_chrom_valid.cdf';
const fileNameJdx = 'resources/CompoundFile.jdx';

// Load file into memory and the parse and print contents.
// This is efficient and suitable for files if they are of moderate size in relation to the available memory.
loadIntoMemoryAndPrintContents(fileNameAndi);
loadIntoMemoryAndPrintContents(fileNameJdx);

// todo: lazy loading
// loadLazilyAndPrintContents(fileName);

function printContent(reader, searchPath) {
  const path = searchPath || '';
  const node = reader.read(path);
  printNodeContent(path, node);

  for (let i = 0; i < node.childNodeNames.length; i += 1) {
    const childPath = path + `/${i}`;
    printContent(reader, childPath);
  }
}

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
    printContent(reader, '');
  } catch (err) {
    console.error(err);
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

// function readOnDemand() {
//   const fd = openSync(fileName);
//   console.log(`successfully opened "${fileName}"`);

//   let buffer = new Uint8Array(5);
//   let numRead = readSync(fd, buffer);
//   console.log(`num bytes read: ${numRead}`);
//   console.log(`bytes read: ${buffer}`);

//   buffer.fill(0);
//   numRead = readSync(fd, buffer);
//   console.log(`num bytes read: ${numRead}`);
//   console.log(`bytes read: ${buffer}`);

//   closeSync(fd)
// }
