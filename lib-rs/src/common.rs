// #[cfg(target_family = "wasm")]
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
// #[cfg(target_family = "wasm")]
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
// #[cfg(target_family = "wasm")]
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

// #[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub struct JsScannerRepository {
    repo: ScannerRepository,
}

// #[cfg(target_family = "wasm")]
#[wasm_bindgen]
impl JsScannerRepository {
    #[wasm_bindgen(constructor)]
    pub fn init_all() -> JsScannerRepository {
        let repo = ScannerRepository::init_all();
        JsScannerRepository { repo }
    }

    // #[cfg(target_family = "wasm")]
    #[wasm_bindgen(js_name = isRecognized)]
    pub fn js_is_recognized(&self, path: &str, input: &Blob) -> bool {
        use web_sys::console;
        let blob = Box::new(BlobWrapper::new(input.clone()));
        console::log_2(
            &"JsScannerRepository.js_is_recognized() path:".into(),
            &path.into(),
        );
        console::log_2(
            &"JsScannerRepository.js_is_recognized() input pos:".into(),
            &blob.get_pos().into(),
        );

        self.repo
            .is_recognized(path, &mut (blob as Box<dyn SeekRead>))
    }

    // #[cfg(target_family = "wasm")]
    #[wasm_bindgen(js_name = getReader)]
    pub fn js_get_reader(&self, path: &str, input: &Blob) -> Result<JsReader, JsError> {
        let blob = BlobWrapper::new(input.clone());
        let reader_result = self.repo.get_reader(path, Box::new(blob));
        match reader_result {
            Ok(reader) => Ok(JsReader::new(reader)),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}

pub trait SeekRead: Seek + Read {}
impl<T: Seek + Read> SeekRead for T {}

pub struct ScannerRepository {
    scanners: Vec<Box<dyn Scanner<Box<dyn SeekRead>>>>,
}

impl ScannerRepository {
    pub fn new(scanners: Vec<Box<dyn Scanner<Box<dyn SeekRead>>>>) -> ScannerRepository {
        ScannerRepository { scanners }
    }

    pub fn init_all() -> ScannerRepository {
        let chrom_box: Box<dyn Scanner<Box<dyn SeekRead>>> = Box::new(AndiChromScanner::new());
        let ms_box: Box<dyn Scanner<Box<dyn SeekRead>>> = Box::new(AndiMsScanner::new());

        let scanners: Vec<Box<dyn Scanner<Box<dyn SeekRead>>>> = vec![chrom_box, ms_box];

        ScannerRepository { scanners }
    }

    pub fn push(&mut self, scanner: Box<dyn Scanner<Box<dyn SeekRead>>>) {
        (&mut self.scanners).push(scanner)
    }
}

impl Default for ScannerRepository {
    fn default() -> Self {
        ScannerRepository::new(vec![])
    }
}

impl ScannerRepository {
    pub fn is_recognized(&self, path: &str, input: &mut Box<dyn SeekRead>) -> bool {
        self.scanners
            .iter()
            .any(|scanner| scanner.is_recognized(path, input))
    }

    pub fn get_reader(
        &self,
        path: &str,
        mut input: Box<dyn SeekRead>,
    ) -> Result<Box<dyn crate::api::Reader>, Box<dyn std::error::Error>> {
        for scanner in &self.scanners {
            input.seek(std::io::SeekFrom::Start(0))?;
            if scanner.is_recognized(path, &mut input) {
                input.seek(std::io::SeekFrom::Start(0))?;
                // TODO: find a way to still try other recognizing readers in case of error
                let reader = scanner.get_reader(path, input)?;
                return Ok(reader);
            }
        }

        Err(SfError::new(&format!(
            "No reader can be initialized for file: {}",
            path,
        )))?
    }
}
