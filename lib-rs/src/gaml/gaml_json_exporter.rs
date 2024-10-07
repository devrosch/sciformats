use crate::api::{Column, Exporter, Parameter, PointXy, Reader, Table, Value};
use serde::ser::{Serialize, SerializeSeq, SerializeStruct, SerializeTuple, Serializer};
use std::{error::Error, io::Write};

// see:
// https://github.com/serde-rs/json/issues/345#issuecomment-636215611
// https://serde.rs/impl-serialize.html#serializing-a-struct
// https://serde.rs/impl-serializer.html
// https://github.com/serde-rs/serde/issues/1665#issuecomment-549097541

pub struct GamlJsonExporter<'a, R: Reader> {
    reader: &'a R,
}

impl<R: Reader> Exporter for GamlJsonExporter<'_, R> {
    fn get_name() -> &'static str {
        "GAML JSON Exporter"
    }

    fn write(&mut self, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let mut serializer = serde_json::Serializer::new(writer);
        let wrapper = NodeWrapper {
            path: "",
            reader: self.reader,
        };
        wrapper.serialize(&mut serializer).map_err(|e| e.into())
    }
}

struct NodeWrapper<'a, R: Reader> {
    path: &'a str,
    reader: &'a R,
}
struct ChildrenWrapper<'a, R: Reader> {
    paths: &'a [String],
    reader: &'a R,
}

impl<R: Reader> Serialize for NodeWrapper<'_, R> {
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
        serializer.serialize_field("metadata", &node.metadata)?;
        match &node.table {
            Some(table) => serializer.serialize_field("table", table)?,
            // if table is none, serialize as empty table
            None => serializer.serialize_field(
                "table",
                &Table {
                    column_names: vec![],
                    rows: vec![],
                },
            )?,
        }

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

impl<R: Reader> Serialize for ChildrenWrapper<'_, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_seq(Some(self.paths.len()))?;
        for path in self.paths {
            let child_wrapper = NodeWrapper {
                path: &path,
                reader: self.reader,
            };
            serializer.serialize_element(&child_wrapper)?;
        }
        serializer.end()
    }
}

impl Serialize for Parameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_struct("parameter", 2)?;
        serializer.serialize_field("key", &self.key)?;
        serializer.serialize_field("value", &self.value)?;
        serializer.end()
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
        // let mut serializer = serializer.serialize_struct("point", 2)?;
        // serializer.serialize_field("x", &self.x)?;
        // serializer.serialize_field("y", &self.y)?;
        let mut serializer = serializer.serialize_tuple(2)?;
        serializer.serialize_element(&self.x)?;
        serializer.serialize_element(&self.y)?;
        serializer.end()
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
        // todo: serialize as map?
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
        api::{self, Column, Node, Parameter, Table},
        gaml::GamlError,
    };
    use core::str;
    use serde_json::{json, Value};
    use std::collections::HashMap;

    struct StubReader {}

    impl Reader for StubReader {
        fn read(&self, path: &str) -> Result<Node, Box<dyn std::error::Error>> {
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
                _ => Err(GamlError::new(&format!("Illegal path: {}", path)).into()),
            }
        }
    }

    #[test]
    fn serializes_node_tree_to_json() {
        let reader = StubReader {};
        let mut exporter = GamlJsonExporter { reader: &reader };
        let mut export = vec![];

        assert_eq!(0, export.len());
        exporter.write(&mut export).unwrap();
        assert!(export.len() > 0);

        // https://docs.rs/serde_json/latest/serde_json/fn.to_value.html#example
        let output_str = String::from_utf8(export).unwrap();
        let output_json: Value = serde_json::from_str(&output_str).unwrap();
        assert_eq!(
            json!({
                "name": "root node name",
                "parameters": [
                    { "key": "param String", "value": "abc"},
                    { "key": "param bool", "value": true},
                    { "key": "param i32", "value": -1},
                    { "key": "param u32", "value": 1},
                    { "key": "param i64", "value": -2},
                    { "key": "param u64", "value": 2},
                    { "key": "param f32", "value": -1.0},
                    { "key": "param f64", "value": 1.0},
                ],
                "data": [
                    // { "x": 1.0, "y": 2.0},
                    // { "x": 3.0, "y": 4.0},
                    [1.0, 2.0],
                    [3.0, 4.0],
                ],
                "metadata": [
                    ["mk0", "mv0"],
                    ["mk1", "mv1"],
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
                // "children": [],
                "children": [
                    { "name": "child node name 0", "parameters": [], "data": [], "metadata": [], "table": { "columnNames": [], "rows": [] }, "children": [], },
                    { "name": "child node name 1", "parameters": [], "data": [], "metadata": [], "table": { "columnNames": [], "rows": [] }, "children": [], },
                ]
            }),
            output_json,
        );
    }
}
