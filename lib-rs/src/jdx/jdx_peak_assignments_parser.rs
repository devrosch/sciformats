use std::f64::NAN;

use super::jdx_parser::PeakAssignment;
use super::jdx_utils::{is_ldr_start, strip_line_comment, BinBufRead};
use super::JdxError;
use crate::api::SeekBufRead;
use lazy_static::lazy_static;

const TUPLE_SEPARATOR_REGEX_PATTERN: &str = r"((?<tuple>.*?[^,\s])(\s*(?:\s|;)\s*))?(?<tail>.*)";
lazy_static! {
    static ref TUPLE_SEPARATOR_REGEX: regex::Regex =
        regex::Regex::new(TUPLE_SEPARATOR_REGEX_PATTERN).unwrap();
}

/// Matches 2 - 5 peak assignments segments  as groups 1-5, corresponding to
/// one of (X[, Y][, W], A), (X[, Y][, M], A), (X[, Y][, M][, W], A), with X
/// as matches[1] and A as matches[5]
const TUPLE_COMPONENTS_REGEX_PATTERN: &str = r"^\s*\(\s*([^,]*)(?:\s*,\s*([^,]*))?(?:\s*,\s*([^,]*))?(?:\s*,\s*([^,]*))?\s*,\s*<(.*)>\s*\)\s*$";
lazy_static! {
    static ref TUPLE_COMPONENTS_REGEX: regex::Regex =
        regex::Regex::new(TUPLE_COMPONENTS_REGEX_PATTERN).unwrap();
}

/// A parser for PEAK ASSIGNMENTS.
pub struct PeakAssignmentsParser<'r, T: SeekBufRead> {
    variable_list: &'r str,
    reader: &'r mut T,
    buf: Vec<u8>,
}

// todo: reduce code duplication
impl<'r, T: SeekBufRead> PeakAssignmentsParser<'r, T> {
    const PEAK_ASSIGNMENTS_VARIABLE_LISTS: [&'static str; 4] =
        ["(XYA)", "(XYWA)", "(XYMA)", "(XYMWA)"];

    pub fn new(
        variable_list: &'r str,
        reader: &'r mut T,
    ) -> Result<PeakAssignmentsParser<'r, T>, JdxError> {
        if !Self::PEAK_ASSIGNMENTS_VARIABLE_LISTS.contains(&variable_list) {
            return Err(JdxError::new(&format!(
                "Unsupported variable list for PEAK ASSIGNMENTS: {}",
                &variable_list
            )));
        }

        Ok(Self {
            variable_list,
            reader,
            buf: vec![],
        })
    }

    /// Next peak assignment.
    ///
    /// Assumes that a peak assignment tuple always starts on a new line,
    /// but may span multiple lines. Returns the next peak assignment,
    /// None if there is none, or JdxError if the next peak assignment is
    /// malformed.
    pub fn next(&mut self) -> Result<Option<PeakAssignment>, JdxError> {
        let tuple_opt = self.next_tuple()?;
        match tuple_opt {
            None => Ok(None),
            Some(tuple) => Ok(Some(self.create_peak_assignment(&tuple)?)),
        }
    }

    fn next_tuple(&mut self) -> Result<Option<String>, JdxError> {
        let mut pos = self.reader.stream_position()?;
        let mut tuple = String::new();

        // find start
        while let Some(line) = self.reader.read_line_iso_8859_1(&mut self.buf)? {
            let (line_start, _comment) = strip_line_comment(&line, true, false);

            if Self::is_tuple_start(line_start) {
                tuple.push_str(line_start);
                break;
            }
            if is_ldr_start(line_start) {
                // LDR ended, no tuple
                self.reader.seek(std::io::SeekFrom::Start(pos))?;
                return Ok(None);
            }
            if !line_start.is_empty() {
                return Err(JdxError::new(&format!(
                    "Illegal string found in PEAK ASSIGNMENTS: {}",
                    line
                )));
            }
            pos = self.reader.stream_position()?;
        }

        if Self::is_tuple_end(&tuple) {
            return Ok(Some(tuple));
        }

        // read to end of tuple
        pos = self.reader.stream_position()?;
        while let Some(line) = self.reader.read_line_iso_8859_1(&mut self.buf)? {
            let (line_start, _comment) = strip_line_comment(&line, true, false);

            if is_ldr_start(line_start) {
                // LDR ended before end of last tuple
                self.reader.seek(std::io::SeekFrom::Start(pos))?;
                return Err(JdxError::new(&format!(
                    "No closing parenthesis found for PEAK ASSIGNMENTS entry: {}",
                    tuple
                )));
            }

            tuple.push(' ');
            tuple.push_str(line_start);

            if Self::is_tuple_end(line_start) {
                return Ok(Some(tuple));
            }

            pos = self.reader.stream_position()?;
        }

        Err(JdxError::new(&format!(
            "File ended before closing parenthesis was found for PEAK ASSIGNMENTS: {}",
            tuple
        )))
    }

    fn is_tuple_start(value: &str) -> bool {
        value.trim_start().starts_with('(')
    }

    fn is_tuple_end(value: &str) -> bool {
        value.trim_end().ends_with(')')
    }

