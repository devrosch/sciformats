use std::{
    error::Error,
    io::{Read, Seek},
};

pub trait SeekRead: Seek + Read {}
impl<T: Seek + Read> SeekRead for T {}

pub trait SciReader<T: Read + Seek, R> {
    fn read(name: &str, input: T) -> Result<R, Box<dyn Error>>;
}
