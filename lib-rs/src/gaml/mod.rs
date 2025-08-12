// Copyright (c) 2025 Robert Schiwon
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::xml_utils::SfXmlError;
use std::{error::Error, fmt};

pub mod gaml_parser;
pub mod gaml_reader;
pub mod gaml_scanner;

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

impl From<std::io::Error> for GamlError {
    fn from(value: std::io::Error) -> Self {
        Self::from_source(value, "I/O error parsing GAML.")
    }
}

impl From<SfXmlError> for GamlError {
    fn from(value: SfXmlError) -> Self {
        // remove SfXmlError from error nesting
        let (message, source) = value.into_inner();
        Self { source, message }
    }
}
