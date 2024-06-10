use regex::bytes::Regex;

pub fn is_ldr_start(line: &[u8]) -> bool {
    // todo: init only once
    let regex = Regex::new("^\\s*##.*=.*").unwrap();
    regex.is_match(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_regular_ldr_start() {
        let s = b"##TITLE= abc";
        assert!(is_ldr_start(s));
    }

    #[test]
    fn recognizes_ldr_start_with_leading_ws() {
        let s = b"\t\n\x0b\x0d\r##TITLE= abc";
        assert!(is_ldr_start(s));
    }

    #[test]
    fn recognizes_ldr_start_with_trailing_line_break() {
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
    fn rejects_ldr_preceeded_by_non_ws() {
        let s = b"xyz ##TITLE= abc";
        assert!(!is_ldr_start(s));
    }

    #[test]
    fn recognizes_ldr_start_with_special_chars() {
        let s = b"##.N_A/M2E$= abc";
        assert!(is_ldr_start(s));
    }

    #[test]
    fn rejects_ldr_non_start() {
        let s = b"#NAME= ##NOT_LDR=abc";
        assert!(!is_ldr_start(s));
    }
}
