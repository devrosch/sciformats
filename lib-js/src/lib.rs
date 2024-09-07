pub mod andi;
pub mod spc;

use js_sys::{Array, Uint8Array};
use sf_rs::{
    api::{Node, Reader, SeekRead},
    common::{BufSeekRead, ScannerRepository},
};
use std::{
    error::Error,
    io::{Read, Seek, SeekFrom},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsError, JsValue};
use web_sys::{Blob, FileReaderSync};

// -------------------------------------------------
// Log library name and version on startup
// -------------------------------------------------

#[wasm_bindgen(start)]
pub fn start() {
    use web_sys::console;

    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    console::log_1(&format!("Rust: {} {} loaded", NAME, VERSION).into());
}

// -------------------------------------------------
// API
// -------------------------------------------------

#[wasm_bindgen(js_name = Node)]
pub struct JsNode {
    node: Node,
}

impl From<Node> for JsNode {
    fn from(value: Node) -> Self {
        Self { node: value }
    }
}

#[wasm_bindgen(js_class = Node)]
impl JsNode {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.node.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn parameters(&self) -> Vec<JsValue> {
        let mut vec: Vec<JsValue> = vec![];
        for param in &self.node.parameters {
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
        for xy in &self.node.data {
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
        for xy in &self.node.metadata {
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

        if let Some(table) = &self.node.table {
            let col_names = &table.column_names;
            for col_name in col_names {
                let key = JsValue::from(&col_name.key);
                let value = JsValue::from(&col_name.name);
                let column = js_sys::Object::new();
                let set_col_key_ret =
                    js_sys::Reflect::set(&column, &JsValue::from("key"), &key).unwrap();
                // TODO: make this "name" not "value"
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

    #[wasm_bindgen(getter, js_name = childNodeNames)]
    pub fn child_node_names(&self) -> Vec<JsValue> {
        let mut vec: Vec<JsValue> = vec![];
        for param in &self.node.child_node_names {
            vec.push(param.into());
        }
        vec
    }
}

#[wasm_bindgen(js_name = Reader)]
pub struct JsReader {
    reader: Box<dyn Reader>,
}

impl From<Box<dyn Reader>> for JsReader {
    fn from(value: Box<dyn Reader>) -> Self {
        Self { reader: value }
    }
}

#[wasm_bindgen(js_class = Reader)]
impl JsReader {
    pub fn read(&self, path: &str) -> Result<JsNode, JsError> {
        let read_result = self.reader.read(path);
        match read_result {
            Ok(node) => Ok(JsNode::from(node)),
            Err(error) => Err(map_to_js_err(&*error)),
        }
    }
}

// -------------------------------------------------
// Common
// -------------------------------------------------

// todo: reduce code duplication
// todo: add SeekRead for lazy loading from Node.js file descriptors
// see https://github.com/rustwasm/wasm-bindgen/issues/1993 and https://nodejs.org/api/fs.html

#[derive(Clone)]
pub struct Uint8ArraySeekRead {
    array: Uint8Array,
    pos: u64,
}

impl Uint8ArraySeekRead {
    pub fn new(array: Uint8Array) -> Uint8ArraySeekRead {
        Self { array, pos: 0 }
    }

    pub fn get_pos(&self) -> u64 {
        self.pos
    }
}

impl Seek for Uint8ArraySeekRead {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        fn to_oob_error<T>(pos: i64) -> std::io::Result<T> {
            // use web_sys::console;
            // console::error_1(&format!("I/O error. Seek position out of bounds: {pos}").into());
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Seek position out of bounds: {pos}"),
            ))
        }

        let file_size = self.array.length() as u64;
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

impl Read for Uint8ArraySeekRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let end_pos = self.pos + buf.len() as u64;
        let slice = self.array.slice(self.pos as u32, end_pos as u32);
        self.pos += slice.length() as u64;
        slice.copy_to(&mut buf[0..slice.length() as usize]);
        Ok(slice.length() as usize)
    }
}

#[derive(Clone)]
pub struct ArraySeekRead {
    array: Array,
    pos: u64,
}

impl ArraySeekRead {
    pub fn new(array: Array) -> ArraySeekRead {
        Self { array, pos: 0 }
    }

    pub fn get_pos(&self) -> u64 {
        self.pos
    }
}

impl Seek for ArraySeekRead {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        fn to_oob_error<T>(pos: i64) -> std::io::Result<T> {
            // use web_sys::console;
            // console::error_1(&format!("I/O error. Seek position out of bounds: {pos}").into());
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Seek position out of bounds: {pos}"),
            ))
        }

