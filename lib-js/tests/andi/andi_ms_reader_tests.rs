use super::{ANDI_MS_CENTROID, open_file};
use sciformats::{
    andi::{andi_ms_parser::AndiMsParser, andi_ms_reader::AndiMsReader},
    api::{Parser, Reader},
};
use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[allow(dead_code)]
#[wasm_bindgen_test]
fn andi_ms_read_valid_file_succeeds() {
    let (path, file) = open_file(ANDI_MS_CENTROID);
    let ms = AndiMsParser::parse(&path, file).unwrap();
    let reader = AndiMsReader::new(&path, ms);

    let root = &reader.read("/");

    assert!(root.is_ok());
}
