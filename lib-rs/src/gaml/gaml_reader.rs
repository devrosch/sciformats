use super::{
    gaml_parser::{Basecurve, Coordinates, Experiment, Gaml, Peaktable, Trace, Xdata, Ydata},
    gaml_utils::{map_gaml_parameters, map_values_attributes, read_elem, TypeName},
    GamlError,
};
use crate::{
    api::{Column, Node, Parameter, PointXy, Reader, Table, Value},
    utils::convert_path_to_node_indices,
};
use std::{collections::HashMap, error::Error, path::Path, vec};

impl TypeName for Experiment {
    fn display_type_name() -> &'static str {
        "experiment"
    }
}

impl TypeName for Trace {
    fn display_type_name() -> &'static str {
        "trace"
    }
}

impl TypeName for Coordinates {
    fn display_type_name() -> &'static str {
        "coordinates"
    }
}

impl TypeName for Xdata {
    fn display_type_name() -> &'static str {
        "Xdata"
    }
}

impl TypeName for Ydata {
    fn display_type_name() -> &'static str {
        "Ydata"
    }
}

impl TypeName for Peaktable {
    fn display_type_name() -> &'static str {
        "peaktable"
    }
}

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
                let experiment = read_elem(&self.file.experiments, exp_idx)?;
                Ok(self.map_experiment(experiment, exp_idx)?)
            }
            [exp_idx, trace_idx] => {
                let experiment = read_elem(&self.file.experiments, exp_idx)?;
                let trace = read_elem(&experiment.traces, trace_idx)?;
                Ok(self.map_trace(trace, trace_idx)?)
            }
            [exp_idx, trace_idx, xy_data_idx] => {
                let experiment = read_elem(&self.file.experiments, exp_idx)?;
                let trace = read_elem(&experiment.traces, trace_idx)?;
                let coordinates = trace.coordinates.as_slice();

                let (x_data_idx, alt_x_data_idx, y_data_idx) =
                    Self::find_xy_indices(trace, xy_data_idx)?;
                let x_data = read_elem(&trace.x_data, x_data_idx)?;
                match alt_x_data_idx {
                    None => Ok(self.map_xy_data(
                        x_data,
                        (x_data_idx, y_data_idx, xy_data_idx),
                        coordinates,
                    )?),
                    Some(alt_x_idx) => Ok(self.map_alt_xy_data(
                        x_data,
                        (x_data_idx, alt_x_idx, y_data_idx, xy_data_idx),
                        coordinates,
                    )?),
                }
            }
            [exp_idx, trace_idx, xy_data_idx, peaktable_idx] => {
                let experiment = read_elem(&self.file.experiments, exp_idx)?;
                let trace = read_elem(&experiment.traces, trace_idx)?;
                let (x_data_idx, alt_x_data_idx, y_data_idx) =
                    Self::find_xy_indices(trace, xy_data_idx)?;
                if alt_x_data_idx.is_some() {
                    return Err(GamlError::new(&format!("Illegal node path: {}", path)).into());
                }
                let x_data = read_elem(&trace.x_data, x_data_idx)?;
                let y_data = read_elem(&x_data.y_data, y_data_idx)?;
                let peaktable = read_elem(&y_data.peaktables, peaktable_idx)?;
                Ok(self.map_peaktable(peaktable, peaktable_idx)?)
            }
            [exp_idx, trace_idx, xy_data_idx, peaktable_idx, basecurve_idx] => {
                let experiment = read_elem(&self.file.experiments, exp_idx)?;
                let trace = read_elem(&experiment.traces, trace_idx)?;
                let (x_data_idx, alt_x_data_idx, y_data_idx) =
                    Self::find_xy_indices(trace, xy_data_idx)?;
                if alt_x_data_idx.is_some() {
                    return Err(GamlError::new(&format!("Illegal node path: {}", path)).into());
                }
                let x_data = read_elem(&trace.x_data, x_data_idx)?;
                let y_data = read_elem(&x_data.y_data, y_data_idx)?;
                let peaktable = read_elem(&y_data.peaktables, peaktable_idx)?;
                let basecurve = Self::find_basecurve(peaktable, basecurve_idx)?;
                Ok(self.map_basecurve(basecurve)?)
            }
            _ => Err(GamlError::new(&format!("Illegal node path: {}", path)).into()),
        }
    }
}

impl GamlReader {
    fn find_xy_indices(
        trace: &Trace,
        xy_data_idx: usize,
    ) -> Result<(usize, Option<usize>, usize), GamlError> {
        let mut index = 0usize;
        for (x_index, x_data) in trace.x_data.iter().enumerate() {
            // first map Xdata - Ydata pairs
            for (y_index, _y_data) in x_data.y_data.iter().enumerate() {
                if index == xy_data_idx {
                    return Ok((x_index, None, y_index));
                }
                index += 1;
            }
            // then map altXdata - Ydata pairs
            for (alt_x_index, _alt_x_data) in x_data.alt_x_data.iter().enumerate() {
                for (y_index, _y_data) in x_data.y_data.iter().enumerate() {
                    if index == xy_data_idx {
                        return Ok((x_index, Some(alt_x_index), y_index));
                    }
                    index += 1;
                }
            }
        }
        Err(GamlError::new(&format!(
            "Illegal xy data index: {xy_data_idx}"
        )))
    }

