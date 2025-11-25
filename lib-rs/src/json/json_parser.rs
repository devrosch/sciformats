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

use crate::api::Parser;
use crate::common::SfError;
use serde::Deserialize;
use std::{
    collections::HashMap,
    io::{Read, Seek},
};

pub struct JsonParser {}

impl<T: Seek + Read + 'static> Parser<T> for JsonParser {
    type R = JsonDocument;
    type E = SfError;

    fn parse(_name: &str, input: T) -> Result<Self::R, Self::E> {
        let doc: JsonDocument = serde_json::from_reader(input)
            .map_err(|e| SfError::from_source(e, "Error deserializing JSON document."))?;
        Ok(doc)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonDocument {
    pub name: String,
    pub parameters: Vec<JsonParameter>,
    pub data: Vec<JsonDataItem>,
    pub metadata: Vec<JsonMetadataItem>,
    pub table: Option<JsonTable>,
    pub children: Vec<JsonDocument>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct JsonParameter {
    pub key: Option<String>,
    pub value: JsonValue,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum JsonValue {
    String(String),
    Bool(bool),
    I64(i64), // not a JSON type but useful for reading numbers with integer values
    Number(f64),
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct JsonDataItem {
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct JsonMetadataItem {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct JsonTable {
    #[serde(rename(deserialize = "columnNames"))]
    pub column_names: Vec<JsonTableColumn>,
    pub rows: Vec<HashMap<String, JsonValue>>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct JsonTableColumn {
    pub key: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parses_json() {
        const JSON: &str = r#"
            {
                "name": "Root node",
                "parameters": [
                    {
                        "key": "Parameter key 0",
                        "value": "Parameter value 0"
                    },
                    {
                        "value": "Parameter value 1"
                    },
                    {
                        "key": "Parameter key 2",
                        "value": true
                    },
                    {
                        "key": "Parameter key 3",
                        "value": 123
                    },
                    {
                        "key": "Parameter key 4",
                        "value": 123.456
                    },
                    {
                        "key": "Parameter key 5",
                        "value": -123
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
                            "col_key1": true
                        },
                        {
                            "col_key1": 123.456
                        }
                    ]
                },
                "children": [
                    {
                        "name": "Nested node 0",
                        "parameters": [],
                        "data": [],
                        "metadata": [],
                        "children": []
                    },
                    {
                        "name": "Nested node 1",
                        "parameters": [],
                        "data": [],
                        "metadata": [],
                        "children": []
                    }
                ]
            }"#;
        let path = "example.json";
        let reader = Cursor::new(JSON);

        let root = JsonParser::parse(path, reader).unwrap();

        assert_eq!("Root node", &root.name);

        assert_eq!(6, root.parameters.len());
        assert_eq!(
            JsonParameter {
                key: Some("Parameter key 0".to_owned()),
                value: JsonValue::String("Parameter value 0".to_owned())
            },
            root.parameters[0]
        );
        assert_eq!(
            JsonParameter {
                key: None,
                value: JsonValue::String("Parameter value 1".to_owned())
            },
            root.parameters[1]
        );
        assert_eq!(
            JsonParameter {
                key: Some("Parameter key 2".to_owned()),
                value: JsonValue::Bool(true)
            },
            root.parameters[2]
        );
        assert_eq!(
            JsonParameter {
                key: Some("Parameter key 3".to_owned()),
                value: JsonValue::I64(123)
            },
            root.parameters[3]
        );
        assert_eq!(
            JsonParameter {
                key: Some("Parameter key 4".to_owned()),
                value: JsonValue::Number(123.456)
            },
            root.parameters[4]
        );
        assert_eq!(
            JsonParameter {
                key: Some("Parameter key 5".to_owned()),
                value: JsonValue::I64(-123)
            },
            root.parameters[5]
        );

        assert_eq!(2, root.data.len());
        assert_eq!(JsonDataItem { x: 0.0, y: 10.1 }, root.data[0]);
        assert_eq!(JsonDataItem { x: 1.0, y: 1000.01 }, root.data[1]);

        assert_eq!(2, root.metadata.len());
        assert_eq!(
            JsonMetadataItem {
                key: "x.unit".to_owned(),
                value: "arbitrary unit".to_owned()
            },
            root.metadata[0]
        );
        assert_eq!(
            JsonMetadataItem {
                key: "y.unit".to_owned(),
                value: "another arbitrary unit".to_owned()
            },
            root.metadata[1]
        );

        assert!(root.table.is_some());
        assert_eq!(1, root.table.as_ref().unwrap().column_names.len());
        assert_eq!(
            JsonTableColumn {
                key: "col_key1".to_owned(),
                name: "Column name 1".to_owned()
            },
            root.table.as_ref().unwrap().column_names[0]
        );
        assert_eq!(3, root.table.as_ref().unwrap().rows.len());
        let row0 = &root.table.as_ref().unwrap().rows[0];
        assert_eq!(1, row0.len());
        assert!(row0.contains_key("col_key1"));
        assert_eq!(
            Some(&JsonValue::String("Cell value 1".to_owned())),
            row0.get("col_key1")
        );
        let row1 = &root.table.as_ref().unwrap().rows[1];
        assert_eq!(1, row1.len());
        assert!(row1.contains_key("col_key1"));
        assert_eq!(Some(&JsonValue::Bool(true)), row1.get("col_key1"));
        let row2 = &root.table.as_ref().unwrap().rows[2];
        assert_eq!(1, row2.len());
        assert!(row2.contains_key("col_key1"));
        assert_eq!(Some(&JsonValue::Number(123.456)), row2.get("col_key1"));

        assert_eq!(2, root.children.len());
        let nested0 = &root.children[0];
        assert_eq!("Nested node 0", nested0.name);
        assert!(nested0.parameters.is_empty());
        assert!(nested0.data.is_empty());
        assert!(nested0.metadata.is_empty());
        assert!(nested0.table.is_none());
        assert!(nested0.children.is_empty());

        let nested1 = &root.children[1];
        assert_eq!("Nested node 1", nested1.name);
        assert!(nested1.parameters.is_empty());
        assert!(nested1.data.is_empty());
        assert!(nested1.metadata.is_empty());
        assert!(nested1.table.is_none());
        assert!(nested1.children.is_empty());
    }

    #[test]
    fn fails_parsing_unexpected_json() {
        const SOME_JSON: &str = r#"
            {
                somefield: "some value"
            }"#;
        let result = JsonParser::parse("somepath.json", Cursor::new(SOME_JSON));
        assert!(result.is_err());
    }

    #[test]
    fn fails_parsing_extra_fields() {
        const EXTRA_PARAM_JSON: &str = r#"
            {
                "name": "Node 0",
                "extraparam": "extra",
                "parameters": [],
                "data": [],
                "metadata": [],
                "children": []
            }
        "#;
        let result = JsonParser::parse("extra.json", Cursor::new(EXTRA_PARAM_JSON));
        assert!(result.is_err());
    }
}
