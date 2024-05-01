use super::{
    gaml_parser::{Gaml, Peaktable},
    gaml_utils::map_gaml_parameters,
    GamlError,
};
use crate::{
    api::{Column, Node, Parameter, PointXy, Reader, Table, Value},
    utils::convert_path_to_node_indices,
};
use std::{collections::HashMap, error::Error, path::Path, vec};

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
            [exp_idx, trace_idx, n] => {
                let experiment =
                    self.file
                        .experiments
                        .get(exp_idx)
                        .ok_or(GamlError::new(&format!(
                            "Illegal experiment index: {exp_idx}"
                        )))?;
                let trace = experiment
                    .traces
                    .get(trace_idx)
                    .ok_or(GamlError::new(&format!("Illegal trace index: {trace_idx}")))?;

                let num_coordinates = trace.coordinates.len();
                if n < num_coordinates {
                    let coordinates_idx = n;
                    Ok(self.read_coordinates((exp_idx, trace_idx, coordinates_idx))?)
                } else {
                    let x_data_idx = n - num_coordinates;
                    Ok(self.read_x_data((exp_idx, trace_idx, x_data_idx))?)
                }
            }
            [exp_idx, trace_idx, x_data_or_coord_idx, n] => {
                let experiment =
                    self.file
                        .experiments
                        .get(exp_idx)
                        .ok_or(GamlError::new(&format!(
                            "Illegal experiment index: {exp_idx}"
                        )))?;
                let trace = experiment
                    .traces
                    .get(trace_idx)
                    .ok_or(GamlError::new(&format!("Illegal trace index: {trace_idx}")))?;
                let x_data_idx = x_data_or_coord_idx - trace.coordinates.len();
                let x_data = trace.x_data.get(x_data_idx).ok_or(GamlError::new(&format!(
                    "Illegal x_data index: {x_data_idx}"
                )))?;
                let num_alt_x_data = x_data.alt_x_data.len();
                if n < num_alt_x_data {
                    let alt_x_data_idx = n;
                    Ok(self.read_alt_x_data((exp_idx, trace_idx, x_data_idx, alt_x_data_idx))?)
                } else {
                    let y_data_idx = n - num_alt_x_data;
                    Ok(self.read_y_data((exp_idx, trace_idx, x_data_idx, y_data_idx))?)
                }
            }
            [exp_idx, trace_idx, x_data_idx, alt_x_data_or_y_data_idx, peaktable_idx] => {
                let experiment =
                    self.file
                        .experiments
                        .get(exp_idx)
                        .ok_or(GamlError::new(&format!(
                            "Illegal experiment index: {exp_idx}"
                        )))?;
                let trace = experiment
                    .traces
                    .get(trace_idx)
                    .ok_or(GamlError::new(&format!("Illegal trace index: {trace_idx}")))?;
                let x_data_idx = x_data_idx - trace.coordinates.len();
                let x_data = trace.x_data.get(x_data_idx).ok_or(GamlError::new(&format!(
                    "Illegal x_data index: {x_data_idx}"
                )))?;
                let y_data_idx = alt_x_data_or_y_data_idx - x_data.alt_x_data.len();
                let y_data = x_data
                    .y_data
                    .get(y_data_idx)
                    .ok_or(GamlError::new(&format!(
                        "Illegal y_data index: {y_data_idx}"
                    )))?;
                let peaktable =
                    y_data
                        .peaktables
                        .get(peaktable_idx)
                        .ok_or(GamlError::new(&format!(
                            "Illegal peaktable index: {peaktable_idx}"
                        )))?;

                Ok(self.read_peaktable(peaktable, peaktable_idx)?)
            }
            // todo: read basecurve
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
                    "Experiment {}{}",
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
            Some(exp_name) => format!("Experiment {index}, {exp_name}"),
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
                    "Trace {}{}",
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
            Some(trace_name) => format!("Trace {trace_idx}, {trace_name}"),
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
                    "Coordinates {}{}",
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
                    "Xdata {}{}",
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

    fn read_coordinates(
        &self,
        (exp_idx, trace_idx, coordinates_idx): (usize, usize, usize),
    ) -> Result<Node, GamlError> {
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
            .ok_or(GamlError::new(&format!("Illegal trace index: {trace_idx}")))?;
        let coordinates = trace
            .coordinates
            .get(coordinates_idx)
            .ok_or(GamlError::new(&format!(
                "Illegal coordinates index: {coordinates_idx}"
            )))?;

        let name = match &coordinates.label {
            None => coordinates_idx.to_string(),
            Some(label) => format!("Coordinates {coordinates_idx}, {label}"),
        };

        let mut parameters = vec![];
        parameters.push(Parameter::from_str_str(
            "Units",
            coordinates.units.to_string(),
        ));
        if let Some(label) = &coordinates.label {
            parameters.push(Parameter::from_str_str("Label", label));
        }
        if let Some(linkid) = &coordinates.linkid {
            parameters.push(Parameter::from_str_str("Linkid", linkid));
        }
        if let Some(valueorder) = &coordinates.valueorder {
            parameters.push(Parameter::from_str_str(
                "Valueorder",
                valueorder.to_string(),
            ));
        }
        for link in &coordinates.links {
            parameters.push(Parameter::from_str_str("Link linkref", &link.linkref));
        }
        parameters.extend(map_gaml_parameters(&coordinates.parameters));

        // map coordinate values as table
        let mut table = Table {
            column_names: vec![Column::new("value", "Value")],
            rows: vec![],
        };
        let values = coordinates.values.get_data()?;
        for value in values {
            let mut row = HashMap::new();
            row.insert("value".to_owned(), Value::F64(value));
            table.rows.push(row);
        }

        Ok(Node {
            name,
            parameters,
            // provide values as part of Ydata instead
            data: vec![],
            metadata: vec![],
            table: Some(table),
            child_node_names: vec![],
        })
    }

    fn read_x_data(
        &self,
        (exp_idx, trace_idx, x_data_idx): (usize, usize, usize),
    ) -> Result<Node, GamlError> {
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
            .ok_or(GamlError::new(&format!("Illegal trace index: {trace_idx}")))?;
        let x_data = trace.x_data.get(x_data_idx).ok_or(GamlError::new(&format!(
            "Illegal x_data index: {x_data_idx}"
        )))?;

        let name = match &x_data.label {
            None => x_data_idx.to_string(),
            Some(label) => format!("Xdata {x_data_idx}, {label}"),
        };

        let mut parameters = vec![];
        parameters.push(Parameter::from_str_str("Units", x_data.units.to_string()));
        if let Some(label) = &x_data.label {
            parameters.push(Parameter::from_str_str("Label", label));
        }
        if let Some(linkid) = &x_data.linkid {
            parameters.push(Parameter::from_str_str("Linkid", linkid));
        }
        if let Some(valueorder) = &x_data.valueorder {
            parameters.push(Parameter::from_str_str(
                "Valueorder",
                valueorder.to_string(),
            ));
        }
        for link in &x_data.links {
            parameters.push(Parameter::from_str_str("Link linkref", &link.linkref));
        }
        parameters.extend(map_gaml_parameters(&x_data.parameters));

        // do not map values here, provide it as part of Ydata instead

        let mut child_node_names: Vec<_> = x_data
            .alt_x_data
            .iter()
            .enumerate()
            .map(|(i, alt_x_data)| {
                format!(
                    "altXdata {}{}",
                    i,
                    alt_x_data
                        .label
                        .as_ref()
                        .map(|label| String::from(", ") + &label)
                        .unwrap_or_default()
                )
            })
            .collect();
        let y_data_names: Vec<_> = x_data
            .y_data
            .iter()
            .enumerate()
            .map(|(i, y_data)| {
                format!(
                    "Ydata {}{}",
                    i,
                    y_data
                        .label
                        .as_ref()
                        .map(|label| String::from(", ") + &label)
                        .unwrap_or_default()
                )
            })
            .collect();
        child_node_names.extend(y_data_names);

        Ok(Node {
            name,
            parameters,
            // provide values as part of Ydata instead
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
        })
    }

    fn read_alt_x_data(
        &self,
        (exp_idx, trace_idx, x_data_idx, alt_x_data_idx): (usize, usize, usize, usize),
    ) -> Result<Node, GamlError> {
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
            .ok_or(GamlError::new(&format!("Illegal trace index: {trace_idx}")))?;
        let x_data = trace.x_data.get(x_data_idx).ok_or(GamlError::new(&format!(
            "Illegal x_data index: {x_data_idx}"
        )))?;
        let alt_x_data = x_data
            .alt_x_data
            .get(alt_x_data_idx)
            .ok_or(GamlError::new(&format!(
                "Illegal alt_x_data index: {alt_x_data_idx}"
            )))?;

        let name = match &alt_x_data.label {
            None => alt_x_data_idx.to_string(),
            Some(label) => format!("altXdata {alt_x_data_idx}, {label}"),
        };

        let mut parameters = vec![];
        parameters.push(Parameter::from_str_str("Units", x_data.units.to_string()));
        if let Some(label) = &x_data.label {
            parameters.push(Parameter::from_str_str("Label", label));
        }
        if let Some(linkid) = &x_data.linkid {
            parameters.push(Parameter::from_str_str("Linkid", linkid));
        }
        if let Some(valueorder) = &x_data.valueorder {
            parameters.push(Parameter::from_str_str(
                "Valueorder",
                valueorder.to_string(),
            ));
        }
        for link in &x_data.links {
            parameters.push(Parameter::from_str_str("Link linkref", &link.linkref));
        }
        parameters.extend(map_gaml_parameters(&x_data.parameters));

        // map altXdata values as table
        let mut table = Table {
            column_names: vec![Column::new("value", "Value")],
            rows: vec![],
        };
        let values = alt_x_data.values.get_data()?;
        for value in values {
            let mut row = HashMap::new();
            row.insert("value".to_owned(), Value::F64(value));
            table.rows.push(row);
        }

        // no child nodes

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: Some(table),
            child_node_names: vec![],
        })
    }

    fn read_y_data(
        &self,
        (exp_idx, trace_idx, x_data_idx, y_data_idx): (usize, usize, usize, usize),
    ) -> Result<Node, GamlError> {
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
            .ok_or(GamlError::new(&format!("Illegal trace index: {trace_idx}")))?;
        let x_data = trace.x_data.get(x_data_idx).ok_or(GamlError::new(&format!(
            "Illegal x_data index: {x_data_idx}"
        )))?;
        let y_data = x_data
            .y_data
            .get(y_data_idx)
            .ok_or(GamlError::new(&format!(
                "Illegal y_data index: {y_data_idx}"
            )))?;

        let name = match &y_data.label {
            None => y_data_idx.to_string(),
            Some(label) => format!("Ydata {y_data_idx}, {label}"),
        };

        let mut parameters = vec![];
        parameters.push(Parameter::from_str_str("Units", y_data.units.to_string()));
        if let Some(label) = &y_data.label {
            parameters.push(Parameter::from_str_str("Label", label));
        }
        parameters.extend(map_gaml_parameters(&y_data.parameters));

        let x_values = x_data.values.get_data()?;
        let y_values = y_data.values.get_data()?;
        let data = x_values
            .into_iter()
            .zip(y_values)
            .map(|(x, y)| PointXy::new(x, y))
            .collect();

        let child_node_names: Vec<_> = y_data
            .peaktables
            .iter()
            .enumerate()
            .map(|(i, peaktable)| {
                format!(
                    "Peaktable {}{}",
                    i,
                    peaktable
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
            data,
            metadata: vec![],
            table: None,
            child_node_names,
        })
    }

    fn read_peaktable(&self, peaktable: &Peaktable, index: usize) -> Result<Node, GamlError> {
        let name = match &peaktable.name {
            None => format!("Peaktable {index}"),
            Some(name) => format!("Peaktable {index}, {}", name),
        };

        let mut parameters = vec![];
        // peaktable attributes and parameters
        if let Some(name) = &peaktable.name {
            parameters.push(Parameter::from_str_str("Name", name));
        }
        parameters.extend(map_gaml_parameters(&peaktable.parameters));
        // peak parameters
        for (i, peak) in peaktable.peaks.iter().enumerate() {
            let mut peak_params = map_gaml_parameters(&peaktable.parameters);
            for param in &mut peak_params {
                param
                    .key
                    .insert_str(0, &format!("Peak {}, number {}, ", i, peak.number));
            }
            parameters.extend(peak_params);
        }

        // map peaks as table
        let mut table = Table {
            column_names: vec![
                Column::new("number", "Number"),
                Column::new("group", "Group"),
                Column::new("name", "Name"),
                Column::new("peak_x_value", "peakXvalue"),
                Column::new("peak_y_value", "peakYvalue"),
                Column::new("baseline_start_x_value", "Baseline Start X Value"),
                Column::new("baseline_start_y_value", "Baseline Start Y Value"),
                Column::new("baseline_end_x_value", "Baseline End X Value"),
                Column::new("baseline_end_y_value", "Baseline End Y Value"),
            ],
            rows: vec![],
        };
        for peak in &peaktable.peaks {
            let mut row = HashMap::new();
            row.insert("number".to_owned(), Value::U64(peak.number));
            if let Some(group) = &peak.group {
                row.insert("group".to_owned(), Value::String(group.to_owned()));
            }
            if let Some(name) = &peak.name {
                row.insert("name".to_owned(), Value::String(name.to_owned()));
            }
            row.insert("peak_x_value".to_owned(), Value::F64(peak.peak_x_value));
            row.insert("peak_y_value".to_owned(), Value::F64(peak.peak_y_value));

            // add baseline values (except basecurve) to table
            if let Some(baseline) = &peak.baseline {
                row.insert(
                    "baseline_start_x_value".to_owned(),
                    Value::F64(baseline.start_x_value),
                );
                row.insert(
                    "baseline_start_y_value".to_owned(),
                    Value::F64(baseline.start_y_value),
                );
                row.insert(
                    "baseline_end_x_value".to_owned(),
                    Value::F64(baseline.end_x_value),
                );
                row.insert(
                    "baseline_end_y_value".to_owned(),
                    Value::F64(baseline.end_y_value),
                );
            }
            table.rows.push(row);
        }

        let mut child_node_names = vec![];
        for (i, peak) in peaktable.peaks.iter().enumerate() {
            if let Some(_basecurve) = peak.baseline.as_ref().and_then(|bl| bl.basecurve.as_ref()) {
                child_node_names.push(format!("Basecurve Peak {}, number {}", i, peak.number));
            }
        }

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: Some(table),
            child_node_names,
        })
    }
}
