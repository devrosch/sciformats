[![pipeline status](https://gitlab.com/devrosch/libsciwrap/badges/master/pipeline.svg)](https://gitlab.com/devrosch/libsciwrap/-/commits/master)
[![coverage report](https://gitlab.com/devrosch/libsciwrap/badges/master/coverage.svg)](https://devrosch.gitlab.io/libsciwrap/coverage)

# libsciwrap

Library for wrapping scientific data format parsers for ease of use.

## Getting Started

tbd

### Prerequisites

On Ubuntu 20.04, you can install all required and optional prerequisites for a native with:

```
apt-get update --yes && apt-get install --yes gcc g++ clang clang-tidy clang-format lcov bc doxygen cmake libicu-dev git
```

If you would also like to cross compile to WebAssembly, you will need to do:
```
apt-get update --yes && apt-get install --yes python3 default-jre
cd ~
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install 2.0.8 && ./emsdk activate 2.0.8
```

More recent versions of `emsdk` should also work.

A pre-configured environment with all the above tools is also available as a Docker container on DockerHub. If you have Docker installed you can download the latest image with:

```
docker pull devrosch/cppcicdenv
```

### Installing

First, clone the repository including its submodules:
```
git clone --recursive <URL>
```

In case you have already cloned the repo without submodules, you can then also run:
```
git submodule update --init --recursive
```

To pull updates to the code later, run:
```
git pull
git submodule foreach --recursive git pull
```

On Linux, MacOS, to build this project do:
```
mkdir build
cd build
cmake ..
make
```

## Running the tests

If the build has completeed successfully, you can then run the tests in the build directory with:
```
ctest -VV
```

The test coverage report from the latest successful CI/CD pipeline run is located at: [GitLab Pages](https://devrosch.gitlab.io/libsciwrap/coverage)

## Documentation

Documentation generated during the build by the CI/CD pipeline is located at: [GitLab Pages](https://devrosch.gitlab.io/libsciwrap/doc)


## Third Party Code

For automated testing, the following third party libraries are used:

* [Catch2](https://github.com/catchorg/Catch2) (license: [Boost Software License](https://github.com/catchorg/Catch2/blob/devel/LICENSE.txt))
* [Trompeloeil](https://github.com/rollbear/trompeloeil) (license: [Boost Software License](https://github.com/rollbear/trompeloeil/blob/master/LICENSE_1_0.txt))

## License

Copyright (C) 2020-2021 Robert Schiwon

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.  If not, see <http://www.gnu.org/licenses/>.

## Misc

em++ --bind -lworkerfs.js -sFORCE_FILESYSTEM=1 -sWASM=1 -Wl,--whole-archive ./src/libsciwrap.a -Wl,--no-whole-archive -sDISABLE_EXCEPTION_CATCHING=0 ../apps/Main.cpp -o ./apps/sciwrap_main.js

python -m SimpleHTTPServer 8000
