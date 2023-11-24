wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use crate::io::open_file;
use sf_rs::{andi::andi_scanner::AndiScanner, api::Scanner};
use std::io::{Cursor, Seek};
use wasm_bindgen_test::wasm_bindgen_test;

const ROOT_PATH: &str = "andi";
const ANDI_CHROM_VALID_FILE_PATH: &str = "andi_chrom_valid.cdf";
const ANDI_MS_VALID_FILE_PATH: &str = "andi_ms_centroid.cdf";
const ANDI_INVALID_FILE_PATH: &str = "dummy.cdf";
const NON_ANDI_CDF_FILE_PATH: &str = "non_andi.cdf";

#[wasm_bindgen_test]
#[test]
fn andi_scanner_recognizes_valid_chrom_file() {
    let scanner = AndiScanner::new();
    let (path, mut file) = open_file(ROOT_PATH, ANDI_CHROM_VALID_FILE_PATH);
    assert!(scanner.is_recognized(&path, &mut file));
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_provides_reader_for_valid_chrom_file() {
    let scanner = AndiScanner::new();
    let (path, file) = open_file(ROOT_PATH, ANDI_CHROM_VALID_FILE_PATH);
    assert!(scanner.get_reader(&path, file).is_ok());
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_recognizes_valid_ms_file() {
    let scanner = AndiScanner::new();
    let (path, mut file) = open_file(ROOT_PATH, ANDI_MS_VALID_FILE_PATH);
    assert!(scanner.is_recognized(&path, &mut file));
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_provides_reader_for_valid_ms_file() {
    let scanner = AndiScanner::new();
    let (path, file) = open_file(ROOT_PATH, ANDI_MS_VALID_FILE_PATH);
    assert!(scanner.get_reader(&path, file).is_ok());
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_rejects_invalid_file() {
    let scanner = AndiScanner::new();
    let (path, mut file) = open_file(ROOT_PATH, ANDI_INVALID_FILE_PATH);
    assert!(!scanner.is_recognized(&path, &mut file));
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_rejects_no_extension_file() {
    let scanner = AndiScanner::new();
    let mut cursor = Cursor::new(vec![0x43u8, 0x44u8, 0x46u8]);
    let path = "no_extension_file_name";
    assert!(!scanner.is_recognized(&path, &mut cursor));
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_rejects_unrecognized_extension_file() {
    let scanner = AndiScanner::new();
    let mut cursor = Cursor::new(vec![0x43u8, 0x44u8, 0x46u8]);
    let path = "unrecognized_extension_file_name.abc";
    assert!(!scanner.is_recognized(&path, &mut cursor));
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_rejects_too_short_file() {
    let scanner = AndiScanner::new();
    let mut cursor = Cursor::new(vec![0x43u8, 0x44u8]);
    let path = "file_name.cdf";
    assert!(!scanner.is_recognized(&path, &mut cursor));
}

#[wasm_bindgen_test]
#[test]
fn andi_scanner_recognizes_non_andi_cdf_file_but_fails_reading() {
    let scanner = AndiScanner::new();
    let (path, mut file) = open_file(ROOT_PATH, NON_ANDI_CDF_FILE_PATH);
    assert!(scanner.is_recognized(&path, &mut file));
    let _ = file.seek(std::io::SeekFrom::Start(0));
    assert!(scanner.get_reader(&path, file).is_err());
}
