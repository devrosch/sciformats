// Copyright (c) 2025 Robert Schiwon
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

mod andi;
mod common;
mod gaml;
mod jdx;
mod spc;

/// Provides access to test resources. For non WASM this happens by opening them from the
/// filesystem, for WASM they are embedded into the binary.
///
/// Example:
///
/// The following example makes two files from the `./resources` directory available.
/// The `open_files($file_name)` function and for each file one constant with the format name
/// identifier holding the file name as &str are emitted.
///
/// ```
/// open_files!(
///     "resources/",
///     (
///         (FORMAT_NAME_IDENTIFIER_1, "sample_data_1.ext"),
///         (FORMAT_NAME_IDENTIFIER_1, "sample_data_2.ext"),
///     )
/// );
/// ```
macro_rules! open_files {
  ($root_path:literal, ($(($const_name:ident, $file_name:literal)),* $(,)?)) => {
      #[cfg(not(target_family = "wasm"))]
      use std::{
          fs::File,
          io::{Read, Seek},
          path::PathBuf,
      };
      #[cfg(target_family = "wasm")]
      use std::io::{Cursor, Read, Seek};

      $(
          const $const_name: &str = $file_name;
      )*

      #[cfg(not(target_family = "wasm"))]
      pub fn open_file(name: &str) -> (String, impl Read + Seek) {
          let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
          let src_file = file!();
          path.push(src_file);
          path.pop();
          path.push($root_path);
          path.push(name);

          let file = File::open(&path).unwrap();

          (path.to_str().unwrap().to_owned(), file)
      }

      #[cfg(target_family = "wasm")]
      pub fn open_file(name: &str) -> (String, impl Read + Seek) {
          $(
              const $const_name: &'static[u8] = include_bytes!(concat!(
                  $root_path,
                  $file_name
              ));
          )*

          let file = match name {
              $(
                  $file_name => Cursor::new($const_name),
              )*
              _ => panic!(),
          };
          (name.to_owned(), file)
      }
  };
}
pub(crate) use open_files;
