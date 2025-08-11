use super::{BlobSeekRead, JsNode, JsReader, create_js_reader, create_js_scanner, map_to_js_err};
use sciformats::{api::Scanner, gaml::gaml_scanner::GamlScanner};
use wasm_bindgen::{JsError, prelude::wasm_bindgen};
use web_sys::Blob;

create_js_scanner!(GamlScanner, JsGamlScanner);
create_js_reader!(JsGamlScanner, GamlReader, JsGamlReader);
