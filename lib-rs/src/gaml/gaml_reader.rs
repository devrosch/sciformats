use super::{
    gaml_parser::{
        AltXdata, Basecurve, Coordinates, Experiment, Gaml, Peaktable, Trace, Xdata, Ydata,
    },
    gaml_utils::{map_gaml_parameters, map_values_attributes, read_elem},
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
            [] => Ok(self.map_root()?), // "", "/"
            [exp_idx] => {
                let experiment = read_elem("experiment", &self.file.experiments, exp_idx)?;
                Ok(self.map_experiment(experiment, exp_idx)?)
            }
            [exp_idx, trace_idx] => {
                let experiment = read_elem("experiment", &self.file.experiments, exp_idx)?;
                let trace = read_elem("trace", &experiment.traces, trace_idx)?;
                Ok(self.map_trace(trace, trace_idx)?)
            }
            [exp_idx, trace_idx, n] => {
                let experiment = read_elem("experiment", &self.file.experiments, exp_idx)?;
                let trace = read_elem("trace", &experiment.traces, trace_idx)?;

                let num_coordinates = trace.coordinates.len();
                if n < num_coordinates {
                    let coordinates_idx = n;
                    let coordinates =
                        read_elem("coordinates", &trace.coordinates, coordinates_idx)?;
                    Ok(self.map_coordinates(coordinates, coordinates_idx)?)
                } else {
                    let x_data_idx = n - num_coordinates;
                    let x_data = read_elem("x_data", &trace.x_data, x_data_idx)?;
                    Ok(self.map_x_data(x_data, x_data_idx)?)
                }
            }
            [exp_idx, trace_idx, x_data_or_coord_idx, n] => {
                let experiment = read_elem("experiment", &self.file.experiments, exp_idx)?;
                let trace = read_elem("trace", &experiment.traces, trace_idx)?;
                if x_data_or_coord_idx < trace.coordinates.len() {
                    return Err(GamlError::new(&format!(
                        "Illegal x_data index: {x_data_or_coord_idx}"
                    )))?;
                }
                let x_data_idx = x_data_or_coord_idx - trace.coordinates.len();
                let x_data = read_elem("x_data", &trace.x_data, x_data_idx)?;
                let num_alt_x_data = x_data.alt_x_data.len();
                if n < num_alt_x_data {
                    let alt_x_data_idx = n;
                    let alt_x_data = read_elem("alt_x_data", &x_data.alt_x_data, alt_x_data_idx)?;
                    Ok(self.map_alt_x_data(alt_x_data, alt_x_data_idx)?)
                } else {
                    let y_data_idx = n - num_alt_x_data;
                    let y_data = read_elem("y_data", &x_data.y_data, y_data_idx)?;
                    Ok(self.map_y_data(x_data, y_data, y_data_idx)?)
                }
            }
            [exp_idx, trace_idx, x_data_or_coord_idx, alt_x_data_or_y_data_idx, peaktable_idx] => {
                let experiment = read_elem("experiment", &self.file.experiments, exp_idx)?;
                let trace = read_elem("trace", &experiment.traces, trace_idx)?;
                if x_data_or_coord_idx < trace.coordinates.len() {
                    return Err(GamlError::new(&format!(
                        "Illegal x_data index: {x_data_or_coord_idx}"
                    )))?;
                }
                let x_data_idx = x_data_or_coord_idx - trace.coordinates.len();
                let x_data = read_elem("x_data", &trace.x_data, x_data_idx)?;
                if alt_x_data_or_y_data_idx < x_data.alt_x_data.len() {
                    return Err(GamlError::new(&format!(
                        "Illegal y_data index: {alt_x_data_or_y_data_idx}"
                    )))?;
                }
                let y_data_idx = alt_x_data_or_y_data_idx - x_data.alt_x_data.len();
                let y_data = read_elem("y_data", &x_data.y_data, y_data_idx)?;
                let peaktable = read_elem("peaktable", &y_data.peaktables, peaktable_idx)?;
                Ok(self.map_peaktable(peaktable, peaktable_idx)?)
            }
            [exp_idx, trace_idx, x_data_or_coord_idx, alt_x_data_or_y_data_idx, peaktable_idx, basecurve_idx] =>
            {
                let experiment = read_elem("experiment", &self.file.experiments, exp_idx)?;
                let trace = read_elem("trace", &experiment.traces, trace_idx)?;
                if x_data_or_coord_idx < trace.coordinates.len() {
                    return Err(GamlError::new(&format!(
                        "Illegal x_data index: {x_data_or_coord_idx}"
                    )))?;
                }
                let x_data_idx = x_data_or_coord_idx - trace.coordinates.len();
                let x_data = read_elem("x_data", &trace.x_data, x_data_idx)?;
                if alt_x_data_or_y_data_idx < x_data.alt_x_data.len() {
                    return Err(GamlError::new(&format!(
                        "Illegal y_data index: {alt_x_data_or_y_data_idx}"
                    )))?;
                }
                let y_data_idx = alt_x_data_or_y_data_idx - x_data.alt_x_data.len();
                let y_data = read_elem("y_data", &x_data.y_data, y_data_idx)?;
                let peaktable = read_elem("peaktable", &y_data.peaktables, peaktable_idx)?;
                for (i, peak) in peaktable.peaks.iter().enumerate() {
                    if let Some(basecurve) =
                        peak.baseline.as_ref().and_then(|bl| bl.basecurve.as_ref())
                    {
                        if i == basecurve_idx {
                            return Ok(self.map_basecurve(basecurve)?);
                        }
                    }
                }
                Err(GamlError::new(&format!(
                    "Illegal basecurve index: {basecurve_idx}"
                )))?
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

    fn map_root(&self) -> Result<Node, GamlError> {
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

    fn map_experiment(&self, experiment: &Experiment, index: usize) -> Result<Node, GamlError> {
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

    fn map_trace(&self, trace: &Trace, index: usize) -> Result<Node, GamlError> {
        let name = match &trace.name {
            None => index.to_string(),
            Some(trace_name) => format!("Trace {index}, {trace_name}"),
        };

        let mut parameters = vec![];
        if let Some(name) = &trace.name {
            parameters.push(Parameter::from_str_str("Name", name));
        }
        parameters.push(Parameter::from_str_str(
            "Technique",
            trace.technique.to_string(),
        ));
        parameters.extend(map_gaml_parameters(&trace.parameters));

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

    fn map_coordinates(&self, coordinates: &Coordinates, index: usize) -> Result<Node, GamlError> {
        let name = match &coordinates.label {
            None => index.to_string(),
            Some(label) => format!("Coordinates {index}, {label}"),
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
        parameters.extend(map_values_attributes("Values", &coordinates.values));

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

    fn map_x_data(&self, x_data: &Xdata, index: usize) -> Result<Node, GamlError> {
        let name = match &x_data.label {
            None => index.to_string(),
            Some(label) => format!("Xdata {index}, {label}"),
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

    fn map_alt_x_data(&self, alt_x_data: &AltXdata, index: usize) -> Result<Node, GamlError> {
        let name = match &alt_x_data.label {
            None => index.to_string(),
            Some(label) => format!("altXdata {index}, {label}"),
        };

        let mut parameters = vec![];
        parameters.push(Parameter::from_str_str(
            "Units",
            alt_x_data.units.to_string(),
        ));
        if let Some(label) = &alt_x_data.label {
            parameters.push(Parameter::from_str_str("Label", label));
        }
        if let Some(linkid) = &alt_x_data.linkid {
            parameters.push(Parameter::from_str_str("Linkid", linkid));
        }
        if let Some(valueorder) = &alt_x_data.valueorder {
            parameters.push(Parameter::from_str_str(
                "Valueorder",
                valueorder.to_string(),
            ));
        }
        for link in &alt_x_data.links {
            parameters.push(Parameter::from_str_str("Link linkref", &link.linkref));
        }
        parameters.extend(map_gaml_parameters(&alt_x_data.parameters));
        parameters.extend(map_values_attributes("Values", &alt_x_data.values));

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

    fn map_y_data(&self, x_data: &Xdata, y_data: &Ydata, index: usize) -> Result<Node, GamlError> {
        let name = match &y_data.label {
            None => index.to_string(),
            Some(label) => format!("Ydata {index}, {label}"),
        };

        let mut parameters = vec![];
        parameters.push(Parameter::from_str_str("Units", y_data.units.to_string()));
        if let Some(label) = &y_data.label {
            parameters.push(Parameter::from_str_str("Label", label));
        }
        parameters.extend(map_gaml_parameters(&y_data.parameters));
        parameters.extend(map_values_attributes("Xdata values", &x_data.values));
        parameters.extend(map_values_attributes("Ydata values", &y_data.values));

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

    fn map_peaktable(&self, peaktable: &Peaktable, index: usize) -> Result<Node, GamlError> {
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

    fn map_basecurve(&self, basecurve: &Basecurve) -> Result<Node, GamlError> {
        let name = "Basecurve".to_owned();

        let mut parameters = vec![];
        // Values attributes
        for (i, values) in basecurve.base_x_data.iter().enumerate() {
            parameters.extend(map_values_attributes(
                &format!("BaseXdata values {i}"),
                values,
            ));
        }
        for (i, values) in basecurve.base_y_data.iter().enumerate() {
            parameters.extend(map_values_attributes(
                &format!("BaseYdata values {i}"),
                values,
            ));
        }

        let x_values = basecurve
            .base_x_data
            .iter()
            .map(|v| v.get_data())
            .collect::<Result<Vec<_>, GamlError>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let y_values = basecurve
            .base_y_data
            .iter()
            .map(|v| v.get_data())
            .collect::<Result<Vec<_>, GamlError>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let data = x_values
            .into_iter()
            .zip(y_values)
            .map(|(x, y)| PointXy::new(x, y))
            .collect();

        Ok(Node {
            name,
            parameters,
            data,
            metadata: vec![],
            table: None,
            child_node_names: vec![],
        })
    }
}
