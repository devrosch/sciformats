# Examples - Node.js

Node.js example application that reads scientific files and prints the contents to the console. Uses sf_js for reading scientific data formats.

## Prerequisites

### Rust

Install the [Rust Toolchain](https://www.rust-lang.org/tools/install) including cargo. It may be necessary to activate the WebAssembly target with: `rustup target add wasm32-unknown-unknown`.

Build the sf_js library. In the `lib-js` directory run:

```
cargo install
wasm-pack build --target nodejs
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

Copyright (C) 2024 Robert Schiwon

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.
