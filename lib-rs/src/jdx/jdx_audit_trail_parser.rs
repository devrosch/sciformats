use super::jdx_utils::{next_multiline_parser_tuple, parse_opt_str};
use super::JdxSequenceParser;
use super::{jdx_parser::AuditTrailEntry, JdxError};
use crate::api::SeekBufRead;
use lazy_static::lazy_static;

/// matches 5 - 7 audit trail entry segments as groups 1-7, groups 5 nd 6
/// being optional, corresponding to one of (NUMBER, WHEN, WHO, WHERE, WHAT),
/// (NUMBER, WHEN, WHO, WHERE, VERSION, WHAT),
/// (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)
const TUPLE_REGEX_PATTERN: &str = concat!(
    r"^\s*\(\s*",
    r"(\d)",
    r"(?:\s*,\s*<([^>]*)>)",
    r"(?:\s*,\s*<([^>]*)>)",
    r"(?:\s*,\s*<([^>]*)>)",
    r"(?:\s*,\s*<([^>]*)>)?",
    r"(?:\s*,\s*<([^>]*)>)?",
    r"(?:\s*,\s*<([^>]*)>)",
    r"\s*\)\s*$",
);
lazy_static! {
    static ref TUPLE_REGEX: regex::Regex = regex::Regex::new(TUPLE_REGEX_PATTERN).unwrap();
}

/// A parser for AUDIT TRAIL.
pub struct AuditTrailParser<'r, T: SeekBufRead> {
    variable_list: &'r str,
    reader: &'r mut T,
    buf: Vec<u8>,
}

impl<'r, T: SeekBufRead> AuditTrailParser<'r, T> {
    const AUDIT_TRAIL_VARIABLE_LISTS: [&'static str; 3] = [
        "(NUMBER, WHEN, WHO, WHERE, WHAT)",
        "(NUMBER, WHEN, WHO, WHERE, VERSION, WHAT)",
        "(NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)",
    ];

    fn next_tuple(&mut self) -> Result<Option<String>, JdxError> {
        next_multiline_parser_tuple("AUDIT TRAIL", self.reader, &mut self.buf, '\n')
    }

    fn create_audit_trail_entry(&self, tuple: &str) -> Result<AuditTrailEntry, JdxError> {
        let caps_opt = TUPLE_REGEX.captures(tuple);
        let caps = caps_opt.ok_or(JdxError::new(&format!(
            "Illegal AUDIT TRAIL tuple: {}",
            tuple
        )))?;

        let number_opt = caps.get(1);
        let when_opt = caps.get(2);
        let who_opt = caps.get(3);
        let where_opt = caps.get(4);
        let process_or_version_opt = caps.get(5);
        let version_opt = caps.get(6);
        let what_opt = caps.get(7);

        if Self::AUDIT_TRAIL_VARIABLE_LISTS[0] == self.variable_list
            && (process_or_version_opt.is_some() || version_opt.is_some())
        {
            return Err(JdxError::new(&format!(
                "Illegal AUDIT TRAIL entry: {}",
                tuple
            )));
        }
        if Self::AUDIT_TRAIL_VARIABLE_LISTS[1] == self.variable_list
            && (process_or_version_opt.is_none() || version_opt.is_some())
        {
            return Err(JdxError::new(&format!(
                "Illegal AUDIT TRAIL entry: {}",
                tuple
            )));
        }
        if Self::AUDIT_TRAIL_VARIABLE_LISTS[2] == self.variable_list
            && (process_or_version_opt.is_none() || version_opt.is_none())
        {
            return Err(JdxError::new(&format!(
                "Illegal AUDIT TRAIL entry: {}",
                tuple
            )));
        }

        // map
        let number = parse_opt_str::<u64>(number_opt.map(|m| m.as_str()), "NUMBER in AUDIT TRAIL")?;
        let when = parse_opt_str::<String>(when_opt.map(|m| m.as_str()), "WHEN in AUDIT TRAIL")?;
        let who = parse_opt_str::<String>(who_opt.map(|m| m.as_str()), "WHO in AUDIT TRAIL")?;
        let r#where =
            parse_opt_str::<String>(where_opt.map(|m| m.as_str()), "WHERE in AUDIT TRAIL")?;
        let (process, version) = match self.variable_list {
            vars if vars == Self::AUDIT_TRAIL_VARIABLE_LISTS[0] => (None, None),
            vars if vars == Self::AUDIT_TRAIL_VARIABLE_LISTS[1] => {
                (None, process_or_version_opt.map(|m| m.as_str()))
            }
            vars if vars == Self::AUDIT_TRAIL_VARIABLE_LISTS[2] => (
                process_or_version_opt.map(|m| m.as_str()),
                version_opt.map(|m| m.as_str()),
            ),
            // unreachable, really
            _ => (None, None),
        };
        let what = parse_opt_str::<String>(what_opt.map(|m| m.as_str()), "WHAT in AUDIT TRAIL")?;

        Ok(AuditTrailEntry {
            number,
            when,
            who,
            r#where,
            process: process.map(|v| v.to_owned()),
            version: version.map(|v| v.to_owned()),
            what,
        })
    }
}

impl<'r, T: SeekBufRead> JdxSequenceParser<'r, T> for AuditTrailParser<'r, T> {
    type Item = AuditTrailEntry;

    fn new(variable_list: &'r str, reader: &'r mut T) -> Result<AuditTrailParser<'r, T>, JdxError> {
        if !Self::AUDIT_TRAIL_VARIABLE_LISTS.contains(&variable_list) {
            return Err(JdxError::new(&format!(
                "Unsupported variable list for AUDIT TRAIL: {}",
                &variable_list
            )));
        }

        Ok(Self {
            variable_list,
            reader,
            buf: vec![],
        })
    }

    /// Next audit trail entry.
    ///
    /// Assumes that an audit trail entry tuple always starts on a new
    /// line, but may span multiple lines. Returns the next audit trail
    /// entry, None if there is none, and JdxError if next audit trail
    /// entry is malformed.
    fn next(&mut self) -> Result<Option<AuditTrailEntry>, JdxError> {
        let tuple_opt = self.next_tuple()?;
        match tuple_opt {
            None => Ok(None),
            Some(tuple) => Ok(Some(self.create_audit_trail_entry(&tuple)?)),
        }
    }

    fn into_reader(self) -> &'r mut T {
        self.reader
    }
}
