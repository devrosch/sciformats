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
