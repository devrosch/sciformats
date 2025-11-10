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

pub mod andi_chrom_parser;
pub mod andi_chrom_reader;
pub mod andi_enums;
pub mod andi_ms_parser;
pub mod andi_ms_reader;
pub mod andi_scanner;
mod andi_utils;

use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use crate::common::SfError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AndiCategory {
    C1,
    C2,
    C3,
    C4,
    C5,
}

impl FromStr for AndiCategory {
    type Err = SfError;

    fn from_str(input: &str) -> Result<AndiCategory, Self::Err> {
        match input {
            "C1" => Ok(AndiCategory::C1),
            "C2" => Ok(AndiCategory::C2),
            "C3" => Ok(AndiCategory::C3),
            "C4" => Ok(AndiCategory::C4),
            "C5" => Ok(AndiCategory::C5),
            _ => Err(SfError::new(&format!("Illegal category: {}", input))),
        }
    }
}

impl fmt::Display for AndiCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AndiCategory::C1 => write!(f, "C1"),
            AndiCategory::C2 => write!(f, "C2"),
            AndiCategory::C3 => write!(f, "C3"),
            AndiCategory::C4 => write!(f, "C4"),
            AndiCategory::C5 => write!(f, "C5"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AndiDatasetCompleteness {
    categories: BTreeSet<AndiCategory>,
}

impl AndiDatasetCompleteness {
    pub fn new(categories: Vec<AndiCategory>) -> AndiDatasetCompleteness {
        let set = categories.into_iter().collect::<BTreeSet<AndiCategory>>();
        AndiDatasetCompleteness { categories: set }
    }
}

impl FromStr for AndiDatasetCompleteness {
    type Err = SfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut categories: BTreeSet<AndiCategory> = BTreeSet::new();
        for cat_str in s.split('+') {
            // quirk: also accept zero terminated string for category
            let non_zero_term_cat_str = cat_str.trim_end_matches(char::from(0));
            let cat = AndiCategory::from_str(non_zero_term_cat_str)?;
            categories.insert(cat);
        }
        Ok(AndiDatasetCompleteness { categories })
    }
}

impl fmt::Display for AndiDatasetCompleteness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: String = self
            .categories
            .iter()
            .map(|cat| cat.to_string())
            .collect::<Vec<String>>()
            .join("+");
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_to_string_returns_error_message() {
        let error = SfError::new("Error message");
        assert_eq!("Error message", error.to_string());
    }

    #[test]
    fn map_valid_strings_to_categories_succeeds() {
        assert_eq!(AndiCategory::from_str("C1").unwrap(), AndiCategory::C1);
        assert_eq!(AndiCategory::from_str("C2").unwrap(), AndiCategory::C2);
        assert_eq!(AndiCategory::from_str("C3").unwrap(), AndiCategory::C3);
        assert_eq!(AndiCategory::from_str("C4").unwrap(), AndiCategory::C4);
        assert_eq!(AndiCategory::from_str("C5").unwrap(), AndiCategory::C5);
    }

    #[test]
    fn map_invalid_string_to_category_fails() {
        assert_eq!(
            AndiCategory::from_str("X9").unwrap_err().to_string(),
            "Illegal category: X9"
        );
    }

    #[test]
    fn map_category_to_string_succeeds() {
        assert_eq!(AndiCategory::C1.to_string(), "C1");
        assert_eq!(AndiCategory::C2.to_string(), "C2");
        assert_eq!(AndiCategory::C3.to_string(), "C3");
        assert_eq!(AndiCategory::C4.to_string(), "C4");
        assert_eq!(AndiCategory::C5.to_string(), "C5");
    }

    #[test]
    fn map_valid_strings_to_dataset_completeness_succeeds() {
        assert_eq!(
            AndiDatasetCompleteness::from_str("C1").unwrap(),
            AndiDatasetCompleteness::new(vec![AndiCategory::C1,])
        );

        assert_eq!(
            AndiDatasetCompleteness::from_str("C1+C3").unwrap(),
            AndiDatasetCompleteness::new(vec![AndiCategory::C1, AndiCategory::C3,])
        );

        assert_eq!(
            AndiDatasetCompleteness::from_str("C1+C2+C3+C4+C5").unwrap(),
            AndiDatasetCompleteness::new(vec![
                AndiCategory::C1,
                AndiCategory::C2,
                AndiCategory::C3,
                AndiCategory::C4,
                AndiCategory::C5,
            ])
        );
    }

    #[test]
    fn map_zero_terminated_string_to_dataset_completeness_succeeds() {
        assert_eq!(
            AndiDatasetCompleteness::from_str("C1+C2\0").unwrap(),
            AndiDatasetCompleteness::new(vec![AndiCategory::C1, AndiCategory::C2,])
        );
    }

    #[test]
    fn map_invalid_strings_to_dataset_completeness_fails() {
        assert_eq!(
            AndiDatasetCompleteness::from_str("C1+X2")
                .unwrap_err()
                .to_string(),
            "Illegal category: X2"
        );

        assert_eq!(
            AndiDatasetCompleteness::from_str("C1 C3")
                .unwrap_err()
                .to_string(),
            "Illegal category: C1 C3"
        );
    }

    #[test]
    fn map_dataset_completeness_to_string_succeeds() {
        assert_eq!(
            AndiDatasetCompleteness::from_str("C1+C3+C2+C5+C4")
                .unwrap()
                .to_string(),
            "C1+C2+C3+C4+C5"
        );
    }
}
