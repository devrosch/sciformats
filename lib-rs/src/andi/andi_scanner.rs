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

use super::{
    andi_chrom_parser::AndiChromParser, andi_chrom_reader::AndiChromReader,
    andi_ms_parser::AndiMsParser, andi_ms_reader::AndiMsReader,
};
use crate::{
    api::{Reader, Scanner},
    common::SfError,
    utils::is_recognized_extension,
};
use std::io::{Read, Seek};

#[derive(Default)]
pub struct AndiScanner {}

impl AndiScanner {
    const ACCEPTED_EXTENSIONS: [&'static str; 2] = ["cdf", "nc"];
    const MAGIC_BYTES: [u8; 3] = [0x43, 0x44, 0x46]; // "CDF"
    const AIA_TEMPLATE_REVISION_ATTR: &'static str = "aia_template_revision";
    const MS_TEMPLATE_REVISION_ATTR: &'static str = "ms_template_revision";
}

impl AndiScanner {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Seek + Read + 'static> Scanner<T> for AndiScanner {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        if !is_recognized_extension(path, &Self::ACCEPTED_EXTENSIONS) {
            return false;
        }

        // recognized extension => check first few bytes ("magic bytes")
        let mut buf = [0u8; 3];
        let read_success = input.read_exact(&mut buf);
        if read_success.is_err() {
            return false;
        }

        buf.as_slice() == Self::MAGIC_BYTES
    }

    fn get_reader(&self, path: &str, input: T) -> Result<Box<dyn Reader>, SfError> {
        let input_seek_read = Box::new(input);
        let cdf_reader = netcdf3::FileReader::open_seek_read(path, input_seek_read)
            .map_err(|e| SfError::from_source(e, "AnDI Error. Error parsing netCDF."))?;

        if cdf_reader
            .data_set()
            .has_global_attr(Self::AIA_TEMPLATE_REVISION_ATTR)
        {
            let file = AndiChromParser::parse_cdf(cdf_reader)?;
            return Ok(Box::new(AndiChromReader::new(path, file)));
        }
        if cdf_reader
            .data_set()
            .has_global_attr(Self::MS_TEMPLATE_REVISION_ATTR)
        {
            let file = AndiMsParser::parse_cdf(cdf_reader)?;
            return Ok(Box::new(AndiMsReader::new(path, file)));
        }

        Err(SfError::new(&format!(
            "Could not parse \"{}\". Expected one attribute of: {}, {}",
            path,
            Self::AIA_TEMPLATE_REVISION_ATTR,
            Self::MS_TEMPLATE_REVISION_ATTR
        ))
        .into())
    }
}
