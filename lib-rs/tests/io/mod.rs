#[cfg(not(target_family = "wasm"))]
use std::{
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
};

#[cfg(target_family = "wasm")]
use std::io::{Cursor, Read, Seek};

#[cfg(not(target_family = "wasm"))]
pub fn open_file(root_path: &str, name: &str) -> (String, impl Read + Seek) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/");
    path.push(root_path);
    path.push(name);

    let file = File::open(&path).unwrap();

    (path.to_str().unwrap().to_owned(), file)
}

#[cfg(target_family = "wasm")]
pub fn open_file(root_path: &str, name: &str) -> (String, impl Read + Seek) {
    const ANDI_CHROM_VALID: &[u8] = include_bytes!("../resources/andi/andi_chrom_valid.cdf");
    const ANDI_CHROM_QUIRKS: &[u8] = include_bytes!("../resources/andi/andi_chrom_quirks.cdf");
    const NON_ANDI_CDF_FILE_PATH: &[u8] = include_bytes!("../resources/andi/non_andi.cdf");
    const DUMMY: &[u8] = include_bytes!("../resources/andi/dummy.cdf");
    const ANDI_MS_LIBRARY: &[u8] = include_bytes!("../resources/andi/andi_ms_library.cdf");
    const ANDI_MS_CENTROID: &[u8] = include_bytes!("../resources/andi/andi_ms_centroid.cdf");
    const ANDI_MS_CONTINUUM: &[u8] = include_bytes!("../resources/andi/andi_ms_continuum.cdf");
    const ANDI_MS_SID: &[u8] = include_bytes!("../resources/andi/andi_ms_sid.cdf");
    const SPC_NEW_FORMAT_LE: &[u8] = include_bytes!("../resources/spc/new_format_le.spc");
    const SPC_NEW_FORMAT_BE: &[u8] = include_bytes!("../resources/spc/new_format_be.spc");
    const SPC_NEW_FORMAT_LE_I16_Y: &[u8] =
        include_bytes!("../resources/spc/new_format_le_i16_y.spc");
    const SPC_NEW_FORMAT_LE_I32_Y: &[u8] =
        include_bytes!("../resources/spc/new_format_le_i32_y.spc");
    const SPC_NEW_FORMAT_LE_TXVALS: &[u8] =
        include_bytes!("../resources/spc/new_format_le_txvals.spc");
    const SPC_NEW_FORMAT_LE_TXYXYS: &[u8] =
        include_bytes!("../resources/spc/new_format_le_txyxys.spc");
    const SPC_NEW_FORMAT_LE_4D: &[u8] = include_bytes!("../resources/spc/new_format_le_4d.spc");
    const SPC_NEW_FORMAT_LE_DIR: &[u8] = include_bytes!("../resources/spc/new_format_le_dir.spc");
    const SPC_INVALID: &[u8] = include_bytes!("../resources/spc/invalid.spc");
    const SPC_NEW_FORMAT_LE_CORRUPT: &[u8] =
        include_bytes!("../resources/spc/new_format_le_corrupt.spc");

    let file = match (root_path, name) {
        ("andi", "andi_chrom_valid.cdf") => Cursor::new(ANDI_CHROM_VALID),
        ("andi", "andi_chrom_quirks.cdf") => Cursor::new(ANDI_CHROM_QUIRKS),
        ("andi", "non_andi.cdf") => Cursor::new(NON_ANDI_CDF_FILE_PATH),
        ("andi", "dummy.cdf") => Cursor::new(DUMMY),
        ("andi", "andi_ms_library.cdf") => Cursor::new(ANDI_MS_LIBRARY),
        ("andi", "andi_ms_centroid.cdf") => Cursor::new(ANDI_MS_CENTROID),
        ("andi", "andi_ms_continuum.cdf") => Cursor::new(ANDI_MS_CONTINUUM),
        ("andi", "andi_ms_sid.cdf") => Cursor::new(ANDI_MS_SID),
        ("spc", "new_format_le.spc") => Cursor::new(SPC_NEW_FORMAT_LE),
        ("spc", "new_format_be.spc") => Cursor::new(SPC_NEW_FORMAT_BE),
        ("spc", "new_format_le_i16_y.spc") => Cursor::new(SPC_NEW_FORMAT_LE_I16_Y),
        ("spc", "new_format_le_i32_y.spc") => Cursor::new(SPC_NEW_FORMAT_LE_I32_Y),
        ("spc", "new_format_le_txvals.spc") => Cursor::new(SPC_NEW_FORMAT_LE_TXVALS),
        ("spc", "new_format_le_txyxys.spc") => Cursor::new(SPC_NEW_FORMAT_LE_TXYXYS),
        ("spc", "new_format_le_4d.spc") => Cursor::new(SPC_NEW_FORMAT_LE_4D),
        ("spc", "new_format_le_dir.spc") => Cursor::new(SPC_NEW_FORMAT_LE_DIR),
        ("spc", "invalid.spc") => Cursor::new(SPC_INVALID),
        ("spc", "new_format_le_corrupt.spc") => Cursor::new(SPC_NEW_FORMAT_LE_CORRUPT),
        _ => panic!(),
    };

    (name.to_owned(), file)
}
