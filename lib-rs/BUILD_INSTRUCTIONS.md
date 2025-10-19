# Build instructions

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

## Code coverage

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
