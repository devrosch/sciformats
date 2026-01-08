// Copyright (c) 2026 Robert Schiwon
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

use crate::{
    api::{Reader, Scanner},
    common::SfError,
    utils::is_recognized_extension,
};
use std::{
    cmp,
    io::{Read, Seek, SeekFrom},
};

#[derive(Default)]
pub struct MzmlScanner {}

impl MzmlScanner {
    const ACCEPTED_EXTENSIONS: [&'static str; 1] = ["mzml"];
    const MAGIC_BYTES: &'static [u8; 4] = b"mzML";
    const NUM_START_BYTES: u64 = 128;
}

impl MzmlScanner {
    pub fn new() -> Self {
        Self::default()
    }

    fn read_start<T: Seek + Read>(&self, input: &mut T) -> Result<Vec<u8>, SfError> {
        let len = input.seek(SeekFrom::End(0))?;
        input.seek(SeekFrom::Start(0))?;
        let len = cmp::min(len, Self::NUM_START_BYTES);
        let mut buf = vec![0; len as usize];
        input.read_exact(&mut buf)?;

        Ok(buf)
    }
}

impl<T: Seek + Read> Scanner<T> for MzmlScanner {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        if !is_recognized_extension(path, &Self::ACCEPTED_EXTENSIONS) {
            return false;
        };

        // start of file contains magic bytes "mzML"?
        match self.read_start(input) {
            Err(_) => false,
            Ok(bytes) => {
                let pos = bytes
                    .windows(Self::MAGIC_BYTES.len())
                    .position(|window| window == Self::MAGIC_BYTES);
                pos.is_some()
            }
        }
    }

    fn get_reader(&self, _path: &str, _input: T) -> Result<Box<dyn Reader>, SfError> {
        todo!()
        // let input_seek_read: Box<dyn SeekRead> = Box::new(input);
        // let gaml = GamlParser::parse(path, input_seek_read)?;
        // Ok(Box::new(GamlReader::new(path, gaml)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn accepts_valid_mzml() {
        let path = "valid.mzML";
        let xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?>
            <mzML
                xmlns="http://psi.hupo.org/ms/mzml"
                xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                xsi:schemaLocation="http://psi.hupo.org/ms/mzml http://psidev.info/files/ms/mzML/xsd/mzML1.1.0.xsd"
                id="sciformats:simple.mzML" version="1.1.0">
            </mzML>"#;
        let mut reader = Cursor::new(xml);
        let scanner = MzmlScanner::new();

        assert_eq!(true, scanner.is_recognized(path, &mut reader));
    }

    #[test]
    fn rejects_invalid_extension() {
        let path = "invalid.notMzML";
        let xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?>
            <mzML
                xmlns="http://psi.hupo.org/ms/mzml"
                xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                xsi:schemaLocation="http://psi.hupo.org/ms/mzml http://psidev.info/files/ms/mzML/xsd/mzML1.1.0.xsd"
                id="sciformats:simple.mzML" version="1.1.0">
            </mzML>"#;
        let mut reader = Cursor::new(xml);
        let scanner = MzmlScanner::new();

        assert_eq!(false, scanner.is_recognized(path, &mut reader));
    }

    #[test]
    fn rejects_invalid_content() {
        let path = "invalid.mzML";
        let xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?>
            <notMzML
                xmlns="http://psi.hupo.org/ms/mzml"
                xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                xsi:schemaLocation="http://psi.hupo.org/ms/mzml http://psidev.info/files/ms/mzML/xsd/mzML1.1.0.xsd"
                id="sciformats:simple.mzML" version="1.1.0">
            </notMzML>"#;
        let mut reader = Cursor::new(xml);
        let scanner = MzmlScanner::new();

        assert_eq!(false, scanner.is_recognized(path, &mut reader));
    }

    // #[test]
    // fn provides_reader_for_valid_mzml() {
    //     let path = "valid.gaml";
    //     let xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?>
    //         <notMzML
    //             xmlns="http://psi.hupo.org/ms/mzml"
    //             xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    //             xsi:schemaLocation="http://psi.hupo.org/ms/mzml http://psidev.info/files/ms/mzML/xsd/mzML1.1.0.xsd"
    //             id="sciformats:simple.mzML" version="1.1.0">
    //         </notMzML>"#;
    //     let reader = Cursor::new(xml);
    //     let scanner = MzmlScanner::new();

    //     assert!(scanner.get_reader(path, reader).is_ok());
    // }
}
