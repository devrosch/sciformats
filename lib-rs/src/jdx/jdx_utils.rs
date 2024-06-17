use std::io::BufRead;

use super::JdxError;
use crate::utils::from_iso_8859_1_cstr;
use lazy_static::lazy_static;
use regex::bytes::Regex;

pub trait BinBufRead: BufRead {
    /// Read all bytes until a newline (the `0xA` byte) is reached, and append
    /// them to the provided `String` buffer.
    ///
    /// This function has the same semantics as `read_until` but will remove
    /// the trailing LF or CRLF. The returned number of bytes read includes
    /// the trailing end of line markers.
    fn read_line_bytes(&mut self, buf: &mut Vec<u8>) -> Result<usize, std::io::Error>;
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
}

// (?-u) disable unicode mode, see: https://docs.rs/regex/latest/regex/#unicode
const LDR_START_REGEX_PATTERN: &str = "(?-u)^\\s*##(.*)=(.*)";

lazy_static! {
    static ref LDR_START_REGEX: Regex = Regex::new(LDR_START_REGEX_PATTERN).unwrap();
}

pub fn is_ldr_start(line: &[u8]) -> bool {
    LDR_START_REGEX.is_match(line)
}

// todo: Is this fn even required? Not use by parse_ldr_start()?
pub fn normalize_ldr_start(line: &[u8]) -> Result<Vec<u8>, JdxError> {
    let caps = LDR_START_REGEX.captures(line);
    if caps.as_ref().is_none() || caps.as_ref().unwrap().len() < 3 {
        return Err(JdxError::new(&format!(
            "Malformed LDR start. Line does not match pattern \"{}\": {}",
            LDR_START_REGEX_PATTERN,
            // todo: use better name
            from_iso_8859_1_cstr(line)
        )));
    }

    let label = &caps.as_ref().unwrap()[1];
    let value = &caps.as_ref().unwrap()[2];

    let mut normalized_label = Vec::<u8>::with_capacity(label.len());
    // normalize label
    for c in label {
        // ignore separators
        if c != &b' ' && c != &b'-' && c != &b'/' && c != &b'_' {
            normalized_label.push(c.to_ascii_uppercase());
        }
    }

    let output = [b"##".as_slice(), &normalized_label, b"=", value].concat();
    Ok(output)
}

// todo: check performance of implementations
// pub fn normalize_ldr_start2(line: &[u8]) -> Result<Vec<u8>, JdxError> {
//     let mut output = Vec::<u8>::with_capacity(line.len());
//     let mut index = 0;
//     // skip leading white spaces
//     for c in line {
//         // 0B is not considered whitespace by is_ascii_whitespace()
//         if !c.is_ascii_whitespace() && c != &0x0Bu8 {
//             break;
//         }
//         index += 1;
//     }
//     // check and skip "##" marking start of LDR
//     for _ in 0..2 {
//         if index >= line.len() || line[index] != b'#' {
//             return Err(JdxError::new(&format!(
//                 "Malformed LDR start, missing double hashes: {}",
//                 // todo: use better name
//                 from_iso_8859_1_cstr(line)
//             )));
//         }
//         index += 1;
//     }
//     output.extend(b"##");
//     // normalize label
//     while index < line.len() && line[index] != b'=' {
//         let c = line[index];
//         // ignore separators
//         if c != b' ' && c != b'-' && c != b'/' && c != b'_' {
//             output.push(c.to_ascii_uppercase());
//         }
//         index += 1;
//     }
//     // add remaining string content
//     if index >= line.len() || line[index] != b'=' {
//         return Err(JdxError::new(&format!(
//             "Malformed LDR start, missing equals: {}",
//             // todo: use better name
//             from_iso_8859_1_cstr(line)
//         )));
//     }
//     output.extend(&line[index..]);
//     Ok(output)
// }

fn normalize_label(raw_label: &[u8]) -> String {
    let mut label = String::with_capacity(raw_label.len());
    // normalize label
    for c in raw_label {
        // ignore separators
        if c != &b' ' && c != &b'-' && c != &b'/' && c != &b'_' {
            label.push(c.to_ascii_uppercase() as char);
        }
    }
    label
}

pub fn parse_ldr_start(line: &[u8]) -> Result<(String, String), JdxError> {
    let caps = LDR_START_REGEX.captures(line);
    if caps.as_ref().is_none() || caps.as_ref().unwrap().len() < 3 {
        return Err(JdxError::new(&format!(
            "Malformed LDR start. Line does not match pattern \"{}\": {}",
            LDR_START_REGEX_PATTERN,
            // todo: use better name
            from_iso_8859_1_cstr(line)
        )));
    }

    let raw_label = &caps.as_ref().unwrap()[1];
    let raw_value = &caps.as_ref().unwrap()[2];

    let label = normalize_label(raw_label);
    let value = match raw_value {
        // strip one leading " " if present
        s if s.starts_with(b" ") => from_iso_8859_1_cstr(&s[1..]),
        s => from_iso_8859_1_cstr(s),
    };

    Ok((label, value))
}

