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

use super::andi_chrom_parser::AndiChromFile;
use crate::{
    api::{Column, Node, Parameter, PointXy, Reader, Table, Value},
    common::SfError,
    utils::convert_path_to_node_indices,
};
use std::{collections::HashMap, error::Error, path::Path};

pub struct AndiChromReader {
    path: String,
    file: AndiChromFile,
}

impl Reader for AndiChromReader {
    fn read(&self, path: &str) -> Result<Node, Box<dyn Error>> {
        let path_indices = convert_path_to_node_indices(path)?;
        match path_indices[..] {
            [] => self.read_root(), // "", "/"
            [0] => self.read_admin_data(),
            [1] => self.read_sample_description(),
            [2] => self.read_detection_method(),
            [3] => self.read_raw_data(),
            [4] => self.read_peak_processing_results(),
            [0, 0] => self.read_error_log(),
            _ => Err(SfError::new(&format!("Illegal node path: {}", path)).into()),
        }
    }
}

impl AndiChromReader {
    pub fn new(path: &str, file: AndiChromFile) -> Self {
        AndiChromReader {
            path: path.to_owned(),
            file,
        }
    }

    fn read_root(&self) -> Result<Node, Box<dyn Error>> {
        let path = Path::new(&self.path);
        let file_name = path.file_name().map_or("", |f| f.to_str().unwrap_or(""));
        Ok(Node {
            name: file_name.to_owned(),
            parameters: Vec::new(),
            data: Vec::new(),
            metadata: Vec::new(),
            table: None,
            child_node_names: vec![
                "Admin Data".to_owned(),
                "Sample Description".to_owned(),
                "Detection Method".to_owned(),
                "Raw Data".to_owned(),
                "Peak Processing Results".to_owned(),
            ],
        })
    }

    fn push_opt_str(key: &str, val: &Option<String>, vec: &mut Vec<Parameter>) {
        if let Some(v) = val {
            vec.push(Parameter::from_str_str(key, v))
        }
    }

    fn push_opt_f32(key: &str, val: &Option<f32>, vec: &mut Vec<Parameter>) {
        if let Some(v) = val {
            vec.push(Parameter::from_str_f32(key, *v))
        }
    }

    fn read_admin_data(&self) -> Result<Node, Box<dyn Error>> {
        let admin_data = &self.file.admin_data;

        let mut parameters: Vec<Parameter> = Vec::new();
        parameters.push(Parameter::from_str_str(
            "Dataset Completeness",
            admin_data.dataset_completeness.to_string(),
        ));
        parameters.push(Parameter::from_str_str(
            "Protocol Template Revision",
            &admin_data.protocol_template_revision,
        ));
        parameters.push(Parameter::from_str_str(
            "NetCDF Revision",
            &admin_data.netcdf_revision,
        ));
        Self::push_opt_str("Languages", &admin_data.languages, &mut parameters);
        Self::push_opt_str(
            "Administrative Comments",
            &admin_data.administrative_comments,
            &mut parameters,
        );
        Self::push_opt_str(
            "Dataset Origin",
            &admin_data.dataset_origin,
            &mut parameters,
        );
        Self::push_opt_str("Dataset Owner", &admin_data.dataset_owner, &mut parameters);
        Self::push_opt_str(
            "Dataset Date/Time Stamp",
            &admin_data.dataset_date_time_stamp,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_str(
            "Injection Date/Time Stamp",
            &admin_data.injection_date_time_stamp,
        ));
        Self::push_opt_str(
            "Experiment Title",
            &admin_data.experiment_title,
            &mut parameters,
        );
        Self::push_opt_str("Operator Name", &admin_data.operator_name, &mut parameters);
        Self::push_opt_str(
            "Separation Experiment Type",
            &admin_data.separation_experiment_type,
            &mut parameters,
        );
        Self::push_opt_str(
            "Company Method Name",
            &admin_data.company_method_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Company Method ID",
            &admin_data.company_method_id,
            &mut parameters,
        );
        Self::push_opt_str(
            "Pre Experiment Program Name",
            &admin_data.pre_experiment_program_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Post Experiment Program Name",
            &admin_data.post_experiment_program_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Source File Reference",
            &admin_data.source_file_reference,
            &mut parameters,
        );

        Ok(Node {
            name: "Admin Data".to_owned(),
            parameters,
            data: Vec::new(),
            metadata: Vec::new(),
            table: None,
            child_node_names: vec!["Error Log".to_owned()],
        })
    }

