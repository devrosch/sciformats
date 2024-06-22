use super::jdx_parser::JdxBlock;
use crate::api::{Node, Reader, SeekBufRead};
use std::error::Error;

pub struct JdxReader {
    path: String,
    file: JdxBlock<Box<dyn SeekBufRead>>,
}

impl Reader for JdxReader {
    fn read(&self, path: &str) -> Result<Node, Box<dyn Error>> {
        todo!()
    }
}

impl JdxReader {
    pub fn new(path: &str, file: JdxBlock<Box<dyn SeekBufRead>>) -> Self {
        Self {
            path: path.to_owned(),
            file,
        }
    }
}
