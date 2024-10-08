use super::{create_js_reader, create_js_scanner, map_to_js_err, BlobSeekRead, JsNode, JsReader};
use sf_rs::{
    api::{Reader, Scanner},
    jdx::{jdx_reader::JdxReader, jdx_scanner::JdxScanner},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use web_sys::Blob;

create_js_scanner!(JdxScanner, JsJdxScanner);
create_js_reader!(JdxReader, JsJdxReader);
