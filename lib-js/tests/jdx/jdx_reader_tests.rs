use super::{open_file, COMPOUND_FILE};
use sf_rs::{
    api::{Parser, Reader, SeekBufRead},
    jdx::{jdx_parser::JdxParser, jdx_reader::JdxReader},
};
use std::io::BufReader;
use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[allow(dead_code)]
#[wasm_bindgen_test]
fn jdx_read_valid_succeeds() {
    let (path, file) = open_file(COMPOUND_FILE);
    let buf_reader = BufReader::new(file);
    let buf_input: Box<dyn SeekBufRead> = Box::new(buf_reader);
    let parser = JdxParser::parse(&path, buf_input).unwrap();
    let reader = JdxReader::new(&path, parser);

    let root_node_result = &reader.read("/");

    assert!(root_node_result.is_ok());
}
