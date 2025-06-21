pub mod andi;
pub mod gaml;
pub mod jdx;
pub mod spc;

#[cfg(feature = "nodejs")]
use js_sys::{Array, Number, Object, Uint8Array};
#[cfg(not(feature = "nodejs"))]
use js_sys::{Array, Uint8Array};
use sf_rs::{
    api::{ExportFormat, Node, Reader, SeekRead, Value},
    common::{BufSeekRead, ScannerRepository},
};
use std::{
    error::Error,
    io::{Read, Seek, SeekFrom, Write},
};
use wasm_bindgen::{JsCast, JsError, JsValue, prelude::wasm_bindgen};
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
            let value = match &param.value {
                Value::String(s) => JsValue::from(s),
                Value::Bool(b) => JsValue::from(b.to_owned()),
                Value::I32(i) => JsValue::from(i.to_owned()),
                Value::U32(u) => JsValue::from(u.to_owned()),
                Value::I64(i) => JsValue::from(i.to_owned()),
                Value::U64(u) => JsValue::from(u.to_owned()),
                Value::F32(f) => JsValue::from(f.to_owned()),
                Value::F64(f) => JsValue::from(f.to_owned()),
            };
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
                    // todo: map to most appropriate JS type, not only string
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

    #[wasm_bindgen(js_name = getExportFormats)]
    pub fn get_export_formats(&self) -> Vec<String> {
        let mut str_formats = vec![];
        let formats = self.reader.get_export_formats();
        for format in formats {
            match format {
                ExportFormat::Json => str_formats.push("Json".to_owned()),
            }
        }
        str_formats
    }

    // pub fn export(&self, format: &str, writer: &mut JsBlobWriter) -> Result<(), JsError> {
    //     match format {
    //         "Json" => self
    //             .reader
    //             .export(ExportFormat::Json, writer)
    //             .map_err(|e| map_to_js_err(&*e)),
    //         _ => Err(JsError::new(&format!("Unknown export format: {}", format))),
    //     }
    // }

    #[wasm_bindgen(js_name = exportToBlob)]
    pub fn export_to_blob(&self, format: &str) -> Result<Blob, JsError> {
        let mut writer = JsBlobWriter::new();
        self.export(format, &mut writer)?;
        writer.into_blob()
    }

    #[cfg(feature = "nodejs")]
    #[wasm_bindgen(js_name = exportToFile)]
    pub fn export_to_file(&self, format: &str, fd: i32) -> Result<(), JsError> {
        let mut writer = JsFdWriter::new(fd);
        self.export(format, &mut writer)?;
        Ok(())
    }

    fn export(&self, format: &str, mut writer: &mut impl Write) -> Result<(), JsError> {
        match format {
            "Json" => self
                .reader
                .export(ExportFormat::Json, &mut writer)
                .map_err(|e| map_to_js_err(&*e)),
            _ => Err(JsError::new(&format!("Unknown export format: {}", format))),
        }?;
        Ok(())
    }
}

// pub(crate) fn map_js_input_to_write(input: &JsValue) -> Result<Box<dyn Write>, JsError> {
//     let input_type = input.js_typeof().as_string().unwrap_or_default();
//     let writer: Box<dyn Write> = if let Ok(w) = JsFdWriter::try_from_js_value(input.clone()) {
//         Box::new(w)
//     } else if let Ok(w) = JsBlobWriter::try_from_js_value(input.clone()) {
//         Box::new(w)
//     } else {
//         return Err(JsError::new(&format!(
//             "Illegal input type for writer: {}",
//             input_type
//         )));
//     };
//     Ok(writer)
// }

// -------------------------------------------------
// Read
// -------------------------------------------------

// todo: reduce code duplication

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
            Err(std::io::Error::other(format!("{:?}", js_error)))
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

