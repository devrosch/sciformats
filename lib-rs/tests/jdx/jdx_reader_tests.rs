use super::{COMPOUND_FILE, open_file};
use sf_rs::{
    api::{Parameter, Parser, Reader, SeekBufRead},
    jdx::{jdx_parser::JdxParser, jdx_reader::JdxReader},
};
use std::io::BufReader;

#[test]
fn jdx_read_valid_succeeds() {
    let (path, file) = open_file(COMPOUND_FILE);
    let buf_reader = BufReader::new(file);
    let buf_input: Box<dyn SeekBufRead> = Box::new(buf_reader);
    let parser = JdxParser::parse(&path, buf_input).unwrap();
    let reader = JdxReader::new(&path, parser);

    let root_node = &reader.read("/").unwrap();
    assert_eq!(COMPOUND_FILE, root_node.name);
    assert!(
        root_node
            .parameters
            .contains(&Parameter::from_str_str("TITLE", "Root LINK BLOCK"))
    );

    let audit_trail_node = &reader.read("/8").unwrap();
    assert_eq!("", audit_trail_node.name);
    let audit_trail = &reader.read("/8/0").unwrap();
    assert!(audit_trail.table.is_some());
}
