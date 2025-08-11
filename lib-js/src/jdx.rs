use super::{BlobSeekRead, JsNode, JsReader, create_js_reader, create_js_scanner, map_to_js_err};
use sciformats::{api::Scanner, jdx::jdx_scanner::JdxScanner};
use wasm_bindgen::{JsError, prelude::wasm_bindgen};
use web_sys::Blob;

create_js_scanner!(JdxScanner, JsJdxScanner);
create_js_reader!(JsJdxScanner, JdxReader, JsJdxReader);