        let file_size = self.array.length() as u64;
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

impl Read for ArraySeekRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let end_pos = self.pos + buf.len() as u64;
        let slice = self.array.slice(self.pos as u32, end_pos as u32);
        self.pos += slice.length() as u64;
        // see: https://stackoverflow.com/questions/67464060/converting-jsvalue-to-vecu8
        let uint8_array = Uint8Array::new(&slice);
        uint8_array.copy_to(&mut buf[0..slice.length() as usize]);
        Ok(slice.length() as usize)
    }
}

#[derive(Clone)]
pub struct BlobSeekRead {
    blob: Blob,
    pos: u64,
}

impl BlobSeekRead {
    pub fn new(blob: Blob) -> BlobSeekRead {
        Self { blob, pos: 0 }
    }

    pub fn get_pos(&self) -> u64 {
        self.pos
    }
}

impl Seek for BlobSeekRead {
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

impl Read for BlobSeekRead {
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

#[wasm_bindgen(js_name = ScannerRepository)]
pub struct JsScannerRepository {
    repo: ScannerRepository,
}

#[wasm_bindgen(js_class = ScannerRepository)]
impl JsScannerRepository {
    #[wasm_bindgen(constructor)]
    pub fn init_all() -> JsScannerRepository {
        let repo = ScannerRepository::init_all();
        JsScannerRepository { repo }
    }

    #[wasm_bindgen(js_name = isRecognized)]
    pub fn js_is_recognized(&self, path: &str, input: &JsValue) -> bool {
        // use web_sys::console;
        // console::log_1(&"JsScannerRepository.js_is_recognized_generic() entered.".into());
        if input.has_type::<Blob>() {
            // console::log_1(&"JsScannerRepository input type recognized as Blob.".into());
            let typed_input = Blob::from(input.clone());
            let seek_read = Box::new(BlobSeekRead::new(typed_input));
            self.repo
                .is_recognized(path, &mut (seek_read as Box<dyn SeekRead>))
        } else if input.has_type::<Uint8Array>() {
            // console::log_1(&"JsScannerRepository input type recognized as Uint8Array.".into());
            let typed_input = Uint8Array::from(input.clone());
            let seek_read = Box::new(Uint8ArraySeekRead::new(typed_input));
            self.repo
                .is_recognized(path, &mut (seek_read as Box<dyn SeekRead>))
        } else if input.has_type::<Array>() {
            // console::log_1(&"JsScannerRepository input type recognized as Array.".into());
            let typed_input = Array::from(input);
            let seek_read = Box::new(ArraySeekRead::new(typed_input));
            self.repo
                .is_recognized(path, &mut (seek_read as Box<dyn SeekRead>))
        } else {
            // console::log_1(&"JsScannerRepository input type not recognized.".into());
            false
        }
    }

