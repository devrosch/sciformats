use crate::{
    andi::{
        andi_chrom_reader::AndiChromReader, andi_ms_reader::AndiMsReader, andi_scanner::AndiScanner,
    },
    api::{Reader, Scanner},
    bind::wasm::{BlobWrapper, JsNode, JsReader},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use web_sys::Blob;

#[wasm_bindgen(js_name = AndiScanner)]
// #[wasm_bindgen]
struct JsAndiScanner {
    scanner: AndiScanner,
}

#[wasm_bindgen(js_class = AndiScanner)]
// #[wasm_bindgen]
impl JsAndiScanner {
    #[wasm_bindgen(constructor)]
    pub fn js_new() -> Self {
        Self {
            scanner: AndiScanner::default(),
        }
    }
}

#[wasm_bindgen(js_class = AndiScanner)]
// #[wasm_bindgen]
impl JsAndiScanner {
    #[wasm_bindgen(js_name = isRecognized)]
    pub fn js_is_recognized(&self, path: &str, input: &Blob) -> bool {
        let mut blob = BlobWrapper::new(input.clone());
        self.scanner.is_recognized(path, &mut blob)
    }

    #[wasm_bindgen(js_name = getReader)]
    pub fn js_get_reader(&self, path: &str, input: &Blob) -> Result<JsReader, JsError> {
        let blob = BlobWrapper::new(input.clone());
        let reader_result = self.scanner.get_reader(path, blob);
        match reader_result {
            Ok(reader) => Ok(JsReader::new(reader)),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}

#[wasm_bindgen]
struct JsAndiChromReader {
    reader: AndiChromReader,
}

#[wasm_bindgen]
impl JsAndiChromReader {
    #[wasm_bindgen(js_name = read)]
    pub fn js_read(&self, path: &str) -> Result<JsNode, JsError> {
        let read_result = self.reader.read(path);
        match read_result {
            Ok(node) => Ok(node.into()),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}

#[wasm_bindgen]
struct JsAndiMsReader {
    reader: AndiMsReader,
}

#[wasm_bindgen]
impl JsAndiMsReader {
    #[wasm_bindgen(js_name = read)]
    pub fn js_read(&self, path: &str) -> Result<JsNode, JsError> {
        let read_result = self.reader.read(path);
        match read_result {
            Ok(node) => Ok(node.into()),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}
