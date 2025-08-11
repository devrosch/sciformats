use super::{ANDI_CHROM_VALID, open_file};
use sciformats::{andi::andi_chrom_parser::AndiChromParser, api::Parser};
use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[allow(dead_code)]
#[wasm_bindgen_test]
fn andi_chrom_parse_valid_file_succeeds() {
    let (path, file) = open_file(ANDI_CHROM_VALID);
    let chrom = AndiChromParser::parse(&path, file);

    assert!(chrom.is_ok());
}
