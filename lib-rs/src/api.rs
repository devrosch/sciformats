use std::{
    error::Error,
    io::{Read, Seek},
};

pub trait SciParser<T: Read + Seek> {
    type R;

    fn parse(name: &str, input: T) -> Result<Self::R, Box<dyn Error>>;
}
