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
    /// read (X++(Y..Y)) data
    pub fn read_xppyy_data<T: SeekBufRead>(reader: &mut T) -> Result<Vec<f64>, JdxError> {
        // todo: possible performance tweak: pass NPOINTS as parameter and initialize Vec::<f64>::with_capacity()
        let mut y_values = Vec::<f64>::new();
        let mut y_value_check = Option::<f64>::None;
        let mut pos = reader.stream_position()?;
        let mut buf = Vec::<u8>::with_capacity(128);

        while let Some(line) = reader.read_line_iso_8859_1(&mut buf)? {
            if is_ldr_start(&line) {
                // next LDR encountered => all data read => move back to start of next LDR
                reader.seek(std::io::SeekFrom::Start(pos))?;
                break;
            }

            // save position to move back if next line read is LDR start
            pos = reader.stream_position()?;
            // pre-process line
            let data = strip_line_comment(&line, true, false).0;
            // read Y values from line
            let (line_y_values, is_dif_encoded) = Self::read_xppyy_line(data, y_value_check)?;
            if !line_y_values.is_empty() && y_value_check.is_some() {
                // y value is duplicated in new line, trust new value
                y_values.pop();
            }
            // append line values to y_values
            y_values.extend(&line_y_values);

            // if last and second to last values are defined, use last as y check
            if !is_dif_encoded
                || line_y_values.is_empty()
                || (line_y_values.len() == 1 && line_y_values.last().unwrap().is_nan())
                || (line_y_values.len() >= 2
                    && (line_y_values.last().unwrap().is_nan()
                        && line_y_values[line_y_values.len() - 2].is_nan()))
            {
                y_value_check = None;
            } else {
                y_value_check = line_y_values.last().copied();
            }
        }

        Ok(y_values)
    }

    /// read (XY..XY) data
    pub fn read_xyxy_data<T: SeekBufRead>(reader: &mut T) -> Result<Vec<(f64, f64)>, JdxError> {
        let mut xy_values = Vec::<(f64, f64)>::new();
        let mut pos = reader.stream_position()?;
        let mut buf = Vec::<u8>::with_capacity(128);

        while let Some(line) = reader.read_line_iso_8859_1(&mut buf)? {
            if is_ldr_start(&line) {
                // next LDR encountered => all data read => move back to start of next LDR
                reader.seek(std::io::SeekFrom::Start(pos))?;
                break;
            }

            // save position to move back if next line read is LDR start
            pos = reader.stream_position()?;
            // pre-process line
            let data = strip_line_comment(&line, true, false).0;
            // read xy values from line
            let line_values = Self::read_values(data, false)?.0;
            if line_values.len() % 2 != 0 {
                return Err(JdxError::new(&format!(
                    "Uneven number of values for xy data encountered in line {}. No y value for x value: {}",
                    &line, line_values.last().unwrap()
                )));
            }

            // turn line values into pairs and append line values to xyValues
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
                let num_repeats = token.parse::<u64>().map_err(|_e| {
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
                let value = token.parse::<f64>().map_err(|_e| {
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
        for (idx, c) in line.char_indices() {
            if !Self::is_token_delimiter(c) {
                line = line.split_at(idx).1;
                break;
            }
        }
        if line.is_empty() {
            // no token
            return Ok((None, line));
        }
        if !Self::is_token_start(line, None, is_asdf) {
            return Err(JdxError::new(&format!(
                "Illegal sequence encountered while parsing data: {}",
                line
            )));
        }

        // find end of token
        let mut end_idx = 1;
        let mut iter = line.char_indices();
        let mut prev_char = Some(iter.next().unwrap().1);
        for (i, c) in iter {
            if !Self::is_token_delimiter(c) && !Self::is_token_start(&line[i..], prev_char, is_asdf)
            {
                end_idx += 1;
                prev_char = Some(c);
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
                ((b'0' as i8 + value) as u8 as char).to_string()
            } else {
                ['-', (b'0' as i8 - value) as u8 as char].iter().collect()
            };
            token.replace_range(..1, &replacement);
        }
        token_type
    }

    fn is_token_start(encoded_values: &str, prev_char: Option<char>, is_asdf: bool) -> bool {
        let c = match encoded_values.chars().next() {
            None => return false,
            Some(c) => c,
        };

        if (Self::is_ascii_digit(c) || c == '.')
            && (prev_char.is_none() || Self::is_token_delimiter(prev_char.unwrap()))
        {
            return true;
        }
        if c == 'E' || c == 'e' {
            // could be either an exponent or SQZ digit (E==+5, e==-5)
            // apply heuristic to provide answer
            return !Self::is_exponent_start(encoded_values, is_asdf);
        }
        if c == '+' || c == '-' {
            // could be either a sign of an exponent or AFFN/PAC start digit
            // apply heuristic to provide answer
            if prev_char.is_none() || (prev_char != Some('E') && prev_char != Some('e')) {
                return true;
            }
            // todo: efficient?
            let prepended_encoded_value = format!("{}{}", prev_char.unwrap(), &encoded_values);
            return !Self::is_exponent_start(&prepended_encoded_value, is_asdf);
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
            return !delimiter.is_empty();
        }
        // for AFFN
        !delimiter.is_empty() || (delimiter.is_empty() && tail.is_empty())
    }

    fn is_ascii_digit(c: char) -> bool {
        // c >= '0' && c <= '9'
        c.is_ascii_digit()
    }

    fn is_sqz_digit(c: char) -> bool {
        // (c >= '@' && c <= 'I') || (c >= 'a' && c <= 'i')
        ('@'..='I').contains(&c) || ('a'..='i').contains(&c)
    }

    fn is_dif_digit(c: char) -> bool {
        // (c >= 'J' && c <= 'R') || (c >= 'j' && c <= 'r') || c == '%'
        ('J'..='R').contains(&c) || ('j'..='r').contains(&c) || c == '%'
    }

    fn is_dup_digit(c: char) -> bool {
        // (c >= 'S' && c <= 'Z') || c == 's'
        ('S'..='Z').contains(&c) || c == 's'
    }

    fn is_sqz_dif_dup_digit(c: char) -> bool {
        // (c >= '@' && c <= 'Z') || (c >= 'a' && c <= 's') || c == '%'
        ('@'..='Z').contains(&c) || ('a'..='s').contains(&c) || c == '%'
    }

    fn get_sqz_digit_value(c: char) -> Option<i8> {
        // positive SQZ digits @ABCDEFGHI
        if ('@'..='I').contains(&c) {
            // c >= '@' && c <= 'I'
            return Some(c as i8 - '@' as i8);
        }
        // negative SQZ digits abcdefghi
        if ('a'..='i').contains(&c) {
            // c >= 'a' && c <= 'i'
            return Some('`' as i8 - c as i8);
        }
        None
    }

    fn get_dif_digit_value(c: char) -> Option<i8> {
        // positive DIF digits %JKLMNOPQR
        if c == '%' {
            return Some(0);
        }
        if ('J'..='R').contains(&c) {
            // c >= 'J' && c <= 'R'
            return Some(c as i8 - 'I' as i8);
        }
        // negative DIF digits jklmnopqr
        if ('j'..='r').contains(&c) {
            // c >= 'j' && c <= 'r'
            return Some('i' as i8 - c as i8);
        }
        None
    }

    fn get_dup_digit_value(c: char) -> Option<i8> {
        // DUP digits STUVWXYZs
        if ('S'..='Z').contains(&c) {
            // c >= 'S' && c <= 'Z'
            return Some(c as i8 - 'R' as i8);
        }
        if c == 's' {
            return Some(9);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parses_affn_data_line() {
        let input = "1.23 4.5E23 4.5e2 7.89E-14 600 1E2";

        let (actual, dif_encoded) = DataParser::read_values(input, false).unwrap();

        assert_eq!(vec![1.23, 4.5E23, 4.5E2, 7.89E-14, 600.0, 1E2], actual);
        assert!(!dif_encoded);
    }

    #[test]
    fn parses_ambiguous_affn_sqz_data_line() {
        let input = "1E2 B23C34D45E56";

        let (actual, dif_encoded) = DataParser::read_values(input, true).unwrap();

        assert_eq!(vec![100.0, 223.0, 334.0, 445.0, 556.0], actual);
        assert!(!dif_encoded);
    }

    #[test]
    fn parses_fix_i3_ascii_data_line() {
        let input = "1  2  3  3  2  1  0 -1 -2 -3";

        let (actual, dif_encoded) = DataParser::read_values(input, false).unwrap();

        assert_eq!(
            vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -3.0],
            actual
        );
        assert!(!dif_encoded);
    }

    #[test]
    fn parses_pac_data_line() {
        let input = "1+2+3+3+2+1+0-1-2-3";

        let (actual, dif_encoded) = DataParser::read_values(input, true).unwrap();

        assert_eq!(
            vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -3.0],
            actual
        );
        assert!(!dif_encoded);
    }

    #[test]
    fn parses_sqz_data_line() {
        let input = "1BCCBA@abc";

        let (actual, dif_encoded) = DataParser::read_values(input, true).unwrap();

        assert_eq!(
            vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -3.0],
            actual
        );
        assert!(!dif_encoded);
    }

    #[test]
    fn parses_dif_data_line() {
        let input = "1JJ%jjjjjj";

        let (actual, dif_encoded) = DataParser::read_values(input, true).unwrap();

        assert_eq!(
            vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -3.0],
            actual
        );
        assert!(dif_encoded);
    }

    #[test]
    fn parsing_fails_if_sequence_starts_with_dif_token() {
        let input = "jjj";

        let actual = DataParser::read_values(input, true);
        assert!(DataParser::read_values(input, true).is_err());
        assert!(actual
            .unwrap_err()
            .to_string()
            .contains("DIF token without preceding token"));
    }

    #[test]
    fn parses_difdup_data_line() {
        let input = "1JT%jX";

        let (actual, dif_encoded) = DataParser::read_values(input, true).unwrap();

        assert_eq!(
            vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -3.0],
            actual
        );
        // last ordinate is in DUP format, but previous value is DIF hence count as
        // DIF (as Bruker does)
        assert!(dif_encoded);
    }

    #[test]
    fn parsing_fails_if_sequence_contains_consecutive_dup_tokens() {
        let input = "1VZ";

        let actual = DataParser::read_values(input, true);
        assert!(DataParser::read_values(input, true).is_err());
        assert!(actual
            .unwrap_err()
            .to_string()
            .contains("DUP token with preceding DUP token"));
    }

    #[test]
    fn parsing_fails_for_illegal_token_start_character() {
        // "u" is an illegal character
        let input = "123 u45";

        let actual = DataParser::read_values(input, true);
        assert!(DataParser::read_values(input, true).is_err());
        assert!(actual.unwrap_err().to_string().contains("Illegal sequence"));
    }

    #[test]
    fn parses_mixed_pac_affn_stream() {
        let input = b"599.860 0 0 0 0 2 4 4 4 7 5 4 4 5 5 7 10 11 11 6 5 7 6 9 9 7\r\n\
            648.081 10 10 9 10 11 12 15 16 16 14 17 38 38 35 38 42 47 54\r\n\
            682.799  59  66  75  78  88  96 104 110 121 128\r\n\
            ##END=";
        let mut reader = Cursor::new(input);

        let actual = DataParser::read_xppyy_data(&mut reader).unwrap();
        let last_line = reader.read_line_iso_8859_1(&mut vec![]);

        assert_eq!(
            vec![
                0.0, 0.0, 0.0, 0.0, 2.0, 4.0, 4.0, 4.0, 7.0, 5.0, 4.0, 4.0, 5.0, 5.0, 7.0, 10.0,
                11.0, 11.0, 6.0, 5.0, 7.0, 6.0, 9.0, 9.0, 7.0, 10.0, 10.0, 9.0, 10.0, 11.0, 12.0,
                15.0, 16.0, 16.0, 14.0, 17.0, 38.0, 38.0, 35.0, 38.0, 42.0, 47.0, 54.0, 59.0, 66.0,
                75.0, 78.0, 88.0, 96.0, 104.0, 110.0, 121.0, 128.0
            ],
            actual
        );
        assert_eq!("##END=", last_line.unwrap().unwrap());
    }

    #[test]
    fn parsing_detects_failing_y_check() {
        let input = b"599.000+1jj\r\n\
                                600.000+4jj\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let actual = DataParser::read_xppyy_data(&mut reader);

        assert!(actual.is_err());
        assert!(actual
            .unwrap_err()
            .to_string()
            .contains("Y value check failed"));
    }

    #[test]
    fn parses_difdup_stream() {
        let input = b"599.860@VKT%TLkj%J%KLJ%njKjL%kL%jJULJ%kLK1%lLMNPNPRLJ0QTOJ1P\r\n\
                                700.158A28\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let actual = DataParser::read_xppyy_data(&mut reader).unwrap();
        let last_line = reader.read_line_iso_8859_1(&mut vec![]);

        assert_eq!(
            vec![
                0.0, 0.0, 0.0, 0.0, 2.0, 4.0, 4.0, 4.0, 7.0, 5.0, 4.0, 4.0, 5.0, 5.0, 7.0, 10.0,
                11.0, 11.0, 6.0, 5.0, 7.0, 6.0, 9.0, 9.0, 7.0, 10.0, 10.0, 9.0, 10.0, 11.0, 12.0,
                15.0, 16.0, 16.0, 14.0, 17.0, 38.0, 38.0, 35.0, 38.0, 42.0, 47.0, 54.0, 59.0, 66.0,
                75.0, 78.0, 88.0, 96.0, 104.0, 110.0, 121.0, 128.0
            ],
            actual
        );
        assert_eq!("##END=", last_line.unwrap().unwrap());
    }
}
