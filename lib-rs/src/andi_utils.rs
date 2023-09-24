use crate::andi::AndiError;
use netcdf3::{DataType, DataVector};
use std::error::Error;

pub fn read_index_from_var_f32(
    var: &Option<(&str, Vec<usize>, DataVector)>,
    index: usize,
) -> Result<Option<f32>, Box<dyn Error>> {
    let res = read_index_from_slice(
        var.as_ref().map(|(_, _, v)| v.get_f32()).flatten(),
        var.as_ref().map(|(name, _, _)| *name).unwrap_or_default(),
        index,
    )?
    .copied();

    Ok(res)
}

fn check_var_is_2d(var_name: &str, dims: &Vec<usize>) -> Result<(), Box<dyn Error>> {
    if dims.len() != 2 {
        return Err(Box::new(AndiError::new(&format!(
            "Unexpected number of dimensions for {}: {}",
            var_name,
            dims.len()
        ))));
    }
    Ok(())
}

pub fn read_index_from_var_2d_string(
    var: &Option<(&str, Vec<usize>, DataVector)>,
    index: usize,
) -> Result<Option<String>, Box<dyn Error>> {
    match var {
        None => Ok(None),
        Some((var_name, dims, data)) => {
            check_var_is_2d(var_name, dims)?;

            let row_length = dims.get(1).unwrap();
            let bytes = data
                .get_u8()
                .ok_or(AndiError::new(&format!("Failed to read {}", var_name)))?;
            let start_index = index * row_length;
            let end_index = start_index + row_length;
            let string_bytes = &bytes[start_index..end_index];
            let s = convert_iso_8859_1_cstr_to_str(string_bytes);

            Ok(Some(s))
        }
    }
}

pub fn read_multi_string_var(
    reader: &mut netcdf3::FileReader,
    var_name: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let var_opt = read_optional_var(reader, var_name)?;
    match var_opt {
        None => Ok(vec![]),
        Some((_, ref dims, _)) => {
            check_var_is_2d(var_name, dims)?;

            let mut vec = vec![];
            for i in 0..dims[0] {
                let value = read_index_from_var_2d_string(&var_opt, i)?
                    .ok_or(AndiError::new(&format!("Failed to read {}", var_name)))?;
                vec.push(value);
            }
            Ok(vec)
        }
    }
}

pub fn read_index_from_slice<'a, T: 'a>(
    slice: Option<&'a [T]>,
    var_name: &str,
    index: usize,
) -> Result<Option<&'a T>, Box<dyn Error>> {
    match slice {
        None => Ok(None),
        Some(sl) => match sl.get(index) {
            None => Err(Box::new(AndiError::new(&format!(
                "Index out of bounds for {}: {}",
                var_name, index
            )))),
            Some(val) => Ok(Some(val)),
        },
    }
}

pub fn read_optional_var<'a>(
    reader: &mut netcdf3::FileReader,
    var_name: &'a str,
) -> Result<Option<(&'a str, Vec<usize>, DataVector)>, Box<dyn Error>> {
    let var = reader.data_set().get_var(var_name);
    match var {
        Some(var) => {
            let dims: Vec<usize> = var.get_dims().iter().map(|dim| dim.size()).collect();
            let vec = reader.read_var(var_name)?;
            Ok(Some((var_name, dims, vec)))
        }
        None => Ok(None),
    }
}

pub fn read_scalar_var_f32(
    reader: &mut netcdf3::FileReader,
    var_name: &str,
) -> Result<Option<f32>, AndiError> {
    let var = reader.data_set().get_var(var_name);
    match var {
        Some(var) => {
            if var.len() != 1 {
                return Err(AndiError::new(&format!("{} not scalar", var_name)));
            }
            if var.data_type() != DataType::F32 {
                return Err(AndiError::new(&format!(
                    "{} unexpected data type: {}",
                    var_name,
                    var.data_type()
                )));
            }
            let val = reader.read_var_f32(var_name).unwrap()[0];
            Ok(Some(val))
        }
        None => Ok(None),
    }
}

pub fn read_optional_var_or_attr_f32(
    reader: &mut netcdf3::FileReader,
    var_name: &str,
) -> Result<Option<f32>, Box<dyn Error>> {
    let mut value = read_scalar_var_f32(reader, "sample_injection_volume")?;
    if value.is_none() {
        let attr_opt = reader.data_set().get_global_attr(var_name);
        if let Some(attr) = attr_opt {
            if let Some(val) = attr.get_f32() {
                match val {
                    [single_val] => value = Some(single_val.to_owned()),
                    _ => {
                        return Err(Box::new(AndiError::new(
                            "Unexpected content for sample_injection_volume.",
                        )))
                    }
                }
            } else if let Some(val) = attr.get_as_string() {
                if !val.is_empty() {
                    let v = val.parse::<f32>()?;
                    value = Some(v);
                }
            }
        }
    }
    Ok(value)
}

#[allow(dead_code)]
pub fn convert_utf8_cstr_to_str(bytes: &[u8]) -> String {
    let len = bytes.iter().position(|&c| c == 0u8).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..len]).to_string()
}

pub fn convert_iso_8859_1_cstr_to_str(bytes: &[u8]) -> String {
    // see https://stackoverflow.com/a/28175593 for why this works
    bytes
        .iter()
        .take_while(|&c| c != &0u8)
        .map(|&c| c as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_utf8_bytes_to_string_conversion() {
        let expect = "aäAÄ";
        // "aäAÄ" UTF-8 encoded with zero terminator in different encodings
        // no zero terminator
        let utf8_data: [u8; 6] = [0x61, 0xc3, 0xa4, 0x41, 0xc3, 0x84];
        // including zero terminator
        let utf8_data_zt: [u8; 7] = [0x61, 0xc3, 0xa4, 0x41, 0xc3, 0x84, 0];
        // additional bytes after zero terminator
        let utf8_data_zt_plus: [u8; 9] = [0x61, 0xc3, 0xa4, 0x41, 0xc3, 0x84, 0, 0xc3, 0xa4];

        assert_eq!(expect, convert_utf8_cstr_to_str(&utf8_data));
        assert_eq!(expect, convert_utf8_cstr_to_str(&utf8_data_zt));
        assert_eq!(expect, convert_utf8_cstr_to_str(&utf8_data_zt_plus));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_iso_8859_1_bytes_to_string_conversion() {
        let expect = "aäAÄ";
        // "aäAÄ" ISO-8859-1 encoded
        // no zero terminator
        let iso8850_1_data: [u8; 4] = [0x61, 0xe4, 0x41, 0xc4];
        // including zero terminator
        let iso8850_1_data_zt: [u8; 5] = [0x61, 0xe4, 0x41, 0xc4, 0];
        // additional bytes after zero terminator
        let iso8850_1_data_zt_plus: [u8; 6] = [0x61, 0xe4, 0x41, 0xc4, 0, 0x62];

        assert_eq!(expect, convert_iso_8859_1_cstr_to_str(&iso8850_1_data));
        assert_eq!(expect, convert_iso_8859_1_cstr_to_str(&iso8850_1_data_zt));
        assert_eq!(
            expect,
            convert_iso_8859_1_cstr_to_str(&iso8850_1_data_zt_plus)
        );
    }
}