// see https://github.com/rustwasm/wasm-bindgen/issues/1993 and https://nodejs.org/api/fs.html
#[cfg(feature = "nodejs")]
#[wasm_bindgen(module = "fs")]
extern "C" {
    // #[wasm_bindgen(js_name = readFileSync)]
    // fn read_file_sync(path: &str, options: &Object) -> JsValue;

    #[wasm_bindgen(js_name = fstatSync)]
    fn fstat_sync(fd: i32, options: &Object) -> JsValue;

    #[wasm_bindgen(js_name = readSync)]
    fn read_sync(fd: i32, buffer: &Uint8Array, offset: u32, length: u32, position: i64) -> Number;

    #[wasm_bindgen(js_name = writeSync)]
    fn write_sync(fd: i32, buffer: &Uint8Array, offset: u32, length: u32, position: i64) -> Number;

    #[wasm_bindgen(js_name = fsyncSync)]
    fn fsync_sync(fd: i32);

    // #[wasm_bindgen(js_name = closeSync)]
    // fn close_sync(fd: i32);
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct FdSeekRead {
    /// File descriptor
    fd: i32,
    /// Position in file
    pos: u64,
}

impl FdSeekRead {
    pub fn new(fd: i32) -> FdSeekRead {
        Self { fd, pos: 0 }
    }

    pub fn get_pos(&self) -> u64 {
        self.pos
    }
}

#[cfg(feature = "nodejs")]
impl Seek for FdSeekRead {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        fn to_oob_error<T>(pos: i64) -> std::io::Result<T> {
            // use web_sys::console;
            // console::error_1(&format!("I/O error. Seek position out of bounds: {pos}").into());
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Seek position out of bounds: {pos}"),
            ))
        }

        // use web_sys::console;
        // console::info_1(&"FdSeekRead::seek() entered".into());
        let stats = fstat_sync(self.fd, &Object::new());
        let file_size = js_sys::Reflect::get(&stats, &JsValue::from("size"))
            .unwrap()
            .as_f64()
            .unwrap() as u64;
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

#[cfg(feature = "nodejs")]
impl Read for FdSeekRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // use web_sys::console;
        // console::info_1(&"FdSeekRead::read() entered".into());
        let uint8_array = Uint8Array::new_with_length(buf.len() as u32);
        let js_num_bytes_read =
            read_sync(self.fd, &uint8_array, 0, buf.len() as u32, self.pos as i64);
        let num_bytes_read = js_num_bytes_read.as_f64().unwrap() as usize;
        self.pos += num_bytes_read as u64;
        uint8_array
            .slice(0, num_bytes_read as u32)
            .copy_to(&mut buf[0..num_bytes_read]);
        Ok(num_bytes_read)
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
        Self { repo }
    }

    #[wasm_bindgen(js_name = isRecognized)]
    pub fn js_is_recognized(&self, path: &str, input: &JsValue) -> bool {
        // use web_sys::console;
        // console::log_1(&"JsScannerRepository.js_is_recognized_generic() entered.".into());
        let seek_read_res = map_js_input_to_seekread(input);
        match seek_read_res {
            Err(_err) => false,
            Ok(mut seek_read) => self.repo.is_recognized(path, &mut seek_read),
        }
    }

    #[wasm_bindgen(js_name = getReader)]
    pub fn js_get_reader(&self, path: &str, input: &JsValue) -> Result<JsReader, JsError> {
        let seek_read = map_js_input_to_seekread(input)?;
        let input = BufSeekRead::new(seek_read);
        let reader_result = self.repo.get_reader(path, Box::new(input));
        match reader_result {
            Ok(reader) => Ok(JsReader::from(reader)),
            Err(error) => Err(map_to_js_err(&*error)),
        }
    }
}

// -------------------------------------------------
// Export
// -------------------------------------------------

#[wasm_bindgen(js_name = BlobWriter)]
pub struct JsBlobWriter {
    data: Vec<Uint8Array>,
}

#[wasm_bindgen(js_class = BlobWriter)]
impl JsBlobWriter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    #[wasm_bindgen(js_name = intoBlob)]
    pub fn into_blob(self) -> Result<Blob, JsError> {
        let js_value = JsValue::from(self.data);
        Blob::new_with_u8_array_sequence(&js_value).map_err(|e| {
            JsError::new(
                &e.as_string()
                    .unwrap_or("Error turning BlobWriter into Blob.".into()),
            )
        })
    }
}

impl Default for JsBlobWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for JsBlobWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let uint8_array = Uint8Array::new_with_length(buf.len() as u32);
        uint8_array.copy_from(buf);
        self.data.push(uint8_array);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // noop
        Ok(())
    }
}

