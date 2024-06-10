use super::JdxError;
use crate::api::{Parser, SeekBufRead};
use std::marker::PhantomData;

pub struct JdxParser {}

impl<T: SeekBufRead + 'static> Parser<T> for JdxParser {
    type R = JdxBlock<T>;
    type E = JdxError;

    fn parse(name: &str, input: T) -> Result<Self::R, Self::E> {
        Self::R::new(name, input)
    }
}

pub struct JdxBlock<T: SeekBufRead> {
    // todo: remove once T is actually used
    phantom: PhantomData<T>,
}

impl<T: SeekBufRead> JdxBlock<T> {
    pub fn new(name: &str, mut reader: T) -> Result<Self, JdxError> {
        todo!()
    }
}
