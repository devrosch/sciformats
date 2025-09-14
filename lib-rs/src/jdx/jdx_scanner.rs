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

use std::{
    error::Error,
    io::{BufReader, Read, Seek},
};

use crate::{
    api::{Parser, Reader, Scanner, SeekBufRead},
    utils::{from_iso_8859_1_cstr, is_recognized_extension},
};

use super::{
    jdx_parser::JdxParser,
    jdx_reader::JdxReader,
    jdx_utils::{is_ldr_start, parse_ldr_start},
};

#[derive(Default)]
pub struct JdxScanner {}

impl JdxScanner {
    const ACCEPTED_EXTENSIONS: [&'static str; 3] = ["jdx", "dx", "jcm"];
    const JDX_START_LABEL: &'static str = "TITLE";
    const NUM_START_BYTES: u64 = 128;
}

impl JdxScanner {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Seek + Read + 'static> Scanner<T> for JdxScanner {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        if !is_recognized_extension(path, &Self::ACCEPTED_EXTENSIONS) {
            return false;
        }

        // recognized extension => check start of content
        let mut buf = Vec::<u8>::with_capacity(Self::NUM_START_BYTES as usize);
        let mut chunk = input.take(Self::NUM_START_BYTES);
        match chunk.read_to_end(&mut buf) {
            Err(_) => false,
            Ok(_) => {
                let s = from_iso_8859_1_cstr(&buf);
                if is_ldr_start(&s)
                    && let Ok((label, _)) = parse_ldr_start(&s)
                {
                    return label == Self::JDX_START_LABEL;
                }
                false
            }
        }
    }

    fn get_reader(&self, path: &str, input: T) -> Result<Box<dyn Reader>, Box<dyn Error>> {
        let buf_reader = BufReader::new(input);
        let buf_input: Box<dyn SeekBufRead> = Box::new(buf_reader);
        let jdx_file = JdxParser::parse(path, buf_input)?;
        let reader = JdxReader::new(path, jdx_file);
        Ok(Box::new(reader))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn scanner_recognizes_valid_jdx() {
        let s = b"##TITLE= Data XYDATA (PAC) Block
            ##JCAMP-DX= 4.24
            ##DATA TYPE= INFRARED SPECTRUM
            ##XUNITS= 1/CM
            ##YUNITS= ABSORBANCE
            ##XFACTOR= 1.0
            ##YFACTOR= 1.0
            ##FIRSTX= 450
            ##LASTX= 451
            ##NPOINTS= 2
            ##FIRSTY= 10
            ##XYDATA= (X++(Y..Y))
            +450+10
            +451+11
            ##END=";
        let mut cursor = Cursor::new(s);

        let scanner = JdxScanner::new();
        assert!(scanner.is_recognized("name.jdx", &mut cursor));
    }

    #[test]
    fn scanner_rejects_invalid_jdx() {
        let s = b"##NOTITLE= Data XYDATA (PAC) Block
            ##JCAMP-DX= 4.24
            ##END=";
        let mut cursor = Cursor::new(s);

        let scanner = JdxScanner::new();
        assert!(!scanner.is_recognized("name.jdx", &mut cursor));
    }
}
