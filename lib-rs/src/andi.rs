use std::collections::BTreeSet;
use std::str::FromStr;
use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub struct AndiError {
    message: String,
}

impl AndiError {
    pub fn new(msg: &str) -> AndiError {
        AndiError {
            message: msg.into(),
        }
    }
}

impl FromStr for AndiError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AndiError { message: s.into() })
    }
}

impl Error for AndiError {}

impl fmt::Display for AndiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AndiCategory {
    C1,
    C2,
    C3,
    C4,
    C5,
}

impl FromStr for AndiCategory {
    type Err = AndiError;

    fn from_str(input: &str) -> Result<AndiCategory, Self::Err> {
        match input {
            "C1" => Ok(AndiCategory::C1),
            "C2" => Ok(AndiCategory::C2),
            "C3" => Ok(AndiCategory::C3),
            "C4" => Ok(AndiCategory::C4),
            "C5" => Ok(AndiCategory::C5),
            _ => Err(AndiError::new(&format!("Illegal category: {}", input))),
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
struct AndiDatasetCompleteness {
    categories: BTreeSet<AndiCategory>,
}

impl AndiDatasetCompleteness {
    pub fn new(categories: Vec<AndiCategory>) -> AndiDatasetCompleteness {
        let set = categories.into_iter().collect::<BTreeSet<AndiCategory>>();
        AndiDatasetCompleteness { categories: set }
    }
}

impl FromStr for AndiDatasetCompleteness {
    type Err = AndiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut categories: BTreeSet<AndiCategory> = BTreeSet::new();
        for cat_str in s.split("+") {
            let cat = AndiCategory::from_str(cat_str)?;
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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_category_string_mapping() {
        assert_eq!(AndiCategory::from_str("C1").unwrap(), AndiCategory::C1);
        assert_eq!(AndiCategory::from_str("C2").unwrap(), AndiCategory::C2);
        assert_eq!(AndiCategory::from_str("C3").unwrap(), AndiCategory::C3);
        assert_eq!(AndiCategory::from_str("C4").unwrap(), AndiCategory::C4);
        assert_eq!(AndiCategory::from_str("C5").unwrap(), AndiCategory::C5);

        assert_eq!(
            AndiCategory::from_str("X9").unwrap_err(),
            AndiError::new("Illegal category: X9")
        );

        assert_eq!(AndiCategory::C1.to_string(), "C1");
        assert_eq!(AndiCategory::C2.to_string(), "C2");
        assert_eq!(AndiCategory::C3.to_string(), "C3");
        assert_eq!(AndiCategory::C4.to_string(), "C4");
        assert_eq!(AndiCategory::C5.to_string(), "C5");
    }

    #[test]
    fn test_dataset_completeness_mapping() {
        assert_eq!(
            AndiDatasetCompleteness::from_str("C1").unwrap(),
            AndiDatasetCompleteness::new(vec![
                AndiCategory::C1,
            ])
        );

        assert_eq!(
            AndiDatasetCompleteness::from_str("C1+C3").unwrap(),
            AndiDatasetCompleteness::new(vec![
                AndiCategory::C1,
                AndiCategory::C3,
            ])
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

        assert_eq!(
            AndiDatasetCompleteness::from_str("C1+X2").unwrap_err(),
            AndiError::new("Illegal category: X2")
        );

        assert_eq!(
            AndiDatasetCompleteness::from_str("C1 C3").unwrap_err(),
            AndiError::new("Illegal category: C1 C3")
        );

        assert_eq!(
            AndiDatasetCompleteness::from_str("C1+C3+C2+C5+C4").unwrap().to_string(),
            "C1+C2+C3+C4+C5"
        );
    }
}