    fn read_sample_description(&self) -> Result<Node, Box<dyn Error>> {
        let sample_description = &self.file.sample_description;

        let mut parameters: Vec<Parameter> = Vec::new();
        Self::push_opt_str(
            "Sample ID Comments",
            &sample_description.sample_id_comments,
            &mut parameters,
        );
        Self::push_opt_str("Sample ID", &sample_description.sample_id, &mut parameters);
        Self::push_opt_str(
            "Sample Name",
            &sample_description.sample_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample Type",
            &sample_description.sample_type,
            &mut parameters,
        );
        Self::push_opt_f32(
            "Sample Injection Volume",
            &sample_description.sample_injection_volume,
            &mut parameters,
        );
        Self::push_opt_f32(
            "Sample Amount",
            &sample_description.sample_amount,
            &mut parameters,
        );

        Ok(Node {
            name: "Sample Description".to_owned(),
            parameters,
            data: Vec::new(),
            metadata: Vec::new(),
            table: None,
            child_node_names: Vec::new(),
        })
    }

    fn read_detection_method(&self) -> Result<Node, Box<dyn Error>> {
        let detection_method = &self.file.detection_method;

        let mut parameters: Vec<Parameter> = Vec::new();
        Self::push_opt_str(
            "Detection Method Table Name",
            &detection_method.detection_method_table_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Detector Method Comments",
            &detection_method.detector_method_comments,
            &mut parameters,
        );
        Self::push_opt_str(
            "Detection Method Name",
            &detection_method.detection_method_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Detector Name",
            &detection_method.detector_name,
            &mut parameters,
        );
        Self::push_opt_f32(
            "Detector Maximum Value",
            &detection_method.detector_maximum_value,
            &mut parameters,
        );
        Self::push_opt_f32(
            "Detector Minimum Value",
            &detection_method.detector_minimum_value,
            &mut parameters,
        );
        Self::push_opt_str(
            "Detector Unit",
            &detection_method.detector_unit,
            &mut parameters,
        );

        Ok(Node {
            name: "Detection Method".to_owned(),
            parameters,
            data: Vec::new(),
            metadata: Vec::new(),
            table: None,
            child_node_names: Vec::new(),
        })
    }

