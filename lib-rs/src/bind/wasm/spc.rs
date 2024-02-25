use super::{create_js_reader, create_js_scanner, map_to_js_err};
use crate::{
    api::{Reader, Scanner},
    bind::wasm::{BlobWrapper, JsNode, JsReader},
    spc::{
        spc_reader::{SpcNewFormatReader, SpcOldFormatReader},
        spc_scanner::SpcScanner,
    },
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use web_sys::Blob;

create_js_scanner!(SpcScanner, JsSpcScanner);
create_js_reader!(SpcNewFormatReader, JsSpcNewFormatReader);
create_js_reader!(SpcOldFormatReader, JsSpcOldFormatReader);

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::{Array, Uint8Array};
    use wasm_bindgen_test::*;
    // see: https://github.com/rustwasm/wasm-bindgen/issues/3340
    // even though this test does not need to run in a worker, other unit tests do and fail if this one is not set to run in a worker
    wasm_bindgen_test_configure!(run_in_worker);
    // wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    // no #[test] as this test cannot run outside a browser engine
    #[wasm_bindgen_test]
    fn js_scanner_test() {
        let arr: [u8; 3] = [1, 2, 3];
        let js_arr = Array::new();
        // see: https://github.com/rustwasm/wasm-bindgen/issues/1693
        js_arr.push(&Uint8Array::from(arr.as_slice()));
        let blob = Blob::new_with_u8_array_sequence(&js_arr).unwrap();

        let scanner = JsSpcScanner::js_new();
        assert!(!scanner.js_is_recognized("some.spc", &blob))
    }
}