    fn generate_xy_name(
        coordinates: &[Coordinates],
        x_index: usize,
        alt_x_index: Option<usize>,
        y_index: usize,
        overall_index: usize,
    ) -> Result<String, GamlError> {
        // Can this repeated reading of values be optimized away? No big perf issue though.
        let coordinate_values = coordinates
            .iter()
            .map(|co| co.values.get_data())
            .collect::<Result<Vec<_>, GamlError>>()?;
        let mut coordinate_details = Vec::<String>::new();
        for (i, coordinate) in coordinates.iter().enumerate() {
            let values = &coordinate_values[i];
            let mut text = String::new();
            if let Some(label) = &coordinate.label {
                text += &format!("{label}=");
            }
            if overall_index < values.len() {
                text += &values[overall_index].to_string();
            }
            text += &format!(" {}", coordinate.units);
            coordinate_details.push(text);
        }

        let coordinate_info = if !coordinate_details.is_empty() {
            format!(" ({})", coordinate_details.join(", "))
        } else {
            "".to_owned()
        };
        let name = match alt_x_index {
            None => format!("XYData {}, {}{}", x_index, y_index, coordinate_info),
            Some(alt_x_index) => format!(
                "AltXYData {}, {}, {}{}",
                x_index, alt_x_index, y_index, coordinate_info
            ),
        };

        Ok(name)
    }

    fn generate_xy_names(trace: &Trace) -> Result<Vec<String>, GamlError> {
        let coordinates = trace.coordinates.as_slice();
        let mut names = vec![];
        let mut overall_index = 0;
        for (x_index, x_data) in trace.x_data.iter().enumerate() {
            for (y_index, _y_data) in x_data.y_data.iter().enumerate() {
                let name =
                    Self::generate_xy_name(coordinates, x_index, None, y_index, overall_index)?;
                names.push(name);
                overall_index += 1;
            }
            for (alt_x_index, _alt_x_data) in x_data.alt_x_data.iter().enumerate() {
                for (y_index, _y_data) in x_data.y_data.iter().enumerate() {
                    let name = Self::generate_xy_name(
                        coordinates,
                        x_index,
                        Some(alt_x_index),
                        y_index,
                        overall_index,
                    )?;
                    names.push(name);
                    overall_index += 1;
                }
            }
        }
        Ok(names)
    }

    fn map_coordinate_parameters(coordinates: &[Coordinates]) -> Result<Vec<Parameter>, GamlError> {
        let mut parameters = vec![];
        for (i, coordinate) in coordinates.iter().enumerate() {
            parameters.push(Parameter::from_str_str(
                format!("Coordinate {i} units"),
                coordinate.units.to_string(),
            ));
            if let Some(label) = &coordinate.label {
                parameters.push(Parameter::from_str_str(
                    format!("Coordinate {i} label"),
                    label,
                ));
            }
            if let Some(linkid) = &coordinate.linkid {
                parameters.push(Parameter::from_str_str(
                    format!("Coordinate {i} linkid"),
                    linkid,
                ));
            }
            if let Some(valueorder) = &coordinate.valueorder {
                parameters.push(Parameter::from_str_str(
                    format!("Coordinate {i} valueorder"),
                    valueorder.to_string(),
                ));
            }

            for link in &coordinate.links {
                parameters.push(Parameter::from_str_str(
                    format!("Coordinate {i} link linkref"),
                    &link.linkref,
                ));
            }
            let mut coordinate_parameters = map_gaml_parameters(&coordinate.parameters);
            for param in &mut coordinate_parameters {
                param.key.insert_str(0, &format!("Coordinate {i} "));
            }
            parameters.extend(coordinate_parameters);
            parameters.extend(map_values_attributes(
                &format!("Coordinate {i} values"),
                &coordinate.values,
            ));
        }

        Ok(parameters)
    }

    fn find_basecurve(
        peaktable: &Peaktable,
        basecurve_idx: usize,
    ) -> Result<&Basecurve, GamlError> {
        for (i, peak) in peaktable.peaks.iter().enumerate() {
            if let Some(basecurve) = peak.baseline.as_ref().and_then(|bl| bl.basecurve.as_ref()) {
                if i == basecurve_idx {
                    return Ok(basecurve);
                }
            }
        }
        Err(GamlError::new(&format!(
            "Illegal basecurve index: {basecurve_idx}"
        )))?
    }

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

