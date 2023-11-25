use std::fmt;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    io::{Read, Seek},
};
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_family = "wasm")]
use wasm_bindgen::JsValue;

#[derive(Debug, PartialEq)]
pub struct SfError {
    message: String,
}

/// A generic error.
impl SfError {
    pub fn new(msg: &str) -> SfError {
        SfError {
            message: msg.into(),
        }
    }
}

impl Error for SfError {}

impl fmt::Display for SfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Parses a (readonly) data set.
pub trait Parser<T: Read + Seek> {
    type R;

    fn parse(name: &str, input: T) -> Result<Self::R, Box<dyn Error>>;
}

/// Scans a data set and provides a reader for recognized formats.
pub trait Scanner<T: Read + Seek> {
    /// Returns whether a data set is recognized. Shallow check.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the data set.
    /// * `input` - The readonly raw data set.
    ///
    /// # Notes
    ///
    /// `path` may be a path in the OS filesystem, in a remote location
    /// or just the file name, e.g. when run in a browser.
    ///
    /// The cursor in `input` is not guaranteed to be reset upon return.
    fn is_recognized(&self, path: &str, input: &mut T) -> bool;

    /// Returns a reader for a recognized data set.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the data set.
    /// * `input` - The readonly raw data set.
    ///
    /// # Notes
    ///
    /// `path` may be a path in the OS filesystem, in a remote location
    /// or just the file name, e.g. when run in a browser.
    ///
    /// May fail even if `is_recognized()` returns true.
    fn get_reader(&self, path: &str, input: T) -> Result<Box<dyn Reader>, Box<dyn Error>>;
}

/// Provides a harmonized view for reading a scientifc data set.
pub trait Reader {
    /// Returns a Node read from the data set.
    ///
    /// # Arguments
    ///
    /// * `path` - The path inside the data set identifying the Node.
    fn read(&self, path: &str) -> Result<Node, Box<dyn Error>>;
}

/// A parameter value.
#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Bool(bool),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(v) => write!(f, "{}", v),
            Value::Bool(v) => write!(f, "{}", v),
            Value::I32(v) => write!(f, "{}", v),
            Value::U32(v) => write!(f, "{}", v),
            Value::I64(v) => write!(f, "{}", v),
            Value::U64(v) => write!(f, "{}", v),
            Value::F32(v) => write!(f, "{}", v),
            Value::F64(v) => write!(f, "{}", v),
        }
    }
}

/// A key value parameter.
#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub key: String,
    pub value: Value,
}

impl Parameter {
    pub fn from_str_str(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: Value::String(value.into()),
        }
    }

    pub fn from_str_bool(key: impl Into<String>, value: bool) -> Self {
        Self {
            key: key.into(),
            value: Value::Bool(value),
        }
    }

    pub fn from_str_i32(key: impl Into<String>, value: i32) -> Self {
        Self {
            key: key.into(),
            value: Value::I32(value),
        }
    }

    pub fn from_str_u32(key: impl Into<String>, value: u32) -> Self {
        Self {
            key: key.into(),
            value: Value::U32(value),
        }
    }

    pub fn from_str_i64(key: impl Into<String>, value: i64) -> Self {
        Self {
            key: key.into(),
            value: Value::I64(value),
        }
    }

    pub fn from_str_u64(key: impl Into<String>, value: u64) -> Self {
        Self {
            key: key.into(),
            value: Value::U64(value),
        }
    }

    pub fn from_str_f32(key: impl Into<String>, value: f32) -> Self {
        Self {
            key: key.into(),
            value: Value::F32(value),
        }
    }

    pub fn from_str_f64(key: impl Into<String>, value: f64) -> Self {
        Self {
            key: key.into(),
            value: Value::F64(value),
        }
    }
}

/// A 2D data point.
#[derive(Debug, PartialEq)]
pub struct PointXy {
    pub x: f64,
    pub y: f64,
}

impl PointXy {
    pub fn new(x: f64, y: f64) -> PointXy {
        PointXy { x, y }
    }
}

impl From<(f64, f64)> for PointXy {
    fn from(value: (f64, f64)) -> Self {
        PointXy::new(value.0, value.1)
    }
}

/// A table column.
///
/// Note: This does not hold any data. It is used to indicate what columns a table consists of.
#[derive(Debug, PartialEq)]
pub struct Column {
    /// A unique key for a table.
    key: String,
    /// A name for the column.
    name: String,
}

impl Column {
    pub fn new(key: impl Into<String>, name: impl Into<String>) -> Column {
        Column {
            key: key.into(),
            name: name.into(),
        }
    }
}

/// A data table.
#[derive(Debug, PartialEq)]
pub struct Table {
    /// A list of column keys and corresponding column names.
    pub column_names: Vec<Column>,

