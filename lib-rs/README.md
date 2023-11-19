# sf_rs

A library for reading scientific data formats.

## Details

This is a library implemented in [Rust](https://www.rust-lang.org/) for reading multiple scientific data formats. Currently, the following formats are supported:
* AnDI/AIA for Chromatographic Data ([ASTM E1947-98(2022)](https://www.astm.org/e1947-98r22.html), [ASTM E1948-98(2022)](https://www.astm.org/e1948-98r22.html))
* AnDI/AIA for Mass Spectrometric Data ([ASTM E2077-00(2016)](https://www.astm.org/e2077-00r16.html), [ASTM E2078-00(2016)](https://www.astm.org/e2078-00r16.html))

## Prerequisites

* Install the [Rust Toolchain](https://www.rust-lang.org/tools/install) including cargo.
    * It may be necessary to activate the WebAssembly target with: `rustup target add wasm32-unknown-unknown`.
* Optionally, for WASM builds install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).
* Optionally, for checking code formatting install `rustfmt` with `rustup component add rustfmt`.
* Optionally, for linting the code install `clippy` with `rustup component add clippy`.

For capturing code coverage, the following additional tools are required:
* `llvm-tools-preview`: Install with: `rustup component add llvm-tools-preview`.
* `grcov`: Install with: `cargo install grcov`.
* Optionally, install `genhtml` if you want to use it to generate a coverage report instead of the `grcov` generated report.

## Native

Native compilation (tested on Ubuntu Linux x86-64 and macOS ARM) and cross-compilation to WebAssembly (WASM) are supported.

### Build

You can build the library with:

```
cargo build
```

### Test

To run unit tests, integration tests, and doc tests, run:

```
cargo test
```

### Code Coverage

For capturing code coverage:

```
export RUSTC_BOOTSTRAP=1
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="your_name-%p-%m.profraw"
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"

cargo build
cargo test

grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```

Alternatively, to generate a report with `genhtml`, run:

```
rm -rf ./target/debug/coverage
mkdir ./target/debug/coverage
grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing --keep-only 'src/*' -o ./target/debug/coverage/lcov.info
genhtml -o ./target/debug/coverage/ --show-details --highlight --ignore-errors source --legend ./target/debug/coverage/lcov.info
```

More information on capturing code coverage can be found at [doc.rust-lang.org](https://doc.rust-lang.org/rustc/instrument-coverage.html) and [grcov](https://github.com/mozilla/grcov).

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

## WASM

### Build

You can build the library with:

```
wasm-pack build
```

The resulting npm package is the available in the `/pkg` directory.

### Test

For running tests involving JavaScript interop, run:

```
wasm-pack test --firefox --headless
```

`--firefox` is a placeholder for one of several browser engines that can be used: `--node`, `--chrome`, `--firefox`, or `--safari`. Note: For Chrome you may have to manually install chromedriver on some platforms to make headless mode work, see [wasm-pack/issues/611](https://github.com/rustwasm/wasm-pack/issues/611). `--safari` will only work on macOS. `--node` will skip tests configured to run in a browser.

## Limitations

Code coverage for WASM is not currently supported. For details, see [lang/rust/issues/81684](https://github.com/rust-lang/rust/issues/81684), [wasm-bindgen/issues/2276](https://github.com/rustwasm/wasm-bindgen/issues/2276), and for a possible workaround [code-coverage-for-webassembly](https://github.com/hknio/code-coverage-for-webassembly).
