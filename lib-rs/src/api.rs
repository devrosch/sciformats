use std::{
    error::Error,
    io::{Read, Seek},
};

pub trait SeekRead: Seek + Read {}
impl<T: Seek + Read> SeekRead for T {}

pub trait SciReader<T: Read + Seek> {
    type R;

    fn read(name: &str, input: T) -> Result<Self::R, Box<dyn Error>>;
}
