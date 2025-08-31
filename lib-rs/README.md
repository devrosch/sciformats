# sciformats

A library for reading scientific data formats.

## Details

This is a library implemented in [Rust](https://www.rust-lang.org/) for reading multiple scientific data formats. Currently, the following formats are supported:
- AnDI/AIA for Chromatographic Data ([ASTM E1947-98(2022)](https://www.astm.org/e1947-98r22.html), [ASTM E1948-98(2022)](https://www.astm.org/e1948-98r22.html))
- AnDI/AIA for Mass Spectrometric Data ([ASTM E2077-00(2016)](https://www.astm.org/e2077-00r16.html), [ASTM E2078-00(2016)](https://www.astm.org/e2078-00r16.html))
- Generalized Analytical Markup Language ([GAML](https://www.gaml.org/))
- JCAMP-DX ([JCAMP-DX](http://www.jcamp-dx.org/))

## Usage

```rust
    // Create a File instance or any other data type that implements the Read and Seek traits.
    let file_path = ...;
    let file = File::open(&file_path)?;
    let mut input: Box<dyn SeekRead> = Box::new(file);

    // Initialize scanner with all supported data types.
    let repo = ScannerRepository::init_all();
    // Ensure that the file has a supported format.
    assert!(repo.is_recognized(&file_path, &mut input));
    // Get a reader that through which data from the file is retrieved.
    let reader = repo.get_reader(&file_path, input)?;

    // Read the root node.
    let root_node_path = "/";
    let root_node = reader.read(root_node_path)?;

    // Output node content.
    println!("name: {}", node.name);
    println!("parameters: {:?}", node.parameters);
    println!("data: {:?}", node.data);
    println!("metadata: {:?}", node.metadata);
    println!("table: {:?}", node.table);
    println!("childNodeNames: {:?}", node.child_node_names);

    // Read the fourth child node. Indexing starts at 0. There are as many child nodes as elements in the child_node_names list.
    let child3_path = "/3";
    let child3_node = reader.read(child3_path)?;

    // Read the first nested child node of the fourth root child node.
    let child30_path = "/3/0";
    let child30_node = reader.read(child30_path)?;
```

## Prerequisites

- Install the [Rust Toolchain](https://www.rust-lang.org/tools/install) including cargo.
    - It may be necessary to activate the WebAssembly target with: `rustup target add wasm32-unknown-unknown`.
- Optionally, for checking code formatting install `rustfmt` with `rustup component add rustfmt`.
- Optionally, for linting the code install `clippy` with `rustup component add clippy`.

For capturing code coverage, the following additional tools are required:
- `llvm-tools-preview`: Install with: `rustup component add llvm-tools-preview`.
- `grcov`: Install with: `cargo install grcov`.
- Optionally, install `genhtml` if you want to use it to generate a coverage report instead of the `grcov` generated report.

## Build

You can build the library (tested on Ubuntu Linux x86-64 and macOS ARM) with:

```
cargo build
```

## Test

To run unit tests, integration tests, and doc tests, run:

```
cargo test
```

## Code Coverage

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

## Formatting

To check correct code formatting, run:
```
cargo fmt --check
```

To fix formatting issues, run the same command without the `--check` flag.

## Linting

To lint the code, run:
```
cargo clippy
```

To fix linting issues where possible, run the same command with the `--fix` flag.

## Author

- [Robert Schiwon (devrosch)](https://github.com/devrosch)

## License

Copyright (c) 2025 Robert Schiwon

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
