use super::{create_js_reader, create_js_scanner, map_to_js_err};
use crate::{
    api::{Reader, Scanner},
    bind::wasm::{BlobSeekRead, JsNode, JsReader},
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
