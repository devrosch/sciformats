# Examples - Node.js

Node.js example application that reads scientific files and prints the contents to the console. Uses sf_js for reading scientific data formats.

## Prerequisites

### Rust

Install the [Rust Toolchain](https://www.rust-lang.org/tools/install) including cargo. It may be necessary to activate the WebAssembly target with: `rustup target add wasm32-unknown-unknown`.

Build the sf_js library. In the `lib-js` directory run:

```
cargo install
wasm-pack build --target nodejs -- --features nodejs
```

The resulting npm package is then available in the `lib-js/pkg` directory.

### Node.js

Download and install [Node.js](https://nodejs.org/en/download/package-manager).

To install the dependencies for Node.js, in the `examples/node-js` directory run:

```
npm install
```

## Run example

To start the application, in the `examples/node-js` directory run:

```
npm start
```

This will read example files and print their contents to the console. Please check the code and comments in `index.js` for more details.

## Author

**Robert Schiwon** - [devrosch](https://gitlab.com/devrosch)

## License

Copyright (c) 2024 Robert Schiwon

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
