use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    io::{BufRead, Read, Seek},
};

/// Abstraction for any kind of random access input.
pub trait SeekRead: Seek + Read {}
impl<T: Seek + Read> SeekRead for T {}

/// Abstraction for any kind of buffered text input with lines and random access.
pub trait SeekBufRead: Seek + BufRead {}
impl<T: Seek + BufRead> SeekBufRead for T {}

/// Parses a (readonly) data set.
pub trait Parser<T: Read + Seek> {
    type R;
    type E: Error;

    fn parse(name: &str, input: T) -> Result<Self::R, Self::E>;
}

/// Scans a data set and provides a reader for recognized formats.
pub trait Scanner<T: Read + Seek> {
    /// Checks whether a data set is recognized. Shallow check.
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

    /// Provides a reader for a recognized data set.
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
    /// Reads a Node from the data set.
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
    pub key: String,
    /// A name for the column.
    pub name: String,
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
/// An harmonized abstraction for a part of a data set.
#[derive(Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub data: Vec<PointXy>,
    pub metadata: Vec<(String, String)>,
    pub table: Option<Table>,
    pub child_node_names: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
    fn point_xy_from_tuple() {
        let point_xy = PointXy::from((1.0, 2.0));
        assert_eq!(1.0, point_xy.x);
        assert_eq!(2.0, point_xy.y);
    }

    #[test]
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
}
