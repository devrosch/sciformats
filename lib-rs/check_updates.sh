#!/bin/sh

# Check for package updates. Run from Rust project root directory.

# Prerequisites required:
# - Rust Toolchain (https://www.rust-lang.org/tools/install) including cargo.

set -eu pipefail

num_outdated_packages="$(cargo update --dry-run 2>&1 >/dev/null | { grep -Po '(?<=Locking ).*(?= packages to latest compatible versions)' || echo 0; } )"

if [ "$num_outdated_packages" -gt 0 ]; then
  echo "Number of outdated packages: $num_outdated_packages"
  echo "Run \"cargo update --dry-run\" to list and \"cargo update\" to update them."
  return 1
fi
