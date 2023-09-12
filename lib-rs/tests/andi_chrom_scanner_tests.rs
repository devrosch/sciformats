use sf_rs::{andi_chrom_scanner::AndiChromScanner, api::Scanner};
use std::{fs::File, path::PathBuf};

const ANDI_CHROM_VALID_FILE_PATH: &str = "andi_chrom_valid.cdf";
const ANDI_CHROM_INVALID_FILE_PATH: &str = "dummy.cdf";

fn open_file(name: &str) -> (String, File) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/");
    path.push(name);
    let file = File::open(&path).unwrap();

    (path.to_str().unwrap().to_owned(), file)
}

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
