use crate::andi::andi_scanner::AndiScanner;
use crate::api::{Reader, Scanner};
use crate::spc::spc_scanner::SpcScanner;
use std::fmt;
use std::io::{BufReader, SeekFrom};
use std::{
    error::Error,
    io::{Read, Seek},
};

pub trait SeekRead: Seek + Read {}
impl<T: Seek + Read> SeekRead for T {}

#[derive(Debug)]
pub struct SfError {
    message: String,
    source: Option<Box<dyn Error>>,
}

/// A generic error.
impl SfError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.into(),
            source: None,
        }
    }

    pub fn from_source(source: Box<dyn Error>, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: Some(source),
        }
    }
}

impl Error for SfError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|b| b.as_ref())
    }
}

impl fmt::Display for SfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct ScannerRepository {
    scanners: Vec<Box<dyn Scanner<Box<dyn SeekRead>>>>,
}

impl ScannerRepository {
    pub fn new(scanners: Vec<Box<dyn Scanner<Box<dyn SeekRead>>>>) -> ScannerRepository {
        ScannerRepository { scanners }
    }

    pub fn init_all() -> ScannerRepository {
        let andi_scanner: Box<dyn Scanner<Box<dyn SeekRead>>> = Box::new(AndiScanner::new());
        let spc_scanner = Box::new(SpcScanner::new());
        let scanners: Vec<Box<dyn Scanner<Box<dyn SeekRead>>>> = vec![andi_scanner, spc_scanner];
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{Node, Reader, Scanner},
        common::{SeekRead, SeekReadWrapper},
    };
    use std::{
        error::Error,
        io::{Cursor, Read, Seek, SeekFrom},
    };

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
        ) -> Result<Box<dyn crate::api::Reader>, Box<dyn Error>> {
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
    fn sf_error_prints_debug_info() {
        let error = SfError::new("Message");
        assert!(format!("{:?}", error).contains("SfError"));
        assert!(format!("{:?}", error).contains("Message"));
    }

    #[test]
    fn sf_error_displays_error_message() {
        let error = SfError::new("Message");
        assert_eq!("Message", error.to_string());
    }

    #[test]
    fn seek_read_wrapper_mimicks_std_seek_read_behavior() {
        let arr: [u8; 3] = [1, 2, 3];
        let mut buf = [0u8; 3];
        let input = Cursor::new(arr);
        let mut seek_read_wrapper = SeekReadWrapper::new(input);

        // read whole input
        let read_len = seek_read_wrapper.read(&mut buf).unwrap();
        assert_eq!(3, read_len);
        assert_eq!(arr, buf);

        // read past end
        buf.fill(0);
        let pos = seek_read_wrapper.seek(SeekFrom::Start(1)).unwrap();
        assert_eq!(1, pos);
        let read_len = seek_read_wrapper.read(&mut buf).unwrap();
        assert_eq!(2, read_len);
        assert_eq!([2, 3, 0], buf);

        // seek beyond end and read
        buf.fill(0);
        let pos = seek_read_wrapper.seek(SeekFrom::Start(10)).unwrap();
        assert_eq!(10, pos);
        let read_len = seek_read_wrapper.read(&mut buf).unwrap();
        assert_eq!(0, read_len);
        assert_eq!([0, 0, 0], buf);

        // seek from end
        let pos = seek_read_wrapper.seek(SeekFrom::End(-1)).unwrap();
        assert_eq!(2, pos);

        // seek to negative position
        let pos = seek_read_wrapper.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(0, pos);
        let seek_err = seek_read_wrapper.seek(SeekFrom::Current(-1)).unwrap_err();
        assert_eq!(std::io::ErrorKind::InvalidInput, seek_err.kind());
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
}
