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
    api::{Column, Exporter, Parameter, PointXy, Reader, Table, Value},
    common::SfError,
};
use serde::{
    Serialize,
    ser::{SerializeSeq, SerializeStruct, Serializer},
};
use std::io::Write;

// see:
// https://github.com/serde-rs/json/issues/345#issuecomment-636215611
// https://serde.rs/impl-serialize.html#serializing-a-struct
// https://serde.rs/impl-serializer.html
// https://github.com/serde-rs/serde/issues/1665#issuecomment-549097541

pub struct JsonExporter<'a, R: Reader + ?Sized> {
    reader: &'a R,
}

impl<'a, R: Reader + ?Sized> JsonExporter<'a, R> {
    const EXPORT_NAME: &'static str = "sciformats";
    const EXPORT_VERSION: &'static str = "0.1.0";

    pub fn new(reader: &'a R) -> Self {
        Self { reader }
    }
}

impl<R: Reader + ?Sized> Exporter for JsonExporter<'_, R> {
    fn get_name(&self) -> &'static str {
        "Canonical JSON Exporter"
    }

    fn write(&mut self, writer: &mut dyn Write) -> Result<(), SfError> {
        let mut serializer = serde_json::Serializer::new(writer);
        let wrapper = NodeWrapper {
            path: "",
            reader: self.reader,
        };
        let export = JsonExport {
            name: Self::EXPORT_NAME,
            version: Self::EXPORT_VERSION,
            nodes: wrapper,
        };
        export
            .serialize(&mut serializer)
            .map_err(|e| SfError::from_source(e, "Error writing export."))
    }
}

struct JsonExport<'a, R: Reader + ?Sized> {
    name: &'static str,
    version: &'static str,
    nodes: NodeWrapper<'a, R>,
}

impl<'a, R: Reader + ?Sized> Serialize for JsonExport<'a, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("JsonExport", 3)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("version", &self.version)?;
        s.serialize_field("nodes", &self.nodes)?;
        s.end()
    }
}

struct NodeWrapper<'a, R: Reader + ?Sized> {
    path: &'a str,
    reader: &'a R,
}

struct ChildrenWrapper<'a, R: Reader + ?Sized> {
    paths: &'a [String],
    reader: &'a R,
}

struct MetadataWrapper<'a> {
    metadata: &'a Vec<(String, String)>,
}

struct MetadataItemWrapper<'a> {
    item: &'a (String, String),
}

impl<R: Reader + ?Sized> Serialize for NodeWrapper<'_, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let node = self.reader.read(self.path).map_err(|_e| {
            serde::ser::Error::custom(format!("error reading node: {}", self.path))
        })?;

        let mut serializer = serializer.serialize_struct("Node", 2)?;
        serializer.serialize_field("name", &node.name)?;
        serializer.serialize_field("parameters", &node.parameters)?;
        serializer.serialize_field("data", &node.data)?;
        let metadata_wrapper = MetadataWrapper {
            metadata: &node.metadata,
        };
        serializer.serialize_field("metadata", &metadata_wrapper)?;
        if let Some(table) = &node.table {
            // if table is some, serialize it
            serializer.serialize_field("table", table)?;
        };
        let mut child_paths = vec![];
        for (i, _name) in node.child_node_names.iter().enumerate() {
            let child_path = format!("{}/{}", self.path, i);
            child_paths.push(child_path);
        }
        let children_wrapper = ChildrenWrapper {
            paths: &child_paths,
            reader: self.reader,
        };
        serializer.serialize_field("children", &children_wrapper)?;
        serializer.end()
    }
}

impl<R: Reader + ?Sized> Serialize for ChildrenWrapper<'_, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_seq(Some(self.paths.len()))?;
        for path in self.paths {
            let child_wrapper = NodeWrapper {
                path,
                reader: self.reader,
            };
            serializer.serialize_element(&child_wrapper)?;
        }
        serializer.end()
    }
}

impl Serialize for MetadataWrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_seq(Some(self.metadata.len()))?;
        for item in self.metadata {
            let item_wrapper = MetadataItemWrapper { item };
            serializer.serialize_element(&item_wrapper)?;
        }
        serializer.end()
    }
}

impl Serialize for MetadataItemWrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_struct("metadataItem", 2)?;
        serializer.serialize_field("key", &self.item.0)?;
        serializer.serialize_field("value", &self.item.1)?;
        serializer.end()
    }
}

impl Serialize for Parameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Parameter::KeyValue(k, v) => {
                let mut serializer = serializer.serialize_struct("parameter", 2)?;
                serializer.serialize_field("key", k)?;
                serializer.serialize_field("value", v)?;
                serializer.end()
            }
            Parameter::Value(v) => {
                let mut serializer = serializer.serialize_struct("parameter", 1)?;
                serializer.serialize_field("value", v)?;
                serializer.end()
            }
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::String(v) => serializer.serialize_str(v),
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::I32(v) => serializer.serialize_i32(*v),
            Value::U32(v) => serializer.serialize_u32(*v),
            Value::I64(v) => serializer.serialize_i64(*v),
            Value::U64(v) => serializer.serialize_u64(*v),
            Value::F32(v) => serializer.serialize_f32(*v),
            Value::F64(v) => serializer.serialize_f64(*v),
        }
    }
}

impl Serialize for PointXy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_struct("point", 2)?;
        serializer.serialize_field("x", &self.x)?;
        serializer.serialize_field("y", &self.y)?;
        serializer.end()
        // Alternative:
        // let mut serializer = serializer.serialize_tuple(2)?;
        // serializer.serialize_element(&self.x)?;
        // serializer.serialize_element(&self.y)?;
        // serializer.end()
    }
}

