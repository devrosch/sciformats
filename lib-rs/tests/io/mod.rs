use std::{fs::File, path::PathBuf};

pub fn open_file(name: &str) -> (String, File) {
  let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  path.push("tests/resources/");
  path.push(name);
  let file = File::open(&path).unwrap();

  (path.to_str().unwrap().to_owned(), file)
}
