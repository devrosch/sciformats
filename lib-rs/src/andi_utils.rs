use crate::andi::AndiError;
use netcdf3::DataVector;
use std::error::Error;

pub fn read_index_from_var_f32(
    var: &Option<(&str, DataVector)>,
    index: usize,
) -> Result<Option<f32>, Box<dyn Error>> {
    let res = read_index_from_slice(
        var.as_ref().map(|(_, v)| v.get_f32()).flatten(),
        var.as_ref().map(|(name, _)| *name).unwrap_or_default(),
        index,
    )?
    .copied();

    Ok(res)
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
) -> Result<Option<(&'a str, DataVector)>, Box<dyn Error>> {
    if reader.data_set().get_var(var_name).is_some() {
        Ok(Some((var_name, reader.read_var(var_name)?)))
    } else {
        Ok(Option::None)
    }
}
