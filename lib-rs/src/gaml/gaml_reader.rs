use super::{
    gaml_parser::{
        AltXdata, Basecurve, Coordinates, Experiment, Gaml, Peak, Peaktable, Trace, Units, Values,
        Xdata, Ydata,
    },
    GamlError,
};
use crate::{
    api::{Column, Node, Parameter, PointXy, Reader, Table, Value},
    utils::convert_path_to_node_indices,
};
use std::{collections::HashMap, error::Error, path::Path, vec};

macro_rules! generate_map_xy_parameters_fn {
    ($xy_data_type:ty, $xy_type_name:literal, $fn_name:ident) => {
        fn $fn_name(
            x_data: &$xy_data_type,
            y_data: &Ydata,
            coordinates: &[Coordinates],
        ) -> Vec<Parameter> {
            let mut parameters = vec![];
            // attributes
            parameters.push(Parameter::from_str_str(
                &format!("{} {}", $xy_type_name, "units"),
                x_data.units.to_string(),
            ));
            if let Some(label) = &x_data.label {
                parameters.push(Parameter::from_str_str(
                    &format!("{} {}", $xy_type_name, "label"),
                    label,
                ));
            }
            if let Some(linkid) = &x_data.linkid {
                parameters.push(Parameter::from_str_str(
                    &format!("{} {}", $xy_type_name, "linkid"),
                    linkid,
                ));
            }
            if let Some(valueorder) = &x_data.valueorder {
                parameters.push(Parameter::from_str_str(
                    &format!("{} {}", $xy_type_name, "valueorder"),
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
            parameters.extend(map_coordinates_attributes_to_parameters(coordinates));
            // elements
            for link in &x_data.links {
                parameters.push(Parameter::from_str_str(
                    &format!("{} {}", $xy_type_name, "linkref"),
                    &link.linkref,
                ));
            }
            parameters.extend(map_coordinates_linkrefs_to_parameters(coordinates));
            parameters.extend(map_gaml_parameters_with_prefix(
                &format!("{} ", $xy_type_name),
                &x_data.parameters,
            ));
            parameters.extend(map_gaml_parameters_with_prefix(
                "Ydata ",
                &y_data.parameters,
            ));
            parameters.extend(map_coordinates_parameters_to_parameters(coordinates));
            parameters.extend(map_values_attributes(
                &format!("{} {}", $xy_type_name, "values"),
                &x_data.values,
            ));
            parameters.extend(map_values_attributes("Ydata values", &y_data.values));
            parameters.extend(map_coordinates_values_attributes_to_parameters(coordinates));

            parameters
        }
    };
}

pub struct GamlReader {
    path: String,
    file: Gaml,
}

impl Reader for GamlReader {
    fn read(&self, path: &str) -> Result<Node, Box<dyn Error>> {
        let path_indices = convert_path_to_node_indices(path)?;
        match &path_indices[..] {
            [] => Ok(Self::map_root(&self.path, &self.file)?), // "", "/"
            [exp_idx, tail @ ..] => {
                let experiment =
                    read_item_at_index(&self.file.experiments, *exp_idx, "experiment")?;
                if tail.is_empty() {
                    return Ok(Self::map_experiment(experiment, *exp_idx)?);
                }

                let (trace_idx, tail) = tail.split_first().unwrap();
                let trace = read_item_at_index(&experiment.traces, *trace_idx, "trace")?;
                if tail.is_empty() {
                    return Ok(Self::map_trace(trace, *trace_idx)?);
                }

                let (xy_data_idx, tail) = tail.split_first().unwrap();
                let (x_data_idx, alt_x_data_idx, y_data_idx) =
                    find_xy_indices(trace, *xy_data_idx)?;
                let x_data = read_item_at_index(&trace.x_data, x_data_idx, "Xdata")?;
                if tail.is_empty() {
                    let coordinates = trace.coordinates.as_slice();
                    match alt_x_data_idx {
                        None => {
                            return Ok(Self::map_xy_data(
                                x_data,
                                (x_data_idx, y_data_idx),
                                coordinates,
                            )?)
                        }
                        Some(alt_x_idx) => {
                            return Ok(Self::map_alt_xy_data(
                                x_data,
                                (x_data_idx, alt_x_idx, y_data_idx),
                                coordinates,
                            )?)
                        }
                    }
                }
                if alt_x_data_idx.is_some() {
                    // no children for altXdata
                    return Err(GamlError::new(&format!("Illegal node path: {}", path)).into());
                }

                let (peaktable_idx, tail) = tail.split_first().unwrap();
                let y_data = read_item_at_index(&x_data.y_data, y_data_idx, "Ydata")?;
                let peaktable =
                    read_item_at_index(&y_data.peaktables, *peaktable_idx, "peaktable")?;
                if tail.is_empty() {
                    return Ok(Self::map_peaktable(peaktable, *peaktable_idx)?);
                }

                let (basecurve_idx, tail) = tail.split_first().unwrap();
                let (basecurve, peak, peak_index) = find_basecurve(peaktable, *basecurve_idx)?;
                if tail.is_empty() {
                    return Ok(Self::map_basecurve(basecurve, peak_index, peak.number)?);
                }

                Err(GamlError::new(&format!("Illegal node path: {}", path)).into())
            }
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

    fn map_root(path: &str, gaml: &Gaml) -> Result<Node, GamlError> {
        let path = Path::new(path);
        let name = path
            .file_name()
            .map_or("", |f| f.to_str().unwrap_or(""))
            .to_owned();
        let parameters = Self::map_root_parameters(gaml);
        let child_node_names =
            generate_child_node_names(&gaml.experiments, &Self::generate_experiment_name);

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
        })
    }

    fn map_root_parameters(gaml: &Gaml) -> Vec<Parameter> {
        let mut parameters = vec![];
        parameters.push(Parameter::from_str_str("Version", gaml.version.to_string()));
        if let Some(name) = &gaml.name {
            let param = Parameter::from_str_str("Name", name);
            parameters.push(param);
        }
        if let Some(integrity) = &gaml.integrity {
            let param = Parameter::from_str_str(
                format!("Integrity (algorithm={})", integrity.algorithm),
                &integrity.value,
            );
            parameters.push(param);
        }
        parameters.extend(map_gaml_parameters(&gaml.parameters));

        parameters
    }

    fn map_experiment(experiment: &Experiment, index: usize) -> Result<Node, GamlError> {
        let name = Self::generate_experiment_name(experiment, index);
        let parameters = Self::map_experiment_parameters(experiment);
        let child_node_names =
            generate_child_node_names(&experiment.traces, &Self::generate_trace_name);

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
        })
    }

    fn generate_experiment_name(experiment: &Experiment, index: usize) -> String {
        match &experiment.name {
            None => format!("Experiment {index}"),
            Some(experiment_name) => format!("Experiment {index}, {experiment_name}"),
        }
    }

    fn map_experiment_parameters(experiment: &Experiment) -> Vec<Parameter> {
        let mut parameters = vec![];
        if let Some(name) = &experiment.name {
            parameters.push(Parameter::from_str_str("Name", name));
        }
        if let Some(date) = &experiment.collectdate {
            parameters.push(Parameter::from_str_str("Collectdate", date));
        }
        parameters.extend(map_gaml_parameters(&experiment.parameters));

        parameters
    }

    fn map_trace(trace: &Trace, index: usize) -> Result<Node, GamlError> {
        let name = Self::generate_trace_name(trace, index);
        let parameters = Self::map_trace_parameters(trace);
        let child_node_names = generate_xy_names(trace)?;

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
        })
    }

    fn generate_trace_name(trace: &Trace, index: usize) -> String {
        match &trace.name {
            None => format!("Trace {index}"),
            Some(trace_name) => format!("Trace {index}, {trace_name}"),
        }
    }

    fn map_trace_parameters(trace: &Trace) -> Vec<Parameter> {
        let mut parameters = vec![];
        if let Some(name) = &trace.name {
            parameters.push(Parameter::from_str_str("Name", name));
        }
        parameters.push(Parameter::from_str_str(
            "Technique",
            trace.technique.to_string(),
        ));
        parameters.extend(map_gaml_parameters(&trace.parameters));

        parameters
    }

    fn map_xy_data(
        x_data: &Xdata,
        (x_index, y_index): (usize, usize),
        coordinates: &[Coordinates],
    ) -> Result<Node, GamlError> {
        let y_data = x_data.y_data.get(y_index).ok_or(GamlError::new(&format!(
            "No Ydata found for Xdata {} at index: {}",
            x_index, y_index
        )))?;

        let name = generate_xy_name(coordinates, x_index, None, y_index)?;

        let parameters = Self::map_xy_data_parameters(x_data, y_data, coordinates);

        let x_values = x_data.values.get_data()?;
        let y_values = y_data.values.get_data()?;
        let data = x_values
            .into_iter()
            .zip(y_values)
            .map(|(x, y)| PointXy::new(x, y))
            .collect();

        let metadata = generate_xy_plot_hints(
            x_data.label.as_deref(),
            &x_data.units,
            y_data.label.as_deref(),
            &y_data.units,
        );

        let child_node_names =
            generate_child_node_names(&y_data.peaktables, &Self::generate_peaktable_name);

        Ok(Node {
            name,
            parameters,
            data,
            metadata,
            table: None,
            child_node_names,
        })
    }

    generate_map_xy_parameters_fn!(Xdata, "Xdata", map_xy_data_parameters);

    fn map_alt_xy_data(
        x_data: &Xdata,
        (x_index, alt_x_index, y_index): (usize, usize, usize),
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

        let name = generate_xy_name(coordinates, x_index, Some(alt_x_index), y_index)?;

        let parameters = Self::map_alt_xy_data_parameters(alt_x_data, y_data, coordinates);

        let x_values = alt_x_data.values.get_data()?;
        let y_values = y_data.values.get_data()?;
        let data = x_values
            .into_iter()
            .zip(y_values)
            .map(|(x, y)| PointXy::new(x, y))
            .collect();

        let metadata = generate_xy_plot_hints(
            alt_x_data.label.as_deref(),
            &alt_x_data.units,
            y_data.label.as_deref(),
            &y_data.units,
        );

        // do not map peaktables for altXdata

        Ok(Node {
            name,
            parameters,
            data,
            metadata,
            table: None,
            child_node_names: vec![],
        })
    }

    generate_map_xy_parameters_fn!(AltXdata, "AltXdata", map_alt_xy_data_parameters);

    fn map_peaktable(peaktable: &Peaktable, index: usize) -> Result<Node, GamlError> {
        let name = Self::generate_peaktable_name(peaktable, index);
        let parameters = Self::map_peaktable_parameters(peaktable);
        // map peaks as table
        let table = Self::map_peaktable_table(peaktable);

        let mut child_node_names = vec![];
        for (i, peak) in peaktable.peaks.iter().enumerate() {
            if let Some(_basecurve) = peak.baseline.as_ref().and_then(|bl| bl.basecurve.as_ref()) {
                child_node_names.push(Self::generate_basecurve_name(i, peak.number));
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

    fn generate_peaktable_name(peaktable: &Peaktable, index: usize) -> String {
        match &peaktable.name {
            None => format!("Peaktable {index}"),
            Some(name) => format!("Peaktable {index}, {}", name),
        }
    }

    fn map_peaktable_parameters(peaktable: &Peaktable) -> Vec<Parameter> {
        let mut parameters = vec![];
        // peaktable attributes and parameters
        if let Some(name) = &peaktable.name {
            parameters.push(Parameter::from_str_str("Peaktable name", name));
        }
        let peaktable_params = map_gaml_parameters_with_prefix("Peaktable ", &peaktable.parameters);
        parameters.extend(peaktable_params);
        // peak parameters
        for (i, peak) in peaktable.peaks.iter().enumerate() {
            let peak_params = map_gaml_parameters_with_prefix(
                &format!("Peak {} number {} ", i, peak.number),
                &peak.parameters,
            );
            parameters.extend(peak_params);
            // baseline parameters
            if let Some(baseline) = &peak.baseline {
                let baseline_params = map_gaml_parameters_with_prefix(
                    &format!("Peak {} number {} baseline ", i, peak.number),
                    &baseline.parameters,
                );
                parameters.extend(baseline_params);
            }
        }

        parameters
    }

    fn map_peaktable_table(peaktable: &Peaktable) -> Table {
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

        table
    }

    fn map_basecurve(
        basecurve: &Basecurve,
        peak_index: usize,
        peak_number: u64,
    ) -> Result<Node, GamlError> {
        let name = Self::generate_basecurve_name(peak_index, peak_number);

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

    fn generate_basecurve_name(peak_index: usize, peak_number: u64) -> String {
        format!("Basecurve Peak {}, number {}", peak_index, peak_number)
    }
}

fn find_basecurve(
    peaktable: &Peaktable,
    basecurve_idx: usize,
) -> Result<(&Basecurve, &Peak, usize), GamlError> {
    for (i, peak) in peaktable.peaks.iter().enumerate() {
        if let Some(basecurve) = peak.baseline.as_ref().and_then(|bl| bl.basecurve.as_ref()) {
            if i == basecurve_idx {
                return Ok((basecurve, peak, i));
            }
        }
    }
    Err(GamlError::new(&format!(
        "Illegal basecurve index: {basecurve_idx}"
    )))?
}

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
        if x_index < values.len() {
            text += &values[x_index].to_string();
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
    for (x_index, x_data) in trace.x_data.iter().enumerate() {
        for (y_index, _y_data) in x_data.y_data.iter().enumerate() {
            let name = generate_xy_name(coordinates, x_index, None, y_index)?;
            names.push(name);
        }
        for (alt_x_index, _alt_x_data) in x_data.alt_x_data.iter().enumerate() {
            for (y_index, _y_data) in x_data.y_data.iter().enumerate() {
                let name = generate_xy_name(coordinates, x_index, Some(alt_x_index), y_index)?;
                names.push(name);
            }
        }
    }
    Ok(names)
}

fn map_coordinates_attributes_to_parameters(coordinates: &[Coordinates]) -> Vec<Parameter> {
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
    }

    parameters
}

fn map_coordinates_linkrefs_to_parameters(coordinates: &[Coordinates]) -> Vec<Parameter> {
    let mut parameters = vec![];
    for (i, coordinate) in coordinates.iter().enumerate() {
        for link in &coordinate.links {
            parameters.push(Parameter::from_str_str(
                format!("Coordinate {i} link linkref"),
                &link.linkref,
            ));
        }
    }

    parameters
}

fn map_coordinates_parameters_to_parameters(coordinates: &[Coordinates]) -> Vec<Parameter> {
    let mut parameters = vec![];
    for (i, coordinate) in coordinates.iter().enumerate() {
        let coordinate_parameters =
            map_gaml_parameters_with_prefix(&format!("Coordinate {i} "), &coordinate.parameters);
        parameters.extend(coordinate_parameters);
    }

    parameters
}

fn map_coordinates_values_attributes_to_parameters(coordinates: &[Coordinates]) -> Vec<Parameter> {
    let mut parameters = vec![];
    for (i, coordinate) in coordinates.iter().enumerate() {
        parameters.extend(map_values_attributes(
            &format!("Coordinate {i} values"),
            &coordinate.values,
        ));
    }

    parameters
}

fn map_values_attributes(prefix: &str, values: &Values) -> Vec<Parameter> {
    let mut parameters = vec![];
    // Values attributes
    let format = Parameter::from_str_str(format!("{prefix} format"), values.format.to_string());
    parameters.push(format);
    let byteorder =
        Parameter::from_str_str(format!("{prefix} byteorder"), values.byteorder.to_string());
    parameters.push(byteorder);
    if let Some(numvalues) = values.numvalues {
        let numvalues = Parameter::from_str_u64(format!("{prefix} numvalues"), numvalues);
        parameters.push(numvalues);
    }

    parameters
}

fn map_gaml_parameters(raw_params: &[super::gaml_parser::Parameter]) -> Vec<crate::api::Parameter> {
    let mut parameters = Vec::with_capacity(raw_params.len());
    for raw_param in raw_params {
        let key = if [&raw_param.group, &raw_param.label, &raw_param.alias]
            .iter()
            .any(|s| s.is_some())
        {
            let mut attributes = vec![];
            if let Some(group) = &raw_param.group {
                attributes.push(format!("group={group}"));
            }
            if let Some(label) = &raw_param.label {
                attributes.push(format!("label={label}"));
            }
            if let Some(alias) = &raw_param.alias {
                attributes.push(format!("alias={alias}"));
            }
            format!("{} ({})", raw_param.name, attributes.join(", "))
        } else {
            raw_param.name.to_string()
        };
        let param = crate::api::Parameter::from_str_str(
            key,
            raw_param.value.as_deref().unwrap_or_default(),
        );
        parameters.push(param);
    }

    parameters
}

fn map_gaml_parameters_with_prefix(
    prefix: &str,
    raw_params: &[super::gaml_parser::Parameter],
) -> Vec<crate::api::Parameter> {
    let mut params = map_gaml_parameters(raw_params);
    for param in &mut params {
        param.key.insert_str(0, prefix);
    }
    params
}

fn generate_child_node_names<T>(
    slice: &[T],
    name_generator: &dyn Fn(&T, usize) -> String,
) -> Vec<String> {
    slice
        .iter()
        .enumerate()
        .map(|(i, item)| name_generator(item, i))
        .collect()
}

fn read_item_at_index<'a, T>(
    slice: &'a [T],
    index: usize,
    context: &str,
) -> Result<&'a T, GamlError> {
    slice.get(index).ok_or(GamlError::new(&format!(
        "Illegal {} index: {}",
        context, index
    )))
}

