// import { openSync, closeSync, readSync, readFileSync } from 'node:fs';
import { openSync, readFileSync } from 'node:fs';
import { ScannerRepository } from 'sf_js';

const fileNameAndi = '../_resources/andi_chrom_valid.cdf';
const fileNameJdx = '../_resources/CompoundFile.jdx';

// Load file into memory and then parse and print contents.
// This is efficient and suitable for files if they are of moderate size in relation to the available memory.
loadIntoMemoryAndPrintContents(fileNameAndi);
loadIntoMemoryAndPrintContents(fileNameJdx);

// readOnDemand(fileNameAndi);
// readOnDemand(fileNameJdx);

// Load file data lazily, parse and print contents.
// This suitable for large files to conserve memory.
loadLazilyAndPrintContents(fileNameAndi);
loadLazilyAndPrintContents(fileNameJdx);

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

function loadLazilyAndPrintContents(fileName) {
  try {
    const scannerRepository = new ScannerRepository();
    // Open file desecriptor.
    let fd = openSync(fileName);
    console.log(`successfully opened "${fileName}"`);
    console.log(`file descriptor: ${fd}`);

    // A file descriptor is an integer and can be used directly.
    const isRecognized = scannerRepository.isRecognized(fileName, fd);
    if (!isRecognized) {
      console.log(`Unrecognized file format: : ${fileName}`);
      return;
    }

    // fd gets closed by isRecognized() method, so create a new fd
    fd = openSync(fileName);
    const reader = scannerRepository.getReader(fileName, fd);
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

// function readOnDemand(fileName) {
//   const fd = openSync(fileName);
//   console.log(`successfully opened "${fileName}"`);
//   console.log(`file descriptor: ${fd}`);

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
