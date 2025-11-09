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

mod jdx_audit_trail_parser;
mod jdx_data_parser;
pub mod jdx_parser;
mod jdx_peak_assignments_parser;
mod jdx_peak_table_parser;
pub mod jdx_reader;
pub mod jdx_scanner;
mod jdx_utils;

use crate::api::SeekBufRead;
use crate::common::SfError;

trait JdxSequenceParser<'r, T: SeekBufRead>: Sized {
    type Item;

    fn new(variable_list: &'r str, reader: &'r mut T) -> Result<Self, SfError>;
    fn next(&mut self) -> Result<Option<Self::Item>, SfError>;
    fn into_reader(self) -> &'r mut T;
}
