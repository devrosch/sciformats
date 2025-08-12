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

use super::{ANDI_CHROM_VALID, ANDI_MS_CENTROID, ANDI_NON_ANDI_CDF, ANDI_NON_CDF_DUMMY, open_file};
use sciformats::{andi::andi_scanner::AndiScanner, api::Scanner};
use std::io::{Cursor, Seek};

#[test]
fn andi_scanner_recognizes_valid_chrom_file() {
    let scanner = AndiScanner::new();
    let (path, mut file) = open_file(ANDI_CHROM_VALID);
    assert!(scanner.is_recognized(&path, &mut file));
}

#[test]
fn andi_scanner_provides_reader_for_valid_chrom_file() {
    let scanner = AndiScanner::new();
    let (path, file) = open_file(ANDI_CHROM_VALID);
    assert!(scanner.get_reader(&path, file).is_ok());
}

#[test]
fn andi_scanner_recognizes_valid_ms_file() {
    let scanner = AndiScanner::new();
    let (path, mut file) = open_file(ANDI_MS_CENTROID);
    assert!(scanner.is_recognized(&path, &mut file));
}

#[test]
fn andi_scanner_provides_reader_for_valid_ms_file() {
    let scanner = AndiScanner::new();
    let (path, file) = open_file(ANDI_MS_CENTROID);
    assert!(scanner.get_reader(&path, file).is_ok());
}

#[test]
fn andi_scanner_rejects_invalid_file() {
    let scanner = AndiScanner::new();
    let (path, mut file) = open_file(ANDI_NON_CDF_DUMMY);
    assert!(!scanner.is_recognized(&path, &mut file));
}

#[test]
fn andi_scanner_rejects_no_extension_file() {
    let scanner = AndiScanner::new();
    let mut cursor = Cursor::new(vec![0x43u8, 0x44u8, 0x46u8]);
    let path = "no_extension_file_name";
    assert!(!scanner.is_recognized(&path, &mut cursor));
}

#[test]
fn andi_scanner_rejects_unrecognized_extension_file() {
    let scanner = AndiScanner::new();
    let mut cursor = Cursor::new(vec![0x43u8, 0x44u8, 0x46u8]);
    let path = "unrecognized_extension_file_name.abc";
    assert!(!scanner.is_recognized(&path, &mut cursor));
}

#[test]
fn andi_scanner_rejects_too_short_file() {
    let scanner = AndiScanner::new();
    let mut cursor = Cursor::new(vec![0x43u8, 0x44u8]);
    let path = "file_name.cdf";
    assert!(!scanner.is_recognized(&path, &mut cursor));
}

#[test]
fn andi_scanner_recognizes_non_andi_cdf_file_but_fails_reading() {
    let scanner = AndiScanner::new();
    let (path, mut file) = open_file(ANDI_NON_ANDI_CDF);
    assert!(scanner.is_recognized(&path, &mut file));
    let _ = file.seek(std::io::SeekFrom::Start(0));
    assert!(scanner.get_reader(&path, file).is_err());
}