    #[wasm_bindgen(js_name = getReader)]
    pub fn js_get_reader(&self, path: &str, input: &JsValue) -> Result<JsReader, JsError> {
        let seek_read: Box<dyn SeekRead> = if input.has_type::<Blob>() {
            Box::new(BlobSeekRead::new(Blob::from(input.clone())))
        } else if input.has_type::<Uint8Array>() {
            Box::new(Uint8ArraySeekRead::new(Uint8Array::from(input.clone())))
        } else if input.has_type::<Array>() {
            Box::new(ArraySeekRead::new(Array::from(input)))
        } else {
            let input_type = input.js_typeof().as_string().unwrap_or_default();
            return Err(JsError::new(&format!(
                "Illegal input type for ScannerRepository::getReader(): {}",
                input_type
            )));
        };

        let input = BufSeekRead::new(seek_read);
        let reader_result = self.repo.get_reader(path, Box::new(input));
        match reader_result {
            Ok(reader) => Ok(JsReader::from(reader)),
            Err(error) => Err(map_to_js_err(&*error)),
        }
    }
}

// -------------------------------------------------
// Utils
// -------------------------------------------------

/// Create JS wrapper for Scanner
macro_rules! create_js_scanner {
    ($scanner_name:ident, $js_scanner_name:ident) => {
        #[wasm_bindgen(js_name = $scanner_name)]
        pub struct $js_scanner_name {
            scanner: $scanner_name,
        }

        #[wasm_bindgen(js_class = $scanner_name)]
        impl $js_scanner_name {
            #[wasm_bindgen(constructor)]
            pub fn js_new() -> Self {
                Self {
                    scanner: $scanner_name::default(),
                }
            }

            #[wasm_bindgen(js_name = isRecognized)]
            pub fn js_is_recognized(&self, path: &str, input: &Blob) -> bool {
                let mut blob = BlobSeekRead::new(input.clone());
                self.scanner.is_recognized(path, &mut blob)
            }

            #[wasm_bindgen(js_name = getReader)]
            pub fn js_get_reader(&self, path: &str, input: &Blob) -> Result<JsReader, JsError> {
                let blob = BlobSeekRead::new(input.clone());
                let reader_result = self.scanner.get_reader(path, blob);
                match reader_result {
                    Ok(reader) => Ok(JsReader::from(reader)),
                    Err(error) => Err(map_to_js_err(&*error)),
                }
            }
        }
    };
}
pub(crate) use create_js_scanner;

/// Create JS wrapper for Reader
macro_rules! create_js_reader {
    ($reader_name:ident, $js_reader_name:ident) => {
        // concat!("Js", $reader_name) does not seem to work for identifiers
        #[wasm_bindgen(js_name = $reader_name)]
        pub struct $js_reader_name {
            reader: $reader_name,
        }

        #[wasm_bindgen(js_class = $reader_name)]
        impl $js_reader_name {
            #[wasm_bindgen(js_name = read)]
            pub fn js_read(&self, path: &str) -> Result<JsNode, JsError> {
                let read_result = self.reader.read(path);
                match read_result {
                    Ok(node) => Ok(node.into()),
                    Err(error) => Err(map_to_js_err(&*error)),
                }
            }
        }
    };
}
pub(crate) use create_js_reader;

pub(crate) fn map_to_js_err(error: &dyn Error) -> JsError {
    let mut err_str = error.to_string();
    let mut source = error.source();
    while let Some(nested_err) = source {
        err_str += "\n";
        err_str += nested_err.to_string().as_str();
        source = nested_err.source();
    }
    JsError::new(&err_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::Array;
    use sf_rs::api::{Column, Parameter, PointXy, Scanner, Table, Value};
    use std::collections::HashMap;
    use wasm_bindgen_test::*;
    // see: https://github.com/rustwasm/wasm-bindgen/issues/3340
    // some need to run in a worker and fail if this one is not set to run in a worker
    wasm_bindgen_test_configure!(run_in_worker);
    // wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    struct StubReader {}
    impl Reader for StubReader {
        fn read(&self, _path: &str) -> Result<Node, Box<dyn Error>> {
            Ok(Node {
                name: "Node name".into(),
                parameters: vec![
                    Parameter {
                        key: "Key 0".into(),
                        value: Value::String("Value 0".into()),
                    },
                    Parameter {
                        key: "Key 1".into(),
                        value: Value::String("Value 1".into()),
                    },
                ],
                data: vec![PointXy::new(1.0, 100.0), PointXy::new(2.0, 200.0)],
                metadata: vec![("x.unit".into(), "X unit".into())],
                table: Some(Table {
                    column_names: vec![Column::new("Col key 0", "Col name 0")],
                    rows: vec![HashMap::from([(
                        "Col key 0".into(),
                        Value::String("Cell value 0".into()),
                    )])],
                }),
                child_node_names: vec!["Child node name 0".into(), "Child node name 1".into()],
            })
        }
    }

    #[derive(Default)]
    struct StubScanner {}
    impl<T: Seek + Read> Scanner<T> for StubScanner {
        fn is_recognized(&self, _path: &str, _input: &mut T) -> bool {
            true
        }

        fn get_reader(&self, _path: &str, _input: T) -> Result<Box<dyn Reader>, Box<dyn Error>> {
            Ok(Box::new(StubReader {}))
        }
    }

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

        let js_node = JsNode::from(node);

        assert_eq!("abc", js_node.name());
        let params = js_node.parameters();
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
    async fn blob_wrapper_mimicks_std_seek_read_behavior() {
        let arr: [u8; 3] = [1, 2, 3];
        let js_arr = Array::new();
        // see: https://github.com/rustwasm/wasm-bindgen/issues/1693
        js_arr.push(&Uint8Array::from(arr.as_slice()));
        let blob = Blob::new_with_u8_array_sequence(&js_arr).unwrap();
        assert_eq!(3, blob.size() as u64);
        // use web_sys::console;
        // console::log_1(&format!("arr: {:?}", arr).into());
        let mut blob_wrapper = BlobSeekRead::new(blob);
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

        // seek from end
        let pos = blob_wrapper.seek(SeekFrom::End(-1)).unwrap();
        assert_eq!(2, pos);

        // seek to negative position
        let pos = blob_wrapper.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(0, pos);
        let seek_err = blob_wrapper.seek(SeekFrom::Current(-1)).unwrap_err();
        assert_eq!(std::io::ErrorKind::InvalidInput, seek_err.kind());
        assert_eq!("Seek position out of bounds: -1", seek_err.to_string());
    }

    // no #[test] as this test cannot run outside a browser engine
    #[wasm_bindgen_test]
    fn js_scanner_calls_wrapped_scanner() {
        create_js_scanner!(StubScanner, JsStubScanner);
        let scanner = JsStubScanner::js_new();
        let blob = Blob::new().unwrap();
        assert!(scanner.js_is_recognized("some_path.xyz", &blob));
    }

    // no #[test] as this test cannot run outside a browser engine
    #[wasm_bindgen_test]
    fn js_reader_calls_wrapped_reader() {
        create_js_reader!(StubReader, JsStubReader);
        let reader = JsStubReader {
            reader: StubReader {},
        };

        let node = reader
            .js_read("some_path.xyz")
            .map_err(|_e| "Stub read failed.")
            .unwrap();

        assert_eq!("Node name", node.name());

        let params = &node.parameters();
        assert_eq!(2, params.len());
        let key_0 = js_sys::Reflect::get(&params[0], &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("Key 0", key_0);
        let value_0 = js_sys::Reflect::get(&params[0], &JsValue::from("value"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("Value 0", value_0);
        let key_1 = js_sys::Reflect::get(&params[1], &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("Key 1", key_1);
        let value_1 = js_sys::Reflect::get(&params[1], &JsValue::from("value"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("Value 1", value_1);

        let data = &node.data();
        assert_eq!(2, data.len());
        let x_0 = js_sys::Reflect::get(&data[0], &JsValue::from("x"))
            .unwrap()
            .as_f64()
            .unwrap();
        assert_eq!(1.0, x_0);
        let y_0 = js_sys::Reflect::get(&data[0], &JsValue::from("y"))
            .unwrap()
            .as_f64()
            .unwrap();
        assert_eq!(100.0, y_0);
        let x_1 = js_sys::Reflect::get(&data[1], &JsValue::from("x"))
            .unwrap()
            .as_f64()
            .unwrap();
        assert_eq!(2.0, x_1);
        let y_1 = js_sys::Reflect::get(&data[1], &JsValue::from("y"))
            .unwrap()
            .as_f64()
            .unwrap();
        assert_eq!(200.0, y_1);

        let metadata = &node.metadata();
        let metadata_value = js_sys::Reflect::get(&metadata, &JsValue::from("x.unit"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("X unit", metadata_value);

        let table = &node.table();
        let column_names = js_sys::Reflect::get(&table, &JsValue::from("columnNames")).unwrap();
        let columns = js_sys::Array::from(&column_names);
        assert_eq!(1, columns.length());
        let column_0 = columns.get(0);
        let column_key_0 = js_sys::Reflect::get(&column_0, &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("Col key 0", column_key_0);
        let column_name_0 = js_sys::Reflect::get(&column_0, &JsValue::from("value"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("Col name 0", column_name_0);
        let table_rows = js_sys::Reflect::get(&table, &JsValue::from("rows")).unwrap();
        let rows = js_sys::Array::from(&table_rows);
        assert_eq!(1, rows.length());
        let row_0 = rows.get(0);
        let cell_value_0 = js_sys::Reflect::get(&row_0, &JsValue::from("Col key 0"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("Cell value 0", cell_value_0);

        let child_node_names = &node.child_node_names();
        assert_eq!(2, data.len());
        assert_eq!(
            "Child node name 0",
            &child_node_names[0].as_string().unwrap()
        );
        assert_eq!(
            "Child node name 1",
            &child_node_names[1].as_string().unwrap()
        );
    }
}
