use std::{error::Error, fmt};

pub mod jdx_parser;
pub mod jdx_reader;
pub mod jdx_scanner;
mod jdx_utils;

#[derive(Debug)]
pub struct JdxError {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl JdxError {
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

impl Error for JdxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|b| b.as_ref())
    }
}

impl fmt::Display for JdxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for JdxError {
    fn from(value: std::io::Error) -> Self {
        Self::from_source(value, "I/O error parsing JCAMP-DX.")
    }
}
