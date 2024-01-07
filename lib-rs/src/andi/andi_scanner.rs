#[cfg(target_family = "wasm")]
use crate::common::{BlobWrapper, JsReader};
use crate::{
    api::{Reader, Scanner},
    utils::{add_scanner_js, is_recognized_extension},
};
use std::{
    error::Error,
    io::{Read, Seek},
};
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_family = "wasm")]
use wasm_bindgen::JsError;
#[cfg(target_family = "wasm")]
use web_sys::Blob;

use super::{
    andi_chrom_parser::AndiChromParser, andi_chrom_reader::AndiChromReader,
    andi_ms_parser::AndiMsParser, andi_ms_reader::AndiMsReader, AndiError,
};

#[wasm_bindgen]
#[derive(Default)]
pub struct AndiScanner {}

impl AndiScanner {
    const ACCEPTED_EXTENSIONS: [&'static str; 2] = ["cdf", "nc"];
    const MAGIC_BYTES: [u8; 3] = [0x43, 0x44, 0x46]; // "CDF"
    const AIA_TEMPLATE_REVISION_ATTR: &'static str = "aia_template_revision";
    const MS_TEMPLATE_REVISION_ATTR: &'static str = "ms_template_revision";
}

#[wasm_bindgen]
impl AndiScanner {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}

add_scanner_js!(AndiScanner);

impl<T: Seek + Read + 'static> Scanner<T> for AndiScanner {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        if !is_recognized_extension(path, &Self::ACCEPTED_EXTENSIONS) {
            return false;
        }

        // recognized extension => check first few bytes ("magic bytes")
        let mut buf = [0u8; 3];
        let read_success = input.read_exact(&mut buf);
        if read_success.is_err() {
            return false;
        }

        buf.as_slice() == Self::MAGIC_BYTES
    }

    fn get_reader(&self, path: &str, input: T) -> Result<Box<dyn Reader>, Box<dyn Error>> {
        let input_seek_read = Box::new(input);
        let cdf_reader = netcdf3::FileReader::open_seek_read(path, input_seek_read)?;

        if cdf_reader
            .data_set()
            .has_global_attr(Self::AIA_TEMPLATE_REVISION_ATTR)
        {
            let file = AndiChromParser::parse_cdf(cdf_reader)?;
            return Ok(Box::new(AndiChromReader::new(path, file)));
        }
        if cdf_reader
            .data_set()
            .has_global_attr(Self::MS_TEMPLATE_REVISION_ATTR)
        {
            let file = AndiMsParser::parse_cdf(cdf_reader)?;
            return Ok(Box::new(AndiMsReader::new(path, file)));
        }

        Err(AndiError::new(&format!(
            "Could not parse \"{}\". Expected one attribute of: {}, {}",
            path,
            Self::AIA_TEMPLATE_REVISION_ATTR,
            Self::MS_TEMPLATE_REVISION_ATTR
        ))
        .into())
    }
}
