use super::jdx_parser::JdxBlock;
use crate::api::{Node, Reader, SeekBufRead};
use std::error::Error;

pub struct JdxReader {
    _path: String,
    _file: JdxBlock<Box<dyn SeekBufRead>>,
}

impl Reader for JdxReader {
    fn read(&self, _path: &str) -> Result<Node, Box<dyn Error>> {
        todo!()
    }
}

impl JdxReader {
    pub fn new(path: &str, file: JdxBlock<Box<dyn SeekBufRead>>) -> Self {
        Self {
            _path: path.to_owned(),
            _file: file,
        }
    }
}
