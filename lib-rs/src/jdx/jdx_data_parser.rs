use super::{jdx_utils::BinBufRead, JdxError};
use crate::{
    api::SeekBufRead,
    jdx::jdx_utils::{is_ldr_start, strip_line_comment},
};
use lazy_static::lazy_static;

const EXPONENT_REGEX_PATTERN: &str = "^[eE][+-]{0,1}\\d{1,3}([;,\\s]{0,1})(.*)";
lazy_static! {
    static ref EXPONENT_REGEX: regex::Regex = regex::Regex::new(EXPONENT_REGEX_PATTERN).unwrap();
}

#[derive(PartialEq)]
enum TokenType {
    Affn,
    Sqz,
    Dif,
    Dup,
    Missing,
}

pub struct DataParser {}

impl DataParser {
    pub fn read_xppyy_data<T: SeekBufRead>(reader: &mut T) -> Result<Vec<(f64, f64)>, JdxError> {
        todo!()
    }

    /// read (XY..XY) data
    pub fn read_xyxy_data<T: SeekBufRead + BinBufRead>(
        reader: &mut T,
    ) -> Result<Vec<(f64, f64)>, JdxError> {
        let mut xy_values = Vec::<(f64, f64)>::new();
        let mut pos = reader.stream_position()?;
        let mut buf = Vec::<u8>::with_capacity(128);

        while let Some(line) = reader.read_line_iso_8859_1(&mut buf)? {
            if is_ldr_start(&line) {
                // next LDR encountered => all data read => move back to start of next LDR
                reader.seek(std::io::SeekFrom::Start(pos))?;
                break;
            }

            // save position to move back if next readLine() encounters LDR start
            pos = reader.stream_position()?;
            // pre-process line
            let data = strip_line_comment(&line, true, false).0;
            // read xy values from line
            let line_values = Self::read_values(data, false)?.0;
            // turn line values into pairs and append line values to xyValues
            if line_values.len() % 2 != 0 {
                return Err(JdxError::new(&format!(
                    "Uneven number of values for xy data encountered in line {}. No y value for x value: {}",
                    &line, line_values.last().unwrap()
                )));
            }

            for (x, y) in line_values
                .iter()
                .step_by(2)
                .zip(line_values.iter().skip(1).step_by(2))
            {
                if x.is_nan() {
                    return Err(JdxError::new(&format!(
                        "NaN value encountered as x value in line: {}",
                        &line
                    )));
                }
                xy_values.push((*x, *y))
            }
        }

        Ok(xy_values)
    }

    fn read_xppyy_line(
        line: &str,
        y_value_check: Option<f64>,
    ) -> Result<(Vec<f64>, bool), JdxError> {
        let (mut values, dif_encoded) = Self::read_values(line, true)?;
        if !values.is_empty() {
            // remove initial x value (not required for (X++(Y..Y)) encoded data)
            values.remove(0);
            // skip X value check
        }
        if let (Some(check), Some(first)) = (y_value_check, values.first()) {
            // first y value is a duplicate, check if roughly the same
            if (first - check).abs() >= 1.0 {
                return Err(JdxError::new(&format!(
                    "Y value check failed in line: {}",
                    line
                )));
            }
        }

        Ok((values, dif_encoded))
    }