        let child_node_names = Self::generate_xy_names(trace)?;

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
        })
    }

    fn map_xy_data(
        &self,
        x_data: &Xdata,
        (x_index, y_index, overall_index): (usize, usize, usize),
        coordinates: &[Coordinates],
    ) -> Result<Node, GamlError> {
        let y_data = x_data.y_data.get(y_index).ok_or(GamlError::new(&format!(
            "No Ydata found for Xdata {} at index: {}",
            x_index, y_index
        )))?;

        let name = Self::generate_xy_name(coordinates, x_index, None, y_index, overall_index)?;

        let mut parameters = vec![];
        // attributes
        parameters.push(Parameter::from_str_str(
            "Xdata units",
            x_data.units.to_string(),
        ));
        if let Some(label) = &x_data.label {
            parameters.push(Parameter::from_str_str("Xdata label", label));
        }
        if let Some(linkid) = &x_data.linkid {
            parameters.push(Parameter::from_str_str("Xdata linkid", linkid));
        }
        if let Some(valueorder) = &x_data.valueorder {
            parameters.push(Parameter::from_str_str(
                "Xdata valueorder",
                valueorder.to_string(),
            ));
        }
        parameters.push(Parameter::from_str_str(
            "Ydata units",
            y_data.units.to_string(),
        ));
        if let Some(label) = &y_data.label {
            parameters.push(Parameter::from_str_str("Ydata label", label));
        }
        // coordinates
        parameters.extend(Self::map_coordinate_parameters(coordinates)?);
        // elements
        for link in &x_data.links {
            parameters.push(Parameter::from_str_str("Xdata link linkref", &link.linkref));
        }
        let mut x_parameters = map_gaml_parameters(&x_data.parameters);
        for param in &mut x_parameters {
            param.key.insert_str(0, "Xdata ");
        }
        parameters.extend(x_parameters);
        let mut y_parameters = map_gaml_parameters(&y_data.parameters);
        for param in &mut y_parameters {
            param.key.insert_str(0, "Ydata ");
        }
        parameters.extend(y_parameters);
        parameters.extend(map_values_attributes("Xdata values", &x_data.values));
        parameters.extend(map_values_attributes("Ydata values", &y_data.values));

        let x_values = x_data.values.get_data()?;
        let y_values = y_data.values.get_data()?;
        let data = x_values
            .into_iter()
            .zip(y_values)
            .map(|(x, y)| PointXy::new(x, y))
            .collect();

        // todo: add metadata

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

    fn map_alt_xy_data(
        &self,
        x_data: &Xdata,
        (x_index, alt_x_index, y_index, overall_index): (usize, usize, usize, usize),
        coordinates: &[Coordinates],
    ) -> Result<Node, GamlError> {
        let alt_x_data = x_data
            .alt_x_data
            .get(alt_x_index)
            .ok_or(GamlError::new(&format!(
                "No altXdata found for Xdata {} at index: {}",
                x_index, alt_x_index
            )))?;
        let y_data = x_data.y_data.get(y_index).ok_or(GamlError::new(&format!(
            "No Ydata found for Xdata {} at index: {}",
            x_index, y_index
        )))?;

        let name = Self::generate_xy_name(
            coordinates,
            x_index,
            Some(alt_x_index),
            y_index,
            overall_index,
        )?;

        let mut parameters = vec![];
        // attributes
        parameters.push(Parameter::from_str_str(
            "AltXdata units",
            alt_x_data.units.to_string(),
        ));
        if let Some(label) = &alt_x_data.label {
            parameters.push(Parameter::from_str_str("AltXdata label", label));
        }
        if let Some(linkid) = &alt_x_data.linkid {
            parameters.push(Parameter::from_str_str("AltXdata linkid", linkid));
        }
        if let Some(valueorder) = &alt_x_data.valueorder {
            parameters.push(Parameter::from_str_str(
                "AltXdata valueorder",
                valueorder.to_string(),
            ));
        }
        parameters.push(Parameter::from_str_str(
            "AltXdata units",
            y_data.units.to_string(),
        ));
        if let Some(label) = &y_data.label {
            parameters.push(Parameter::from_str_str("Ydata label", label));
        }
        // coordinates
        parameters.extend(Self::map_coordinate_parameters(coordinates)?);
        // elements
        for link in &alt_x_data.links {
            parameters.push(Parameter::from_str_str(
                "AltXdata link linkref",
                &link.linkref,
            ));
        }
        let mut x_parameters = map_gaml_parameters(&alt_x_data.parameters);
        for param in &mut x_parameters {
            param.key.insert_str(0, "AltXdata ");
        }
        parameters.extend(x_parameters);
        let mut y_parameters = map_gaml_parameters(&y_data.parameters);
        for param in &mut y_parameters {
            param.key.insert_str(0, "Ydata ");
        }
        parameters.extend(y_parameters);
        parameters.extend(map_values_attributes("AltXdata values", &alt_x_data.values));
        parameters.extend(map_values_attributes("Ydata values", &y_data.values));

        let x_values = alt_x_data.values.get_data()?;
        let y_values = y_data.values.get_data()?;
        let data = x_values
            .into_iter()
            .zip(y_values)
            .map(|(x, y)| PointXy::new(x, y))
            .collect();

        // todo: add metadata

        // do not map peaktables for altXdata

        Ok(Node {
            name,
            parameters,
            data,
            metadata: vec![],
            table: None,
            child_node_names: vec![],
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
