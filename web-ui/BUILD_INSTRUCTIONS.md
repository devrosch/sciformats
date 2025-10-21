# Build instructions

## Prerequisites

- Node.js v22 or later (including npm) needs to be installed.
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

## Details

More detailed documentation can be found in the [doc](doc) directory.
