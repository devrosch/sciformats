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
    const DUMMY: &[u8] = include_bytes!("../resources/dummy.cdf");

    let file = match name {
        "andi_chrom_valid.cdf" => Cursor::new(ANDI_CHROM_VALID),
        "dummy.cdf" => Cursor::new(DUMMY),
        _ => panic!(),
    };

    (name.to_owned(), file)
}
