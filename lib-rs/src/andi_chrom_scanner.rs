use crate::{
    andi_chrom_parser::AndiChromParser,
    andi_chrom_reader::AndiChromReader,
    api::{Parser, Scanner},
};
use std::{
    error::Error,
    io::{Read, Seek},
    path::Path,
};

pub struct AndiChromScanner {}

impl AndiChromScanner {
    const ACCEPTED_EXTENSIONS: [&str; 2] = ["cdf", "nc"];
    const MAGIC_BYTES: [u8; 3] = [0x43, 0x44, 0x46]; // "CDF"
}

impl<T: Seek + Read + 'static> Scanner<T> for AndiChromScanner {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        let p = Path::new(path);
        let extension = p
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| Some(ext.to_lowercase()));
        match extension {
            None => return false,
            Some(ext) => {
                let is_recognized_extension = Self::ACCEPTED_EXTENSIONS
                    .iter()
                    .any(|accept_ext| *accept_ext == ext);
                if !is_recognized_extension {
                    return false;
                }
            }
        }

        // recognized extension => check first few bytes ("magic bytes")

        let mut buf = [0u8; 3];
        let read_success = input.read_exact(&mut buf);
        if read_success.is_err() {
            return false;
        }

        buf.as_slice() == Self::MAGIC_BYTES
    }

    fn get_reader(
        &self,
        path: &str,
        input: T,
    ) -> Result<Box<dyn crate::api::Reader>, Box<dyn Error>> {
        let file = AndiChromParser::parse(path, input)?;
        Ok(Box::new(AndiChromReader::new(file)))
    }
}
