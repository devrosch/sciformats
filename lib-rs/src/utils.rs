use std::{error::Error, path::Path};

// -------------------------------------------------
// Util functions
// -------------------------------------------------

/// Check if path ends with one of the accepted extensions
pub(crate) fn is_recognized_extension(path: &str, accepted_extensions: &[&str]) -> bool {
    let p = Path::new(path);
    let extension = p
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());
    match extension {
        None => false,
        Some(ext) => accepted_extensions
            .iter()
            .any(|accept_ext| *accept_ext == ext),
    }
}

/// Convert path to indices
///
/// The path segments are expected to be separated by forward slashes.
/// Each segment needs to start with a positive integer, optionally followed by a minus and arbitrary text.
/// The root path may be designated by "" or "/".
///
/// Examples:
///     - "/": root path (also "" is accepted)
///     - "/"": root path
///     - "/3/2/1": indices 3, 2, 1
///     - "/0-some_text/1-some_more_text": indices 0, 1
///     - "2-some_text/3": indices 2, 3
///
/// Invalid examples:
///     - "/some_text/1": missing index for first segment
///     - "\1\1": wrong separator
///     - " /1": missing index for first (blank) segment
pub(crate) fn convert_path_to_node_indices(path: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut path_segments: Vec<&str> = path.split('/').collect();
    // remove blank start segment(s)
    match path_segments[..] {
        // "/" or ""
        ["", ""] | [""] => path_segments = vec![],
        // "/xyz"
        ["", ..] => {
            path_segments.remove(0);
        }
        _ => (),
    };
    // map segments to indices, expected segment structure is "n-some optional name"
    let mut indices: Vec<usize> = vec![];
    for seg in path_segments {
        let idx_str = seg.split_once('-').map_or(seg, |p| p.0);
        let idx = idx_str.parse::<usize>()?;
        indices.push(idx);
    }

    Ok(indices)
}

/// Convert UTF-8 C string to String
#[allow(dead_code)]
pub(crate) fn convert_utf8_cstr_to_str(bytes: &[u8]) -> String {
    let len = bytes.iter().position(|&c| c == 0u8).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..len]).to_string()
}

/// Convert ISO 8859-1 C string to String
pub(crate) fn from_iso_8859_1_cstr(bytes: &[u8]) -> String {
    // see https://stackoverflow.com/a/28175593 for why this works
    bytes
        .iter()
        .take_while(|&c| c != &0u8)
        .map(|&c| c as char)
        .collect()
}

/// Parse N zero terminated ISO 8859-1 strings each with a maximum (always consumed) length up to str_size.
#[allow(dead_code)]
pub(crate) fn from_iso_8859_1_fixed_size_cstr_arr<const N: usize>(
    bytes: &[u8],
    str_size: usize,
) -> [String; N] {
    let mut v: Vec<String> = vec![];
    for i in (0..(N * str_size)).step_by(str_size) {
        let slice = &bytes[i..(i + str_size)];
        let s = from_iso_8859_1_cstr(slice);
        v.push(s);
    }
    v.try_into().unwrap()
}

/// Parse N zero terminated ISO 8859-1 strings of variable length. If additional strings exist they are discarded.
pub(crate) fn from_iso_8859_1_cstr_arr<const N: usize>(bytes: &[u8]) -> Option<[String; N]> {
    let split: Vec<&[u8]> = bytes.split(|byte| *byte == 0).collect();
    if split.len() < N {
        return None;
    }
    let vec: Vec<String> = split
        .iter()
        .take(N)
        .map(|slice| from_iso_8859_1_cstr(slice))
        .collect();
    let res: [String; N] = vec.try_into().unwrap();
    Some(res)
}