pub fn strip_line_comment(
    line: &str,
    trim_content: bool,
    trim_comment: bool,
) -> (&str, Option<&str>) {
    const COMMENT_REGEX_PATTERN: &str = "^(.*)\\$\\$(.*)$";
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
    } else {
        if trim_content {
            (line.trim(), None)
        } else {
            (line, None)
        }
    }
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
    fn is_ldr_start_recognizes_regular_ldr_start() {
        let s = b"##TITLE= abc";
        assert!(is_ldr_start(s));
    }

    #[test]
    fn is_ldr_start_recognizes_ldr_start_with_non_ascii_chars() {
        // label: abcdeäöüÄÖÜ in ISO-8859-1 / Windows-1252 encoding
        let s = b"##TITLE\xE4\xF6\xFC\xC4\xD6\xDC= abc";
        assert!(is_ldr_start(s));
    }

    #[test]
    fn is_ldr_start_recognizes_ldr_start_with_leading_ws() {
        let s = b"\t\n\x0B\x0C\r ##TITLE= abc";
        assert!(is_ldr_start(s));
    }

    #[test]
    fn is_ldr_start_recognizes_ldr_start_with_trailing_line_break() {
        let slf = b"##TITLE= abc\n";
        let scr = b"##TITLE= abc\r";
        let scrlf = b"##TITLE= abc\r\n";
        let scrlfu = "##TITLE= abc\r\n\u{2028}\u{2029}".as_bytes();

        assert!(is_ldr_start(slf));
        assert!(is_ldr_start(scr));
        assert!(is_ldr_start(scrlf));
        assert!(is_ldr_start(scrlfu));
    }

    #[test]
    fn is_ldr_start_rejects_ldr_preceeded_by_non_ws() {
        let s = b"xyz ##TITLE= abc";
        assert!(!is_ldr_start(s));
    }

    #[test]
    fn is_ldr_start_recognizes_ldr_start_with_special_chars() {
        let s = b"##.N_A/M2E$= abc";
        assert!(is_ldr_start(s));
    }

    #[test]
    fn is_ldr_start_rejects_ldr_non_start() {
        let s = b"#NAME= ##NOT_LDR=abc";
        assert!(!is_ldr_start(s));
    }

    #[test]
    fn normalize_ldr_start_removes_separators_from_label() {
        let s = b"##A B-C/D_E= abc";
        assert_eq!(b"##ABCDE= abc", normalize_ldr_start(s).unwrap().as_slice());
    }

    #[test]
    fn normalize_ldr_start_leaves_normalized_label_intact() {
        let s = b"##ABCDE= abc";
        assert_eq!(b"##ABCDE= abc", normalize_ldr_start(s).unwrap().as_slice());
    }

    #[test]
    fn normalize_ldr_start_removes_leading_ws() {
        let s = b"\t\n\x0B\x0C\r ##ABCDE= abc";
        assert_eq!(b"##ABCDE= abc", normalize_ldr_start(s).unwrap().as_slice());
    }

    #[test]
    fn normalize_ldr_start_turns_ascii_chars_to_upper_case() {
        // label: abcdeäöüÄÖÜ in ISO-8859-1 / Windows-1252 encoding
        let s = b"##abcde\xE4\xF6\xFC\xC4\xD6\xDC= abc";
        assert_eq!(
            b"##ABCDE\xE4\xF6\xFC\xC4\xD6\xDC= abc",
            normalize_ldr_start(s).unwrap().as_slice()
        );
    }

    #[test]
    fn normalize_ldr_start_rejects_missing_double_hashes() {
        let s = b"#LABEL= abc";
        assert!(normalize_ldr_start(s).is_err());
    }

    #[test]
    fn normalize_ldr_start_rejects_missing_equals() {
        let s = b"##LABEL abc";
        assert!(normalize_ldr_start(s).is_err());
    }

    #[test]
    fn parse_ldr_start_tokenizes_regular_ldr_start() {
        let s = b"##LABEL=abc";
        assert_eq!(
            ("LABEL".to_owned(), "abc".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_tokenizes_ldr_start_with_missing_value() {
        let s = b"##LABEL=";
        assert_eq!(
            ("LABEL".to_owned(), "".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_removes_first_leading_space_in_value() {
        let s = b"##LABEL=  abc";
        assert_eq!(
            ("LABEL".to_owned(), " abc".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_normalizes_label() {
        // label: abcdeäöüÄÖÜ in ISO-8859-1 encoding
        let s = b"\t\n\x0B\x0C\r ##abcde\xE4\xF6\xFC\xC4\xD6\xDC= abc";
        assert_eq!(
            ("ABCDEäöüÄÖÜ".to_owned(), "abc".to_owned()),
            parse_ldr_start(s).unwrap()
        );
    }

    #[test]
    fn parse_ldr_start_rejects_malformed_ldr_start() {
        let s = b"##LABEL";
        assert!(parse_ldr_start(s).is_err());
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
}
