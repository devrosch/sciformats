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

use super::jdx_parser::PeakAssignment;
use super::jdx_utils::{next_multiline_parser_tuple, parse_opt_str, parse_str};
use super::{JdxError, JdxSequenceParser};
use crate::api::SeekBufRead;
use std::sync::LazyLock;

/// Matches 2 - 5 peak assignments segments  as groups 1-5, corresponding to
/// one of (X[, Y][, W], A), (X[, Y][, M], A), (X[, Y][, M][, W], A), with X
/// as matches[1] and A as matches[5]
const TUPLE_COMPONENTS_REGEX_PATTERN: &str = r"^\s*\(\s*([^,]*)(?:\s*,\s*([^,]*))?(?:\s*,\s*([^,]*))?(?:\s*,\s*([^,]*))?\s*,\s*<(.*)>\s*\)\s*$";
static TUPLE_COMPONENTS_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(TUPLE_COMPONENTS_REGEX_PATTERN).unwrap());

/// A parser for PEAK ASSIGNMENTS.
pub struct PeakAssignmentsParser<'r, T: SeekBufRead> {
    variable_list: &'r str,
    reader: &'r mut T,
    buf: Vec<u8>,
}

impl<T: SeekBufRead> PeakAssignmentsParser<'_, T> {
    const PEAK_ASSIGNMENTS_VARIABLE_LISTS: [&'static str; 4] =
        ["(XYA)", "(XYWA)", "(XYMA)", "(XYMWA)"];

    fn next_tuple(&mut self) -> Result<Option<String>, JdxError> {
        next_multiline_parser_tuple("PEAK ASSIGNMENTS", self.reader, &mut self.buf, ' ')
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

        let x = parse_opt_str(x_opt.map(|m| m.as_str()), "x value in PEAK ASSIGNMENTS")?;
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
                #[allow(clippy::unnecessary_unwrap)]
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
            "" => Ok(f64::NAN),
            v => parse_str(v, "numeric token in PEAK ASSIGNMENTS"),
        }
    }
}

impl<'r, T: SeekBufRead> JdxSequenceParser<'r, T> for PeakAssignmentsParser<'r, T> {
    type Item = PeakAssignment;

    fn new(
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
    fn next(&mut self) -> Result<Option<PeakAssignment>, JdxError> {
        let tuple_opt = self.next_tuple()?;
        match tuple_opt {
            None => Ok(None),
            Some(tuple) => Ok(Some(self.create_peak_assignment(&tuple)?)),
        }
    }

    fn into_reader(self) -> &'r mut T {
        self.reader
    }
}
