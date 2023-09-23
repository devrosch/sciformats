use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_family = "wasm")]
use wasm_bindgen::JsValue;

use crate::{
    andi_chrom_parser::AndiChromParser,
    andi_chrom_reader::AndiChromReader,
    api::{Parser, Scanner},
};
#[cfg(target_family = "wasm")]
use crate::{api::Node, FileWrapper};
use std::{
    error::Error,
    io::{Read, Seek},
    path::Path,
};

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub struct JsReader {
    reader: Box<dyn crate::api::Reader>,
}

#[cfg(target_family = "wasm")]
impl JsReader {
    pub fn new(reader: Box<dyn crate::api::Reader>) -> Self {
        JsReader { reader }
    }
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
impl JsReader {
    pub fn read(&self, path: &str) -> Result<Node, JsValue> {
        let read_result = self.reader.read(path);
        match read_result {
            Ok(node) => Ok(node),
            Err(error) => Err(error.to_string().into()),
        }
    }
}

#[wasm_bindgen]
pub struct AndiChromScanner {}

impl AndiChromScanner {
    const ACCEPTED_EXTENSIONS: [&str; 2] = ["cdf", "nc"];
    const MAGIC_BYTES: [u8; 3] = [0x43, 0x44, 0x46]; // "CDF"
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
impl AndiChromScanner {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AndiChromScanner {
        AndiChromScanner {}
    }

    pub fn js_is_recognized(&self, path: &str, input: &mut FileWrapper) -> bool {
        use web_sys::console;

        console::log_2(
            &"AndiChromScanner.js_is_recognized() path:".into(),
            &path.into(),
        );
        console::log_2(
            &"AndiChromScanner.js_is_recognized() input pos:".into(),
            &input.pos.into(),
        );
        console::log_2(
            &"AndiChromScanner.js_is_recognized() input file:".into(),
            &input.file,
        );

        Scanner::is_recognized(self, path, input)
    }

    pub fn js_get_reader(&self, path: &str, input: FileWrapper) -> Result<JsReader, JsValue> {
        let reader_result = self.get_reader(path, input);
        match reader_result {
            Ok(reader) => Ok(JsReader { reader }),
            Err(error) => Err(error.to_string().into()),
        }
    }
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
        Ok(Box::new(AndiChromReader::new(path, file)))
    }
}
