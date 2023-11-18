// #[cfg(target_family = "wasm")]
use js_sys::Uint8Array;
// #[cfg(target_family = "wasm")]
use std::io::{BufReader, SeekFrom};
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    io::{Read, Seek},
};
// #[cfg(target_family = "wasm")]
use wasm_bindgen::JsError;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
// #[cfg(target_family = "wasm")]
use web_sys::{Blob, FileReaderSync};

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

// -------------------------------------------------
// Wrappers
// -------------------------------------------------

pub struct SeekReadWrapper<T: Seek + Read> {
    input: BufReader<T>,
    pos: u64,
}

impl<T: Seek + Read> SeekReadWrapper<T> {
    pub fn new(raw_input: T) -> Self {
        SeekReadWrapper {
            // input: BufReader::with_capacity(64 * 1024, raw_input),
            input: BufReader::new(raw_input),
            pos: 0,
        }
    }
}

impl<T: Seek + Read> Seek for SeekReadWrapper<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Current(offset) => {
                self.input.seek_relative(offset)?;
                // todo: handle error cases
                self.pos = (self.pos as i64 + offset) as u64;
                Ok(self.pos)
            }
            SeekFrom::Start(offset) => {
                // todo: handle error cases
                let rel_offset = (offset as i64) - (self.pos as i64);
                self.input.seek_relative(rel_offset)?;
                self.pos = offset as u64;
                Ok(self.pos)
            }
            SeekFrom::End(offset) => {
                // todo: make more efficient
                self.pos = self.input.seek(SeekFrom::End(offset))?;
                Ok(self.pos)
            }
        }
    }
}

impl<T: Seek + Read> Read for SeekReadWrapper<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let num_read = self.input.read(buf)?;
        self.pos += num_read as u64;
        Ok(num_read)
    }
}

// -------------------------------------------------
// WASM specific
// -------------------------------------------------

// #[cfg(target_family = "wasm")]
#[derive(Clone)]
pub struct BlobWrapper {
    blob: Blob,
    pos: u64,
}

// #[cfg(target_family = "wasm")]
impl BlobWrapper {
    pub fn new(blob: Blob) -> BlobWrapper {
        BlobWrapper { blob, pos: 0 }
    }

    pub fn get_pos(&self) -> u64 {
        self.pos
    }
}

// #[cfg(target_family = "wasm")]
impl Seek for BlobWrapper {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        fn to_oob_error<T>(pos: i64) -> std::io::Result<T> {
            // use web_sys::console;
            // console::error_1(&format!("I/O error. Seek position out of bounds: {pos}").into());
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Seek position out of bounds: {pos}"),
            ))
        }

        let file_size = self.blob.size() as u64;
        match pos {
            SeekFrom::Start(seek_pos) => {
                self.pos = seek_pos;
            }
            SeekFrom::End(seek_pos) => {
                let new_pos = file_size as i64 + seek_pos;
                if new_pos < 0 {
                    return to_oob_error(new_pos);
                }
                self.pos = new_pos as u64;
            }
            SeekFrom::Current(seek_pos) => {
                let new_pos = self.pos as i64 + seek_pos;
                if new_pos < 0 {
                    return to_oob_error(new_pos);
                }
                self.pos = new_pos as u64;
            }
        }
        Ok(self.pos)
    }
}

// #[cfg(target_family = "wasm")]
impl Read for BlobWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        fn to_io_error<T>(js_error: JsValue) -> std::io::Result<T> {
            // use web_sys::console;
            // console::error_1(&format!("I/O error: {:?}", js_error).into());
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", js_error),
            ))
        }

        let end_pos = self.pos + buf.len() as u64;
        let result = self
            .blob
            .slice_with_f64_and_f64(self.pos as f64, end_pos as f64);
        match result {
            Ok(slice) => {
                self.pos += slice.size() as u64;
                let reader = match FileReaderSync::new() {
                    Ok(frs) => frs,
                    Err(err) => return to_io_error(err),
                };
                let array_buffer = match reader.read_as_array_buffer(&slice) {
                    Ok(buf) => buf,
                    Err(err) => return to_io_error(err),
                };
                // see: https://stackoverflow.com/questions/67464060/converting-jsvalue-to-vecu8
                let uint8_array = Uint8Array::new(&array_buffer);
                uint8_array.copy_to(&mut buf[0..slice.size() as usize]);
                Ok(slice.size() as usize)
            }
            Err(js_error) => to_io_error(js_error),
        }
    }
}

#[wasm_bindgen]
// #[cfg(target_family = "wasm")]
pub struct JsReader {
    reader: Box<dyn Reader>,
}

// #[cfg(target_family = "wasm")]
impl JsReader {
    pub fn new(reader: Box<dyn Reader>) -> Self {
        JsReader { reader }
    }
}

#[wasm_bindgen]
// #[cfg(target_family = "wasm")]
impl JsReader {
    pub fn read(&self, path: &str) -> Result<Node, JsError> {
        let read_result = self.reader.read(path);
        match read_result {
            Ok(node) => Ok(node),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}

#[cfg(test)]
// #[cfg(target_family = "wasm")]
mod tests {
    // see: https://github.com/rustwasm/wasm-bindgen/issues/3340
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_worker);
    // wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use js_sys::Array;
    use wasm_bindgen_test::*;
    use web_sys::Blob;

    // no #[test] as this test cannot run outside a browser engine
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

    // no #[test] as this test cannot run outside a browser engine
    #[wasm_bindgen_test]
    async fn blob_wrapper_mimicks_std_seek_read_behavior_test() {
        let arr: [u8; 3] = [1, 2, 3];
        let js_arr = Array::new();
        // see: https://github.com/rustwasm/wasm-bindgen/issues/1693
        js_arr.push(&Uint8Array::from(arr.as_slice()));
        let blob = Blob::new_with_u8_array_sequence(&js_arr).unwrap();
        assert_eq!(3, blob.size() as u64);
        // use web_sys::console;
        // console::log_1(&format!("arr: {:?}", arr).into());
        let mut blob_wrapper = BlobWrapper::new(blob);
        let mut buf = [0u8; 3];

        // read whole blob
        let read_len = blob_wrapper.read(&mut buf).unwrap();
        assert_eq!(3, read_len);
        assert_eq!(arr, buf);

        // read past end
        buf.fill(0);
        let pos = blob_wrapper.seek(SeekFrom::Start(1)).unwrap();
        assert_eq!(1, pos);
        let read_len = blob_wrapper.read(&mut buf).unwrap();
        assert_eq!(2, read_len);
        assert_eq!([2, 3, 0], buf);

        // seek beyond end and read
        buf.fill(0);
        let pos = blob_wrapper.seek(SeekFrom::Start(10)).unwrap();
        assert_eq!(10, pos);
        let read_len = blob_wrapper.read(&mut buf).unwrap();
        assert_eq!(0, read_len);
        assert_eq!([0, 0, 0], buf);

        // seek to negative position
        let pos = blob_wrapper.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(0, pos);
        let seek_err = blob_wrapper.seek(SeekFrom::Current(-1)).unwrap_err();
        assert_eq!(std::io::ErrorKind::InvalidInput, seek_err.kind());
        assert_eq!("Seek position out of bounds: -1", seek_err.to_string());
    }
}