    fn create_peak_assignment(&self, tuple: &str) -> Result<PeakAssignment, JdxError> {
        let caps_opt = TUPLE_COMPONENTS_REGEX.captures(tuple);
        let caps = caps_opt.ok_or(JdxError::new(&format!(
            "Illegal PEAK ASSIGNMENTS tuple: {}",
            tuple
        )))?;

        let x_opt = caps.get(1);
        let y_opt = caps.get(2);
        let wm_opt = caps.get(3);
        let w_opt = caps.get(4);
        let a_opt = caps.get(5);

        // todo: reduce code duplication
        if x_opt.is_none() || a_opt.is_none() {
            return Err(JdxError::new(&format!(
                "Illegal PEAK ASSIGNMENTS entry: {}",
                tuple
            )));
        }
        if Self::PEAK_ASSIGNMENTS_VARIABLE_LISTS[0] == self.variable_list
            && (wm_opt.is_some() || w_opt.is_some())
        {
            return Err(JdxError::new(&format!(
                "Illegal PEAK ASSIGNMENTS entry for {}: {}",
                self.variable_list, tuple
            )));
        }
        if Self::PEAK_ASSIGNMENTS_VARIABLE_LISTS[1] == self.variable_list && w_opt.is_some() {
            return Err(JdxError::new(&format!(
                "Illegal PEAK ASSIGNMENTS entry for {}: {}",
                self.variable_list, tuple
            )));
        }
        if Self::PEAK_ASSIGNMENTS_VARIABLE_LISTS[1] == self.variable_list
            && (y_opt.is_some() && wm_opt.is_none())
        {
            return Err(JdxError::new(&format!(
                "Ambiguous PEAK ASSIGNMENTS entry for {}: {}",
                self.variable_list, tuple
            )));
        }
        if Self::PEAK_ASSIGNMENTS_VARIABLE_LISTS[2] == self.variable_list && w_opt.is_some() {
            return Err(JdxError::new(&format!(
                "Illegal PEAK ASSIGNMENTS entry for {}: {}",
                self.variable_list, tuple
            )));
        }
        if Self::PEAK_ASSIGNMENTS_VARIABLE_LISTS[2] == self.variable_list
            && (y_opt.is_some() && wm_opt.is_none())
        {
            return Err(JdxError::new(&format!(
                "Ambiguous PEAK ASSIGNMENTS entry for {}: {}",
                self.variable_list, tuple
            )));
        }
        if Self::PEAK_ASSIGNMENTS_VARIABLE_LISTS[3] == self.variable_list
            && (!(y_opt.is_some() && wm_opt.is_some() && w_opt.is_some())
                && (y_opt.is_some() || wm_opt.is_some() || w_opt.is_some()))
        {
            return Err(JdxError::new(&format!(
                "Ambiguous PEAK ASSIGNMENTS entry for {}: {}",
                self.variable_list, tuple
            )));
        }

        // todo: unwrap?
        let x = x_opt.unwrap().as_str().parse::<f64>().map_err(|_e| {
            JdxError::new(&format!(
                "Illegal x value encountered while parsing PEAK ASSIGNMENTS: {}",
                tuple
            ))
        })?;
        let a = a_opt.unwrap().as_str().to_owned();
        let (y, m, w) =
            if Self::PEAK_ASSIGNMENTS_VARIABLE_LISTS[0] == self.variable_list && y_opt.is_some() {
                let y = Self::parse_f64_token(y_opt.unwrap().as_str())?;
                (Some(y), None, None)
            } else if Self::PEAK_ASSIGNMENTS_VARIABLE_LISTS[1] == self.variable_list
                && y_opt.is_some()
                && wm_opt.is_some()
            {
                let y = Self::parse_f64_token(y_opt.unwrap().as_str())?;
                let w = Self::parse_f64_token(wm_opt.unwrap().as_str())?;
                (Some(y), None, Some(w))
            } else if Self::PEAK_ASSIGNMENTS_VARIABLE_LISTS[2] == self.variable_list
                && y_opt.is_some()
                && wm_opt.is_some()
            {
                let y = Self::parse_f64_token(y_opt.unwrap().as_str())?;
                let m = wm_opt.unwrap().as_str();
                (Some(y), Some(m), None)
            } else if Self::PEAK_ASSIGNMENTS_VARIABLE_LISTS[3] == self.variable_list
                && y_opt.is_some()
                && wm_opt.is_some()
                && w_opt.is_some()
            {
                let y = Self::parse_f64_token(y_opt.unwrap().as_str())?;
                let m = wm_opt.unwrap().as_str();
                let w = Self::parse_f64_token(w_opt.unwrap().as_str())?;
                (Some(y), Some(m), Some(w))
            } else {
                (None, None, None)
            };

        Ok(PeakAssignment {
            x,
            y,
            m: m.map(str::to_string),
            w,
            a,
        })
    }

    fn parse_f64_token(token: &str) -> Result<f64, JdxError> {
        match token.trim() {
            "" => Ok(NAN),
            v => Ok(v.parse::<f64>().map_err(|_e| {
                JdxError::new(&format!(
                    "Illegal numeric token encountered while parsing PEAK ASSIGNMENTS: {}",
                    v
                ))
            })?),
        }
    }
}
