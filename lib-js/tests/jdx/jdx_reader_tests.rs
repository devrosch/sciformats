// Copyright (c) 2025 Robert Schiwon
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use super::{COMPOUND_FILE, open_file};
use sciformats::{
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
