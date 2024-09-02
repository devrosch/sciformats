use super::{open_file, ANDI_CHROM_VALID};
use sf_rs::{
    andi::{andi_chrom_parser::AndiChromParser, andi_chrom_reader::AndiChromReader},
    api::{Parser, Reader},
};
use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn andi_chrom_read_valid_file_succeeds() {
    let (path, file) = open_file(ANDI_CHROM_VALID);
    let chrom = AndiChromParser::parse(&path, file).unwrap();
    let reader = AndiChromReader::new(&path, chrom);

    let root = &reader.read("/");

    assert!(root.is_ok());
}
