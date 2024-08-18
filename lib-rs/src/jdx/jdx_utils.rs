use super::{jdx_parser::StringLdr, JdxError};
use crate::{api::SeekBufRead, utils::from_iso_8859_1_cstr};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cell::{RefCell, RefMut},
    io::BufRead,
    rc::Rc,
    str::FromStr,
};

pub trait BinBufRead: BufRead {
    /// Read all bytes until a newline (the `0xA` byte) is reached, and append
    /// them to the provided `Vec` buffer.
    ///
    /// This function has the same semantics as `read_until` but will remove
    /// the trailing LF or CRLF. The returned number of bytes read includes
    /// the trailing end of line markers.
    fn read_line_bytes(&mut self, buf: &mut Vec<u8>) -> Result<usize, std::io::Error>;

    /// Read all bytes until a newline (the `0xA` byte) is reached. Replace
    /// the content of the provided `Vec` buffer with the content read.
    ///
    /// Returns an ISO 8859-1 string corresponding to the content.
    ///
    /// Trailing LF or CRLF will be removed from the `Vec` buffer and returned
    /// string.
    fn read_line_iso_8859_1(&mut self, buf: &mut Vec<u8>)
        -> Result<Option<String>, std::io::Error>;
}

impl<T: BufRead> BinBufRead for T {
    /// like read_line() but for bytes
    fn read_line_bytes(&mut self, buf: &mut Vec<u8>) -> Result<usize, std::io::Error> {
        let bytes_read = self.read_until(b'\n', buf)?;
        if bytes_read > 0 && buf.last().unwrap() == &b'\n' {
            // remove trailing LF
            buf.pop();
            if bytes_read > 1 && buf.last().unwrap() == &b'\r' {
                // remove trailing CR (if CRLF)
                buf.pop();
            }
        }
        Ok(bytes_read)
    }

    fn read_line_iso_8859_1(
        &mut self,
        buf: &mut Vec<u8>,
    ) -> Result<Option<String>, std::io::Error> {
        buf.clear();
        let num_bytes_read = self.read_line_bytes(buf)?;
        match num_bytes_read {
            0 => Ok(None),
            _ => Ok(Some(from_iso_8859_1_cstr(buf))),
        }
    }
}

const LDR_START_REGEX_PATTERN: &str = "^\\s*##(.*?)=(.*)";
lazy_static! {
    static ref LDR_START_REGEX: Regex = Regex::new(LDR_START_REGEX_PATTERN).unwrap();
}

pub fn is_ldr_start(line: &str) -> bool {
    LDR_START_REGEX.is_match(line)
}

fn normalize_label(raw_label: &str) -> String {
    let mut label = String::with_capacity(raw_label.len());
    // normalize label
    for c in raw_label.chars() {
        // ignore separators
        if c != ' ' && c != '-' && c != '/' && c != '_' {
            label.push(c.to_ascii_uppercase());
        }
    }
    label
}

pub fn parse_ldr_start(line: &str) -> Result<(String, String), JdxError> {
    let caps = LDR_START_REGEX.captures(line);
    if caps.as_ref().is_none() || caps.as_ref().unwrap().len() < 3 {
        return Err(JdxError::new(&format!(
            "Malformed LDR start. Line does not match pattern \"{}\": {}",
            LDR_START_REGEX_PATTERN, line
        )));
    }

    let raw_label = &caps.as_ref().unwrap()[1];
    let raw_value = &caps.as_ref().unwrap()[2];

    let label = normalize_label(raw_label);
    let value = match raw_value {
        // strip one leading " " if present
        s if s.starts_with(' ') => &s[1..],
        s => s,
    };

    Ok((label, value.to_owned()))
}

pub fn strip_line_comment(
    line: &str,
    trim_content: bool,
    trim_comment: bool,
) -> (&str, Option<&str>) {
    const COMMENT_REGEX_PATTERN: &str = "^(.*?)\\$\\$(.*)$";
    lazy_static! {
        static ref COMMENT_REGEX: regex::Regex = regex::Regex::new(COMMENT_REGEX_PATTERN).unwrap();
    }
    let caps = COMMENT_REGEX.captures(line);

    if let Some((_, [mut content, mut comment])) = caps.map(|c| c.extract()) {
        if trim_content {
            content = content.trim();
        }
        if trim_comment {
            comment = comment.trim();
        }
        (content, Some(comment))
    } else if trim_content {
        (line.trim(), None)
    } else {
        (line, None)
    }
}

pub fn is_pure_comment(line: &str) -> bool {
    strip_line_comment(line, true, false).0.is_empty()
}

