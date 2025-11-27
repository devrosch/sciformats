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

use super::{EXAMPLE, open_file};
use sciformats::{
    api::{ExportFormat, Parser, Reader},
    json::{json_parser::JsonParser, json_reader::JsonReader},
};
use serde_json::Value;
use std::io::{Cursor, Read, Seek};

#[test]
fn json_roundtrip_succeeds() {
    let (path, mut file) = open_file(EXAMPLE);

    // read original content
    let mut original_content = vec![];
    file.read_to_end(&mut original_content).unwrap();
    file.seek(std::io::SeekFrom::Start(0)).unwrap();

    // read and export content using JsonReader
    let mut exported_content = vec![];
    let doc = JsonParser::parse(&path, file).unwrap();
    let reader = JsonReader::new(&path, doc);
    let mut writer = Cursor::new(&mut exported_content);
    reader.export(ExportFormat::Json, &mut writer).unwrap();

    // turn original and exported content as json values
    let original_json: Value = serde_json::from_slice(&original_content).unwrap();
    let exported_json: Value = serde_json::from_slice(&exported_content).unwrap();

    assert_eq!(original_json, exported_json);
}
