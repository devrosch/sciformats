use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Seek},
};

pub trait Parser<T: Read + Seek> {
    type R;

    fn parse(name: &str, input: T) -> Result<Self::R, Box<dyn Error>>;
}

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
    fn is_recognized(path: &str, input: &mut T) -> bool;

    /// Returns a converter that provides a harmonized view for reading a scientifc file.
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
    fn get_converter(path: &str, input: T) -> Result<Box<dyn Converter>, Box<dyn Error>>;
}

pub trait Converter {
    /// Returns a Node read from the data set.
    ///
    /// # Arguments
    ///
    /// * `path` - The path inside the data set identifying the Node.
    fn read(&self, path: &str) -> Result<Node, Box<dyn Error>>;
}

pub struct Node {
    pub name: String,
    pub parameters: Vec<(String, String)>,
    pub data: Vec<(f64, f64)>,
    pub metadata: Vec<(String, String)>,
    pub table: Table,
    pub child_node_names: Vec<String>,
}

/// A data table.
pub struct Table {
    /// A list of column keys and corresponding column names.
    pub column_names: Vec<(String, String)>,

    /// A list of rows.
    ///
    /// Each key-value pair in the map represents a single cell,
    /// e.g., peak parameters such as position or height.
    /// Only keys from the coulmn_names may occur but not all keys from
    /// that list need to occur as there may be missing values for cells.
    pub rows: Vec<HashMap<String, String>>,
}
