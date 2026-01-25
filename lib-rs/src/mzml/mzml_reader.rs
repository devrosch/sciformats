use super::mzml_parser::MzMl;
use crate::{
    api::{Node, Reader},
    common::SfError,
};

#[allow(dead_code)] // TODO: remove when fully implemented
pub struct MzMlReader {
    path: String,
    file: MzMl,
}

impl Reader for MzMlReader {
    #[allow(unused_variables)] // TODO: remove when implemented
    fn read(&self, path: &str) -> Result<Node, SfError> {
        todo!()
    }
}

impl MzMlReader {
    pub fn new(path: &str, file: MzMl) -> Self {
        Self {
            path: path.to_owned(),
            file,
        }
    }
}