    fn read_values(mut line: &str, is_asdf: bool) -> Result<(Vec<f64>, bool), JdxError> {
        let mut y_values = Vec::<f64>::new();
        let mut dif_encoded = false;
        // state
        // for DIF/DUP previousTokenValue not same as last yValues value
        let mut previous_token_value = Option::<f64>::None;
        let mut previous_token_type = TokenType::Affn;

        while let (Some(token), tail) = Self::next_token(line, is_asdf)? {
            let mut token = token.to_owned();
            // todo: change fn signature to accept &str and return tuple
            let token_type = Self::to_affn(&mut token);
            // it's not quite clear if DUP of DIF should also count as DIF encoded
            // Bruker seems to think so => apply same logic here
            dif_encoded =
                token_type == TokenType::Dif || (dif_encoded && token_type == TokenType::Dup);

            // check for logical errors
            if (token_type == TokenType::Dif || token_type == TokenType::Dup)
                && previous_token_value.is_none()
            {
                let token_name = if token_type == TokenType::Dif {
                    "DIF"
                } else {
                    "DUP"
                };
                return Err(JdxError::new(&format!(
                    "{} token without preceding token encountered in sequence: {}",
                    token_name, line
                )));
            }
            if token_type == TokenType::Dup
                && previous_token_value.is_some()
                && previous_token_type == TokenType::Dup
            {
                return Err(JdxError::new(&format!(
                    "DUP token with preceding DUP token encountered in sequence: {}",
                    line
                )));
            }

            // process token
            if token_type == TokenType::Missing {
                // ?
                y_values.push(f64::NAN);
                previous_token_value = Some(f64::NAN);
            } else if token_type == TokenType::Dup {
                let num_repeats = token.parse::<u64>().map_err(|e| {
                    JdxError::new(&format!(
                        "Illegal DUP token encountered in sequence \"{}\": {}",
                        line, token
                    ))
                })?;
                // todo: "-1" correct?
                for _ in 0..(num_repeats - 1) {
                    if previous_token_type == TokenType::Dif {
                        // todo: empty y_value or none previous_value caught earlier in loop; necessary there?
                        let last_value = y_values.last().unwrap();
                        let next_value = last_value + previous_token_value.unwrap();
                        y_values.push(next_value);
                    } else {
                        y_values.push(*y_values.last().unwrap());
                    }
                }
            } else {
                let value = token.parse::<f64>().map_err(|e| {
                    JdxError::new(&format!(
                        "Illegal token encountered in sequence \"{}\": {}",
                        line, token
                    ))
                })?;
                if token_type == TokenType::Dif {
                    if previous_token_type == TokenType::Missing {
                        return Err(JdxError::new(&format!(
                            "DIF token with preceding ? token encountered in sequence: \"{}\": {}",
                            line, token
                        )));
                    }
                    let last_value = y_values.last().unwrap();
                    let next_value = last_value + value;
                    y_values.push(next_value);
                } else {
                    y_values.push(value);
                }
                previous_token_value = Some(value);
            }
            previous_token_type = token_type;
            line = tail;
        }

        Ok((y_values, dif_encoded))
    }

    fn next_token(mut line: &str, is_asdf: bool) -> Result<(Option<&str>, &str), JdxError> {
        // skip delimiters
        let start_delimiter_pos_opt = line.find(Self::is_token_delimiter);
        if let Some(pos) = start_delimiter_pos_opt {
            line = line.split_at(pos).1;
        }
        if line.len() == 0 {
            // no token
            return Ok((None, line));
        }
        if !Self::is_token_start(line, 0, is_asdf) {
            return Err(JdxError::new(&format!(
                "Illegal sequence encountered while parsing data: {}",
                line
            )));
        }

        // find end of token
        let mut end_idx = 1;
        for (idx, c) in line[1..].char_indices() {
            if !Self::is_token_delimiter(c) && !Self::is_token_start(line, idx, is_asdf) {
                end_idx += 1;
            } else {
                break;
            }
        }

        match line[0..end_idx].len() {
            0 => Ok((None, "")),
            _ => Ok((Some(&line[0..end_idx]), &line[end_idx..])),
        }
    }

    fn to_affn(token: &mut String) -> TokenType {
        // todo: dangerous, token may be empty(?)
        let c = token.chars().next().unwrap();
        let (first_digit, token_type) = match c {
            '?' => (None, TokenType::Missing),
            ch if Self::is_sqz_digit(ch) => (Self::get_sqz_digit_value(ch), TokenType::Sqz),
            ch if Self::is_dif_digit(ch) => (Self::get_dif_digit_value(ch), TokenType::Dif),
            ch if Self::is_dup_digit(ch) => (Self::get_dup_digit_value(ch), TokenType::Dup),
            _ => (None, TokenType::Affn),
        };

        if token_type != TokenType::Affn && token_type != TokenType::Missing {
            // replace SQZ/DIF/DUP char (first char) with (signed) value
            let value = first_digit.unwrap();
            let replacement = if value >= 0 {
                ((b'0' + value) as char).to_string()
            } else {
                ['-', (b'0' - value) as char].iter().collect()
            };
            token.replace_range(..1, &replacement);
        }
        token_type
    }

