#!/bin/sh

# Capture test coverage. Run from Rust project root directory.

# Prerequisites required:
# - Rust Toolchain (https://www.rust-lang.org/tools/install) including cargo.
# - llvm-tools-preview. Install with: `rustup component add llvm-tools-preview`.
# - grcov. Install with: `cargo install grcov`.
# - genhtml. On Debian based systems, install with `apt-get install lcov`.

set -eu pipefail

# Build with coverage instrumentation
export RUSTFLAGS="-Cinstrument-coverage -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
# Output file names
export LLVM_PROFILE_FILE="./target/debug/coverage/sciformats-%p-%m.profraw"
# Full build
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=abort"

# Alternative approach (with currently less clear results):
# Use nightly Rust toolchain
# export RUSTC_BOOTSTRAP=1
# export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"

rm -rf ./target/debug/coverage
mkdir -p ./target/debug/coverage

cargo build
cargo test

# Uncommenting the following line and commenting out the ones below will generate an HTML report with grcov
# grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing --keep-only 'src/*' -o ./target/debug/coverage/
grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing --keep-only 'src/*' -o ./target/debug/coverage/lcov.info
genhtml -o ./target/debug/coverage/ --show-details --highlight --ignore-errors source --legend ./target/debug/coverage/lcov.info
