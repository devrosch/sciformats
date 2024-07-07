use super::jdx_utils::{is_ldr_start, strip_line_comment, BinBufRead};
use super::{jdx_parser::Peak, JdxError};
use crate::api::SeekBufRead;
use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::f64::NAN;

const TUPLE_SEPARATOR_REGEX_PATTERN: &str = r"((?<tuple>.*?[^,\s])(\s*(?:\s|;)\s*))?(?<tail>.*)";
lazy_static! {
    static ref TUPLE_SEPARATOR_REGEX: regex::Regex =
        regex::Regex::new(TUPLE_SEPARATOR_REGEX_PATTERN).unwrap();
}

/// Matches 2-3 peak segments as groups 1-3, corresponding to
/// (XY..XY), (XYW..XYW), or (XYM..XYM), with X as group 1, Y as group 2
/// and W or M as group 3
const TUPLE_COMPONENTS_REGEX_PATTERN: &str = r"^\s*([^,]*)(?:\s*,\s*([^,]*))(?:\s*,\s*([^,]*))?$";
lazy_static! {
    static ref TUPLE_COMPONENTS_REGEX: regex::Regex =
        regex::Regex::new(TUPLE_COMPONENTS_REGEX_PATTERN).unwrap();
}

pub struct PeakTableParser<'r, T: SeekBufRead> {
    variable_list: &'r str,
    reader: &'r mut T,
    buf: Vec<u8>,
    tuple_queue: VecDeque<String>,
}

impl<'r, T: SeekBufRead> PeakTableParser<'r, T> {
    const PEAK_TABLE_VARIABLE_LISTS: [&'static str; 3] = ["(XY..XY)", "(XYW..XYW)", "(XYM..XYM)"];

    pub fn new(
        variable_list: &'r str,
        reader: &'r mut T,
    ) -> Result<PeakTableParser<'r, T>, JdxError> {
        if !Self::PEAK_TABLE_VARIABLE_LISTS.contains(&variable_list) {
            return Err(JdxError::new(&format!(
                "Unsupported variable list for PEAK TABLE: {}",
                &variable_list
            )));
        }

        Ok(PeakTableParser {
            variable_list,
            reader,
            buf: vec![],
            tuple_queue: VecDeque::new(),
        })
    }

    pub fn next(&mut self) -> Result<Option<Peak>, JdxError> {
        let tuple_opt = self.next_tuple()?;
        match tuple_opt {
            None => Ok(None),
            Some(tuple) => Ok(Some(self.create_peak(&tuple)?)),
        }
    }

    fn next_tuple(&mut self) -> Result<Option<String>, JdxError> {
        while self.tuple_queue.is_empty() {
            let pos = self.reader.stream_position()?;
            let next_line = self.reader.read_line_iso_8859_1(&mut self.buf)?;

            match next_line {
                // EOF
                None => break,
                Some(line) => {
                    if is_ldr_start(&line) {
                        // next LDR => end of PEAK TABLE
                        self.reader.seek(std::io::SeekFrom::Start(pos))?;
                        break;
                    }

                    let (value, _comment) = strip_line_comment(&line, true, false);
                    if value.is_empty() {
                        // skip pure comments
                        continue;
                    }

                    let mut tail = value;
                    while !tail.is_empty() {
                        let caps = TUPLE_SEPARATOR_REGEX.captures(tail);
                        if caps.as_ref().is_none() {
                            break;
                        }

                        let tuple_match = caps.as_ref().unwrap().name("tuple");
                        let tail_match = caps.as_ref().unwrap().name("tail");

                        match (tuple_match, tail_match) {
                            (Some(tuple), Some(rest)) => {
                                self.tuple_queue.push_back(tuple.as_str().to_owned());
                                tail = rest.as_str();
                            }
                            (Some(tuple), None) => {
                                self.tuple_queue.push_back(tuple.as_str().to_owned());
                                tail = "";
                            }
                            (None, Some(rest)) => {
                                // the last tuple in a line tuple does not have a trailing separator
                                // thus the regex reports it as tail
                                self.tuple_queue.push_back(rest.as_str().to_owned());
                                tail = "";
                            }
                            (None, None) => {
                                tail = "";
                            }
                        }
                    }
                    if !tail.trim().is_empty() {
                        return Err(JdxError::new(&format!(
                            "Unexpected content found while parsing PEAK TABLE: {}",
                            tail
                        )));
                    }
                }
            }
        }

        Ok(self.tuple_queue.pop_front())
    }

    fn create_peak(&self, tuple: &str) -> Result<Peak, JdxError> {
        let caps = TUPLE_COMPONENTS_REGEX.captures(tuple);
        if caps.as_ref().is_none() {
            return Err(JdxError::new(&format!(
                "Illegal PEAK TABLE tuple: {}",
                tuple
            )));
        }

        let x_opt = caps.as_ref().unwrap().get(1);
        let y_opt = caps.as_ref().unwrap().get(2);
        let wm_opt = caps.as_ref().unwrap().get(3);

        if x_opt.is_none()
            || y_opt.is_none()
            || (self.variable_list == Self::PEAK_TABLE_VARIABLE_LISTS[0] && wm_opt.is_some())
            || (Self::PEAK_TABLE_VARIABLE_LISTS[1..].contains(&self.variable_list)
                && wm_opt.is_none())
        {
            return Err(JdxError::new(&format!(
                "Illegal PEAK TABLE entry for {}: {}",
                self.variable_list, tuple
            )));
        }

        // todo: reduce code duplication
        let x = x_opt.unwrap().as_str().parse::<f64>().map_err(|_e| {
            JdxError::new(&format!(
                "Illegal x value encountered while parsing PEAK TABLE token: {}",
                tuple
            ))
        })?;
        let y = match y_opt.unwrap().as_str() {
            s if s.trim().is_empty() => NAN,
            s => s.parse::<f64>().map_err(|_e| {
                JdxError::new(&format!(
                    "Illegal y value encountered while parsing PEAK TABLE token: {}",
                    tuple
                ))
            })?,
        };
        let (w, m) = match self.variable_list {
            s if s == Self::PEAK_TABLE_VARIABLE_LISTS[0] => (None, None),
            s if s == Self::PEAK_TABLE_VARIABLE_LISTS[1] => {
                let w = match wm_opt.unwrap().as_str() {
                    wm if wm.trim().is_empty() => NAN,
                    wm => wm.parse::<f64>().map_err(|_e| {
                        JdxError::new(&format!(
                            "Illegal w value encountered while parsing PEAK TABLE token: {}",
                            tuple
                        ))
                    })?,
                };
                (Some(w), None)
            }
            s if s == Self::PEAK_TABLE_VARIABLE_LISTS[2] => {
                let m = wm_opt.unwrap().as_str().trim().to_owned();
                (None, Some(m))
            }
            _ => (None, None),
        };

        Ok(Peak { x, y, m, w })
    }
}
