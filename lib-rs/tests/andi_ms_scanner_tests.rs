mod io;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use crate::io::open_file;
use sf_rs::{andi::andi_chrom_scanner::AndiChromScanner, api::Scanner};
use wasm_bindgen_test::wasm_bindgen_test;

const ANDI_MS_VALID_FILE_PATH: &str = "andi_ms_centroid.cdf";
const ANDI_MS_INVALID_FILE_PATH: &str = "dummy.cdf";

#[wasm_bindgen_test]
#[test]
fn andi_ms_recognize_valid_succeeds() {
    let scanner = AndiChromScanner {};
    let (valid_path, mut valid_file) = open_file(ANDI_MS_VALID_FILE_PATH);
    assert!(scanner.is_recognized(&valid_path, &mut valid_file));
}

#[wasm_bindgen_test]
#[test]
fn andi_ms_recognize_invalid_fails() {
    let scanner = AndiChromScanner {};
    let (invalid_path, mut invalid_file) = open_file(ANDI_MS_INVALID_FILE_PATH);
    assert!(!scanner.is_recognized(&invalid_path, &mut invalid_file));
}
