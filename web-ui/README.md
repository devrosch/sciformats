# SciFormats Web UI

An HTML/CSS/JS web UI for visualizing scientific data read witrh sciformats, a library for reading scientific data formats.

You can find the application published at [Github Pages](https://devrosch.github.io/sciformats/index.html).

## Prerequisites

- Node.js v20 or later (including npm) needs to be installed.
- The WASM library needs to have been built as described in `lib-js`.

## Build

To install the required packages run:

```
npm install
```

After that, to run the tests, run:

```
npm test
```

Test coverage information can be generated with `npm test -- --coverage`.

To check code formatting, run:

```
npm run format
```

To lint the code, run:

```
npm run lint
```

To start the development server, run:

```
npm start
```

For building a release version, run:

```
npm run build
```

Build artifacts are placed into the `dist` directory.

## Documentation

More detailed documentation can be found in the [doc](doc) directory.

## Author

- [Robert Schiwon (devrosch)](https://github.com/devrosch)

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
