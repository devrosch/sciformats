mod io;

use crate::io::open_file;
use sf_rs::{andi_chrom_scanner::AndiChromScanner, api::Scanner};

const ANDI_CHROM_VALID_FILE_PATH: &str = "andi_chrom_valid.cdf";
const ANDI_CHROM_INVALID_FILE_PATH: &str = "dummy.cdf";

#[test]
fn andi_chrom_recognize_valid_succeeds() {
    let scanner = AndiChromScanner {};
    let (valid_path, mut valid_file) = open_file(ANDI_CHROM_VALID_FILE_PATH);
    assert!(scanner.is_recognized(&valid_path, &mut valid_file));
}

#[test]
fn andi_chrom_recognize_invalid_fails() {
    let scanner = AndiChromScanner {};
    let (invalid_path, mut invalid_file) = open_file(ANDI_CHROM_INVALID_FILE_PATH);
    assert!(!scanner.is_recognized(&invalid_path, &mut invalid_file));
}
