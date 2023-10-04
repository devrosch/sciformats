use wasm_bindgen::prelude::wasm_bindgen;
// #[cfg(target_family = "wasm")]
use wasm_bindgen::JsError;

use crate::{
    andi::AndiError,
    andi_chrom_parser::AndiChromFile,
    api::{Node, Reader, Table},
};
use std::{collections::HashMap, error::Error, path::Path};

#[wasm_bindgen]
pub struct AndiChromReader {
    path: String,
    file: AndiChromFile,
}

// #[cfg(target_family = "wasm")]
#[wasm_bindgen]
impl AndiChromReader {
    pub fn js_read(&self, path: &str) -> Result<Node, JsError> {
        let read_result = Reader::read(self, path);
        match read_result {
            Ok(node) => Ok(node),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}

impl Reader for AndiChromReader {
    fn read(&self, path: &str) -> Result<Node, Box<dyn Error>> {
        let path_indices = Self::convert_path_to_node_indices(path)?;
        match path_indices[..] {
            [] => {
                // "", "/"
                return self.read_root();
            }
            [0] => self.read_admin_data(),
            [1] => self.read_sample_description(),
            [2] => self.read_detection_method(),
            [3] => self.read_raw_data(),
            [4] => self.read_peak_processing_results(),
            [0, 0] => {
                return self.read_error_log();
            }
            _ => return Err(AndiError::new(&format!("Illegal node path: {}", path)).into()),
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
                "Admin Data".to_string(),
                "Sample Description".to_string(),
                "Detection Method".to_string(),
                "Raw Data".to_string(),
                "Peak Processing Results".to_string(),
            ],
        })
    }

    fn push_opt_str(key: &str, val: &Option<String>, vec: &mut Vec<(String, String)>) -> () {
        match val {
            None => (),
            Some(s) => vec.push((key.into(), s.into())),
        }
    }

    fn push_opt_f32(key: &str, val: &Option<f32>, vec: &mut Vec<(String, String)>) -> () {
        match val {
            None => (),
            Some(f) => vec.push((key.into(), f.to_string())),
        }
    }

