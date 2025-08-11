use super::{GAML_SAMPLE_FILE, open_file};
use sciformats::{api::Parser, gaml::gaml_parser::GamlParser};
use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[allow(dead_code)]
#[wasm_bindgen_test]
fn gaml_parse_valid_file_succeeds() {
    let (path, file) = open_file(GAML_SAMPLE_FILE);
    let gaml = GamlParser::parse(&path, file);

    assert!(gaml.is_ok());
}
