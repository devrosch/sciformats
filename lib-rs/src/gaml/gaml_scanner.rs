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
    const MAGIC_BYTES: &'static [u8; 4] = b"GAML";
    const NUM_START_BYTES: u64 = 128;
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
        let len = cmp::min(len, Self::NUM_START_BYTES);
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

        // start of file contains magic bytes "GAML"?
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
        let input_seek_read: Box<dyn SeekRead> = Box::new(input);
        let gaml = GamlParser::parse(path, input_seek_read)?;
        Ok(Box::new(GamlReader::new(path, gaml)))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn accepts_valid_gaml() {
        let path = "valid.gaml";
        let gaml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                            <GAML version=\"1.20\" name=\"Gaml test file\"></GAML>";
        let mut reader = Cursor::new(gaml);
        let scanner = GamlScanner::new();

        assert_eq!(true, scanner.is_recognized(path, &mut reader));
    }

    #[test]
    fn accepts_valid_gaml_upper_case_extension() {
        let path = "valid.GAML";
        let gaml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                            <GAML version=\"1.20\" name=\"Gaml test file\"></GAML>";
        let mut reader = Cursor::new(gaml);
        let scanner = GamlScanner::new();

        assert_eq!(true, scanner.is_recognized(path, &mut reader));
    }

    #[test]
    fn rejects_invalid_extension() {
        let path = "invalid.notgaml";
        let gaml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                            <GAML version=\"1.20\" name=\"Gaml test file\"></GAML>";
        let mut reader = Cursor::new(gaml);
        let scanner = GamlScanner::new();

        assert_eq!(false, scanner.is_recognized(path, &mut reader));
    }

    #[test]
    fn rejects_invalid_content() {
        let path = "invalid.gaml";
        let gaml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                            <some><other><xml>content</xml></other></some>";
        let mut reader = Cursor::new(gaml);
        let scanner = GamlScanner::new();

        assert_eq!(false, scanner.is_recognized(path, &mut reader));
    }

    #[test]
    fn provides_reader_for_valid_gaml() {
        let path = "valid.gaml";
        let gaml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                            <GAML version=\"1.20\" name=\"Gaml test file\"></GAML>";
        let reader = Cursor::new(gaml);
        let scanner = GamlScanner::new();

        assert!(scanner.get_reader(path, reader).is_ok());
    }
}
