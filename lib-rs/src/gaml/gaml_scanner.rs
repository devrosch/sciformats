use super::{gaml_parser::GamlParser, gaml_reader::GamlReader};
use crate::{
    api::{Parser, Reader, Scanner},
    common::SeekRead,
    utils::is_recognized_extension,
};
use std::{
    cmp,
    error::Error,
    io::{Read, Seek, SeekFrom},
};

#[derive(Default)]
pub struct GamlScanner {}

impl GamlScanner {
    const ACCEPTED_EXTENSIONS: [&'static str; 1] = ["gaml"];
}

impl GamlScanner {
    pub fn new() -> Self {
        Self::default()
    }

    fn read_start<T: Seek + Read + 'static>(
        &self,
        input: &mut T,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let len = input.seek(SeekFrom::End(0))?;
        input.seek(SeekFrom::Start(0))?;
        let len = cmp::min(len, 64);
        let mut buf = vec![0; len as usize];
        input.read_exact(&mut buf)?;

        Ok(buf)
    }
}

impl<T: Seek + Read + 'static> Scanner<T> for GamlScanner {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        if !is_recognized_extension(path, &Self::ACCEPTED_EXTENSIONS) {
            return false;
        };

        match self.read_start(input) {
            Err(_) => false,
            Ok(bytes) => {
                let str = String::from_utf8_lossy(&bytes);
                str.contains("GAML")
            }
        }
    }

    fn get_reader(&self, path: &str, input: T) -> Result<Box<dyn Reader>, Box<dyn Error>> {
        let input_seek_read: Box<dyn SeekRead> = Box::new(input);
        let gaml = GamlParser::parse(path, input_seek_read)?;
        Ok(Box::new(GamlReader::new(path, gaml)))
    }
}