    fn read_raw_data(&self) -> Result<Node, Box<dyn Error>> {
        let raw_data = &self.file.raw_data;

        let mut parameters: Vec<Parameter> = Vec::new();
        parameters.push(Parameter {
            key: "Point Number".into(),
            value: Value::I32(raw_data.point_number),
        });
        Self::push_opt_str(
            "Raw Data Table Name",
            &raw_data.raw_data_table_name,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_str(
            "Retention Unit",
            &raw_data.retention_unit,
        ));
        parameters.push(Parameter::from_str_f32(
            "Actual Run Time Length",
            raw_data.actual_run_time_length,
        ));
        parameters.push(Parameter::from_str_f32(
            "Actual Sampling Interval",
            raw_data.actual_sampling_interval,
        ));
        parameters.push(Parameter::from_str_f32(
            "Actual Delay Time",
            raw_data.actual_delay_time,
        ));
        parameters.push(Parameter::from_str_bool(
            "Uniform Sampling Flag",
            raw_data.uniform_sampling_flag,
        ));
        Self::push_opt_str(
            "Autosampler Position",
            &raw_data.autosampler_position,
            &mut parameters,
        );

        // map to xy pairs
        let raw_data_retention = raw_data.get_raw_data_retention()?;
        let data = match &raw_data_retention {
            Some(x_values) => {
                // x values present
                let y_values = &raw_data.get_ordinate_values()?;
                if x_values.len() != y_values.len() {
                    return Err(Box::new(SfError::new(
                        "Numbers of ordinate and retention values do not match.",
                    )));
                }
                let xy_values: Vec<PointXy> = x_values
                    .iter()
                    .enumerate()
                    .map(|(i, &x)| {
                        PointXy::new(x as f64, y_values.get(i).unwrap().to_owned() as f64)
                    })
                    .collect();
                xy_values
            }
            None => {
                // x values need to be calculated
                let actual_delay_time = raw_data.actual_delay_time as f64;
                let actual_sampling_interval = raw_data.actual_sampling_interval as f64;
                let y_values = &raw_data.get_ordinate_values()?;
                let xy_values: Vec<PointXy> = y_values
                    .iter()
                    .enumerate()
                    .map(|(i, &y)| {
                        // spec is ambigious, could be i or (i+1)
                        let x = actual_delay_time + i as f64 * actual_sampling_interval;
                        PointXy::new(x, y as f64)
                    })
                    .collect();
                xy_values
            }
        };

        let mut metadata: Vec<(String, String)> = vec![];
        metadata.push(("x.unit".to_owned(), raw_data.retention_unit.to_owned()));
        let y_unit = &self.file.detection_method.detector_unit;
        if let Some(y_unit) = y_unit {
            metadata.push(("y.unit".to_owned(), y_unit.to_owned()));
        }

        Ok(Node {
            name: "Raw Data".to_owned(),
            parameters,
            data,
            metadata,
            table: None,
            child_node_names: Vec::new(),
        })
    }

    fn read_peak_processing_results(&self) -> Result<Node, Box<dyn Error>> {
        let peak_processing_results = &self.file.peak_processing_results;

        let mut parameters: Vec<Parameter> = Vec::new();
        parameters.push(Parameter::from_str_i32(
            "Peak Number",
            peak_processing_results.peak_number,
        ));
        Self::push_opt_str(
            "Peak Processing Results Table Name",
            &peak_processing_results.peak_processing_results_table_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Peak Processing Results Comments",
            &peak_processing_results.peak_processing_results_comments,
            &mut parameters,
        );
        Self::push_opt_str(
            "Peak Processing Method Name",
            &peak_processing_results.peak_processing_method_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Peak Processing Date Time Stamp",
            &peak_processing_results.peak_processing_date_time_stamp,
            &mut parameters,
        );
        Self::push_opt_str(
            "Peak Amount Unit",
            &peak_processing_results.peak_amount_unit,
            &mut parameters,
        );

        let table = match &peak_processing_results.peak_number {
            0 => None,
            _ => Some(self.read_peaks(peak_processing_results.peak_number as usize)?),
        };

        Ok(Node {
            name: "Peak Processing Results".to_owned(),
            parameters,
            data: Vec::new(),
            metadata: Vec::new(),
            table,
            child_node_names: Vec::new(),
        })
    }

