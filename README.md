# SciFormats

Libraries and tools for reading and visualizing scientific data.

You can try the application at [Github Pages](https://devrosch.github.io/sciformats/index.html).

## Structure

- lib-cpp (**Deprecated**): Native C++ library that supports reading the JCAMP-DX format. Can be compiled to WASM and used in JS. No longer used. Will be removed in a future release as its feature set is covered by lib-rs.
- lib-rs: Native Rust library that supports reading multiple formats. Core library used by other lib-xxx libraries except lib-cpp.
- lib-js: JavaScript/TypeScript bindings for lib-rs. This requires the JS runtime to support WebAssembly (WASM). All current browsers and major runtimes do.
- web-ui: An HTML/CSS/JS UI for data viewing that leverages lib-js.
- examples: Code examples for using the library and its bindings.

## Documentation

More detailed documentation can be found in the individual directories.

## Author

* [Robert Schiwon (devrosch)](https://github.com/devrosch)

## License

The MIT License (MIT)

Copyright (c) 2025 Robert Schiwon

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
OR OTHER DEALINGS IN THE SOFTWARE.
