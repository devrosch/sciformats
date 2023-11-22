#[cfg(not(target_family = "wasm"))]
use std::{
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
};

#[cfg(target_family = "wasm")]
use std::io::{Cursor, Read, Seek};

#[cfg(not(target_family = "wasm"))]
pub fn open_file(name: &str) -> (String, impl Read + Seek) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/");
    path.push(name);

    let file = File::open(&path).unwrap();

    (path.to_str().unwrap().to_owned(), file)
}

#[cfg(target_family = "wasm")]
pub fn open_file(name: &str) -> (String, impl Read + Seek) {
    const ANDI_CHROM_VALID: &[u8] = include_bytes!("../resources/andi_chrom_valid.cdf");
    const ANDI_CHROM_QUIRKS: &[u8] = include_bytes!("../resources/andi_chrom_quirks.cdf");
    const NON_ANDI_CDF_FILE_PATH: &[u8] = include_bytes!("../resources/non_andi.cdf");
    const DUMMY: &[u8] = include_bytes!("../resources/dummy.cdf");
    const ANDI_MS_LIBRARY: &[u8] = include_bytes!("../resources/andi_ms_library.cdf");
    const ANDI_MS_CENTROID: &[u8] = include_bytes!("../resources/andi_ms_centroid.cdf");
    const ANDI_MS_CONTINUUM: &[u8] = include_bytes!("../resources/andi_ms_continuum.cdf");
    const ANDI_MS_SID: &[u8] = include_bytes!("../resources/andi_ms_sid.cdf");

    let file = match name {
        "andi_chrom_valid.cdf" => Cursor::new(ANDI_CHROM_VALID),
        "andi_chrom_quirks.cdf" => Cursor::new(ANDI_CHROM_QUIRKS),
        "non_andi.cdf" => Cursor::new(NON_ANDI_CDF_FILE_PATH),
        "dummy.cdf" => Cursor::new(DUMMY),
        "andi_ms_library.cdf" => Cursor::new(ANDI_MS_LIBRARY),
        "andi_ms_centroid.cdf" => Cursor::new(ANDI_MS_CENTROID),
        "andi_ms_continuum.cdf" => Cursor::new(ANDI_MS_CONTINUUM),
        "andi_ms_sid.cdf" => Cursor::new(ANDI_MS_SID),
        _ => panic!(),
    };

    (name.to_owned(), file)
}
