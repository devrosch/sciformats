use super::{create_js_reader, create_js_scanner, map_to_js_err, BlobSeekRead, JsNode, JsReader};
use sf_rs::{
    api::{Reader, Scanner},
    gaml::{gaml_reader::GamlReader, gaml_scanner::GamlScanner},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use web_sys::Blob;

create_js_scanner!(GamlScanner, JsGamlScanner);
create_js_reader!(GamlReader, JsGamlReader);
