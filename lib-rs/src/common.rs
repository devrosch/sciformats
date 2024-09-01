use crate::andi::andi_scanner::AndiScanner;
use crate::api::{Reader, Scanner, SeekRead};
use crate::gaml::gaml_scanner::GamlScanner;
use crate::jdx::jdx_scanner::JdxScanner;
use crate::spc::spc_scanner::SpcScanner;
use std::fmt;
use std::io::{BufReader, ErrorKind, SeekFrom};
use std::{
    error::Error,
    io::{Read, Seek},
};

/// A generic error.
#[derive(Debug)]
pub struct SfError {
    message: String,
    source: Option<Box<dyn Error>>,
}

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

/// A repository for scanners.
pub struct ScannerRepository {
    scanners: Vec<Box<dyn Scanner<Box<dyn SeekRead>>>>,
}

impl ScannerRepository {
    /// Create a repository containing the passed scanners.
    pub fn new(scanners: Vec<Box<dyn Scanner<Box<dyn SeekRead>>>>) -> ScannerRepository {
        ScannerRepository { scanners }
    }

    /// Create a repository containing all available scanners.
    pub fn init_all() -> ScannerRepository {
        let andi_scanner: Box<dyn Scanner<Box<dyn SeekRead>>> = Box::new(AndiScanner::new());
        let spc_scanner = Box::new(SpcScanner::new());
        let gaml_scanner = Box::new(GamlScanner::new());
        let jdx_scanner = Box::new(JdxScanner::new());
        let scanners: Vec<Box<dyn Scanner<Box<dyn SeekRead>>>> =
            vec![andi_scanner, spc_scanner, gaml_scanner, jdx_scanner];
        ScannerRepository { scanners }
    }

    /// Add a scanner to the repository.
    pub fn push(&mut self, scanner: Box<dyn Scanner<Box<dyn SeekRead>>>) {
        self.scanners.push(scanner)
    }

    /// Checks whether a data set is recognized by any contained scanner. Shallow check.
    pub fn is_recognized(&self, path: &str, input: &mut Box<dyn SeekRead>) -> bool {
        self.scanners
            .iter()
            .any(|scanner| scanner.is_recognized(path, input))
    }

    /// Provides a reader for a recognized data set.
    ///
    /// If multiple scanners recognize a data set, any of them may be returned.
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

/// A buffered implementation of the SeekRead trait.
///
/// Unlike the std BufReader, this implementation tries to avoid clearing the buffer on seek.
pub struct BufSeekRead<T: Seek + Read> {
    input: BufReader<T>,
    pos: u64,
}

impl<T: Seek + Read> BufSeekRead<T> {
    pub fn new(raw_input: T) -> Self {
        Self {
            input: BufReader::new(raw_input),
            pos: 0,
        }
    }
}

impl<T: Seek + Read> Seek for BufSeekRead<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Current(offset) => {
                let new_pos = self
                    .pos
                    .checked_add_signed(offset)
                    .ok_or(std::io::Error::from(ErrorKind::InvalidInput))?;
                self.input.seek_relative(offset)?;
                self.pos = new_pos;
                Ok(self.pos)
            }
            SeekFrom::Start(offset) => {
                let (is_positive_diff, abs_diff) = if offset >= self.pos {
                    (
                        true,
                        offset
                            .checked_sub(self.pos)
                            .ok_or(std::io::Error::from(ErrorKind::InvalidInput))?,
                    )
                } else {
                    (
                        false,
                        self.pos
                            .checked_sub(offset)
                            .ok_or(std::io::Error::from(ErrorKind::InvalidInput))?,
                    )
                };
                let rel_offset = if is_positive_diff {
                    i64::try_from(abs_diff)
                        .map_err(|_| std::io::Error::from(ErrorKind::InvalidInput))?
                } else {
                    i64::try_from(abs_diff)
                        .map_err(|_| std::io::Error::from(ErrorKind::InvalidInput))?
                        .checked_neg()
                        .ok_or(std::io::Error::from(ErrorKind::InvalidInput))?
                };
                self.input.seek_relative(rel_offset)?;
                self.pos = offset;
                Ok(self.pos)
            }
            SeekFrom::End(offset) => {
                // clears buffer which is inefficient but it's rarely used
                self.pos = self.input.seek(SeekFrom::End(offset))?;
                Ok(self.pos)
            }
        }
    }
}

impl<T: Seek + Read> Read for BufSeekRead<T> {
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
        common::{BufSeekRead, SeekRead},
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
    fn buf_seek_read_mimicks_std_seek_read_behavior() {
        let arr: [u8; 3] = [1, 2, 3];
        let mut buf = [0u8; 3];
        let input = Cursor::new(arr);
        let mut buf_seek_read = BufSeekRead::new(input);

        // read whole input
        let read_len = buf_seek_read.read(&mut buf).unwrap();
        assert_eq!(3, read_len);
        assert_eq!(arr, buf);

        // read past end
        buf.fill(0);
        let pos = buf_seek_read.seek(SeekFrom::Start(1)).unwrap();
        assert_eq!(1, pos);
        let read_len = buf_seek_read.read(&mut buf).unwrap();
        assert_eq!(2, read_len);
        assert_eq!([2, 3, 0], buf);

        // seek beyond end and read
        buf.fill(0);
        let pos = buf_seek_read.seek(SeekFrom::Start(10)).unwrap();
        assert_eq!(10, pos);
        let read_len = buf_seek_read.read(&mut buf).unwrap();
        assert_eq!(0, read_len);
        assert_eq!([0, 0, 0], buf);

        // seek from end
        let pos = buf_seek_read.seek(SeekFrom::End(-1)).unwrap();
        assert_eq!(2, pos);

        // seek to negative position
        let pos = buf_seek_read.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(0, pos);
        let seek_err = buf_seek_read.seek(SeekFrom::Current(-1)).unwrap_err();
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
