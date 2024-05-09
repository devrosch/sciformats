use super::{
    gaml_parser::{
        Basecurve, Coordinates, Experiment, Gaml, Peak, Peaktable, Trace, Units, Xdata, Ydata,
    },
    gaml_utils::{
        generate_child_node_names, map_gaml_parameters, map_values_attributes, read_elem, TypeName,
    },
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
        match &path_indices[..] {
            [] => Ok(self.map_root()?), // "", "/"
            [exp_idx, tail @ ..] => {
                let experiment = read_elem(&self.file.experiments, *exp_idx)?;
                if tail.is_empty() {
                    return Ok(self.map_experiment(experiment, *exp_idx)?);
                }

                let (trace_idx, tail) = tail.split_first().unwrap();
                let trace = read_elem(&experiment.traces, *trace_idx)?;
                if tail.is_empty() {
                    return Ok(self.map_trace(trace, *trace_idx)?);
                }

                let (xy_data_idx, tail) = tail.split_first().unwrap();
                let (x_data_idx, alt_x_data_idx, y_data_idx) =
                    Self::find_xy_indices(trace, *xy_data_idx)?;
                let x_data = read_elem(&trace.x_data, x_data_idx)?;
                if tail.is_empty() {
                    let coordinates = trace.coordinates.as_slice();
                    match alt_x_data_idx {
                        None => {
                            return Ok(self.map_xy_data(
                                x_data,
                                (x_data_idx, y_data_idx, *xy_data_idx),
                                coordinates,
                            )?)
                        }
                        Some(alt_x_idx) => {
                            return Ok(self.map_alt_xy_data(
                                x_data,
                                (x_data_idx, alt_x_idx, y_data_idx, *xy_data_idx),
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
                let y_data = read_elem(&x_data.y_data, y_data_idx)?;
                let peaktable = read_elem(&y_data.peaktables, *peaktable_idx)?;
                if tail.is_empty() {
                    return Ok(self.map_peaktable(peaktable, *peaktable_idx)?);
                }

                let (basecurve_idx, tail) = tail.split_first().unwrap();
                let (basecurve, peak, peak_index) =
                    Self::find_basecurve(peaktable, *basecurve_idx)?;
                if tail.is_empty() {
                    return Ok(self.map_basecurve(basecurve, peak_index, peak.number)?);
                }

                Err(GamlError::new(&format!("Illegal node path: {}", path)).into())
            }
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

        let child_node_names =
            generate_child_node_names(&self.file.experiments, &Self::generate_experiment_name);

        Ok(Node {
            name: file_name.to_owned(),
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

    fn map_experiment(&self, experiment: &Experiment, index: usize) -> Result<Node, GamlError> {
        let name = Self::generate_experiment_name(experiment, index);

        let mut parameters = vec![];
        if let Some(name) = &experiment.name {
            parameters.push(Parameter::from_str_str("Name", name));
        }
        if let Some(date) = &experiment.collectdate {
            parameters.push(Parameter::from_str_str("Collectdate", date.to_rfc3339()));
        }
        parameters.extend(map_gaml_parameters(&experiment.parameters));

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

    fn generate_trace_name(trace: &Trace, index: usize) -> String {
        match &trace.name {
            None => format!("Trace {index}"),
            Some(trace_name) => format!("Trace {index}, {trace_name}"),
        }
    }

    fn map_trace(&self, trace: &Trace, index: usize) -> Result<Node, GamlError> {
        let name = Self::generate_trace_name(trace, index);

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

        let metadata = Self::generate_xy_plot_hints(
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

        let metadata = Self::generate_xy_plot_hints(
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

    fn generate_peaktable_name(peaktable: &Peaktable, index: usize) -> String {
        match &peaktable.name {
            None => format!("Peaktable {index}"),
            Some(name) => format!("Peaktable {index}, {}", name),
        }
    }

    fn map_peaktable(&self, peaktable: &Peaktable, index: usize) -> Result<Node, GamlError> {
        let name = Self::generate_peaktable_name(peaktable, index);

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

    fn generate_basecurve_name(peak_index: usize, peak_number: u64) -> String {
        format!("Basecurve Peak {}, number {}", peak_index, peak_number)
    }

    fn map_basecurve(
        &self,
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
}

#[cfg(test)]
mod tests {
    use crate::gaml::gaml_parser::Integrity;
    use crate::gaml::gaml_parser::Parameter as RawParameter;

    use super::*;

    #[test]
    fn accepts_valid_gaml() {
        let path = "gaml_file.gaml";
        let gaml = Gaml {
            version: "1.20".into(),
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

        let root = reader.read("/").unwrap();
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
            root
        );
    }
}
