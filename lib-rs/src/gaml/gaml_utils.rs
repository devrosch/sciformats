use super::GamlError;
use std::str;

pub(super) fn read_item_at_index<'a, T>(
    slice: &'a [T],
    index: usize,
    context: &str,
) -> Result<&'a T, GamlError> {
    slice.get(index).ok_or(GamlError::new(&format!(
        "Illegal {} index: {}",
        context, index
    )))
}
