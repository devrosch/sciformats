use super::JdxError;
use crate::utils::from_iso_8859_1_cstr;
use regex::bytes::Regex;

// (?-u) disable unicode mode, see: https://docs.rs/regex/latest/regex/#unicode
const LDR_START_REGEX: &str = "(?-u)^\\s*##(.*)=(.*)";

pub fn is_ldr_start(line: &[u8]) -> bool {
    // todo: init only once
    // this could be achieved by using https://crates.io/crates/lazy_static
    // as suggested in https://docs.rs/regex/1.10.5/regex/
    let regex = Regex::new(LDR_START_REGEX).unwrap();
    regex.is_match(line)
}

pub fn normalize_ldr_start(line: &[u8]) -> Result<Vec<u8>, JdxError> {
    // todo: init only once
    let regex = Regex::new(LDR_START_REGEX).unwrap();

    let caps = regex.captures(line);
    if caps.as_ref().is_none() || caps.as_ref().unwrap().len() < 3 {
        return Err(JdxError::new(&format!(
            "Malformed LDR start does not match pattern \"{}\": {}",
            LDR_START_REGEX,
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
