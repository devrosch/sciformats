# `libsf-rs`

A library for reading scientific data formats implemented in [Rust](https://www.rust-lang.org/).

## Prerequisites

* Install the [Rust Toolchain](https://www.rust-lang.org/tools/install) including cargo.
    * It may be necessary to activate the WebAssembly target with: `rustup target add wasm32-unknown-unknown`.
* Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

## Run

You can build the example locally with:

```
$ wasm-pack build
```

The resulting npm package is the available in the `/pkg` directory.

## Test

### WASM

For running tests involving JavaScript interop, run:

```
wasm-pack test --firefox --headless
```

`--firefox` is a placeholder for one of several browser engines that can be used: `--node`, `--chrome`, `--firefox`, or `--safari`. Note: For Chrome you may have to manually install chromedriver on some platforms to make headless mode work, see [wasm-pack/issues/611](https://github.com/rustwasm/wasm-pack/issues/611). Safari will only work on macOS.
