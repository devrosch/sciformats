use crate::andi::andi_scanner::AndiScanner;
#[cfg(target_family = "wasm")]
use crate::api::Node;
use crate::api::{Reader, Scanner, SfError};
#[cfg(target_family = "wasm")]
use js_sys::Uint8Array;
use std::io::{BufReader, SeekFrom};
use std::{
    error::Error,
    io::{Read, Seek},
};
#[cfg(target_family = "wasm")]
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};
#[cfg(target_family = "wasm")]
use web_sys::{Blob, FileReaderSync};

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub struct JsScannerRepository {
    repo: ScannerRepository,
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
        let andi_scanner: Box<dyn Scanner<Box<dyn SeekRead>>> = Box::new(AndiScanner::new());
        let scanners: Vec<Box<dyn Scanner<Box<dyn SeekRead>>>> = vec![andi_scanner];
        ScannerRepository { scanners }
    }

    pub fn push(&mut self, scanner: Box<dyn Scanner<Box<dyn SeekRead>>>) {
        self.scanners.push(scanner)
    }

    pub fn is_recognized(&self, path: &str, input: &mut Box<dyn SeekRead>) -> bool {
        self.scanners
            .iter()
            .any(|scanner| scanner.is_recognized(path, input))
    }

    pub fn get_reader(
        &self,
        path: &str,
        mut input: Box<dyn SeekRead>,
    ) -> Result<Box<dyn Reader>, Box<dyn Error>> {
        for scanner in &self.scanners {
            input.seek(SeekFrom::Start(0))?;
            if scanner.is_recognized(path, &mut input) {
                input.seek(SeekFrom::Start(0))?;
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

impl Default for ScannerRepository {
    fn default() -> Self {
        ScannerRepository::new(vec![])
    }
}

// -------------------------------------------------
// Wrappers
// -------------------------------------------------

pub struct SeekReadWrapper<T: Seek + Read> {
    input: BufReader<T>,
    pos: u64,
}

impl<T: Seek + Read> SeekReadWrapper<T> {
    pub fn new(raw_input: T) -> Self {
        SeekReadWrapper {
            input: BufReader::new(raw_input),
            pos: 0,
        }
    }
}

impl<T: Seek + Read> Seek for SeekReadWrapper<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Current(offset) => {
                self.input.seek_relative(offset)?;
                // todo: handle error cases
                self.pos = (self.pos as i64 + offset) as u64;
                Ok(self.pos)
            }
            SeekFrom::Start(offset) => {
                // todo: handle error cases
                let rel_offset = (offset as i64) - (self.pos as i64);
                self.input.seek_relative(rel_offset)?;
                self.pos = offset;
                Ok(self.pos)
            }
            SeekFrom::End(offset) => {
                // todo: make more efficient
                self.pos = self.input.seek(SeekFrom::End(offset))?;
                Ok(self.pos)
            }
        }
    }
}

impl<T: Seek + Read> Read for SeekReadWrapper<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let num_read = self.input.read(buf)?;
        self.pos += num_read as u64;
        Ok(num_read)
    }
}

// -------------------------------------------------
// WASM specific
// -------------------------------------------------

#[cfg(target_family = "wasm")]
#[derive(Clone)]
pub struct BlobWrapper {
    blob: Blob,
    pos: u64,
}

#[cfg(target_family = "wasm")]
impl BlobWrapper {
    pub fn new(blob: Blob) -> BlobWrapper {
        BlobWrapper { blob, pos: 0 }
    }

    pub fn get_pos(&self) -> u64 {
        self.pos
    }
}

#[cfg(target_family = "wasm")]
impl Seek for BlobWrapper {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        fn to_oob_error<T>(pos: i64) -> std::io::Result<T> {
            // use web_sys::console;
            // console::error_1(&format!("I/O error. Seek position out of bounds: {pos}").into());
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Seek position out of bounds: {pos}"),
            ))
        }

        let file_size = self.blob.size() as u64;
        match pos {
            SeekFrom::Start(seek_pos) => {
                self.pos = seek_pos;
            }
            SeekFrom::End(seek_pos) => {
                let new_pos = file_size as i64 + seek_pos;
                if new_pos < 0 {
                    return to_oob_error(new_pos);
                }
                self.pos = new_pos as u64;
            }
            SeekFrom::Current(seek_pos) => {
                let new_pos = self.pos as i64 + seek_pos;
                if new_pos < 0 {
                    return to_oob_error(new_pos);
                }
                self.pos = new_pos as u64;
            }
        }
        Ok(self.pos)
    }
}

