# SciFormats

Libraries and tools for reading and visualizing scientific data.

You can try the application at [GitLab Pages](https://devrosch.gitlab.io/sf/index.html).

## Structure

- lib-cpp (**Deprecated**): Native C++ library that supports reading the JCAMP-DX format. Can be compiles to WASM and used in JS. Will be removed in a future release as its feature set is covered by lib-rs.
- lib-rs: Native Rust library that supports reading multiple formats. Core library used by other lib-xxx libraries except lib-cpp.
- lib-js: JavaScript/TypeScript bindings for lib-rs. This requires the JS runtime to support WebAssembly (WASM). All current browsers and major runtimes do.
- web-ui: An HTML/CSS/JS UI for data viewing that leverages lib-js.
- examples: Code examples code for using the library and its bindings.

## Documentation

More detailed documentation can be found in the individual directories.

## Author

**Robert Schiwon** - [devrosch](https://gitlab.com/devrosch)

## License

Copyright (C) 2022-2024 Robert Schiwon

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.
