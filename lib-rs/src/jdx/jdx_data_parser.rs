use super::JdxError;
use crate::api::{PointXy, SeekBufRead};
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
    pub fn read_xpp_yy_data<T: SeekBufRead>(reader: &mut T) -> Result<Vec<PointXy>, JdxError> {
        todo!()
    }

    fn read_values(mut line: &str, is_asdf: bool) -> Result<(Vec<f64>, bool), JdxError> {
        todo!()
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