#[cfg(feature = "nodejs")]
#[wasm_bindgen(js_name = FdWriter)]
pub struct JsFdWriter {
    /// File descriptor
    fd: i32,
    /// Position in file
    pos: u64,
}

#[cfg(feature = "nodejs")]
#[wasm_bindgen(js_class = FdWriter)]
impl JsFdWriter {
    #[wasm_bindgen(constructor)]
    pub fn new(fd: i32) -> JsFdWriter {
        Self { fd, pos: 0 }
    }
}

#[cfg(feature = "nodejs")]
impl Write for JsFdWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let uint8_array = Uint8Array::new_with_length(buf.len() as u32);
        uint8_array.copy_from(buf);
        let num_bytes = write_sync(self.fd, &uint8_array, 0, buf.len() as u32, self.pos as i64);
        let bytes_written = num_bytes.as_f64().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "write_sync() did not return a number.",
        ))? as u64;
        self.pos += bytes_written;
        Ok(bytes_written as usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        fsync_sync(self.fd);
        Ok(())
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
    ($scanner_name:ident, $reader_name:ident, $js_reader_name:ident) => {
        // concat!("Js", $reader_name) does not seem to work for identifiers
        #[wasm_bindgen(js_name = $reader_name)]
        pub struct $js_reader_name {
            reader: JsReader,
        }

        #[wasm_bindgen(js_class = $reader_name)]
        impl $js_reader_name {
            #[wasm_bindgen(constructor)]
            pub fn js_new(path: &str, input: &Blob) -> Result<$js_reader_name, JsError> {
                let scanner = $scanner_name::js_new();
                let reader = scanner.js_get_reader(path, input)?;
                Ok(Self { reader })
            }

            #[wasm_bindgen(js_name = read)]
            pub fn js_read(&self, path: &str) -> Result<JsNode, JsError> {
                self.reader.read(path)
            }

            #[wasm_bindgen(js_name = getExportFormats)]
            pub fn get_export_formats(&self) -> Vec<String> {
                self.reader.get_export_formats()
            }

            #[wasm_bindgen(js_name = exportToBlob)]
            pub fn export_to_blob(&self, format: &str) -> Result<Blob, JsError> {
                self.reader.export_to_blob(format)
            }

            #[cfg(feature = "nodejs")]
            #[wasm_bindgen(js_name = exportToFile)]
            pub fn export_to_file(&self, format: &str, fd: i32) -> Result<(), JsError> {
                self.reader.export_to_file(format, fd)
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

#[cfg(feature = "nodejs")]
pub(crate) fn map_js_input_to_seekread(input: &JsValue) -> Result<Box<dyn SeekRead>, JsError> {
    let seek_read: Box<dyn SeekRead> = if input.has_type::<Blob>() {
        Box::new(BlobSeekRead::new(Blob::from(input.clone())))
    } else if input.has_type::<Uint8Array>() {
        Box::new(Uint8ArraySeekRead::new(Uint8Array::from(input.clone())))
    } else if input.has_type::<Array>() {
        Box::new(ArraySeekRead::new(Array::from(input)))
    // only available in Node.js environment
    } else if input.has_type::<Number>() {
        Box::new(FdSeekRead::new(
            Number::from(input.clone()).as_f64().unwrap() as i32,
        ))
    } else {
        let input_type = input.js_typeof().as_string().unwrap_or_default();
        return Err(JsError::new(&format!(
            "Illegal input type for data: {}",
            input_type
        )));
    };

    Ok(seek_read)
}

#[cfg(not(feature = "nodejs"))]
pub(crate) fn map_js_input_to_seekread(input: &JsValue) -> Result<Box<dyn SeekRead>, JsError> {
    let seek_read: Box<dyn SeekRead> = if input.has_type::<Blob>() {
        Box::new(BlobSeekRead::new(Blob::from(input.clone())))
    } else if input.has_type::<Uint8Array>() {
        Box::new(Uint8ArraySeekRead::new(Uint8Array::from(input.clone())))
    } else if input.has_type::<Array>() {
        Box::new(ArraySeekRead::new(Array::from(input)))
    } else {
        let input_type = input.js_typeof().as_string().unwrap_or_default();
        return Err(JsError::new(&format!(
            "Illegal input type for data: {}",
            input_type
        )));
    };

    Ok(seek_read)
}

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::{Array, BigInt};
    use serde_json::json;
    use sf_rs::{
        api::{self, Column, Parameter, PointXy, Scanner, Table, Value},
        common::SfError,
    };
    use std::collections::HashMap;
    use wasm_bindgen_test::*;
    // see: https://github.com/rustwasm/wasm-bindgen/issues/3340
    // some need to run in a worker and fail if this one is not set to run in a worker
    wasm_bindgen_test_configure!(run_in_worker);
    // wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

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
    create_js_scanner!(StubScanner, JsStubScanner);

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
                _ => Err(SfError::new(&format!("Illegal path: {}", path)).into()),
            }
        }
    }
    create_js_reader!(JsStubScanner, StubReader, JsStubReader);

    // no #[test] as this test cannot run outside a browser engine
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn js_scanner_calls_wrapped_scanner() {
        let scanner = JsStubScanner::js_new();
        let blob = Blob::new().unwrap();
        assert!(scanner.js_is_recognized("some_path.xyz", &blob));
    }

    // no #[test] as this test cannot run outside a browser engine
    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn js_reader_calls_wrapped_reader() {
        let reader = JsStubReader::js_new("", &Blob::new().unwrap())
            .map_err(|_e| "Error instantiating StubReader.")
            .unwrap();
        let node = reader
            .js_read("/")
            .map_err(|_e| "Stub read failed.")
            .unwrap();

        assert_eq!("root node name", node.name());

        let params = &node.parameters();
        assert_eq!(8, params.len());
        let key_0 = js_sys::Reflect::get(&params[0], &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("param String", key_0);
        let value_0 = js_sys::Reflect::get(&params[0], &JsValue::from("value"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("abc", value_0);
        let key_1 = js_sys::Reflect::get(&params[1], &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("param bool", key_1);
        let value_1 = js_sys::Reflect::get(&params[1], &JsValue::from("value"))
            .unwrap()
            .as_bool()
            .unwrap();
        assert_eq!(true, value_1);
        let key_2 = js_sys::Reflect::get(&params[2], &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("param i32", key_2);
        let value_2 = js_sys::Reflect::get(&params[2], &JsValue::from("value"))
            .unwrap()
            .as_f64()
            .unwrap();
        assert_eq!(-1f64, value_2);
        let key_3 = js_sys::Reflect::get(&params[3], &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("param u32", key_3);
        let value_3 = js_sys::Reflect::get(&params[3], &JsValue::from("value"))
            .unwrap()
            .as_f64()
            .unwrap();
        assert_eq!(1f64, value_3);
        let key_4 = js_sys::Reflect::get(&params[4], &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("param i64", key_4);
        let js_value_4 = js_sys::Reflect::get(&params[4], &JsValue::from("value")).unwrap();
        assert!(js_value_4.is_bigint());
        let value_4: BigInt = js_value_4.into();
        assert_eq!(BigInt::from(-2), value_4);
        let key_5 = js_sys::Reflect::get(&params[5], &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("param u64", key_5);
        let js_value_5 = js_sys::Reflect::get(&params[5], &JsValue::from("value")).unwrap();
        assert!(js_value_5.is_bigint());
        let value_5: BigInt = js_value_5.into();
        assert_eq!(BigInt::from(2), value_5);
        let key_6 = js_sys::Reflect::get(&params[6], &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("param f32", key_6);
        let value_6 = js_sys::Reflect::get(&params[6], &JsValue::from("value"))
            .unwrap()
            .as_f64()
            .unwrap();
        assert_eq!(-1f64, value_6);
        let key_7 = js_sys::Reflect::get(&params[7], &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("param f64", key_7);
        let value_7 = js_sys::Reflect::get(&params[7], &JsValue::from("value"))
            .unwrap()
            .as_f64()
            .unwrap();
        assert_eq!(1f64, value_7);

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
        assert_eq!(2.0, y_0);
        let x_1 = js_sys::Reflect::get(&data[1], &JsValue::from("x"))
            .unwrap()
            .as_f64()
            .unwrap();
        assert_eq!(3.0, x_1);
        let y_1 = js_sys::Reflect::get(&data[1], &JsValue::from("y"))
            .unwrap()
            .as_f64()
            .unwrap();
        assert_eq!(4.0, y_1);

        let metadata = &node.metadata();
        let metadata_value0 = js_sys::Reflect::get(&metadata, &JsValue::from("mk0"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("mv0", metadata_value0);
        let metadata_value1 = js_sys::Reflect::get(&metadata, &JsValue::from("mk1"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("mv1", metadata_value1);

        let table = &node.table();
        let column_names = js_sys::Reflect::get(&table, &JsValue::from("columnNames")).unwrap();
        let columns = js_sys::Array::from(&column_names);
        assert_eq!(1, columns.length());
        let column_0 = columns.get(0);
        let column_key_0 = js_sys::Reflect::get(&column_0, &JsValue::from("key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("col key", column_key_0);
        let column_name_0 = js_sys::Reflect::get(&column_0, &JsValue::from("value"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("col name", column_name_0);
        let table_rows = js_sys::Reflect::get(&table, &JsValue::from("rows")).unwrap();
        let rows = js_sys::Array::from(&table_rows);
        assert_eq!(8, rows.length());
        let row_0 = rows.get(0);
        let cell_value_0 = js_sys::Reflect::get(&row_0, &JsValue::from("col key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("String value", cell_value_0);
        let row_1 = rows.get(1);
        let cell_value_1 = js_sys::Reflect::get(&row_1, &JsValue::from("col key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("true", cell_value_1);
        let row_2 = rows.get(2);
        let cell_value_2 = js_sys::Reflect::get(&row_2, &JsValue::from("col key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("-1", cell_value_2);
        let row_3 = rows.get(3);
        let cell_value_3 = js_sys::Reflect::get(&row_3, &JsValue::from("col key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("1", cell_value_3);
        let row_4 = rows.get(4);
        let cell_value_4 = js_sys::Reflect::get(&row_4, &JsValue::from("col key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("-2", cell_value_4);
        let row_5 = rows.get(5);
        let cell_value_5 = js_sys::Reflect::get(&row_5, &JsValue::from("col key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("2", cell_value_5);
        let row_6 = rows.get(6);
        let cell_value_6 = js_sys::Reflect::get(&row_6, &JsValue::from("col key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("-1", cell_value_6);
        let row_7 = rows.get(7);
        let cell_value_7 = js_sys::Reflect::get(&row_7, &JsValue::from("col key"))
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!("1", cell_value_7);

        let child_node_names = &node.child_node_names();
        assert_eq!(2, data.len());
        assert_eq!(
            "child node name 0",
            &child_node_names[0].as_string().unwrap()
        );
        assert_eq!(
            "child node name 1",
            &child_node_names[1].as_string().unwrap()
        );
    }

    // no #[test] as this test cannot run outside a browser engine
    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn js_reader_exports_to_blob() {
        let reader = JsStubReader::js_new("", &Blob::new().unwrap())
            .map_err(|_e| "Error instantiating StubReader.")
            .unwrap();
        let blob = reader
            .export_to_blob("Json")
            .map_err(|_e| "Export failed.")
            .unwrap();
        let reader = FileReaderSync::new().unwrap();
        let array_buffer = reader.read_as_array_buffer(&blob).unwrap();
        // see: https://stackoverflow.com/questions/67464060/converting-jsvalue-to-vecu8
        let uint8_array = Uint8Array::new(&array_buffer);
        let mut buf = vec![0u8; blob.size() as usize]; // Vec::<u8>::with_capacity(blob.size() as usize);
        uint8_array.copy_to(&mut buf[0..blob.size() as usize]);

        assert!(blob.size() as usize > 0);

        // https://docs.rs/serde_json/latest/serde_json/fn.to_value.html#example
        let output_str = String::from_utf8(buf).unwrap();
        let output_json: serde_json::Value = serde_json::from_str(&output_str).unwrap();

        let expected = json!({
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
            // "children": [],
            "children": [
                {
                    "name": "child node name 0",
                    "parameters": [], "data": [],
                    "metadata": [],
                    "table": {"columnNames": [], "rows": []},
                    "children": [],
                },
                {
                    "name": "child node name 1",
                    "parameters": [], "data": [],
                    "metadata": [],
                    "table": {"columnNames": [], "rows": []},
                    "children": [],
                },
            ]
        });

        assert_eq!(expected, output_json);
    }
}