    fn read_peaks(&self, num_peaks: usize) -> Result<Table, Box<dyn Error>> {
        let peaks = self
            .file
            .peak_processing_results
            .get_peaks()?
            // .peaks
            // .as_ref()
            .ok_or(SfError::new(&format!(
                "No peaks found but peak_number paramater not zero: {}",
                num_peaks
            )))?;

        // table columns
        let mut column_names: Vec<Column> = vec![];
        if peaks.iter().any(|p| p.peak_retention_time.is_some()) {
            column_names.push(Column::new("peak_retention_time", "Peak Retention Time"));
        }
        if peaks.iter().any(|p| p.peak_name.is_some()) {
            column_names.push(Column::new("peak_name", "Peak Name"));
        }
        if peaks.iter().any(|p| p.peak_amount.is_some()) {
            column_names.push(Column::new("peak_amount", "Peak Amount"));
        }
        if peaks.iter().any(|p| p.peak_start_time.is_some()) {
            column_names.push(Column::new("peak_start_time", "Peak Start Time"));
        }
        if peaks.iter().any(|p| p.peak_end_time.is_some()) {
            column_names.push(Column::new("peak_end_time", "Peak End Time"));
        }
        if peaks.iter().any(|p| p.peak_width.is_some()) {
            column_names.push(Column::new("peak_width", "Peak Width"));
        }
        if peaks.iter().any(|p| p.peak_area.is_some()) {
            column_names.push(Column::new("peak_area", "Peak Area"));
        }
        if peaks.iter().any(|p| p.peak_area_percent.is_some()) {
            column_names.push(Column::new("peak_area_percent", "Peak Area Percent"));
        }
        if peaks.iter().any(|p| p.peak_height.is_some()) {
            column_names.push(Column::new("peak_height", "Peak Height"));
        }
        if peaks.iter().any(|p| p.peak_height_percent.is_some()) {
            column_names.push(Column::new("peak_height_percent", "Peak Height Percent"));
        }
        if peaks.iter().any(|p| p.baseline_start_time.is_some()) {
            column_names.push(Column::new("baseline_start_time", "Baseline Start Time"));
        }
        if peaks.iter().any(|p| p.baseline_start_value.is_some()) {
            column_names.push(Column::new("baseline_start_value", "Baseline Start Value"));
        }
        if peaks.iter().any(|p| p.baseline_stop_time.is_some()) {
            column_names.push(Column::new("baseline_stop_time", "Baseline Stop Time"));
        }
        if peaks.iter().any(|p| p.baseline_stop_value.is_some()) {
            column_names.push(Column::new("baseline_stop_value", "Baseline Stop Value"));
        }
        if peaks.iter().any(|p| p.peak_start_detection_code.is_some()) {
            column_names.push(Column::new(
                "peak_start_detection_code",
                "Peak Start Detection Code",
            ));
        }
        if peaks.iter().any(|p| p.peak_stop_detection_code.is_some()) {
            column_names.push(Column::new(
                "peak_stop_detection_code",
                "Peak Stop Detection Code",
            ));
        }
        if peaks.iter().any(|p| p.retention_index.is_some()) {
            column_names.push(Column::new("retention_index", "Retention Index"));
        }
        if peaks.iter().any(|p| p.migration_time.is_some()) {
            column_names.push(Column::new("migration_time", "Migration Time"));
        }
        if peaks.iter().any(|p| p.peak_asymmetry.is_some()) {
            column_names.push(Column::new("peak_asymmetry", "Peak Asymmetry"));
        }
        if peaks.iter().any(|p| p.peak_efficiency.is_some()) {
            column_names.push(Column::new("peak_efficiency", "Peak Efficiency"));
        }
        if peaks.iter().any(|p| p.mass_on_column.is_some()) {
            column_names.push(Column::new("mass_on_column", "Mass On Column"));
        }
        column_names.push(Column::new(
            "manually_reintegrated_peaks",
            "Manually Reintegrated Peak",
        ));
        column_names.push(Column::new("peak_retention_unit", "Peak Retention Unit"));
        if peaks.iter().any(|p| p.peak_amount_unit.is_some()) {
            column_names.push(Column::new("peak_amount_unit", "Peak Amount Unit"));
        }
        if peaks.iter().any(|p| p.detector_unit.is_some()) {
            column_names.push(Column::new("detector_unit", "Detector Unit"));
        }

        // table rows
        let mut rows: Vec<HashMap<String, Value>> = vec![];
        for peak in peaks {
            let mut row: HashMap<String, Value> = HashMap::new();
            if let Some(val) = peak.peak_retention_time {
                row.insert("peak_retention_time".into(), Value::F32(val));
            }
            if let Some(val) = peak.peak_name {
                row.insert("peak_name".into(), Value::String(val));
            }
            if let Some(val) = peak.peak_amount {
                row.insert("peak_amount".into(), Value::F32(val));
            }
            if let Some(val) = peak.peak_start_time {
                row.insert("peak_start_time".into(), Value::F32(val));
            }
            if let Some(val) = peak.peak_end_time {
                row.insert("peak_end_time".into(), Value::F32(val));
            }
            if let Some(val) = peak.peak_width {
                row.insert("peak_width".into(), Value::F32(val));
            }
            if let Some(val) = peak.peak_area {
                row.insert("peak_area".into(), Value::F32(val));
            }
            if let Some(val) = peak.peak_area_percent {
                row.insert("peak_area_percent".into(), Value::F32(val));
            }
            if let Some(val) = peak.peak_height {
                row.insert("peak_height".into(), Value::F32(val));
            }
            if let Some(val) = peak.peak_height_percent {
                row.insert("peak_height_percent".into(), Value::F32(val));
            }
            if let Some(val) = peak.baseline_start_time {
                row.insert("baseline_start_time".into(), Value::F32(val));
            }
            if let Some(val) = peak.baseline_start_value {
                row.insert("baseline_start_value".into(), Value::F32(val));
            }
            if let Some(val) = peak.baseline_stop_time {
                row.insert("baseline_stop_time".into(), Value::F32(val));
            }
            if let Some(val) = peak.baseline_stop_value {
                row.insert("baseline_stop_value".into(), Value::F32(val));
            }
            if let Some(val) = peak.peak_start_detection_code {
                row.insert("peak_start_detection_code".into(), Value::String(val));
            }
            if let Some(val) = peak.peak_stop_detection_code {
                row.insert("peak_stop_detection_code".into(), Value::String(val));
            }
            if let Some(val) = peak.retention_index {
                row.insert("retention_index".into(), Value::F32(val));
            }
            if let Some(val) = peak.migration_time {
                row.insert("migration_time".into(), Value::F32(val));
            }
            if let Some(val) = peak.peak_asymmetry {
                row.insert("peak_asymmetry".into(), Value::F32(val));
            }
            if let Some(val) = peak.peak_efficiency {
                row.insert("peak_efficiency".into(), Value::F32(val));
            }
            if let Some(val) = peak.mass_on_column {
                row.insert("mass_on_column".into(), Value::F32(val));
            }
            row.insert(
                "manually_reintegrated_peaks".into(),
                Value::Bool(peak.manually_reintegrated_peaks),
            );
            row.insert(
                "peak_retention_unit".into(),
                Value::String(peak.peak_retention_unit),
            );
            if let Some(val) = peak.peak_amount_unit {
                row.insert("peak_amount_unit".into(), Value::String(val));
            }
            if let Some(val) = peak.detector_unit {
                row.insert("detector_unit".into(), Value::String(val));
            }

            rows.push(row);
        }

        Ok(Table { column_names, rows })
    }

    fn read_error_log(&self) -> Result<Node, Box<dyn Error>> {
        let column_names: Vec<Column> = vec![Column::new("message", "Message")];
        let rows: Vec<HashMap<String, Value>> = self
            .file
            .admin_data
            .error_log
            .iter()
            .map(|e| {
                let mut map: HashMap<String, Value> = HashMap::new();
                map.insert("message".into(), Value::String(e.to_owned()));
                map
            })
            .collect();

        Ok(Node {
            name: "Error Log".into(),
            parameters: vec![],
            data: vec![],
            metadata: vec![],
            table: Some(Table { column_names, rows }),
            child_node_names: vec![],
        })
    }
}

// TODO: needed?
// pub struct AndiNonStandardVariables {}

// pub struct AndiNonStandardAttributes {}
