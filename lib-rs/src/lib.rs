use js_sys::Uint8Array;
use netcdf3::FileReader;
use std::io::{Error, Read, Seek, SeekFrom};
use wasm_bindgen::prelude::*;
use web_sys::{File, FileReaderSync};

struct FileWrapper {
    file: File,
    pos: u64,
}

impl FileWrapper {
    fn new(file: File) -> FileWrapper {
        FileWrapper { file, pos: 0 }
    }
}

impl Seek for FileWrapper {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        fn to_oob_error<T>(pos: i64) -> std::io::Result<T> {
            Err(Error::new(
                std::io::ErrorKind::Other,
                format!("Seek position out of bounds: {pos}"),
            ))
        }

        let file_size = self.file.size() as u64;
        match pos {
            SeekFrom::Start(seek_pos) => {
                if seek_pos > file_size {
                    return to_oob_error(seek_pos as i64);
                }
                self.pos = seek_pos;
            }
            SeekFrom::End(seek_pos) => {
                let new_pos = file_size as i64 + seek_pos;
                if new_pos < 0 || new_pos as u64 > file_size {
                    return to_oob_error(new_pos);
                }
                self.pos = new_pos as u64;
            }
            SeekFrom::Current(seek_pos) => {
                let new_pos = self.pos as i64 + seek_pos;
                if new_pos < 0 || new_pos as u64 > file_size {
                    return to_oob_error(new_pos);
                }
                self.pos = new_pos as u64;
            }
        }
        Ok(self.pos)
    }
}

impl Read for FileWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        fn to_io_error<T>(js_error: JsValue) -> std::io::Result<T> {
            Err(Error::new(
                std::io::ErrorKind::Other,
                js_error.as_string().unwrap_or_default(),
            ))
        }

        let end_pos = self.pos + buf.len() as u64;
        let result = self
            .file
            .slice_with_f64_and_f64(self.pos as f64, end_pos as f64);
        match result {
            Ok(slice) => {
                self.pos = self.pos + slice.size() as u64;
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
                uint8_array.copy_to(buf);
                return Ok(slice.size() as usize);
            }
            Err(js_error) => {
                return to_io_error(js_error);

            }
        }
    }
}

#[wasm_bindgen]
pub fn parse_file(file: File) {
    use web_sys::console;

    let name = file.name();
    let js: JsValue = name.clone().into();
    console::log_2(&"Rust parse_file(): File name:".into(), &js);

    let file_wrapper = FileWrapper::new(file);
    let input_file = Box::new(file_wrapper);
    let res = FileReader::open_seek_read(&name, input_file);
    match res {
        Ok(reader) => {
            console::log_1(&"Rust parse_file(): parsing succeeded.".into());
            console::log_1(&"--- Rust parse_file(): attributes ---".into());
            for attr in reader.data_set().get_global_attrs().iter() {
                console::log_3(
                    &attr.name().into(),
                    &": ".into(),
                    &(attr.get_as_string().unwrap_or_default()).into(),
                );
            }
            console::log_1(&"Rust parse_file(): closing...".into());
            reader.close();
        }
        Err(err) => {
            console::log_1(&"Rust parse_file(): parsing failed.".into());
            console::log_1(&(err.to_string()).into());
        }
    }
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use wasm_bindgen_test::*;

    // no #[test] as this test cannot run outside a browser engine
    #[wasm_bindgen_test]
    fn parse_illegal_file() {
        let data: [u8; 3] = [1u8, 2, 3];
        let array = Uint8Array::new_with_length(3);
        array.copy_from(&data);
        let file = File::new_with_u8_array_sequence(&array, "test.txt");

        assert!(file.is_ok());

        parse_file(file.unwrap());
        // TODO: implement parse_file() return value and check
    }
}
