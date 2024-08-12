use super::{jdx_parser::AuditTrailEntry, JdxError};
use crate::api::SeekBufRead;
// use lazy_static::lazy_static;

// const TUPLE_SEPARATOR_REGEX_PATTERN: &str = r"((?<tuple>.*?[^,\s])(\s*(?:\s|;)\s*))?(?<tail>.*)";
// lazy_static! {
//     static ref TUPLE_SEPARATOR_REGEX: regex::Regex =
//         regex::Regex::new(TUPLE_SEPARATOR_REGEX_PATTERN).unwrap();
// }

pub struct AuditTrailParser<'r, T: SeekBufRead> {
    variable_list: &'r str,
    reader: &'r mut T,
    buf: Vec<u8>,
}

// todo: reduce code duplication
impl<'r, T: SeekBufRead> AuditTrailParser<'r, T> {
    pub fn new(
        variable_list: &'r str,
        reader: &'r mut T,
    ) -> Result<AuditTrailParser<'r, T>, JdxError> {
        todo!()
    }

    pub fn next(&mut self) -> Result<Option<AuditTrailEntry>, JdxError> {
        todo!()
    }
}
