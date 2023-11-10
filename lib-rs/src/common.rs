#[cfg(target_family = "wasm")]
use crate::api::{BlobWrapper, JsReader};
use crate::{
    andi::{andi_chrom_scanner::AndiChromScanner, andi_ms_scanner::AndiMsScanner},
    api::Scanner,
};
use std::{
    error::Error,
    fmt,
    io::{Read, Seek},
};
#[cfg(target_family = "wasm")]
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
#[cfg(target_family = "wasm")]
use web_sys::Blob;

#[derive(Debug, PartialEq)]
pub struct SfError {
    message: String,
}

impl SfError {
    pub fn new(msg: &str) -> SfError {
        SfError {
            message: msg.into(),
        }
    }
}

impl Error for SfError {}

impl fmt::Display for SfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub struct JsScannerRepository {
    repo: ScannerRepository<BlobWrapper>,
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
impl JsScannerRepository {
    #[wasm_bindgen(constructor)]
    pub fn init_all() -> JsScannerRepository {
        let repo = ScannerRepository::<BlobWrapper>::init_all();
        JsScannerRepository { repo }
    }

    #[cfg(target_family = "wasm")]
    #[wasm_bindgen(js_name = isRecognized)]
    pub fn js_is_recognized(&self, path: &str, input: &Blob) -> bool {
        use web_sys::console;

        let mut blob = BlobWrapper::new(input.clone());

        console::log_2(
            &"JsScannerRepository.js_is_recognized() path:".into(),
            &path.into(),
        );
        console::log_2(
            &"JsScannerRepository.js_is_recognized() input pos:".into(),
            &blob.get_pos().into(),
        );

        self.repo.is_recognized(path, &mut blob)
    }

    #[cfg(target_family = "wasm")]
    #[wasm_bindgen(js_name = getReader)]
    pub fn js_get_reader(&self, path: &str, input: &Blob) -> Result<JsReader, JsError> {
        let blob = BlobWrapper::new(input.clone());
        let reader_result = self.repo.get_reader(path, blob);
        match reader_result {
            Ok(reader) => Ok(JsReader::new(reader)),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}

pub struct ScannerRepository<T> {
    scanners: Vec<Box<dyn Scanner<T>>>,
}

impl<T: Seek + Read + 'static> ScannerRepository<T> {
    pub fn new(scanners: Vec<Box<dyn Scanner<T>>>) -> ScannerRepository<T> {
        ScannerRepository { scanners }
    }

    pub fn init_all() -> ScannerRepository<T> {
        let scanners: Vec<Box<dyn Scanner<T>>> = vec![
            Box::new(AndiChromScanner::new()),
            Box::new(AndiMsScanner::new()),
        ];
        ScannerRepository { scanners }
    }
}

impl<T: Seek + Read + 'static> Default for ScannerRepository<T> {
    fn default() -> Self {
        ScannerRepository::new(vec![])
    }
}

impl<T: Seek + Read + Clone + 'static> Scanner<T> for ScannerRepository<T> {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        self.scanners
            .iter()
            .any(|scanner| scanner.is_recognized(path, input))
    }

    fn get_reader(
        &self,
        path: &str,
        mut input: T,
    ) -> Result<Box<dyn crate::api::Reader>, Box<dyn std::error::Error>> {
        let mut error_messages = Vec::<String>::new();
        for scanner in &self.scanners {
            input.seek(std::io::SeekFrom::Start(0))?;
            if scanner.is_recognized(path, &mut input) {
                let result = scanner.get_reader(path, input.clone());
                if let Ok(reader) = result {
                    return Ok(reader);
                }
                let error_message = result.err().unwrap().to_string();
                error_messages.push(error_message);
            }
        }

        Err(SfError::new(&format!(
            "No reader can be initialized for file: {}. Error message(s): {}",
            path,
            error_messages.join("\n")
        )))?
    }
}
