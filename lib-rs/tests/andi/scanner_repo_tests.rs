wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use crate::io::open_file;
use sf_rs::common::{ScannerRepository, SeekRead};
use wasm_bindgen_test::wasm_bindgen_test;

const ROOT_PATH: &str = "andi";
const ANDI_CHROM_VALID_FILE_PATH: &str = "andi_chrom_valid.cdf";
const ANDI_INVALID_FILE_PATH: &str = "dummy.cdf";

#[wasm_bindgen_test]
#[test]
fn scanner_repository_recognizes_valid_file() {
    let repo = ScannerRepository::init_all();
    let (path, file) = open_file(ROOT_PATH, ANDI_CHROM_VALID_FILE_PATH);
    let mut input: Box<dyn SeekRead> = Box::new(file);
    assert!(repo.is_recognized(&path, &mut input));
}

#[wasm_bindgen_test]
#[test]
fn scanner_repository_rejects_invalid_file() {
    let repo = ScannerRepository::init_all();
    let (path, file) = open_file(ROOT_PATH, ANDI_INVALID_FILE_PATH);
    let mut input: Box<dyn SeekRead> = Box::new(file);
    assert!(!repo.is_recognized(&path, &mut input));
}

#[wasm_bindgen_test]
#[test]
fn scanner_repository_returns_reader_for_valid_file() {
    let repo = ScannerRepository::init_all();
    let (path, file) = open_file(ROOT_PATH, ANDI_CHROM_VALID_FILE_PATH);
    let input: Box<dyn SeekRead> = Box::new(file);
    assert!(repo.get_reader(&path, input).is_ok());
}

#[wasm_bindgen_test]
#[test]
fn scanner_repository_returns_error_for_valid_file() {
    let repo = ScannerRepository::init_all();
    let (path, file) = open_file(ROOT_PATH, ANDI_INVALID_FILE_PATH);
    let input: Box<dyn SeekRead> = Box::new(file);
    assert!(repo.get_reader(&path, input).is_err());
}
