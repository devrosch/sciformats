use super::{open_file, ANDI_CHROM_VALID, ANDI_MS_CENTROID};
use sf_rs::{andi::andi_scanner::AndiScanner, api::Scanner};
use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[allow(dead_code)]
#[wasm_bindgen_test]
fn andi_scanner_recognizes_valid_chrom_file() {
    let scanner = AndiScanner::new();
    let (path, mut file) = open_file(ANDI_CHROM_VALID);
    assert!(scanner.is_recognized(&path, &mut file));
}

#[allow(dead_code)]
#[wasm_bindgen_test]
fn andi_scanner_recognizes_valid_ms_file() {
    let scanner = AndiScanner::new();
    let (path, mut file) = open_file(ANDI_MS_CENTROID);
    assert!(scanner.is_recognized(&path, &mut file));
}
