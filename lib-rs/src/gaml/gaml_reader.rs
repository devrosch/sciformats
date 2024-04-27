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
            [exp_idx] => Ok(self.read_experiment(exp_idx)?),
            [exp_idx, trace_idx] => Ok(self.read_trace((exp_idx, trace_idx))?),
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

    fn read_experiment(&self, index: usize) -> Result<Node, GamlError> {
        let experiment = self
            .file
            .experiments
            .get(index)
            .ok_or(GamlError::new(&format!(
                "Illegal experiment index: {index}"
            )))?;

        let name = match &experiment.name {
            None => index.to_string(),
            Some(exp_name) => format!("{index}, {exp_name}"),
        };

        let mut parameters = vec![];
        if let Some(name) = &experiment.name {
            parameters.push(Parameter::from_str_str("Name", name));
        }
        if let Some(date) = &experiment.collectdate {
            parameters.push(Parameter::from_str_str("Collectdate", date.to_rfc3339()));
        }
        parameters.extend(map_gaml_parameters(&experiment.parameters));

        let child_node_names = experiment
            .traces
            .iter()
            .enumerate()
            .map(|(i, trace)| {
                format!(
                    "{}{}",
                    i,
                    trace
                        .name
                        .as_ref()
                        .map(|name| String::from(", ") + &name)
                        .unwrap_or_default()
                )
            })
            .collect();

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
        })
    }

    fn read_trace(&self, (exp_idx, trace_idx): (usize, usize)) -> Result<Node, GamlError> {
        let experiment = self
            .file
            .experiments
            .get(exp_idx)
            .ok_or(GamlError::new(&format!(
                "Illegal experiment index: {exp_idx}"
            )))?;
        let trace = experiment
            .traces
            .get(trace_idx)
            .ok_or(GamlError::new(&format!("Illegal trace index: {exp_idx}")))?;

        let name = match &trace.name {
            None => trace_idx.to_string(),
            Some(trace_name) => format!("{trace_idx}, {trace_name}"),
        };

        let mut parameters = vec![];
        if let Some(name) = &trace.name {
            parameters.push(Parameter::from_str_str("Name", name));
        }
        parameters.push(Parameter::from_str_str(
            "Technique",
            trace.technique.to_string(),
        ));
        parameters.extend(map_gaml_parameters(&experiment.parameters));

        let mut child_node_names: Vec<_> = trace
            .coordinates
            .iter()
            .enumerate()
            .map(|(i, coordinates)| {
                format!(
                    "{}{}",
                    i,
                    coordinates
                        .label
                        .as_ref()
                        .map(|label| String::from(", ") + &label)
                        .unwrap_or_default()
                )
            })
            .collect();

        let x_data_names: Vec<_> = trace
            .x_data
            .iter()
            .enumerate()
            .map(|(i, x_data)| {
                format!(
                    "{}{}",
                    i,
                    x_data
                        .label
                        .as_ref()
                        .map(|label| String::from(", ") + &label)
                        .unwrap_or_default()
                )
            })
            .collect();
        child_node_names.extend(x_data_names);

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
        })
    }
}
