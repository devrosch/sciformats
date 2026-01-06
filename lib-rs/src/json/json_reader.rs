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

use std::io::{Read, Seek};

use crate::{
    api::{Column, Node, Parameter, PointXy, Reader, Table, Value},
    common::SfError,
    json::json_parser::{JsonDocument, JsonNode, JsonValue},
};

pub struct JsonReader<T: Seek + Read> {
    _path: String,
    file: JsonDocument<T>,
}

impl<T: Seek + Read> Reader for JsonReader<T> {
    fn read(&self, node_path: &str) -> Result<Node, SfError> {
        let json_node = self.file.get_node(node_path)?;
        let node = Self::map_node(json_node)?;
        Ok(node)
    }
}

impl<T: Seek + Read> JsonReader<T> {
    pub fn new(path: &str, file: JsonDocument<T>) -> Self {
        Self {
            _path: path.to_owned(),
            file,
        }
    }

    fn map_node(json_node: JsonNode) -> Result<Node, SfError> {
        // Map name
        let name = json_node.name.clone();

        // Map parameters
        let parameters = json_node
            .parameters
            .iter()
            .map(|json_param| match (&json_param.key, &json_param.value) {
                (Some(key), JsonValue::String(s)) => Parameter::from_str_str(key, s),
                (Some(key), JsonValue::Bool(b)) => Parameter::from_str_bool(key, *b),
                (Some(key), JsonValue::I64(i)) => Parameter::from_str_i64(key, *i),
                (Some(key), JsonValue::Number(f)) => Parameter::from_str_f64(key, *f),
                (None, JsonValue::String(s)) => Parameter::from_str(s),
                (None, JsonValue::Bool(b)) => Parameter::from_bool(*b),
                (None, JsonValue::I64(i)) => Parameter::from_i64(*i),
                (None, JsonValue::Number(f)) => Parameter::from_f64(*f),
            })
            .collect();

        // Map data
        let data = json_node
            .data
            .iter()
            .map(|json_data_point| PointXy::new(json_data_point.x, json_data_point.y))
            .collect();

        // Map metadata
        let metadata = json_node
            .metadata
            .iter()
            .map(|json_metadata_item| {
                (
                    json_metadata_item.key.clone(),
                    json_metadata_item.value.clone(),
                )
            })
            .collect();

        // Map table
        let table = if let Some(json_table) = &json_node.table {
            let column_names = json_table
                .column_names
                .iter()
                .map(|json_column| Column::new(&json_column.key, &json_column.name))
                .collect();
            let rows = json_table
                .rows
                .iter()
                .map(|row| {
                    let mut map = std::collections::HashMap::new();
                    for (json_key, json_value) in row.iter() {
                        let value = match json_value {
                            JsonValue::String(s) => Value::String(s.clone()),
                            JsonValue::Bool(b) => Value::Bool(*b),
                            JsonValue::I64(i) => Value::I64(*i),
                            JsonValue::Number(f) => Value::F64(*f),
                        };
                        map.insert(json_key.clone(), value);
                    }
                    map
                })
                .collect();
            Some(Table { column_names, rows })
        } else {
            None
        };

        // Map child node names
        let child_node_names = json_node.child_node_names;

        Ok(Node {
            name,
            parameters,
            data,
            metadata,
            table,
            child_node_names,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::json_parser::{
        JsonLazyNode, JsonMetadataItem, JsonParameter, JsonTable, JsonTableColumn, JsonValue,
    };
    use sciformats_serde_json::span::Span;
    use std::{cell::RefCell, collections::HashMap, io::Cursor, rc::Rc};

    fn create_sample_json_doc() -> JsonDocument<Cursor<String>> {
        let root_data = r#""data": [{"x": 0, "y": 10.1}, {"x": 1, "y": 1000.01}], "blank": []"#;
        let reader = Cursor::new(root_data.to_owned());
        let reader_rc: Rc<RefCell<Cursor<String>>> = Rc::new(RefCell::new(reader));

        let root_node = JsonLazyNode {
            name: "Root node".to_owned(),
            parameters: vec![
                JsonParameter {
                    key: Some("Parameter key 0".to_owned()),
                    value: JsonValue::String("Parameter value 0".to_owned()),
                },
                JsonParameter {
                    key: None,
                    value: JsonValue::String("Parameter value 1".to_owned()),
                },
                JsonParameter {
                    key: Some("Parameter key 2".to_owned()),
                    value: JsonValue::Bool(true),
                },
                JsonParameter {
                    key: Some("Parameter key 3".to_owned()),
                    value: JsonValue::I64(123),
                },
                JsonParameter {
                    key: Some("Parameter key 4".to_owned()),
                    value: JsonValue::Number(123.456),
                },
                JsonParameter {
                    key: Some("Parameter key 5".to_owned()),
                    value: JsonValue::I64(-123),
                },
            ],
            data: Span { span: 7..81 }, // [{"x": 0, "y": 10.1}, {"x": 1, "y": 1000.01}]
            metadata: vec![
                JsonMetadataItem {
                    key: "x.unit".to_owned(),
                    value: "arbitrary unit".to_owned(),
                },
                JsonMetadataItem {
                    key: "y.unit".to_owned(),
                    value: "another arbitrary unit".to_owned(),
                },
            ],
            table: Some(JsonTable {
                column_names: vec![JsonTableColumn {
                    key: "col_key1".to_owned(),
                    name: "Column name 1".to_owned(),
                }],
                rows: vec![
                    HashMap::from([(
                        "col_key1".to_owned(),
                        JsonValue::String("Cell value 1".to_owned()),
                    )]),
                    HashMap::from([("col_key1".to_owned(), JsonValue::Bool(true))]),
                    HashMap::from([("col_key1".to_owned(), JsonValue::Number(123.456))]),
                ],
            }),
            children: vec![
                JsonLazyNode {
                    name: "Nested node 0".to_owned(),
                    parameters: vec![],
                    data: Span { span: 64..66 }, // []
                    metadata: vec![],
                    table: None,
                    children: vec![],
                },
                JsonLazyNode {
                    name: "Nested node 1".to_owned(),
                    parameters: vec![],
                    data: Span { span: 64..66 }, // []
                    metadata: vec![],
                    table: None,
                    children: vec![],
                },
            ],
        };

        let doc = JsonDocument {
            format: "sciformats".to_owned(),
            version: "0.1.0".to_owned(),
            nodes: root_node,
            input: reader_rc,
        };

        doc
    }

    #[test]
    fn maps_root_node() {
        let doc = create_sample_json_doc();
        let reader = JsonReader::new("example.json", doc);
        let root = &reader.read("/").unwrap();

        assert_eq!(root.name, "Root node");
        assert_eq!(root.parameters.len(), 6);
        assert_eq!(
            &Parameter::from_str_str("Parameter key 0", "Parameter value 0"),
            &root.parameters[0]
        );
        assert_eq!(
            &Parameter::from_str("Parameter value 1"),
            &root.parameters[1]
        );
        assert_eq!(
            &Parameter::from_str_bool("Parameter key 2", true),
            &root.parameters[2]
        );
        assert_eq!(
            &Parameter::from_str_i64("Parameter key 3", 123),
            &root.parameters[3]
        );
        assert_eq!(
            &Parameter::from_str_f64("Parameter key 4", 123.456),
            &root.parameters[4]
        );
        assert_eq!(
            &Parameter::from_str_i64("Parameter key 5", -123),
            &root.parameters[5]
        );

        assert_eq!(2, root.data.len());
        assert_eq!(&PointXy::new(0.0, 10.1), &root.data[0]);
        assert_eq!(&PointXy::new(1.0, 1000.01), &root.data[1]);

        assert_eq!(2, root.metadata.len());
        assert_eq!(
            &("x.unit".to_owned(), "arbitrary unit".to_owned()),
            &root.metadata[0]
        );
        assert_eq!(
            &("y.unit".to_owned(), "another arbitrary unit".to_owned()),
            &root.metadata[1]
        );

        assert!(root.table.is_some());
        let table = root.table.as_ref().unwrap();
        assert_eq!(1, table.column_names.len());
        assert_eq!(
            &Column::new("col_key1", "Column name 1"),
            &table.column_names[0]
        );
        assert_eq!(3, table.rows.len());
        assert_eq!(1, table.rows[0].len());
        assert_eq!(
            Value::String("Cell value 1".to_owned()),
            table.rows[0]["col_key1"]
        );
        assert_eq!(1, table.rows[1].len());
        assert_eq!(Value::Bool(true), table.rows[1]["col_key1"]);
        assert_eq!(1, table.rows[2].len());
        assert_eq!(Value::F64(123.456), table.rows[2]["col_key1"]);

        assert_eq!(
            root.child_node_names,
            vec!["Nested node 0", "Nested node 1"]
        );
    }

    #[test]
    fn maps_child_nodes() {
        let doc = create_sample_json_doc();
        let reader = JsonReader::new("example.json", doc);

        let child_node0 = reader.read("/0").unwrap();
        assert_eq!(child_node0.name, "Nested node 0");
        assert!(child_node0.parameters.is_empty());
        assert!(child_node0.data.is_empty());
        assert!(child_node0.metadata.is_empty());
        assert!(child_node0.table.is_none());
        assert!(child_node0.child_node_names.is_empty());

        let child_node1 = reader.read("/1").unwrap();
        assert_eq!(child_node1.name, "Nested node 1");
        assert!(child_node1.parameters.is_empty());
        assert!(child_node1.data.is_empty());
        assert!(child_node1.metadata.is_empty());
        assert!(child_node1.table.is_none());
        assert!(child_node1.child_node_names.is_empty());

        let err = reader.read("/2").unwrap_err();
        assert_eq!(err.to_string(), "Illegal node path: /2");
    }
}
