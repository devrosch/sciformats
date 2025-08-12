// Copyright (c) 2025 Robert Schiwon
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
