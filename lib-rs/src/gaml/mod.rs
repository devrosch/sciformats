use std::{
    error::Error,
    fmt,
    io::{BufRead, Seek},
};

pub mod gaml_parser;
pub mod gaml_reader;
pub mod gaml_scanner;
pub mod gaml_utils;

trait SeekBufRead: Seek + BufRead {}
impl<T: Seek + BufRead> SeekBufRead for T {}

#[derive(Debug)]
pub struct GamlError {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl GamlError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.into(),
            source: None,
        }
    }

    pub fn from_source(source: impl Into<Box<dyn Error>>, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: Some(source.into()),
        }
    }
}

impl Error for GamlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|b| b.as_ref())
    }
}

impl fmt::Display for GamlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<quick_xml::Error> for GamlError {
    fn from(value: quick_xml::Error) -> Self {
        Self::from_source(value, "Error parsing GAML.")
    }
}

impl From<std::io::Error> for GamlError {
    fn from(value: std::io::Error) -> Self {
        Self::from_source(value, "I/O error when reading GAML.")
    }
}
