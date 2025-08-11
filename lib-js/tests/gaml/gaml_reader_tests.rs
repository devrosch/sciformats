use super::{GAML_SAMPLE_FILE, open_file};
use sciformats::{
    api::{Parser, Reader},
    gaml::{gaml_parser::GamlParser, gaml_reader::GamlReader},
};
use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[allow(dead_code)]
#[wasm_bindgen_test]
fn gaml_read_valid_file_succeeds() {
    let (path, file) = open_file(GAML_SAMPLE_FILE);
    let gaml = GamlParser::parse(&path, file).unwrap();
    let reader = GamlReader::new(&path, gaml);

    let root_res = &reader.read("/");

    assert!(root_res.is_ok());
}