pub fn skip_pure_comments<T: BufRead>(
    mut next_line: Option<String>,
    must_precede_ldr: bool,
    reader: &mut T,
    buf: &mut Vec<u8>,
) -> Result<Option<String>, JdxError> {
    while let Some(ref line) = next_line {
        if is_pure_comment(line) {
            next_line = reader.read_line_iso_8859_1(buf)?;
            continue;
        }
        if must_precede_ldr && !is_ldr_start(line) {
            // pure $$ comment lines must be followed by LDR start
            // if not this special case, give up
            return Err(JdxError::new(&format!(
                "Unexpected content found instead of pure comment ($$): {}",
                line
            )));
        }
        break;
    }

    Ok(next_line)
}

pub fn skip_to_next_ldr<T: SeekBufRead>(
    mut next_line: Option<String>,
    force_skip_first_line: bool,
    reader: &mut T,
    buf: &mut Vec<u8>,
) -> Result<Option<String>, JdxError> {
    if force_skip_first_line {
        next_line = reader.read_line_iso_8859_1(buf)?;
    }
    while next_line.is_some() && !is_ldr_start(next_line.as_ref().unwrap()) {
        next_line = reader.read_line_iso_8859_1(buf)?;
    }

    Ok(next_line)
}

pub fn is_bruker_specific_section_start(line: &str) -> bool {
    line.starts_with("$$ Bruker specific parameters")
}

pub fn is_bruker_specific_section_end(line: &str) -> bool {
    line.starts_with("$$ End of Bruker specific parameters")
}

pub fn parse_string_value<T: SeekBufRead>(
    value: &str,
    reader: &mut T,
    buf: &mut Vec<u8>,
) -> Result<(String, Option<String>), JdxError> {
    let mut output = value.trim().to_owned();
    while let Some(line) = reader.read_line_iso_8859_1(buf)? {
        if is_ldr_start(&line) {
            return Ok((output, Some(line)));
        }
        let (content, comment) = strip_line_comment(&line, false, false);
        if !content.is_empty() && output.ends_with('=') {
            // account for terminal "=" as non line breaking marker
            output.pop();
            output.push_str(&line);
        } else if content.is_empty()
            && comment.is_some()
            && (is_bruker_specific_section_start(&line) || is_bruker_specific_section_end(&line))
        {
            // Bruker quirk: specific comments indicate the end of the previous LDR
            return Ok((output, Some(line)));
        } else {
            output.push('\n');
            output.push_str(&line);
        }
    }
    Ok((output, None))
}

pub fn find_ldr<'ldrs>(raw_label: &str, ldrs: &'ldrs [StringLdr]) -> Option<&'ldrs StringLdr> {
    let label = normalize_label(raw_label);
    ldrs.iter().find(|&ldr| label == ldr.label)
}

pub fn parse_str<P: FromStr>(value: &str, context: &str) -> Result<P, JdxError> {
    value
        .parse::<P>()
        .map_err(|_e| JdxError::new(&format!("Illegal value for \"{}\": {}", context, value)))
}

pub fn parse_str_opt<P: FromStr>(value: &str, context: &str) -> Result<Option<P>, JdxError> {
    if value.is_empty() {
        return Ok(None);
    }
    let parsed_value = value
        .parse::<P>()
        .map_err(|_e| JdxError::new(&format!("Illegal value for \"{}\": {}", context, value)))?;
    Ok(Some(parsed_value))
}

pub fn parse_parameter<P: FromStr>(ldr: &StringLdr) -> Result<Option<P>, JdxError> {
    let value = strip_line_comment(&ldr.value, true, false).0;
    parse_str_opt(value, &ldr.label)
}

pub fn find_and_parse_parameter<P: FromStr>(
    key: &str,
    ldrs: &[StringLdr],
) -> Result<Option<P>, JdxError> {
    match find_ldr(key, ldrs) {
        None => Ok(None),
        Some(ldr) => parse_parameter(ldr),
    }
}

pub fn validate_input(
    label: &str,
    variable_list: Option<&str>,
    expected_label: &str,
    expected_variable_lists: Option<&[&str]>,
) -> Result<(), JdxError> {
    if label != expected_label {
        return Err(JdxError::new(&format!(
            "Illegal label at \"{}\" start encountered: {}",
            expected_label, label
        )));
    }
    if let Some(var_lists) = expected_variable_lists {
        match variable_list {
            None => {
                return Err(JdxError::new(&format!(
                    "Missing variable list encountered for: {}",
                    label
                )));
            }
            Some(var_list) => {
                if !var_lists.contains(&var_list) {
                    return Err(JdxError::new(&format!(
                        "Illegal variable list for \"{}\" encountered: {}",
                        label, var_list
                    )));
                }
            }
        }
    }
    Ok(())
}

