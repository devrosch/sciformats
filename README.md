[![pipeline](https://gitlab.com/devrosch/libjdx/badges/master/pipeline.svg)](https://gitlab.com/devrosch/libjdx/commits/master)
[![coverage](https://gitlab.com/devrosch/libjdx/badges/master/coverage.svg)](https://devrosch.gitlab.io/libjdx/coverage)

# libjdx

Library for reading JCAMP-DX data.

## Getting Started

tbd

### Prerequisites

You will need a recent version of git, CMake >= 3.15, a C++ compiler that is compliant with C++17 (e.g. recent versions of GCC and Clang/LLVM) to download the source code and to build this library.

### Installing

tbd

## Running the tests

The test report from the CI/CD pipeline is located at: [GitLab Pages](https://devrosch.gitlab.io/libjdx/coverage)

## Documentation

Documentation generated during the build by the CI/CD pipeline is located at: [GitLab Pages](https://devrosch.gitlab.io/libjdx/doc)

### Sample data

Public domain sample data taken from [JSpecView2](http://wwwchem.uwimona.edu.jm/spectra/JSpecView2/sample/).


## Deployment

tbd

### Emscripten

Build WebAssembly with Emscripten using the following commands:
```
emcmake cmake <path/to/project/dir>
```
followed by:
```
make
```
or
```
cmake --build . --verbose
```

For running tests for Emscripten compiled code, Node.js (available as command "node") is required. Run:

```
ctest -VV
```

see: [Emscripten](https://emscripten.org/docs/getting_started/downloads.html)
see: [Stack Overflow](https://stackoverflow.com/questions/15724357/using-boost-with-emscripten)


```
docker run --rm -v $(pwd):/src -u $(id -u):$(id -g) emscripten/emsdk emcc -std=c++17 USE_BOOST_HEADERS=1 -Iinclude src/binary_reader.cpp -o binary_reader.js
```

```
docker run --rm -v $(pwd):/src -u $(id -u):$(id -g) emscripten/emsdk emcc -std=c++17 -s USE_ICU=1 -Iinclude src/binary_reader.cpp apps/main.cpp -o binary_reader.html
```

```
python -m SimpleHTTPServer
```

```
docker run --rm -v $(pwd):/build -p 8080:8080 -u $(id -u):$(id -g) emscripten/emsdk emrun --port=8080 --no_browser /build/main.html
```

## Built With

For development:

* [Catch2](https://github.com/catchorg/Catch2/releases/download/v2.13.1/catch.hpp) (license: [Boost](https://github.com/catchorg/Catch2/blob/master/LICENSE.txt), source code: [GitHub](https://github.com/catchorg/Catch2))

and their various dependencies.

## Authors

* **Robert Schiwon** - [devrosch](https://gitlab.com/devrosch)

## License

Copyright (C) 2020 Robert Schiwon

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.  If not, see <http://www.gnu.org/licenses/>.
