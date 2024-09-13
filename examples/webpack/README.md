# Examples - Webpack

An JavaScript example application that reads scientific files and prints the contents in the browser. Uses sf_js for reading scientific data formats and Webpack for bundling. The application allows selecting files and if the format is recognized shows their content.

This example demonstrates how to read data eagerly, loading file contents fully into memory for parsing. This is suitable for running in the main thread but files must be small enough to fit into memory.

## Prerequisites

### Rust

Install the [Rust Toolchain](https://www.rust-lang.org/tools/install) including cargo. It may be necessary to activate the WebAssembly target with: `rustup target add wasm32-unknown-unknown`.

Build the sf_js library. In the `lib-js` directory run:

```
cargo install
wasm-pack build
```

The resulting npm package is then available in the `lib-js/pkg` directory.

### Node.js

Download and install [Node.js](https://nodejs.org/en/download/package-manager).

To install the dependencies for serving the application, in the `examples/webpack` directory run:

```
npm install
```

Note that this is merely required for the bundler. All application logic runs in the browser.

## Run example

To start the application, in the `examples/webpack` directory run:

```
npm start
```

This will start the dev server and the application that can be opened at https://localhost:8080.

For building the application for production run:

```
npm build
```

This will place the build output in the `dist` directory.

## Author

**Robert Schiwon** - [devrosch](https://gitlab.com/devrosch)

## License

Copyright (c) 2024 Robert Schiwon

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
