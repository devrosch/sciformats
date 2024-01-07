pub(crate) fn is_recognized_extension(path: &str, accepted_extensions: &[&str]) -> bool {
    let p = Path::new(path);
    let extension = p
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());
    match extension {
        None => false,
        Some(ext) => {
            let is_recognized_extension = accepted_extensions
                .iter()
                .any(|accept_ext| *accept_ext == ext);
            is_recognized_extension
        }
    }
}

// -------------------------------------------------
// WASM specific
// -------------------------------------------------

macro_rules! add_scanner_js {
    ($scanner_name:ident) => {
        #[wasm_bindgen]
        impl $scanner_name {
            #[cfg(target_family = "wasm")]
            #[wasm_bindgen(js_name = isRecognized)]
            pub fn js_is_recognized(&self, path: &str, input: &Blob) -> bool {
                let mut blob = BlobWrapper::new(input.clone());
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
    };
}
use std::path::Path;

pub(crate) use add_scanner_js;

macro_rules! add_reader_js {
    ($struct_name:ident) => {
        #[cfg(target_family = "wasm")]
        #[wasm_bindgen]
        impl $struct_name {
            #[wasm_bindgen(js_name = read)]
            pub fn js_read(&self, path: &str) -> Result<Node, JsError> {
                let read_result = Reader::read(self, path);
                match read_result {
                    Ok(node) => Ok(node),
                    Err(error) => Err(JsError::new(&error.to_string())),
                }
            }
        }
    };
}
pub(crate) use add_reader_js;
