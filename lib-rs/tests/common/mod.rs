use super::open_files;
use sciformats::{
    api::SeekRead,
    common::{BufSeekRead, ScannerRepository},
};

open_files!(
    "../andi/resources/",
    (
        (ANDI_CHROM_VALID_FILE_PATH, "andi_chrom_valid.cdf"),
        (ANDI_INVALID_FILE_PATH, "dummy.cdf"),
    )
);

#[test]
fn scanner_repository_recognizes_valid_file() {
    let repo = ScannerRepository::init_all();
    let (path, file) = open_file(ANDI_CHROM_VALID_FILE_PATH);
    let mut input: Box<dyn SeekRead> = Box::new(file);
    assert!(repo.is_recognized(&path, &mut input));
}

#[test]
fn scanner_repository_rejects_invalid_file() {
    let repo = ScannerRepository::init_all();
    let (path, file) = open_file(ANDI_INVALID_FILE_PATH);
    let mut input: Box<dyn SeekRead> = Box::new(file);
    assert!(!repo.is_recognized(&path, &mut input));
}

#[test]
fn scanner_repository_returns_reader_for_valid_file() {
    let repo = ScannerRepository::init_all();
    let (path, file) = open_file(ANDI_CHROM_VALID_FILE_PATH);
    let input: Box<dyn SeekRead> = Box::new(file);
    assert!(repo.get_reader(&path, input).is_ok());
}

#[test]
fn scanner_repository_returns_error_for_valid_file() {
    let repo = ScannerRepository::init_all();
    let (path, file) = open_file(ANDI_INVALID_FILE_PATH);
    let input: Box<dyn SeekRead> = Box::new(file);
    assert!(repo.get_reader(&path, input).is_err());
}

#[test]
fn buf_seek_read_allows_valid_file_reading() {
    let repo = ScannerRepository::init_all();
    let (path, file) = open_file(ANDI_CHROM_VALID_FILE_PATH);
    let buf_seek_read = BufSeekRead::new(file);
    let input: Box<dyn SeekRead> = Box::new(buf_seek_read);
    let reader = repo.get_reader(&path, input).unwrap();
    assert!(reader.read("/").is_ok());
}
