use super::{gaml_parser::Gaml, gaml_utils::map_gaml_parameters, GamlError};
use crate::{
    api::{Node, Parameter, Reader},
    utils::convert_path_to_node_indices,
};
use std::{error::Error, path::Path};

pub struct GamlReader {
    path: String,
    file: Gaml,
}

impl Reader for GamlReader {
    fn read(&self, path: &str) -> Result<Node, Box<dyn Error>> {
        let path_indices = convert_path_to_node_indices(path)?;
        match path_indices[..] {
            [] => Ok(self.read_root()?), // "", "/"
            [n] => {
                // todo: add logic
                Err(GamlError::new(&format!("Illegal node index: {n}")).into())
            }
            _ => Err(GamlError::new(&format!("Illegal node path: {}", path)).into()),
        }
    }
}

impl GamlReader {
    pub fn new(path: &str, file: Gaml) -> Self {
        Self {
            path: path.to_owned(),
            file,
        }
    }

    fn read_root(&self) -> Result<Node, GamlError> {
        let path = Path::new(&self.path);
        let file_name = path.file_name().map_or("", |f| f.to_str().unwrap_or(""));

        let mut parameters = vec![];
        parameters.push(Parameter::from_str_str("Version", &self.file.version));
        if let Some(name) = &self.file.name {
            let param = Parameter::from_str_str("Name", name);
            parameters.push(param);
        }
        if let Some(integrity) = &self.file.integrity {
            let param = Parameter::from_str_str(
                format!("Integrity (algorithm={})", integrity.algorithm),
                &integrity.value,
            );
            parameters.push(param);
        }
        parameters.extend(map_gaml_parameters(&self.file.parameters));

        let child_node_names = self
            .file
            .experiments
            .iter()
            .enumerate()
            .map(|(i, exper)| {
                format!(
                    "{}{}",
                    i,
                    exper
                        .name
                        .as_ref()
                        .map(|name| String::from(", ") + &name)
                        .unwrap_or_default()
                )
            })
            .collect();

        Ok(Node {
            name: file_name.to_owned(),
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
        })
    }
}