#[cfg(target_family = "wasm")]
impl Read for BlobWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        fn to_io_error<T>(js_error: JsValue) -> std::io::Result<T> {
            // use web_sys::console;
            // console::error_1(&format!("I/O error: {:?}", js_error).into());
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", js_error),
            ))
        }

        let end_pos = self.pos + buf.len() as u64;
        let result = self
            .blob
            .slice_with_f64_and_f64(self.pos as f64, end_pos as f64);
        match result {
            Ok(slice) => {
                self.pos += slice.size() as u64;
                let reader = match FileReaderSync::new() {
                    Ok(frs) => frs,
                    Err(err) => return to_io_error(err),
                };
                let array_buffer = match reader.read_as_array_buffer(&slice) {
                    Ok(buf) => buf,
                    Err(err) => return to_io_error(err),
                };
                // see: https://stackoverflow.com/questions/67464060/converting-jsvalue-to-vecu8
                let uint8_array = Uint8Array::new(&array_buffer);
                uint8_array.copy_to(&mut buf[0..slice.size() as usize]);
                Ok(slice.size() as usize)
            }
            Err(js_error) => to_io_error(js_error),
        }
    }
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub struct JsReader {
    reader: Box<dyn Reader>,
}

#[cfg(target_family = "wasm")]
impl JsReader {
    pub fn new(reader: Box<dyn Reader>) -> Self {
        JsReader { reader }
    }
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
impl JsReader {
    pub fn read(&self, path: &str) -> Result<Node, JsError> {
        let read_result = self.reader.read(path);
        match read_result {
            Ok(node) => Ok(node),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
impl JsScannerRepository {
    #[wasm_bindgen(constructor)]
    pub fn init_all() -> JsScannerRepository {
        let repo = ScannerRepository::init_all();
        JsScannerRepository { repo }
    }

    #[cfg(target_family = "wasm")]
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

    #[cfg(target_family = "wasm")]
    #[wasm_bindgen(js_name = getReader)]
    pub fn js_get_reader(&self, path: &str, input: &Blob) -> Result<JsReader, JsError> {
        let blob = BlobWrapper::new(input.clone());
        let input = SeekReadWrapper::new(blob);
        let reader_result = self.repo.get_reader(path, Box::new(input));
        match reader_result {
            Ok(reader) => Ok(JsReader::new(reader)),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_family = "wasm")]
    use crate::common::BlobWrapper;
    use crate::{
        api::{Node, Reader, Scanner, SfError},
        common::SeekRead,
    };
    #[cfg(target_family = "wasm")]
    use js_sys::{Array, Uint8Array};
    #[cfg(target_family = "wasm")]
    use std::io::SeekFrom;
    use std::io::{Cursor, Read, Seek};
    #[cfg(target_family = "wasm")]
    use wasm_bindgen_test::wasm_bindgen_test;
    #[cfg(target_family = "wasm")]
    use web_sys::Blob;

    use super::ScannerRepository;

    // see: https://github.com/rustwasm/wasm-bindgen/issues/3340
    #[cfg(target_family = "wasm")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_worker);
    // wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    struct StubReader {
        pub name: String,
        node_ok: bool,
    }
    impl Reader for StubReader {
        fn read(&self, _path: &str) -> Result<crate::api::Node, Box<dyn std::error::Error>> {
            match self.node_ok {
                true => Ok(Node {
                    name: self.name.clone(),
                    parameters: vec![],
                    data: vec![],
                    metadata: vec![],
                    table: None,
                    child_node_names: vec![],
                }),
                _ => Err(SfError::new("Error"))?,
            }
        }
    }

    struct StubScanner {
        recognized: bool,
        reader_name: Option<String>,
    }
    impl<T: Seek + Read> Scanner<T> for StubScanner {
        fn is_recognized(&self, _path: &str, _input: &mut T) -> bool {
            self.recognized
        }

        fn get_reader(
            &self,
            _path: &str,
            _input: T,
        ) -> Result<Box<dyn crate::api::Reader>, Box<dyn std::error::Error>> {
            match &self.reader_name {
                Some(name) => Ok(Box::new(StubReader {
                    name: name.to_owned(),
                    node_ok: true,
                })),
                None => Err(SfError::new("Error"))?,
            }
        }
    }

    #[test]
    fn scanner_repository_recognizes_data_if_any_scanner_recognizes() {
        let scanner_non_recognizing_0 = StubScanner {
            recognized: false,
            reader_name: None,
        };
        let scanner_recognizing_1 = StubScanner {
            recognized: true,
            reader_name: Some("1".to_owned()),
        };
        let scanner_non_recognizing_2 = StubScanner {
            recognized: false,
            reader_name: None,
        };
        let mut input: Box<dyn SeekRead> = Box::new(Cursor::new("abc"));
        let mut repo = ScannerRepository::default();
        repo.push(Box::new(scanner_non_recognizing_0));

        assert!(!repo.is_recognized("a/b/c", &mut input));

        repo.push(Box::new(scanner_recognizing_1));
        repo.push(Box::new(scanner_non_recognizing_2));
        assert!(repo.is_recognized("a/b/c", &mut input));
    }

    #[test]
    fn scanner_repository_returns_first_applicable_scanner() {
        let scanner_non_recognizing_0 = Box::new(StubScanner {
            recognized: false,
            reader_name: None,
        });
        let scanner_recognizing_1 = Box::new(StubScanner {
            recognized: true,
            reader_name: Some("1".to_owned()),
        });
        let scanner_recognizing_2 = Box::new(StubScanner {
            recognized: true,
            reader_name: Some("2".to_owned()),
        });
        let mut input: Box<dyn SeekRead> = Box::new(Cursor::new("abc"));
        let repo = ScannerRepository::new(vec![
            scanner_non_recognizing_0,
            scanner_recognizing_1,
            scanner_recognizing_2,
        ]);

        assert!(repo.is_recognized("a/b/c", &mut input));
        let reader_result = repo.get_reader("path", input).unwrap();
        assert_eq!("1", reader_result.read("").unwrap().name);
    }

    #[test]
    fn scanner_repository_returns_error_if_no_applicable_scanner() {
        let input: Box<dyn SeekRead> = Box::new(Cursor::new("abc"));
        let repo = ScannerRepository::new(vec![]);

        let reader_result = repo.get_reader("path", input);
        assert!(reader_result.is_err());
    }

    // no #[test] as this test cannot run outside a browser engine
    #[cfg(target_family = "wasm")]
    #[wasm_bindgen_test]
    async fn blob_wrapper_mimicks_std_seek_read_behavior() {
        let arr: [u8; 3] = [1, 2, 3];
        let js_arr = Array::new();
        // see: https://github.com/rustwasm/wasm-bindgen/issues/1693
        js_arr.push(&Uint8Array::from(arr.as_slice()));
        let blob = Blob::new_with_u8_array_sequence(&js_arr).unwrap();
        assert_eq!(3, blob.size() as u64);
        // use web_sys::console;
        // console::log_1(&format!("arr: {:?}", arr).into());
        let mut blob_wrapper = BlobWrapper::new(blob);
        let mut buf = [0u8; 3];

        // read whole blob
        let read_len = blob_wrapper.read(&mut buf).unwrap();
        assert_eq!(3, read_len);
        assert_eq!(arr, buf);

        // read past end
        buf.fill(0);
        let pos = blob_wrapper.seek(SeekFrom::Start(1)).unwrap();
        assert_eq!(1, pos);
        let read_len = blob_wrapper.read(&mut buf).unwrap();
        assert_eq!(2, read_len);
        assert_eq!([2, 3, 0], buf);

        // seek beyond end and read
        buf.fill(0);
        let pos = blob_wrapper.seek(SeekFrom::Start(10)).unwrap();
        assert_eq!(10, pos);
        let read_len = blob_wrapper.read(&mut buf).unwrap();
        assert_eq!(0, read_len);
        assert_eq!([0, 0, 0], buf);

        // seek to negative position
        let pos = blob_wrapper.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(0, pos);
        let seek_err = blob_wrapper.seek(SeekFrom::Current(-1)).unwrap_err();
        assert_eq!(std::io::ErrorKind::InvalidInput, seek_err.kind());
        assert_eq!("Seek position out of bounds: -1", seek_err.to_string());
    }
}