#[allow(clippy::type_complexity)]
pub(crate) fn parse_element<'t, T: 't, E>(
    label: &str,
    title: &str,
    o: &Option<E>,
    builder: impl FnOnce() -> Result<(E, Option<String>), JdxError>,
    reader: RefMut<'t, T>,
    reader_ref: &'t Rc<RefCell<T>>,
) -> Result<(Option<E>, RefMut<'t, T>, Option<String>), JdxError> {
    if o.is_some() {
        return Err(JdxError::new(&format!(
            "Multiple \"{}\" LDRs found in block: {}",
            label, title
        )));
    }
    drop(reader);
    let (element, next_line) = builder()?;
    let reader = reader_ref.borrow_mut();
    Ok((Some(element), reader, next_line))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn read_line_bytes_recognizes_lf() {
        let mut buf_read = Cursor::new(b"##LABEL0= abc\n##LABEL1= xyz");
        let mut buf = vec![];

        let bytes_read = buf_read.read_line_bytes(&mut buf).unwrap();
        assert_eq!(14, bytes_read);
        assert_eq!(b"##LABEL0= abc".as_ref(), buf);
    }

    #[test]
    fn read_line_bytes_recognizes_crlf() {
        let mut buf_read = Cursor::new(b"##LABEL0= abc\r\n##LABEL1= xyz");
        let mut buf = vec![];

        let bytes_read = buf_read.read_line_bytes(&mut buf).unwrap();
        assert_eq!(15, bytes_read);
        assert_eq!(b"##LABEL0= abc".as_ref(), buf);
    }

    #[test]
    fn read_line_bytes_recognizes_eof() {
        let mut buf_read = Cursor::new(b"##LABEL0= abc");
        let mut buf = vec![];

        let bytes_read = buf_read.read_line_bytes(&mut buf).unwrap();
        assert_eq!(13, bytes_read);
        assert_eq!(b"##LABEL0= abc".as_ref(), buf);
    }

    #[test]
    fn read_line_bytes_ignores_lone_cr() {
        let mut buf_read = Cursor::new(b"##LABEL0= abc\r##LABEL1= xyz");
        let mut buf = vec![];

        let bytes_read = buf_read.read_line_bytes(&mut buf).unwrap();
        assert_eq!(27, bytes_read);
        assert_eq!(b"##LABEL0= abc\r##LABEL1= xyz".as_ref(), buf);
    }

    #[test]
    fn read_line_iso_8859_1_interprets_input_as_iso_8859_1_chars() {
        // string: abcäöüÄÖÜ in ISO-8859-1 / Windows-1252 encoding
        let mut buf_read = Cursor::new(b"abc\xE4\xF6\xFC\xC4\xD6\xDC\n");
        let mut buf = vec![];

        let string = buf_read.read_line_iso_8859_1(&mut buf).unwrap();
        assert_eq!(b"abc\xE4\xF6\xFC\xC4\xD6\xDC".as_ref(), buf);
        assert_eq!(Some("abcäöüÄÖÜ".to_owned()), string);
    }

    #[test]
    fn is_ldr_start_recognizes_regular_ldr_start() {
        let s = "##TITLE= abc";
        assert!(is_ldr_start(s));
    }

    #[test]
    fn is_ldr_start_recognizes_ldr_start_with_non_ascii_chars() {
        let s = "##TITLEabcdeäöüÄÖÜ= abc";
        assert!(is_ldr_start(s));
    }

    #[test]
    fn is_ldr_start_recognizes_ldr_start_with_leading_ws() {
        let s = "\t\n\x0B\x0C\r ##TITLE= abc";
        assert!(is_ldr_start(s));
    }

    #[test]
    fn is_ldr_start_recognizes_ldr_start_with_trailing_line_break() {
        let slf = "##TITLE= abc\n";
        let scr = "##TITLE= abc\r";
        let scrlf = "##TITLE= abc\r\n";
        let scrlfu = "##TITLE= abc\r\n\u{2028}\u{2029}";

        assert!(is_ldr_start(slf));
        assert!(is_ldr_start(scr));
        assert!(is_ldr_start(scrlf));
        assert!(is_ldr_start(scrlfu));
    }

    #[test]
    fn is_ldr_start_rejects_ldr_preceeded_by_non_ws() {
        let s = "xyz ##TITLE= abc";
        assert!(!is_ldr_start(s));
    }

    #[test]
    fn is_ldr_start_recognizes_ldr_start_with_special_chars() {
        let s = "##.N_A/M2E$= abc";
        assert!(is_ldr_start(s));
    }

    #[test]
    fn is_ldr_start_rejects_ldr_non_start() {
        let s = "#NAME= ##NOT_LDR=abc";
        assert!(!is_ldr_start(s));
    }

    #[test]
    fn parse_ldr_start_tokenizes_regular_ldr_start() {
        let s = "##LABEL=abc";
        assert_eq!(
            ("LABEL".to_owned(), "abc".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_tokenizes_ldr_start_with_missing_value() {
        let s = "##LABEL=";
        assert_eq!(
            ("LABEL".to_owned(), "".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_removes_first_leading_space_in_value() {
        let s = "##LABEL=  abc";
        assert_eq!(
            ("LABEL".to_owned(), " abc".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_normalizes_label() {
        let s = "\t\n\x0B\x0C\r ##abcdeäöüÄÖÜ= abc";
        assert_eq!(
            ("ABCDEäöüÄÖÜ".to_owned(), "abc".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_removes_separators_from_label() {
        let s = "##A B-C/D_E= abc";
        assert_eq!(
            ("ABCDE".to_owned(), "abc".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_leaves_normalized_label_intact() {
        let s = "##ABCDE= abc";
        assert_eq!(
            ("ABCDE".to_owned(), "abc".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_removes_leading_ws() {
        let s = "\t\n\x0B\x0C\r ##ABCDE= abc";
        assert_eq!(
            ("ABCDE".to_owned(), "abc".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_turns_ascii_chars_to_upper_case() {
        let s = "##abcdeäöüÄÖÜ= abc";
        assert_eq!(
            ("ABCDEäöüÄÖÜ".to_owned(), "abc".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_rejects_malformed_ldr_start() {
        let s = "##LABEL";
        assert!(parse_ldr_start(s).is_err());
    }

    #[test]
    fn parse_ldr_start_rejects_missing_double_hashes() {
        let s = "#LABEL= abc";
        assert!(parse_ldr_start(s).is_err());
    }

    #[test]
    fn parse_ldr_start_rejects_missing_equals() {
        let s = "##LABEL abc";
        assert!(parse_ldr_start(s).is_err());
    }

    #[test]
    fn parse_ldr_start_correctly_handles_multiple_equals() {
        let s = "##LABEL= abc=";
        assert_eq!(
            ("LABEL".to_owned(), "abc=".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn strip_line_comment_strips_line_comment() {
        let s = "line start $$ comment";
        assert_eq!(
            ("line start ", Some(" comment")),
            strip_line_comment(s, false, false)
        );
    }

    #[test]
    fn strip_line_comment_indicates_missing_comment_with_none() {
        let s = "line content";
        assert_eq!(("line content", None), strip_line_comment(s, false, false));
    }

    #[test]
    fn strip_line_comment_has_empty_content_for_whole_line_comment() {
        let s = "$$line comment";
        assert_eq!(
            ("", Some("line comment")),
            strip_line_comment(s, false, false)
        );
    }

    #[test]
    fn strip_line_comment_indicates_empty_comment_with_empty_string() {
        let s = "line content$$";
        assert_eq!(
            ("line content", Some("")),
            strip_line_comment(s, false, false)
        );
    }

    #[test]
    fn strip_line_comment_trims_content_and_comment_if_indicated() {
        let s = " content $$ comment ";
        assert_eq!(
            (" content ", Some(" comment ")),
            strip_line_comment(s, false, false)
        );
        assert_eq!(
            (" content ", Some("comment")),
            strip_line_comment(s, false, true)
        );
        assert_eq!(
            ("content", Some(" comment ")),
            strip_line_comment(s, true, false)
        );
        assert_eq!(
            ("content", Some("comment")),
            strip_line_comment(s, true, true)
        );
    }

    #[test]
    fn is_pure_comment_accepts_whole_line_comment_and_rejects_others() {
        let pure_comment = "$$comment";
        let pure_comment_with_leading_spaces = "  $$comment";
        let non_pure_comment = "abc $$comment";

        assert!(is_pure_comment(pure_comment));
        assert!(is_pure_comment(pure_comment_with_leading_spaces));
        assert!(!is_pure_comment(non_pure_comment));
    }

    #[test]
    fn is_bruker_specific_section_recognizes_bruker_comments() {
        let bruker_section_start = "$$ Bruker specific parameters";
        let bruker_section_start_f1 = "$$ Bruker specific parameters for F1";
        let bruker_section_end = "$$ End of Bruker specific parameters";
        let bruker_dashes = "$$ ---------------------------------";
        let regular_ldr = "##ORIGIN= Test";

        assert!(is_bruker_specific_section_start(bruker_section_start));
        assert!(is_bruker_specific_section_start(bruker_section_start_f1));
        assert!(!is_bruker_specific_section_start(bruker_section_end));
        assert!(!is_bruker_specific_section_start(bruker_dashes));
        assert!(!is_bruker_specific_section_start(regular_ldr));
    }
}
