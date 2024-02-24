use super::{create_js_reader, create_js_scanner};
use crate::{
    andi::{
        andi_chrom_reader::AndiChromReader, andi_ms_reader::AndiMsReader, andi_scanner::AndiScanner,
    },
    api::{Reader, Scanner},
    bind::wasm::{BlobWrapper, JsNode, JsReader},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use web_sys::Blob;

create_js_scanner!(AndiScanner, JsAndiScanner);
create_js_reader!(AndiChromReader, JsAndiChromReader);
create_js_reader!(AndiMsReader, JsAndiMsReader);