impl Serialize for Table {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_struct("table", 2)?;
        serializer.serialize_field("columnNames", &self.column_names)?;
        serializer.serialize_field("rows", &self.rows)?;
        serializer.end()
    }
}

impl Serialize for Column {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Alternative: serialize as map.
        // let mut serializer = serializer.serialize_map(Some(1))?;
        // serializer.serialize_key(&self.key)?;
        // serializer.serialize_value(&self.name)?;
        // serializer.end()
        let mut serializer = serializer.serialize_struct("column", 1)?;
        serializer.serialize_field("key", &self.key)?;
        serializer.serialize_field("name", &self.name)?;
        serializer.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Column, ExportFormat, Node, Parameter, Table},
        common::SfError,
    };
    use core::str;
    use serde_json::{Value, json};
    use std::collections::HashMap;

    struct StubReader {}

    impl Reader for StubReader {
        fn read(&self, path: &str) -> Result<Node, SfError> {
            let root = Node {
                name: "root node name".to_owned(),
                parameters: vec![
                    Parameter::from_str_str("param String", "abc"),
                    Parameter::from_str_bool("param bool", true),
                    Parameter::from_str_i32("param i32", -1),
                    Parameter::from_str_u32("param u32", 1),
                    Parameter::from_str_i64("param i64", -2),
                    Parameter::from_str_u64("param u64", 2),
                    Parameter::from_str_f32("param f32", -1.0),
                    Parameter::from_str_f64("param f64", 1.0),
                ],
                data: vec![PointXy { x: 1.0, y: 2.0 }, PointXy { x: 3.0, y: 4.0 }],
                metadata: vec![
                    ("mk0".to_owned(), "mv0".to_owned()),
                    ("mk1".to_owned(), "mv1".to_owned()),
                ],
                table: Some(Table {
                    column_names: vec![Column {
                        key: "col key".to_owned(),
                        name: "col name".to_owned(),
                    }],
                    rows: vec![
                        HashMap::from([(
                            "col key".to_owned(),
                            api::Value::String("String value".to_owned()),
                        )]),
                        HashMap::from([("col key".to_owned(), api::Value::Bool(true))]),
                        HashMap::from([("col key".to_owned(), api::Value::I32(-1))]),
                        HashMap::from([("col key".to_owned(), api::Value::U32(1))]),
                        HashMap::from([("col key".to_owned(), api::Value::I64(-2))]),
                        HashMap::from([("col key".to_owned(), api::Value::U64(2))]),
                        HashMap::from([("col key".to_owned(), api::Value::F32(-1.0))]),
                        HashMap::from([("col key".to_owned(), api::Value::F64(1.0))]),
                    ],
                }),
                // child_node_names: vec![],
                child_node_names: vec![
                    "child node name 0".to_owned(),
                    "child node name 1".to_owned(),
                ],
            };
            let child0 = Node {
                name: "child node name 0".to_owned(),
                parameters: vec![],
                data: vec![],
                metadata: vec![],
                table: None,
                child_node_names: vec![],
            };
            let child1 = Node {
                name: "child node name 1".to_owned(),
                parameters: vec![],
                data: vec![],
                metadata: vec![],
                table: None,
                child_node_names: vec![],
            };

            match path {
                "" | "/" => Ok(root),
                "/0" => Ok(child0),
                "/1" => Ok(child1),
                _ => Err(SfError::new(&format!("Illegal path: {}", path)).into()),
            }
        }
    }

    #[test]
    fn serializes_node_tree_to_json() {
        let reader = StubReader {};
        let mut export = vec![];
        assert_eq!(0, export.len());

        reader.export(ExportFormat::Json, &mut export).unwrap();
        assert!(export.len() > 0);

        // https://docs.rs/serde_json/latest/serde_json/fn.to_value.html#example
        let output_str = String::from_utf8(export).unwrap();
        let output_json: Value = serde_json::from_str(&output_str).unwrap();

        let expected = json!({
            "name": "sciformats",
            "version": "0.1.0",
            "nodes":{
                "name": "root node name",
                "parameters": [
                    {"key": "param String", "value": "abc"},
                    {"key": "param bool", "value": true},
                    {"key": "param i32", "value": -1},
                    {"key": "param u32", "value": 1},
                    {"key": "param i64", "value": -2},
                    {"key": "param u64", "value": 2},
                    {"key": "param f32", "value": -1.0},
                    {"key": "param f64", "value": 1.0},
                ],
                "data": [
                    { "x": 1.0, "y": 2.0},
                    { "x": 3.0, "y": 4.0},
                    // [1.0, 2.0],
                    // [3.0, 4.0],
                ],
                "metadata": [
                    {"key": "mk0", "value": "mv0"},
                    {"key": "mk1", "value": "mv1"},
                    // ["mk0", "mv0"],
                    // ["mk1", "mv1"],
                ],
                "table": {
                    // "columnNames": [{"col key": "col name"}],
                    "columnNames": [{"key": "col key", "name": "col name"}],
                    "rows": [
                        {"col key": "String value"},
                        {"col key": true},
                        {"col key": -1},
                        {"col key": 1},
                        {"col key": -2},
                        {"col key": 2},
                        {"col key": -1.0},
                        {"col key": 1.0}
                    ],
                },
                "children": [
                    {
                        "name": "child node name 0",
                        "parameters": [], "data": [],
                        "metadata": [],
                        "children": [],
                    },
                    {
                        "name": "child node name 1",
                        "parameters": [], "data": [],
                        "metadata": [],
                        "children": [],
                    },
                ]
            }
        });

        assert_eq!(expected, output_json);
    }
}