    fn read_admin_data(&self) -> Result<Node, Box<dyn Error>> {
        let admin_data = &self.file.admin_data;

        let mut parameters: Vec<(String, String)> = Vec::new();
        parameters.push((
            "Dataset Completeness".into(),
            admin_data.dataset_completeness.to_string(),
        ));
        parameters.push((
            "Protocol Template Revision".into(),
            admin_data.protocol_template_revision.to_owned(),
        ));
        parameters.push((
            "NetCDF Revision".into(),
            admin_data.netcdf_revision.to_owned(),
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
        parameters.push((
            "Injection Date/Time Stamp".into(),
            admin_data.injection_date_time_stamp.clone(),
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
            child_node_names: vec!["Error Log".to_string()],
        })
    }

    fn read_sample_description(&self) -> Result<Node, Box<dyn Error>> {
        let sample_description = &self.file.sample_description;

        let mut parameters: Vec<(String, String)> = Vec::new();
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

        let mut parameters: Vec<(String, String)> = Vec::new();
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

        let mut parameters: Vec<(String, String)> = Vec::new();
        parameters.push(("Point Number".into(), raw_data.point_number.to_string()));
        Self::push_opt_str(
            "Raw Data Table Name",
            &raw_data.raw_data_table_name,
            &mut parameters,
        );
        parameters.push(("Retention Unit".into(), raw_data.retention_unit.clone()));
        parameters.push((
            "Actual Run Time Length".into(),
            raw_data.actual_run_time_length.to_string(),
        ));
        parameters.push((
            "Actual Sampling Interval".into(),
            raw_data.actual_sampling_interval.to_string(),
        ));
        parameters.push((
            "Actual Delay Time".into(),
            raw_data.actual_delay_time.to_string(),
        ));
        parameters.push((
            "Uniform Sampling Flag".into(),
            raw_data.uniform_sampling_flag.to_string(),
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
                    return Err(Box::new(AndiError::new(
                        "Numbers of ordinate and retention values do not match.",
                    )));
                }
                let xy_values: Vec<(f64, f64)> = x_values
                    .iter()
                    .enumerate()
                    .map(|(i, &x)| (x as f64, y_values.get(i).unwrap().to_owned() as f64))
                    .collect();
                xy_values
            }
            None => {
                // x values need to be calculated
                let actual_delay_time = raw_data.actual_delay_time as f64;
                let actual_sampling_interval = raw_data.actual_sampling_interval as f64;
                let y_values = &raw_data.get_ordinate_values()?;
                let xy_values: Vec<(f64, f64)> = y_values
                    .iter()
                    .enumerate()
                    .map(|(i, &y)| {
                        // spec is ambigious, could be i or (i+1)
                        let x = actual_delay_time + i as f64 * actual_sampling_interval;
                        (x, y as f64)
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

        let mut parameters: Vec<(String, String)> = Vec::new();
        parameters.push((
            "Peak Number".into(),
            peak_processing_results.peak_number.to_string(),
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
            .ok_or(AndiError::new(&format!(
                "No peaks found but peak_number paramater not zero: {}",
                num_peaks
            )))?;

        // table columns
        let mut column_names: Vec<(String, String)> = vec![];
        if peaks.iter().any(|p| p.peak_retention_time.is_some()) {
            column_names.push((
                "peak_retention_time".to_owned(),
                "Peak Retention Time".to_owned(),
            ));
        }
        if peaks.iter().any(|p| p.peak_name.is_some()) {
            column_names.push(("peak_name".to_owned(), "Peak Name".to_owned()));
        }
        if peaks.iter().any(|p| p.peak_amount.is_some()) {
            column_names.push(("peak_amount".to_owned(), "Peak Amount".to_owned()));
        }
        if peaks.iter().any(|p| p.peak_start_time.is_some()) {
            column_names.push(("peak_start_time".to_owned(), "Peak Start Time".to_owned()));
        }
        if peaks.iter().any(|p| p.peak_end_time.is_some()) {
            column_names.push(("peak_end_time".to_owned(), "Peak End Time".to_owned()));
        }
        if peaks.iter().any(|p| p.peak_width.is_some()) {
            column_names.push(("peak_width".to_owned(), "Peak Width".to_owned()));
        }
        if peaks.iter().any(|p| p.peak_area.is_some()) {
            column_names.push(("peak_area".to_owned(), "Peak Area".to_owned()));
        }
        if peaks.iter().any(|p| p.peak_area_percent.is_some()) {
            column_names.push((
                "peak_area_percent".to_owned(),
                "Peak Area Percent".to_owned(),
            ));
        }
        if peaks.iter().any(|p| p.peak_height.is_some()) {
            column_names.push(("peak_height".to_owned(), "Peak Height".to_owned()));
        }
        if peaks.iter().any(|p| p.peak_height_percent.is_some()) {
            column_names.push((
                "peak_height_percent".to_owned(),
                "Peak Height Percent".to_owned(),
            ));
        }
        if peaks.iter().any(|p| p.baseline_start_time.is_some()) {
            column_names.push((
                "baseline_start_time".to_owned(),
                "Baseline Start Time".to_owned(),
            ));
        }
        if peaks.iter().any(|p| p.baseline_start_value.is_some()) {
            column_names.push((
                "baseline_start_value".to_owned(),
                "Baseline Start Value".to_owned(),
            ));
        }
        if peaks.iter().any(|p| p.baseline_stop_time.is_some()) {
            column_names.push((
                "baseline_stop_time".to_owned(),
                "Baseline Stop Time".to_owned(),
            ));
        }
        if peaks.iter().any(|p| p.baseline_stop_value.is_some()) {
            column_names.push((
                "baseline_stop_value".to_owned(),
                "Baseline Stop Value".to_owned(),
            ));
        }
        if peaks.iter().any(|p| p.peak_start_detection_code.is_some()) {
            column_names.push((
                "peak_start_detection_code".to_owned(),
                "Peak Start Detection Code".to_owned(),
            ));
        }
        if peaks.iter().any(|p| p.peak_stop_detection_code.is_some()) {
            column_names.push((
                "peak_stop_detection_code".to_owned(),
                "Peak Stop Detection Code".to_owned(),
            ));
        }
        if peaks.iter().any(|p| p.retention_index.is_some()) {
            column_names.push(("retention_index".to_owned(), "Retention Index".to_owned()));
        }
        if peaks.iter().any(|p| p.migration_time.is_some()) {
            column_names.push(("migration_time".to_owned(), "Migration Time".to_owned()));
        }
        if peaks.iter().any(|p| p.peak_asymmetry.is_some()) {
            column_names.push(("peak_asymmetry".to_owned(), "Peak Asymmetry".to_owned()));
        }
        if peaks.iter().any(|p| p.peak_efficiency.is_some()) {
            column_names.push(("peak_efficiency".to_owned(), "Peak Efficiency".to_owned()));
        }
        if peaks.iter().any(|p| p.mass_on_column.is_some()) {
            column_names.push(("mass_on_column".to_owned(), "Mass On Column".to_owned()));
        }
        column_names.push((
            "manually_reintegrated_peaks".to_owned(),
            "Manually Reintegrated Peak".to_owned(),
        ));
        column_names.push((
            "peak_retention_unit".to_owned(),
            "Peak Retention Unit".to_owned(),
        ));
        if peaks.iter().any(|p| p.peak_amount_unit.is_some()) {
            column_names.push(("peak_amount_unit".to_owned(), "Peak Amount Unit".to_owned()));
        }
        if peaks.iter().any(|p| p.detector_unit.is_some()) {
            column_names.push(("detector_unit".to_owned(), "Detector Unit".to_owned()));
        }

        // table rows
        let mut rows: Vec<HashMap<String, String>> = vec![];
        for peak in peaks {
            let mut row: HashMap<String, String> = HashMap::new();
            if let Some(val) = &peak.peak_retention_time {
                row.insert("peak_retention_time".into(), val.to_string());
            }
            if let Some(val) = &peak.peak_name {
                row.insert("peak_name".into(), val.into());
            }
            if let Some(val) = &peak.peak_amount {
                row.insert("peak_amount".into(), val.to_string());
            }
            if let Some(val) = &peak.peak_start_time {
                row.insert("peak_start_time".into(), val.to_string());
            }
            if let Some(val) = &peak.peak_end_time {
                row.insert("peak_end_time".into(), val.to_string());
            }
            if let Some(val) = &peak.peak_width {
                row.insert("peak_width".into(), val.to_string());
            }
            if let Some(val) = &peak.peak_area {
                row.insert("peak_area".into(), val.to_string());
            }
            if let Some(val) = &peak.peak_area_percent {
                row.insert("peak_area_percent".into(), val.to_string());
            }
            if let Some(val) = &peak.peak_height {
                row.insert("peak_height".into(), val.to_string());
            }
            if let Some(val) = &peak.peak_height_percent {
                row.insert("peak_height_percent".into(), val.to_string());
            }
            if let Some(val) = &peak.baseline_start_time {
                row.insert("baseline_start_time".into(), val.to_string());
            }
            if let Some(val) = &peak.baseline_start_value {
                row.insert("baseline_start_value".into(), val.to_string());
            }
            if let Some(val) = &peak.baseline_stop_time {
                row.insert("baseline_stop_time".into(), val.to_string());
            }
            if let Some(val) = &peak.baseline_stop_value {
                row.insert("baseline_stop_value".into(), val.to_string());
            }
            if let Some(val) = &peak.peak_start_detection_code {
                row.insert("peak_start_detection_code".into(), val.into());
            }
            if let Some(val) = &peak.peak_stop_detection_code {
                row.insert("peak_stop_detection_code".into(), val.into());
            }
            if let Some(val) = &peak.retention_index {
                row.insert("retention_index".into(), val.to_string());
            }
            if let Some(val) = &peak.migration_time {
                row.insert("migration_time".into(), val.to_string());
            }
            if let Some(val) = &peak.peak_asymmetry {
                row.insert("peak_asymmetry".into(), val.to_string());
            }
            if let Some(val) = &peak.peak_efficiency {
                row.insert("peak_efficiency".into(), val.to_string());
            }
            if let Some(val) = &peak.mass_on_column {
                row.insert("mass_on_column".into(), val.to_string());
            }
            row.insert(
                "manually_reintegrated_peaks".into(),
                peak.manually_reintegrated_peaks.to_string(),
            );
            row.insert(
                "peak_retention_unit".into(),
                peak.peak_retention_unit.clone(),
            );
            if let Some(val) = &peak.peak_amount_unit {
                row.insert("peak_amount_unit".into(), val.to_string());
            }
            if let Some(val) = &peak.detector_unit {
                row.insert("detector_unit".into(), val.to_string());
            }

            rows.push(row);
        }

        Ok(Table { column_names, rows })
    }

    fn read_error_log(&self) -> Result<Node, Box<dyn Error>> {
        let column_names: Vec<(String, String)> = vec![("message".into(), "Message".into())];
        let rows: Vec<HashMap<String, String>> = self
            .file
            .admin_data
            .error_log
            .iter()
            .map(|e| {
                let mut map: HashMap<String, String> = HashMap::new();
                map.insert("message".into(), e.into());
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

    fn convert_path_to_node_indices(path: &str) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut path_segments: Vec<&str> = path.split('/').collect();
        // remove blank start segment(s)
        match path_segments[..] {
            // "/" or ""
            ["", ""] | [""] => path_segments = vec![],
            // "/xyz"
            ["", ..] => {
                path_segments.remove(0);
            }
            _ => (),
        };
        // map segments to indices, expected segment structure is "n-some optional name"
        let mut indices: Vec<usize> = vec![];
        for seg in path_segments {
            let idx_str = seg.split_once("-").map_or(seg, |p| p.0);
            let idx = idx_str.parse::<usize>()?;
            indices.push(idx);
        }

        Ok(indices)
    }
}

// TODO: needed?
// pub struct AndiNonStandardVariables {}

// pub struct AndiNonStandardAttributes {}
