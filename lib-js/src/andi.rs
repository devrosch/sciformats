use super::{create_js_reader, create_js_scanner, map_to_js_err, BlobSeekRead, JsNode, JsReader};
use sf_rs::{andi::andi_scanner::AndiScanner, api::Scanner};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use web_sys::Blob;

create_js_scanner!(AndiScanner, JsAndiScanner);
create_js_reader!(JsAndiScanner, AndiChromReader, JsAndiChromReader);
create_js_reader!(JsAndiScanner, AndiMsReader, JsAndiMsReader);
