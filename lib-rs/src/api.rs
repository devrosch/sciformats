use std::{io::{Seek, Read}, error::Error};

pub trait SeekRead: Seek + Read {}
impl<T: Seek + Read> SeekRead for T {}

// pub trait SciReader<T> {
//   fn read(input: Box<dyn SeekRead>) -> Result<T, Box<dyn Error>>;
// }

pub trait SciReader<T: Read + Seek, R> {
  fn read(name: &str, input: T) -> Result<R, Box<dyn Error>>;
}
