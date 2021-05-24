# libsciwrap

Library for wrapping scientific data format parsers for ease of use.

## Getting Started

tbd

### Prerequisites

tbd

### Installing

tbd

## Running the tests

The test report from the CI/CD pipeline is located at: [GitLab Pages](https://devrosch.gitlab.io/libsciwrap/coverage)

## Documentation

Documentation generated during the build by the CI/CD pipeline is located at: [GitLab Pages](https://devrosch.gitlab.io/libsciwrap/doc)

## Misc

em++ --bind -lworkerfs.js -sFORCE_FILESYSTEM=1 -sWASM=1 -Wl,--whole-archive ./src/libsciwrap.a -Wl,--no-whole-archive -sDISABLE_EXCEPTION_CATCHING=0 ../apps/Main.cpp -o ./apps/sciwrap_main.js

python -m SimpleHTTPServer 8000
