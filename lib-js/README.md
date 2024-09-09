## sf_js

JavaScript/TypeScript bindings for sf_rs, a library for reading scientific data formats.

### Build

You can build this library with:

```
wasm-pack build
```

By default the build will generate a package suitable for use with a bundler (e.g., webpack) to be run in a browser. For building a package for use in Node.js, run:

```
wasm-pack build --target nodejs -- --features nodejs
```

For direct import into a browser, run:
```
wasm-pack build --target web
```

The resulting npm package in each case is available in the `/pkg` directory after the build completes.

### Test

For running tests involving JavaScript interop, run:

```
wasm-pack test --firefox --headless
```

`--firefox` is a placeholder for one of several browser engines that can be used: `--node`, `--chrome`, `--firefox`, or `--safari`. Note: For Chrome you may have to manually install chromedriver on some platforms to make headless mode work, see [wasm-pack/issues/611](https://github.com/rustwasm/wasm-pack/issues/611). On macOS, to install geckodriver and chromedriver you may be able to use `brew` and install via `brew install geckodriver` and `brew install --cask chromedriver`. `--safari` will only work on macOS. `--node` will skip tests configured to run in a browser.

## Limitations

Code coverage for WASM is not currently supported. For details, see [lang/rust/issues/81684](https://github.com/rust-lang/rust/issues/81684), [wasm-bindgen/issues/2276](https://github.com/rustwasm/wasm-bindgen/issues/2276), and for a possible workaround [code-coverage-for-webassembly](https://github.com/hknio/code-coverage-for-webassembly).
