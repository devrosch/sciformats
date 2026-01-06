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

use crate::{
    api::{Parser, Reader, Scanner},
    common::SfError,
    json::{json_parser::JsonParser, json_reader::JsonReader},
    utils::{from_iso_8859_1_cstr, is_recognized_extension},
};
use std::io::{Read, Seek, SeekFrom};

#[derive(Default)]
pub struct JsonScanner {}

impl JsonScanner {
    const ACCEPTED_EXTENSIONS: [&'static str; 1] = ["json"];
    const EXPECTED_STRINGS: [&'static str; 4] =
        ["\"name\"", "\"sciformats\"", "\"version\"", "\"0.1.0\""];
}

impl JsonScanner {
    pub fn new() -> Self {
        Self::default()
    }

    fn read_start<T: Read + Seek>(input: &mut T) -> std::io::Result<String> {
        let pos = input.stream_position()?;
        let mut buf: Vec<u8> = vec![];
        let res = input.take(128).read_to_end(&mut buf);
        input.seek(SeekFrom::Start(pos))?;

        match res {
            Err(e) => Err(e),
            Ok(_) => Ok(from_iso_8859_1_cstr(&buf)),
        }
    }
}

impl<T: Seek + Read + 'static> Scanner<T> for JsonScanner {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        if !is_recognized_extension(path, &Self::ACCEPTED_EXTENSIONS) {
            return false;
        };

        let start = match Self::read_start(input) {
            Ok(s) => s,
            Err(_) => return false,
        };

        for expected in Self::EXPECTED_STRINGS.iter() {
            if !start.contains(expected) {
                return false;
            }
        }

        true
    }

    fn get_reader(&self, path: &str, input: T) -> Result<Box<dyn Reader>, SfError> {
        let doc = JsonParser::parse(path, input)?;
        let reader = JsonReader::new(path, doc);
        Ok(Box::new(reader))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SINGLE_NODE_JSON: &str = r#"
        {
            "format": "sciformats",
            "version": "0.1.0",
            "nodes": {
                "name": "Single Node",
                "parameters": [
                    {
                        "key": "Parameter key 1",
                        "value": "Parameter value 1"
                    },
                    {
                        "key": "Parameter key 2",
                        "value": "Parameter value 2"
                    }
                ],
                "data": [
                    {
                        "x": 0,
                        "y": 10.1
                    },
                    {
                        "x": 1,
                        "y": 1000.01
                    }
                ],
                "metadata": [
                    {
                        "key": "x.unit",
                        "value": "arbitrary unit"
                    },
                    {
                        "key": "y.unit",
                        "value": "another arbitrary unit"
                    }
                ],
                "table": {
                    "columnNames": [
                        {
                            "key": "col_key1",
                            "name": "Column name 1"
                        }
                    ],
                    "rows": [
                        {
                            "col_key1": "Cell value 1"
                        },
                        {
                            "col_key1": "Cell value 2"
                        }
                    ]
                },
                "children": []
            }
        }"#;

    #[test]
    fn recognizes_single_node_json() {
        let path = "example.json";
        let mut input = Cursor::new(SINGLE_NODE_JSON);
        let scanner = JsonScanner::new();

        assert_eq!(true, scanner.is_recognized(path, &mut input));
    }

    #[test]
    fn rejects_illegal_json() {
        let path = "example.json";
        let content_missing_format = r#"
            {
                "version": "0.1.0",
                "nodes": {
                    "name": "Single Node",
                    "parameters": [],
                    "data": [],
                    "metadata": [],
                    "children": []
                }
            "#;
        let mut input = Cursor::new(content_missing_format);
        let scanner = JsonScanner::new();

        assert!(!scanner.is_recognized(path, &mut input));
    }

    #[test]
    fn provides_reader_for_valid_json() {
        let path = "example.json";
        let input = Cursor::new(SINGLE_NODE_JSON);
        let scanner = JsonScanner::new();

        assert!(scanner.get_reader(path, input).is_ok());
    }
}