    // todo: refactor as to not require .as_bytes() and char lookup
    fn is_token_start(encoded_values: &str, index: usize, is_asdf: bool) -> bool {
        let c = match encoded_values.as_bytes().get(index) {
            None => return false,
            Some(c) => c.to_owned() as char,
        };

        if (Self::is_ascii_digit(c as char) || c == '.')
            && (index == 0
                // todo: correct?
                || Self::is_token_delimiter(encoded_values.as_bytes()[index - 1] as char))
        {
            return true;
        }
        if c == 'E' || c == 'e' {
            // could be either an exponent or SQZ digit (E==+5, e==-5)
            // apply heuristic to provide answer
            // todo: correct? dangerous?
            return !Self::is_exponent_start(&encoded_values[index..], is_asdf);
        }
        if c == '+' || c == '-' {
            // could be either a sign of an exponent or AFFN/PAC start digit
            // apply heuristic to provide answer
            if index == 0 {
                return true;
            }
            return !Self::is_exponent_start(&encoded_values[index - 1..], is_asdf);
        }
        if Self::is_sqz_dif_dup_digit(c) {
            return true;
        }
        if c == '?' {
            // "invalid" data symbol
            return true;
        }
        false
    }

    fn is_token_delimiter(c: char) -> bool {
        c == ',' || c == ';' || c.is_ascii_whitespace()
    }

    fn is_exponent_start(candidate: &str, is_asdf: bool) -> bool {
        let caps = EXPONENT_REGEX.captures(candidate);
        if caps.as_ref().is_none() {
            return false;
        }
        let delimiter = &caps.as_ref().unwrap()[1];
        let tail = &caps.as_ref().unwrap()[2];
        if is_asdf {
            // compressed values can easily end in a line on, e.g., "E567", so
            // require delimiter
            return delimiter.len() > 0;
        }
        // for AFFN
        (delimiter.len() == 0 && tail.len() == 0) || delimiter.len() > 1
    }

    fn is_ascii_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_sqz_digit(c: char) -> bool {
        (c >= '@' && c <= 'I') || (c >= 'a' && c <= 'i')
    }

    fn is_dif_digit(c: char) -> bool {
        (c >= 'J' && c <= 'R') || (c >= 'j' && c <= 'r') || c == '%'
    }

    fn is_dup_digit(c: char) -> bool {
        (c >= 'S' && c <= 'Z') || c == 's'
    }

    fn is_sqz_dif_dup_digit(c: char) -> bool {
        (c >= '@' && c <= 'Z') || (c >= 'a' && c <= 's') || c == '%'
    }

    fn get_ascii_digit_value(c: char) -> Option<u8> {
        if Self::is_ascii_digit(c) {
            return Some(c as u8 - '0' as u8);
        }
        None
    }

    fn get_sqz_digit_value(c: char) -> Option<u8> {
        // positive SQZ digits @ABCDEFGHI
        if c >= '@' && c <= 'I' {
            return Some(c as u8 - '@' as u8);
        }
        // negative SQZ digits abcdefghi
        if c >= 'a' && c <= 'i' {
            return Some('`' as u8 - c as u8);
        }
        None
    }

    fn get_dif_digit_value(c: char) -> Option<u8> {
        // positive DIF digits %JKLMNOPQR
        if c == '%' {
            return Some(0);
        }
        if c >= 'J' && c <= 'R' {
            return Some(c as u8 - 'I' as u8);
        }
        // negative DIF digits jklmnopqr
        if c >= 'j' && c <= 'r' {
            return Some('i' as u8 - c as u8);
        }
        None
    }

    fn get_dup_digit_value(c: char) -> Option<u8> {
        // DUP digits STUVWXYZs
        if c >= 'S' && c <= 'Z' {
            return Some(c as u8 - 'R' as u8);
        }
        if c == 's' {
            return Some(9);
        }
        None
    }
}