fn generate_xy_plot_hints(
    x_label: Option<&str>,
    x_units: &Units,
    y_label: Option<&str>,
    y_units: &Units,
) -> Vec<(String, String)> {
    let mut metadata = Vec::<(String, String)>::new();
    if let Some(label) = x_label {
        metadata.push(("x.label".to_owned(), label.to_owned()));
    };
    if x_units != &Units::Unknown {
        metadata.push(("x.unit".to_owned(), x_units.to_string()));
    }
    if let Some(label) = y_label {
        metadata.push(("y.label".to_owned(), label.to_owned()));
    };
    if y_units != &Units::Unknown {
        metadata.push(("y.unit".to_owned(), y_units.to_string()));
    }
    if x_units == &Units::Masschargeratio {
        // possibly use more refined heuristic in the future
        metadata.push(("plot.style".to_owned(), "sticks".to_owned()));
    }
    if x_units == &Units::Wavenumber || x_units == &Units::Ramanshift {
        metadata.push(("x.reverse".to_owned(), "true".to_owned()));
    }

    metadata
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gaml::gaml_parser::{
        AltXdata, Baseline, Byteorder, Format, Integrity, Parameter as RawParameter, Technique,
        Valueorder, Values, Version, Ydata,
    };

    fn create_values_f32(data: &[f32]) -> Values {
        let bytes: Vec<u8> = data.iter().map(|v| v.to_le_bytes()).flatten().collect();
        Values::create_values_with(bytes.as_slice(), Format::Float32, Byteorder::Intel)
    }

    fn create_values_f64(data: &[f64]) -> Values {
        let bytes: Vec<u8> = data.iter().map(|v| v.to_le_bytes()).flatten().collect();
        Values::create_values_with(bytes.as_slice(), Format::Float64, Byteorder::Intel)
    }

    #[test]
    fn maps_gaml_root() {
        let path = "gaml_file.gaml";
        let gaml = Gaml {
            version: Version::Version1_20,
            name: Some("GAML name".into()),
            integrity: Some(Integrity {
                algorithm: "SHA1".into(),
                value: "03cfd743661f07975fa2f1220c5194cbaff48451".into(),
            }),
            parameters: vec![RawParameter {
                group: Some("param 0 group".into()),
                name: "param 0 name".into(),
                label: Some("param 0 label".into()),
                alias: Some("param 0 alias".into()),
                value: Some("param 0 value".into()),
            }],
            experiments: vec![],
        };
        let reader = GamlReader::new(path, gaml);

        let root_node = reader.read("/").unwrap();
        assert_eq!(
            Node {
                name: "gaml_file.gaml".into(),
                parameters: vec![
                    Parameter::from_str_str("Version", "1.20"),
                    Parameter::from_str_str("Name", "GAML name"),
                    Parameter::from_str_str(
                        "Integrity (algorithm=SHA1)",
                        "03cfd743661f07975fa2f1220c5194cbaff48451"
                    ),
                    Parameter::from_str_str(
                        "param 0 name (group=param 0 group, label=param 0 label, alias=param 0 alias)",
                        "param 0 value"
                    ),
                ],
                data: vec![],
                metadata: vec![],
                table: None,
                child_node_names: vec![],
            },
            root_node
        );
    }

    #[test]
    fn maps_gaml_root_minimal() {
        let path = "gaml_file.gaml";
        let gaml = Gaml {
            version: Version::Version1_20,
            name: None,
            integrity: None,
            parameters: vec![],
            experiments: vec![],
        };
        let reader = GamlReader::new(path, gaml);

        let root_node = reader.read("/").unwrap();
        assert_eq!(
            Node {
                name: "gaml_file.gaml".into(),
                parameters: vec![Parameter::from_str_str("Version", "1.20"),],
                data: vec![],
                metadata: vec![],
                table: None,
                child_node_names: vec![],
            },
            root_node
        );
    }

    #[test]
    fn maps_gaml_experiment() {
        let path = "gaml_file.gaml";
        let gaml = Gaml {
            version: Version::Version1_20,
            name: None,
            integrity: None,
            parameters: vec![],
            experiments: vec![Experiment {
                name: Some("experiment 0 name".into()),
                collectdate: Some("2024-03-27T06:46:00Z".into()),
                parameters: vec![RawParameter {
                    group: None,
                    name: "param 0 name".into(),
                    label: None,
                    alias: None,
                    value: Some("param 0 value".into()),
                }],
                traces: vec![],
            }],
        };
        let reader = GamlReader::new(path, gaml);

        let root_node = reader.read("/").unwrap();
        assert_eq!(1, root_node.child_node_names.len());
        assert_eq!(
            "Experiment 0, experiment 0 name",
            root_node.child_node_names[0]
        );

        let exp_node = reader.read("/0").unwrap();
        assert_eq!(
            Node {
                name: "Experiment 0, experiment 0 name".into(),
                parameters: vec![
                    Parameter::from_str_str("Name", "experiment 0 name"),
                    Parameter::from_str_str("Collectdate", "2024-03-27T06:46:00Z"),
                    Parameter::from_str_str("param 0 name", "param 0 value"),
                ],
                data: vec![],
                metadata: vec![],
                table: None,
                child_node_names: vec![]
            },
            exp_node
        );
    }

    #[test]
    fn maps_gaml_trace() {
        let path = "gaml_file.gaml";
        let gaml = Gaml {
            version: Version::Version1_20,
            name: None,
            integrity: None,
            parameters: vec![],
            experiments: vec![Experiment {
                name: None,
                collectdate: None,
                parameters: vec![],
                traces: vec![Trace {
                    name: Some("trace 0 name".into()),
                    technique: Technique::Ir,
                    parameters: vec![RawParameter {
                        group: None,
                        name: "param 0 name".into(),
                        label: None,
                        alias: None,
                        value: Some("param 0 value".into()),
                    }],
                    coordinates: vec![],
                    x_data: vec![],
                }],
            }],
        };
        let reader = GamlReader::new(path, gaml);

        let exp_node = reader.read("/0").unwrap();
        assert_eq!(1, exp_node.child_node_names.len());
        assert_eq!("Trace 0, trace 0 name", exp_node.child_node_names[0]);

        let trace_node = reader.read("/0/0").unwrap();

        assert_eq!(
            Node {
                name: "Trace 0, trace 0 name".into(),
                parameters: vec![
                    Parameter::from_str_str("Name", "trace 0 name"),
                    Parameter::from_str_str("Technique", "IR"),
                    Parameter::from_str_str("param 0 name", "param 0 value"),
                ],
                data: vec![],
                metadata: vec![],
                table: None,
                child_node_names: vec![]
            },
            trace_node
        );
    }

    #[test]
    fn maps_gaml_xydata() {
        let path = "gaml_file.gaml";
        let gaml = Gaml {
            version: Version::Version1_20,
            name: None,
            integrity: None,
            parameters: vec![],
            experiments: vec![Experiment {
                name: None,
                collectdate: None,
                parameters: vec![],
                traces: vec![Trace {
                    name: None,
                    technique: Technique::Unknown,
                    parameters: vec![],
                    coordinates: vec![],
                    x_data: vec![Xdata {
                        units: Units::Nanometers,
                        label: Some("xdata label".into()),
                        linkid: Some("xdata linkid".into()),
                        valueorder: Some(Valueorder::Unspecified),
                        links: vec![],
                        parameters: vec![RawParameter {
                            group: None,
                            name: "param 0 name".into(),
                            label: None,
                            alias: None,
                            value: Some("param 0 value".into()),
                        }],
                        values: create_values_f32(&[1f32, 2f32, 3f32]),
                        alt_x_data: vec![],
                        y_data: vec![Ydata {
                            units: Units::Absorbance,
                            label: Some("ydata label".into()),
                            parameters: vec![RawParameter {
                                group: None,
                                name: "param 0 name".into(),
                                label: None,
                                alias: None,
                                value: Some("param 0 value".into()),
                            }],
                            values: create_values_f32(&[10f32, 20f32, 30f32]),
                            peaktables: vec![],
                        }],
                    }],
                }],
            }],
        };
        let reader = GamlReader::new(path, gaml);

        let trace_node = reader.read("/0/0").unwrap();
        assert_eq!(1, trace_node.child_node_names.len());

        let xydata_node = reader.read("/0/0/0").unwrap();
        assert_eq!("XYData 0, 0", &xydata_node.name);
        assert_eq!(
            &vec![
                Parameter::from_str_str("Xdata units", "NANOMETERS"),
                Parameter::from_str_str("Xdata label", "xdata label"),
                Parameter::from_str_str("Xdata linkid", "xdata linkid"),
                Parameter::from_str_str("Xdata valueorder", "UNSPECIFIED"),
                Parameter::from_str_str("Ydata units", "ABSORBANCE"),
                Parameter::from_str_str("Ydata label", "ydata label"),
                Parameter::from_str_str("Xdata param 0 name", "param 0 value"),
                Parameter::from_str_str("Ydata param 0 name", "param 0 value"),
                Parameter::from_str_str("Xdata values format", "FLOAT32"),
                Parameter::from_str_str("Xdata values byteorder", "INTEL"),
                Parameter::from_str_u64("Xdata values numvalues", 3),
                Parameter::from_str_str("Ydata values format", "FLOAT32"),
                Parameter::from_str_str("Ydata values byteorder", "INTEL"),
                Parameter::from_str_u64("Ydata values numvalues", 3),
            ],
            &xydata_node.parameters
        );
        assert_eq!(
            &vec![
                PointXy::new(1.0f32 as f64, 10.0f32 as f64),
                PointXy::new(2.0f32 as f64, 20.0f32 as f64),
                PointXy::new(3.0f32 as f64, 30.0f32 as f64)
            ],
            &xydata_node.data,
        );
        assert_eq!(
            &vec![
                ("x.label".into(), "xdata label".into()),
                ("x.unit".into(), "NANOMETERS".into()),
                ("y.label".into(), "ydata label".into()),
                ("y.unit".into(), "ABSORBANCE".into()),
            ],
            &xydata_node.metadata
        );
        assert_eq!(&None, &xydata_node.table);
        assert!(&xydata_node.child_node_names.is_empty());
    }

    #[test]
    fn maps_gaml_xydata_with_coordinates_and_altxdata() {
        let path = "gaml_file.gaml";
        let gaml = Gaml {
            version: Version::Version1_20,
            name: None,
            integrity: None,
            parameters: vec![],
            experiments: vec![Experiment {
                name: None,
                collectdate: None,
                parameters: vec![],
                traces: vec![Trace {
                    name: None,
                    technique: Technique::Unknown,
                    parameters: vec![],
                    coordinates: vec![Coordinates {
                        units: Units::Seconds,
                        label: Some("coordinates label".into()),
                        linkid: Some("coordinates".into()),
                        valueorder: Some(Valueorder::Unspecified),
                        links: vec![],
                        parameters: vec![RawParameter {
                            group: None,
                            name: "param 0 name".into(),
                            label: None,
                            alias: None,
                            value: Some("param 0 value".into()),
                        }],
                        values: create_values_f64(&[100.0]),
                    }],
                    x_data: vec![Xdata {
                        units: Units::Nanometers,
                        label: Some("xdata label".into()),
                        linkid: Some("xdata linkid".into()),
                        valueorder: Some(Valueorder::Unspecified),
                        links: vec![],
                        parameters: vec![RawParameter {
                            group: None,
                            name: "param 0 name".into(),
                            label: None,
                            alias: None,
                            value: Some("param 0 value".into()),
                        }],
                        values: create_values_f64(&[1.0, 2.0, 3.0]),
                        alt_x_data: vec![AltXdata {
                            units: Units::Meters,
                            label: Some("altxdata label".into()),
                            linkid: Some("altxdata linkid".into()),
                            valueorder: Some(Valueorder::Unspecified),
                            links: vec![],
                            parameters: vec![RawParameter {
                                group: None,
                                name: "param 0 name".into(),
                                label: None,
                                alias: None,
                                value: Some("param 0 value".into()),
                            }],
                            values: create_values_f64(&[1.1, 2.1, 3.1]),
                        }],
                        y_data: vec![Ydata {
                            units: Units::Absorbance,
                            label: Some("ydata label".into()),
                            parameters: vec![RawParameter {
                                group: None,
                                name: "param 0 name".into(),
                                label: None,
                                alias: None,
                                value: Some("param 0 value".into()),
                            }],
                            values: create_values_f64(&[10.0, 20.0, 30.0]),
                            peaktables: vec![],
                        }],
                    }],
                }],
            }],
        };
        let reader = GamlReader::new(path, gaml);

        let trace_node = reader.read("/0/0").unwrap();
        assert_eq!(2, trace_node.child_node_names.len());

        let xydata_node_0 = reader.read("/0/0/0").unwrap();
        assert_eq!(
            "XYData 0, 0 (coordinates label=100 SECONDS)",
            &xydata_node_0.name
        );
        assert_eq!(
            &vec![
                // attributes
                Parameter::from_str_str("Xdata units", "NANOMETERS"),
                Parameter::from_str_str("Xdata label", "xdata label"),
                Parameter::from_str_str("Xdata linkid", "xdata linkid"),
                Parameter::from_str_str("Xdata valueorder", "UNSPECIFIED"),
                Parameter::from_str_str("Ydata units", "ABSORBANCE"),
                Parameter::from_str_str("Ydata label", "ydata label"),
                Parameter::from_str_str("Coordinate 0 units", "SECONDS"),
                Parameter::from_str_str("Coordinate 0 label", "coordinates label"),
                Parameter::from_str_str("Coordinate 0 linkid", "coordinates"),
                Parameter::from_str_str("Coordinate 0 valueorder", "UNSPECIFIED"),
                // parameters
                Parameter::from_str_str("Xdata param 0 name", "param 0 value"),
                Parameter::from_str_str("Ydata param 0 name", "param 0 value"),
                Parameter::from_str_str("Coordinate 0 param 0 name", "param 0 value"),
                // values attributes
                Parameter::from_str_str("Xdata values format", "FLOAT64"),
                Parameter::from_str_str("Xdata values byteorder", "INTEL"),
                Parameter::from_str_u64("Xdata values numvalues", 3),
                Parameter::from_str_str("Ydata values format", "FLOAT64"),
                Parameter::from_str_str("Ydata values byteorder", "INTEL"),
                Parameter::from_str_u64("Ydata values numvalues", 3),
                Parameter::from_str_str("Coordinate 0 values format", "FLOAT64"),
                Parameter::from_str_str("Coordinate 0 values byteorder", "INTEL"),
                Parameter::from_str_u64("Coordinate 0 values numvalues", 1),
            ],
            &xydata_node_0.parameters
        );
        assert_eq!(
            &vec![
                PointXy::new(1.0, 10.0),
                PointXy::new(2.0, 20.0),
                PointXy::new(3.0, 30.0)
            ],
            &xydata_node_0.data,
        );
        assert_eq!(
            &vec![
                ("x.label".into(), "xdata label".into()),
                ("x.unit".into(), "NANOMETERS".into()),
                ("y.label".into(), "ydata label".into()),
                ("y.unit".into(), "ABSORBANCE".into()),
            ],
            &xydata_node_0.metadata
        );
        assert_eq!(&None, &xydata_node_0.table);
        assert!(&xydata_node_0.child_node_names.is_empty());

        let xydata_node_1 = reader.read("/0/0/1").unwrap();
        assert_eq!(
            "AltXYData 0, 0, 0 (coordinates label=100 SECONDS)",
            &xydata_node_1.name
        );
        assert_eq!(
            &vec![
                Parameter::from_str_str("AltXdata units", "METERS"),
                Parameter::from_str_str("AltXdata label", "altxdata label"),
                Parameter::from_str_str("AltXdata linkid", "altxdata linkid"),
                Parameter::from_str_str("AltXdata valueorder", "UNSPECIFIED"),
                Parameter::from_str_str("Ydata units", "ABSORBANCE"),
                Parameter::from_str_str("Ydata label", "ydata label"),
                Parameter::from_str_str("Coordinate 0 units", "SECONDS"),
                Parameter::from_str_str("Coordinate 0 label", "coordinates label"),
                Parameter::from_str_str("Coordinate 0 linkid", "coordinates"),
                Parameter::from_str_str("Coordinate 0 valueorder", "UNSPECIFIED"),
                Parameter::from_str_str("AltXdata param 0 name", "param 0 value"),
                Parameter::from_str_str("Ydata param 0 name", "param 0 value"),
                Parameter::from_str_str("Coordinate 0 param 0 name", "param 0 value"),
                Parameter::from_str_str("AltXdata values format", "FLOAT64"),
                Parameter::from_str_str("AltXdata values byteorder", "INTEL"),
                Parameter::from_str_u64("AltXdata values numvalues", 3),
                Parameter::from_str_str("Ydata values format", "FLOAT64"),
                Parameter::from_str_str("Ydata values byteorder", "INTEL"),
                Parameter::from_str_u64("Ydata values numvalues", 3),
                Parameter::from_str_str("Coordinate 0 values format", "FLOAT64"),
                Parameter::from_str_str("Coordinate 0 values byteorder", "INTEL"),
                Parameter::from_str_u64("Coordinate 0 values numvalues", 1),
            ],
            &xydata_node_1.parameters
        );
        assert_eq!(
            &vec![
                PointXy::new(1.1, 10.0),
                PointXy::new(2.1, 20.0),
                PointXy::new(3.1, 30.0),
            ],
            &xydata_node_1.data,
        );
        assert_eq!(
            &vec![
                ("x.label".into(), "altxdata label".into()),
                ("x.unit".into(), "METERS".into()),
                ("y.label".into(), "ydata label".into()),
                ("y.unit".into(), "ABSORBANCE".into()),
            ],
            &xydata_node_1.metadata
        );
        assert_eq!(&None, &xydata_node_1.table);
        assert!(&xydata_node_1.child_node_names.is_empty());
    }

    #[test]
    fn maps_gaml_peaktable() {
        let path = "gaml_file.gaml";
        let gaml = Gaml {
            version: Version::Version1_20,
            name: None,
            integrity: None,
            parameters: vec![],
            experiments: vec![Experiment {
                name: None,
                collectdate: None,
                parameters: vec![],
                traces: vec![Trace {
                    name: None,
                    technique: Technique::Unknown,
                    parameters: vec![],
                    coordinates: vec![],
                    x_data: vec![Xdata {
                        units: Units::Nanometers,
                        label: None,
                        linkid: None,
                        valueorder: None,
                        links: vec![],
                        parameters: vec![],
                        values: create_values_f64(&[1.0, 2.0, 3.0]),
                        alt_x_data: vec![],
                        y_data: vec![Ydata {
                            units: Units::Absorbance,
                            label: None,
                            parameters: vec![],
                            values: create_values_f64(&[10.0, 20.0, 30.0]),
                            peaktables: vec![Peaktable {
                                name: Some("peaktable name".into()),
                                parameters: vec![RawParameter {
                                    group: None,
                                    name: "param 0 name".into(),
                                    label: None,
                                    alias: None,
                                    value: Some("param 0 value".into()),
                                }],
                                peaks: vec![
                                    Peak {
                                        number: 1,
                                        group: Some("peak group".into()),
                                        name: Some("peak name 1".into()),
                                        parameters: vec![RawParameter {
                                            group: None,
                                            name: "param 0 name".into(),
                                            label: None,
                                            alias: None,
                                            value: Some("param 0 value".into()),
                                        }],
                                        peak_x_value: 1.0,
                                        peak_y_value: 10.0,
                                        baseline: None,
                                    },
                                    Peak {
                                        number: 2,
                                        group: Some("peak group".into()),
                                        name: Some("peak name 2".into()),
                                        parameters: vec![RawParameter {
                                            group: None,
                                            name: "param 0 name".into(),
                                            label: None,
                                            alias: None,
                                            value: Some("param 0 value".into()),
                                        }],
                                        peak_x_value: 2.0,
                                        peak_y_value: 20.0,
                                        baseline: None,
                                    },
                                ],
                            }],
                        }],
                    }],
                }],
            }],
        };
        let reader = GamlReader::new(path, gaml);

        let xydata_node = reader.read("/0/0/0").unwrap();
        assert_eq!("XYData 0, 0", &xydata_node.name);
        assert_eq!(1, xydata_node.child_node_names.len());
        assert_eq!(
            "Peaktable 0, peaktable name",
            xydata_node.child_node_names[0]
        );

        let peaktable_node = reader.read("/0/0/0/0").unwrap();
        assert_eq!("Peaktable 0, peaktable name", &peaktable_node.name);

        assert_eq!(
            &vec![
                Parameter::from_str_str("Peaktable name", "peaktable name"),
                Parameter::from_str_str("Peaktable param 0 name", "param 0 value"),
                Parameter::from_str_str("Peak 0 number 1 param 0 name", "param 0 value"),
                Parameter::from_str_str("Peak 1 number 2 param 0 name", "param 0 value"),
            ],
            &peaktable_node.parameters
        );
        assert!(&peaktable_node.data.is_empty());
        assert!(&peaktable_node.metadata.is_empty());
        assert_eq!(
            &Table {
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
                rows: vec![
                    HashMap::from([
                        ("number".into(), Value::U64(1)),
                        ("group".into(), Value::String("peak group".into())),
                        ("name".into(), Value::String("peak name 1".into())),
                        ("peak_x_value".into(), Value::F64(1.0)),
                        ("peak_y_value".into(), Value::F64(10.0)),
                    ]),
                    HashMap::from([
                        ("number".into(), Value::U64(2)),
                        ("group".into(), Value::String("peak group".into())),
                        ("name".into(), Value::String("peak name 2".into())),
                        ("peak_x_value".into(), Value::F64(2.0)),
                        ("peak_y_value".into(), Value::F64(20.0)),
                    ]),
                ]
            },
            &peaktable_node.table.unwrap(),
        );
        assert!(&peaktable_node.child_node_names.is_empty());
    }

    #[test]
    fn maps_gaml_baseline_and_basecurve() {
        let path = "gaml_file.gaml";
        let gaml = Gaml {
            version: Version::Version1_20,
            name: None,
            integrity: None,
            parameters: vec![],
            experiments: vec![Experiment {
                name: None,
                collectdate: None,
                parameters: vec![],
                traces: vec![Trace {
                    name: None,
                    technique: Technique::Unknown,
                    parameters: vec![],
                    coordinates: vec![],
                    x_data: vec![Xdata {
                        units: Units::Nanometers,
                        label: None,
                        linkid: None,
                        valueorder: None,
                        links: vec![],
                        parameters: vec![],
                        values: create_values_f64(&[]),
                        alt_x_data: vec![],
                        y_data: vec![Ydata {
                            units: Units::Absorbance,
                            label: None,
                            parameters: vec![],
                            values: create_values_f64(&[]),
                            peaktables: vec![Peaktable {
                                name: None,
                                parameters: vec![],
                                peaks: vec![Peak {
                                    number: 1,
                                    group: None,
                                    name: None,
                                    parameters: vec![],
                                    peak_x_value: 1.0,
                                    peak_y_value: 10.0,
                                    baseline: Some(Baseline {
                                        parameters: vec![RawParameter {
                                            group: None,
                                            name: "param 0 name".into(),
                                            label: None,
                                            alias: None,
                                            value: Some("param 0 value".into()),
                                        }],
                                        start_x_value: 0.5,
                                        start_y_value: 4.5,
                                        end_x_value: 1.5,
                                        end_y_value: 5.5,
                                        basecurve: Some(Basecurve {
                                            base_x_data: vec![
                                                create_values_f64(&[0.5, 1.0]),
                                                create_values_f64(&[1.5]),
                                            ],
                                            base_y_data: vec![
                                                create_values_f64(&[0.0, 0.1]),
                                                create_values_f64(&[0.0]),
                                            ],
                                        }),
                                    }),
                                }],
                            }],
                        }],
                    }],
                }],
            }],
        };
        let reader = GamlReader::new(path, gaml);

        let peaktable_node = reader.read("/0/0/0/0").unwrap();
        assert_eq!("Peaktable 0", &peaktable_node.name);
        assert_eq!(
            &vec![Parameter::from_str_str(
                "Peak 0 number 1 baseline param 0 name",
                "param 0 value"
            ),],
            &peaktable_node.parameters
        );
        assert!(&peaktable_node.data.is_empty());
        assert!(&peaktable_node.metadata.is_empty());
        assert_eq!(
            &Table {
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
                rows: vec![HashMap::from([
                    ("number".into(), Value::U64(1)),
                    ("peak_x_value".into(), Value::F64(1.0)),
                    ("peak_y_value".into(), Value::F64(10.0)),
                    ("baseline_start_x_value".into(), Value::F64(0.5)),
                    ("baseline_start_y_value".into(), Value::F64(4.5)),
                    ("baseline_end_x_value".into(), Value::F64(1.5)),
                    ("baseline_end_y_value".into(), Value::F64(5.5)),
                ]),]
            },
            &peaktable_node.table.unwrap(),
        );
        assert_eq!(1, peaktable_node.child_node_names.len());
        assert_eq!(
            "Basecurve Peak 0, number 1",
            peaktable_node.child_node_names[0]
        );

        let basecurve_node = reader.read("/0/0/0/0/0").unwrap();
        assert_eq!("Basecurve Peak 0, number 1", &basecurve_node.name);
        assert_eq!(
            &vec![
                Parameter::from_str_str("BaseXdata values 0 format", "FLOAT64"),
                Parameter::from_str_str("BaseXdata values 0 byteorder", "INTEL"),
                Parameter::from_str_u64("BaseXdata values 0 numvalues", 2),
                Parameter::from_str_str("BaseXdata values 1 format", "FLOAT64"),
                Parameter::from_str_str("BaseXdata values 1 byteorder", "INTEL"),
                Parameter::from_str_u64("BaseXdata values 1 numvalues", 1),
                Parameter::from_str_str("BaseYdata values 0 format", "FLOAT64"),
                Parameter::from_str_str("BaseYdata values 0 byteorder", "INTEL"),
                Parameter::from_str_u64("BaseYdata values 0 numvalues", 2),
                Parameter::from_str_str("BaseYdata values 1 format", "FLOAT64"),
                Parameter::from_str_str("BaseYdata values 1 byteorder", "INTEL"),
                Parameter::from_str_u64("BaseYdata values 1 numvalues", 1),
            ],
            &basecurve_node.parameters
        );
        assert_eq!(
            &vec![
                PointXy { x: 0.5, y: 0.0 },
                PointXy { x: 1.0, y: 0.1 },
                PointXy { x: 1.5, y: 0.0 }
            ],
            &basecurve_node.data
        );
        assert!(&basecurve_node.metadata.is_empty());
        assert_eq!(&None, &basecurve_node.table);
        assert!(&basecurve_node.child_node_names.is_empty());
    }
}
