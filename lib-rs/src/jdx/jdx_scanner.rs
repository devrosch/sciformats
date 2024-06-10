use std::{
    cmp,
    error::Error,
    io::{BufReader, Read, Seek, SeekFrom},
};

use crate::{
    api::{Parser, Reader, Scanner, SeekBufRead},
    utils::is_recognized_extension,
};

use super::{jdx_parser::JdxParser, jdx_reader::JdxReader};

#[derive(Default)]
pub struct JdxScanner {}

impl JdxScanner {
    const ACCEPTED_EXTENSIONS: [&'static str; 3] = ["jdx", "dx", "jcm"];
    const MAGIC_BYTES: &'static [u8; 5] = b"TITLE";
    const NUM_START_BYTES: u64 = 128;
}

impl JdxScanner {
    pub fn new() -> Self {
        Self::default()
    }

    // todo: duplicate code in GamlScanner
    fn read_start<T: Seek + Read + 'static>(
        &self,
        input: &mut T,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let len = input.seek(SeekFrom::End(0))?;
        input.seek(SeekFrom::Start(0))?;
        let len = cmp::min(len, Self::NUM_START_BYTES);
        let mut buf = vec![0; len as usize];
        input.read_exact(&mut buf)?;

        Ok(buf)
    }
}

impl<T: Seek + Read + 'static> Scanner<T> for JdxScanner {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        if !is_recognized_extension(path, &Self::ACCEPTED_EXTENSIONS) {
            return false;
        }

        // recognized extension => check start of content
        // todo: duplicate code in GamlScanner
        // todo: actually parse LDR
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

    fn get_reader(&self, path: &str, input: T) -> Result<Box<dyn Reader>, Box<dyn Error>> {
        let buf_reader = BufReader::new(input);
        let buf_input: Box<dyn SeekBufRead> = Box::new(buf_reader);
        let jdx_file = JdxParser::parse(path, buf_input)?;
        let reader = JdxReader::new(path, jdx_file);
        Ok(Box::new(reader))
    }
}