    /// A list of rows.
    ///
    /// Each key-value pair in the map represents a single cell,
    /// e.g., peak parameters such as position or height.
    /// Only keys from the coulmn_names may occur but not all keys from
    /// that list need to occur as there may be missing values for cells.
    pub rows: Vec<HashMap<String, Value>>,
}

/// A tree node representing a section of data.
#[wasm_bindgen]
/// An harmonized abstraction for a part of a data set.
#[derive(Debug, PartialEq)]
pub struct Node {
    #[wasm_bindgen(skip)]
    pub name: String,
    #[wasm_bindgen(skip)]
    pub parameters: Vec<Parameter>,
    #[wasm_bindgen(skip)]
    pub data: Vec<PointXy>,
    #[wasm_bindgen(skip)]
    pub metadata: Vec<(String, String)>,
    #[wasm_bindgen(skip)]
    pub table: Option<Table>,
    #[wasm_bindgen(skip)]
    pub child_node_names: Vec<String>,
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn parameters(&self) -> Vec<JsValue> {
        let mut vec: Vec<JsValue> = vec![];
        for param in &self.parameters {
            let key = JsValue::from(&param.key);
            let value = JsValue::from(&param.value.to_string());
            let js_param = js_sys::Object::new();
            let set_key_ret = js_sys::Reflect::set(&js_param, &JsValue::from("key"), &key).unwrap();
            let set_val_ret =
                js_sys::Reflect::set(&js_param, &JsValue::from("value"), &value).unwrap();
            if !set_key_ret || !set_val_ret {
                panic!("Could not convert parameter to JS Object.");
            }
            vec.push(js_param.into());
        }
        vec
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<JsValue> {
        let mut vec: Vec<JsValue> = vec![];
        for xy in &self.data {
            let x = JsValue::from_f64(xy.x);
            let y = JsValue::from_f64(xy.y);
            let js_xy = js_sys::Object::new();
            let set_x_ret = js_sys::Reflect::set(&js_xy, &JsValue::from("x"), &x).unwrap();
            let set_y_ret = js_sys::Reflect::set(&js_xy, &JsValue::from("y"), &y).unwrap();
            if !set_x_ret || !set_y_ret {
                panic!("Could not convert data point to JS Object.");
            }
            vec.push(js_xy.into());
        }
        vec
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> js_sys::Object {
        let meta = js_sys::Object::new();
        for xy in &self.metadata {
            let key = JsValue::from(&xy.0);
            let value = JsValue::from(&xy.1);
            let set_meta_ret = js_sys::Reflect::set(&meta, &key, &value).unwrap();
            if !set_meta_ret {
                panic!("Could not convert metadata to JS Object.");
            }
        }
        meta
    }

    #[wasm_bindgen(getter)]
    pub fn table(&self) -> js_sys::Object {
        let js_table = js_sys::Object::new();
        let js_column_names: js_sys::Array = js_sys::Array::new();
        let js_rows: js_sys::Array = js_sys::Array::new();

        if let Some(table) = &self.table {
            let col_names = &table.column_names;
            for col_name in col_names {
                let key = JsValue::from(&col_name.key);
                let value = JsValue::from(&col_name.name);
                let column = js_sys::Object::new();
                let set_col_key_ret =
                    js_sys::Reflect::set(&column, &JsValue::from("key"), &key).unwrap();
                let set_col_val_ret =
                    js_sys::Reflect::set(&column, &JsValue::from("value"), &value).unwrap();
                if !set_col_key_ret || !set_col_val_ret {
                    panic!("Could not convert table column to JS Object.");
                }
                js_column_names.push(&column);
            }

            let rows = &table.rows;
            for row in rows {
                let js_row = js_sys::Object::new();
                for cell in row {
                    let key = JsValue::from(cell.0);
                    let val = JsValue::from(cell.1.to_string());
                    let set_cell_ret = js_sys::Reflect::set(&js_row, &key, &val).unwrap();
                    if !set_cell_ret {
                        panic!("Could not convert table cell to JS Object.");
                    }
                }
                js_rows.push(&js_row);
            }
        }

        let set_col_names_ret =
            js_sys::Reflect::set(&js_table, &JsValue::from("columnNames"), &js_column_names)
                .unwrap();
        let set_rows_ret =
            js_sys::Reflect::set(&js_table, &JsValue::from("rows"), &js_rows).unwrap();
        if !set_col_names_ret || !set_rows_ret {
            panic!("Could not populate table JS Object.");
        }

        js_table
    }

    #[wasm_bindgen(getter)]
    #[wasm_bindgen(js_name = childNodeNames)]
    pub fn child_node_names(&self) -> Vec<JsValue> {
        let mut vec: Vec<JsValue> = vec![];
        for param in &self.child_node_names {
            vec.push(param.into());
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    // see: https://github.com/rustwasm/wasm-bindgen/issues/3340
    // even though this test does not need to run in a worker, other unit tests do and fail if this one is not set to run in a worker
    #[cfg(target_family = "wasm")]
    wasm_bindgen_test_configure!(run_in_worker);
    // wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    #[wasm_bindgen_test]
    fn sf_error_prints_debug_info() {
        let error = SfError::new("Message");
        assert!(format!("{:?}", error).contains("SfError"));
        assert!(format!("{:?}", error).contains("Message"));
    }

    #[test]
    #[wasm_bindgen_test]
    fn sf_error_displays_error_message() {
        let error = SfError::new("Message");
        assert_eq!("Message", error.to_string());
    }

    #[test]
    #[wasm_bindgen_test]
    fn table_value_displays_value() {
        let val_str = Value::String("abc".to_owned());
        let val_bool = Value::Bool(true);
        let val_i32 = Value::I32(1);
        let val_u32 = Value::U32(2);
        let val_i64 = Value::I64(3);
        let val_u64 = Value::U64(4);
        let val_f32 = Value::F32(5.0);
        let val_f64 = Value::F64(6.0);

        assert_eq!("abc", val_str.to_string());
        assert_eq!("true", val_bool.to_string());
        assert_eq!("1", val_i32.to_string());
        assert_eq!("2", val_u32.to_string());
        assert_eq!("3", val_i64.to_string());
        assert_eq!("4", val_u64.to_string());
        assert_eq!("5", val_f32.to_string());
        assert_eq!("6", val_f64.to_string());
    }

    #[test]
    #[wasm_bindgen_test]
    fn parameters_are_correctly_initialized_for_all_value_types() {
        assert_eq!(
            Parameter {
                key: "x".to_owned(),
                value: Value::String("abc".to_owned())
            },
            Parameter::from_str_str("x", "abc")
        );
        assert_eq!(
            Parameter {
                key: "x".to_owned(),
                value: Value::Bool(true)
            },
            Parameter::from_str_bool("x", true)
        );
        assert_eq!(
            Parameter {
                key: "x".to_owned(),
                value: Value::I32(1)
            },
            Parameter::from_str_i32("x", 1)
        );
        assert_eq!(
            Parameter {
                key: "x".to_owned(),
                value: Value::U32(2)
            },
            Parameter::from_str_u32("x", 2)
        );
        assert_eq!(
            Parameter {
                key: "x".to_owned(),
                value: Value::I64(3)
            },
            Parameter::from_str_i64("x", 3)
        );
        assert_eq!(
            Parameter {
                key: "x".to_owned(),
                value: Value::U64(4)
            },
            Parameter::from_str_u64("x", 4)
        );
        assert_eq!(
            Parameter {
                key: "x".to_owned(),
                value: Value::F32(5.0)
            },
            Parameter::from_str_f32("x", 5.0)
        );
        assert_eq!(
            Parameter {
                key: "x".to_owned(),
                value: Value::F64(6.0)
            },
            Parameter::from_str_f64("x", 6.0)
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn point_xy_from_tuple() {
        let point_xy = PointXy::from((1.0, 2.0));
        assert_eq!(1.0, point_xy.x);
        assert_eq!(2.0, point_xy.y);
    }

    #[test]
    #[wasm_bindgen_test]
    fn table_prints_debug_info() {
        let table = Table {
            column_names: vec![],
            rows: vec![],
        };
        assert!(format!("{:?}", table).contains("Table"));
        assert!(format!("{:?}", table).contains("column_names"));
        assert!(format!("{:?}", table).contains("rows"));
    }

    #[test]
    #[wasm_bindgen_test]
    fn node_prints_debug_info() {
        let node = Node {
            name: "".to_owned(),
            parameters: vec![],
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: vec![],
        };
        assert!(format!("{:?}", node).contains("Node"));
        assert!(format!("{:?}", node).contains("name"));
        assert!(format!("{:?}", node).contains("parameters"));
        assert!(format!("{:?}", node).contains("data"));
        assert!(format!("{:?}", node).contains("metadata"));
        assert!(format!("{:?}", node).contains("table"));
        assert!(format!("{:?}", node).contains("child_node_names"));
    }

    // no #[test] as this test cannot run outside a browser engine
    #[cfg(target_family = "wasm")]
    #[wasm_bindgen_test]
    fn map_node_to_js() {
        let node = Node {
            name: "abc".to_owned(),
            parameters: vec![Parameter {
                key: "a".into(),
                value: Value::String("b".into()),
            }],
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: vec![],
        };

        assert_eq!("abc", node.name());
        let params = node.parameters();
        assert_eq!(1, params.len());
        let key = js_sys::Reflect::get(&params[0], &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        let value = js_sys::Reflect::get(&params[0], &JsValue::from("value"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("a", key);
        assert_eq!("b", value);
    }
}
