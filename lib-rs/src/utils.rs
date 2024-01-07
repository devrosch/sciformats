// -------------------------------------------------
// Util functions
// -------------------------------------------------

/// Check if path ends with one of the accepted extensions
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

/// Convert UTF-8 C string to String
#[allow(dead_code)]
pub(crate) fn convert_utf8_cstr_to_str(bytes: &[u8]) -> String {
    let len = bytes.iter().position(|&c| c == 0u8).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..len]).to_string()
}

/// Convert ISO 8859-1 C string to String
pub(crate) fn from_iso_8859_1_cstr(bytes: &[u8]) -> String {
    // see https://stackoverflow.com/a/28175593 for why this works
    bytes
        .iter()
        .take_while(|&c| c != &0u8)
        .map(|&c| c as char)
        .collect()
}

/// Parse N zero terminated ISO 8859-1 strings each with a maximum (always consumed) length up to str_size.
#[allow(dead_code)]
pub(crate) fn from_iso_8859_1_fixed_size_cstr_arr<const N: usize>(
    bytes: &[u8],
    str_size: usize,
) -> [String; N] {
    let mut v: Vec<String> = vec![];
    for i in (0..(N * str_size)).step_by(str_size) {
        let slice = &bytes[i..(i + str_size)];
        let s = from_iso_8859_1_cstr(slice);
        v.push(s);
    }
    v.try_into().unwrap()
}

/// Parse N zero terminated ISO 8859-1 strings of variable length. If additional strings exist they are discarded.
pub(crate) fn from_iso_8859_1_cstr_arr<const N: usize>(bytes: &[u8]) -> Option<[String; N]> {
    let split: Vec<&[u8]> = bytes.split(|byte| *byte == 0).collect();
    if split.len() < N {
        return None;
    }
    let vec: Vec<String> = split
        .iter()
        .take(N)
        .map(|slice| from_iso_8859_1_cstr(slice))
        .collect();
    let res: [String; N] = vec.try_into().unwrap();
    Some(res)
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
