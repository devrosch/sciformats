# sciformats_js

JavaScript/TypeScript bindings for sciformats, a library for reading scientific data formats.

## Prerequisites

- Install the [Rust Toolchain](https://www.rust-lang.org/tools/install) including cargo.
    - It may be necessary to activate the WebAssembly target with: `rustup target add wasm32-unknown-unknown`.
- For WASM builds install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).
- Optionally, for checking code formatting install `rustfmt` with `rustup component add rustfmt`.
- Optionally, for linting the code install `clippy` with `rustup component add clippy`.

## Build

You can build this library with:

```
wasm-pack build
```

By default the build will generate a package suitable for use with a bundler (e.g., webpack) to be run in a browser. For building a package for use in Node.js, run:

```
wasm-pack build --target nodejs -- --features nodejs
```

For direct import into a browser, run:
```
wasm-pack build --target web
```

The resulting npm package in each case is available in the `/pkg` directory after the build completes.

## Test

For running tests involving JavaScript interop, run:

```
wasm-pack test --firefox --headless
```

`--firefox` is a placeholder for one of several browser engines that can be used: `--node`, `--chrome`, `--firefox`, or `--safari`. Note: For Chrome you may have to manually install chromedriver on some platforms to make headless mode work, see [wasm-pack/issues/611](https://github.com/rustwasm/wasm-pack/issues/611). On macOS, to install geckodriver and chromedriver you may be able to use `brew` and install via `brew install geckodriver` and `brew install --cask chromedriver`. `--safari` will only work on macOS. `--node` will skip tests configured to run in a browser.

### Formatting

To check correct code formatting, run:
```
cargo fmt --check
```

To fix formatting issues, run the same command without the `--check` flag.

### Linting

To lint the code, run:
```
cargo clippy
```

To fix linting issues where possible, run the same command with the `--fix` flag.

## Limitations

Code coverage for WASM is not currently supported. For details, see [lang/rust/issues/81684](https://github.com/rust-lang/rust/issues/81684), [wasm-bindgen/issues/2276](https://github.com/rustwasm/wasm-bindgen/issues/2276), and for a possible workaround [code-coverage-for-webassembly](https://github.com/hknio/code-coverage-for-webassembly).

## Author

- [Robert Schiwon (devrosch)](https://github.com/devrosch)

## License

Copyright (c) 2025 Robert Schiwon

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
