# sciformats_js

JavaScript/TypeScript bindings for sciformats, a library for reading scientific data formats.

## Details

This library allows reading multiple scientific data formats. Currently, the following formats are supported:
- AnDI/AIA for Chromatographic Data ([ASTM E1947-98(2022)](https://www.astm.org/e1947-98r22.html), [ASTM E1948-98(2022)](https://www.astm.org/e1948-98r22.html))
- AnDI/AIA for Mass Spectrometric Data ([ASTM E2077-00(2016)](https://www.astm.org/e2077-00r16.html), [ASTM E2078-00(2016)](https://www.astm.org/e2078-00r16.html))
- Generalized Analytical Markup Language ([GAML](https://www.gaml.org/))
- JCAMP-DX ([JCAMP-DX](http://www.jcamp-dx.org/))

## Usage

```js
    // Initialize scanner repository with all supported data types.
    const scannerRepository = new ScannerRepository();

    // Read file from an <input> element of type "file".
    const file = input.files[0];
    const fileName = file.name;

    // Read file content.
    const buffer = await file.arrayBuffer();
    // As a Uint8Array is expected, an ArrayBuffer cannot be used directly.
    const uint8Array = new Uint8Array(buffer);
    // Ensure that the file has a supported format.
    const isRecognized = scannerRepository.isRecognized(fileName, uint8Array);
    if (!isRecognized) {
        return;
    }
    // Get a reader through which data from the file is retrieved.
    const reader = scannerRepository.getReader(fileName, uint8Array);
    
    // Read the root node.
    const rootNode = reader.read('/');

    // Use node content.
    const name = rootNode.name;
    const parameters = rootNode.parameters;
    const data = rootNode.data;
    const metadata = rootNode.metadata;
    const table = rootNode.table;
    const childNodeNames = rootNode.childNodeNames;

    // Read the fourth child node. Indexing starts at 0. There are as many child nodes as elements in the child_node_names list.
    let child3Node = reader.read('/3');

    // Read the first nested child node of the fourth root child node.
    let child30Node = reader.read('/3/0');
```

See the "examples" directory in the repository for more example code.

## How to build

See [Build instructions](./BUILD_INSTRUCTIONS.md).

## Author

- [Robert Schiwon (devrosch)](https://github.com/devrosch)

## License

Copyright (c) 2025 Robert Schiwon

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
