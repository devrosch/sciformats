mod io;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use crate::io::open_file;
use sf_rs::{andi::andi_scanner::AndiScanner, api::Scanner};
use wasm_bindgen_test::wasm_bindgen_test;

const ANDI_CHROM_VALID_FILE_PATH: &str = "andi_chrom_valid.cdf";
const ANDI_MS_VALID_FILE_PATH: &str = "andi_ms_centroid.cdf";
const ANDI_INVALID_FILE_PATH: &str = "dummy.cdf";

#[wasm_bindgen_test]
#[test]
fn andi_scanner_recognizes_valid_chrom_file() {
    let scanner = AndiScanner {};
    let (path, mut file) = open_file(ANDI_CHROM_VALID_FILE_PATH);
    assert!(scanner.is_recognized(&path, &mut file));
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_provides_reader_for_valid_chrom_file() {
    let scanner = AndiScanner {};
    let (path, file) = open_file(ANDI_CHROM_VALID_FILE_PATH);
    assert!(scanner.get_reader(&path, file).is_ok());
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_recognizes_valid_ms_file() {
    let scanner = AndiScanner {};
    let (path, mut file) = open_file(ANDI_MS_VALID_FILE_PATH);
    assert!(scanner.is_recognized(&path, &mut file));
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_provides_reader_for_valid_ms_file() {
    let scanner = AndiScanner {};
    let (path, file) = open_file(ANDI_MS_VALID_FILE_PATH);
    assert!(scanner.get_reader(&path, file).is_ok());
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_rejects_invalid_file() {
    let scanner = AndiScanner {};
    let (path, mut file) = open_file(ANDI_INVALID_FILE_PATH);
    assert!(!scanner.is_recognized(&path, &mut file));
}
