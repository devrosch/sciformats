use crate::{
    api::{Reader, Scanner},
    bind::wasm::{BlobWrapper, JsNode, JsReader},
    spc::{spc_reader::{SpcReaderNewFormat, SpcReaderOldFormat}, spc_scanner::SpcScanner},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use web_sys::Blob;

#[wasm_bindgen(js_name = SpcScanner)]
struct JsSpcScanner {
    scanner: SpcScanner,
}

#[wasm_bindgen]
impl JsSpcScanner {
    #[wasm_bindgen(constructor)]
    pub fn js_new() -> Self {
        Self {
            scanner: SpcScanner::default(),
        }
    }
}

#[wasm_bindgen]
impl JsSpcScanner {
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
struct JsSpcReaderNewFormat {
    reader: SpcReaderNewFormat,
}

#[wasm_bindgen]
impl JsSpcReaderNewFormat {
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
struct JsSpcReaderOldFormat {
    reader: SpcReaderOldFormat,
}

#[wasm_bindgen]
impl JsSpcReaderOldFormat {
    #[wasm_bindgen(js_name = read)]
    pub fn js_read(&self, path: &str) -> Result<JsNode, JsError> {
        let read_result = self.reader.read(path);
        match read_result {
            Ok(node) => Ok(node.into()),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}
