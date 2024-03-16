use std::{error::Error, fmt};

pub mod gaml_parser;
pub mod gaml_utils;

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

// impl From<AttrError> for GamlError {
//     fn from(value: AttrError) -> Self {
//         Self::from_source(value, "Error parsing attribute.")
//     }
// }

// impl From<Utf8Error> for GamlError {
//     fn from(value: Utf8Error) -> Self {
//         Self::from_source(value, "Error parsing attribute.")
//     }
// }

impl From<quick_xml::Error> for GamlError {
    fn from(value: quick_xml::Error) -> Self {
        Self::from_source(value, "Error parsing GAML.")
    }
}
