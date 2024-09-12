# Examples - Vanilla JS Web Eager

An vanilla JavaScript example application that reads scientific files and prints the contents in the browser. Uses sf_js for reading scientific data formats. The application allows selecting files and if the format is recognized shows their content.

this example demonstrates how to read data eagerly, loading file contents fully into memory for parsing. This is suitable for running in the main thread but files must be small enough to fit into memory.

## Prerequisites

### Rust

Install the [Rust Toolchain](https://www.rust-lang.org/tools/install) including cargo. It may be necessary to activate the WebAssembly target with: `rustup target add wasm32-unknown-unknown`.

Build the sf_js library. In the `lib-js` directory run:

```
cargo install
wasm-pack build --target web
```

The resulting npm package is then available in the `lib-js/pkg` directory.

### Node.js

Download and install [Node.js](https://nodejs.org/en/download/package-manager).

To install the dependencies for serving the application, in the `examples/web` directory run:

```
npm install
```

Note that this is merely required for serving HTML and JavaScript to the browser. All application logic runs in the browser.

## Run example

To start the application, in the `examples/node-js` directory run:

```
npm start
```

This will start a web application that can be opened at https://localhost:3000.

## Author

**Robert Schiwon** - [devrosch](https://gitlab.com/devrosch)

## License

Copyright (C) 2024 Robert Schiwon

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.
