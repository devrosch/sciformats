#[cfg(target_family = "wasm")]
use crate::common::{BlobWrapper, JsReader};
use crate::api::{Reader, Scanner};
use std::{
    error::Error,
    io::{Read, Seek},
    path::Path,
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
pub struct AndiScanner {}

impl AndiScanner {
    const ACCEPTED_EXTENSIONS: [&str; 2] = ["cdf", "nc"];
    const MAGIC_BYTES: [u8; 3] = [0x43, 0x44, 0x46]; // "CDF"
    const AIA_TEMPLATE_REVISION_ATTR: &str = "aia_template_revision";
    const MS_TEMPLATE_REVISION_ATTR: &str = "ms_template_revision";
}

impl Default for AndiScanner {
    fn default() -> Self {
        AndiScanner::new()
    }
}

#[wasm_bindgen]
impl AndiScanner {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AndiScanner {
        AndiScanner {}
    }

    #[cfg(target_family = "wasm")]
    #[wasm_bindgen(js_name = isRecognized)]
    pub fn js_is_recognized(&self, path: &str, input: &Blob) -> bool {
        use web_sys::console;

        let mut blob = BlobWrapper::new(input.clone());

        console::log_2(&"AndiScanner.js_is_recognized() path:".into(), &path.into());
        console::log_2(
            &"AndiScanner.js_is_recognized() input pos:".into(),
            &blob.get_pos().into(),
        );

        Scanner::is_recognized(self, path, &mut blob)
    }

    #[cfg(target_family = "wasm")]
    #[wasm_bindgen(js_name = getReader)]
    pub fn js_get_reader(&self, path: &str, input: &Blob) -> Result<JsReader, JsError> {
        let blob = BlobWrapper::new(input.clone());
        let reader_result = self.get_reader(path, blob);
        match reader_result {
            Ok(reader) => Ok(JsReader::new(reader)),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}

impl<T: Seek + Read + 'static> Scanner<T> for AndiScanner {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        let p = Path::new(path);
        let extension = p
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());
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
