use super::andi_utils::{
    read_index_from_slice, read_index_from_var_2d_string, read_index_from_var_f32,
    read_multi_string_var, read_optional_var,
};
use crate::{
    andi::{AndiDatasetCompleteness, AndiError},
    api::{Node, Parser, Reader, Scanner, Table},
};
use netcdf3::DataType;
use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Seek},
    path::Path,
    str::FromStr,
};

pub struct AndiChromScanner {}

impl AndiChromScanner {
    const ACCEPTED_EXTENSIONS: [&str; 2] = ["cdf", "nc"];
    const MAGIC_BYTES: [u8; 3] = [0x43, 0x44, 0x46]; // "CDF"
}

impl<T: Seek + Read + 'static> Scanner<T> for AndiChromScanner {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        let p = Path::new(path);
        let extension = p
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| Some(ext.to_lowercase()));
        match extension {
            None => return false,
            Some(ext) => {
                let is_recognized_extension = Self::ACCEPTED_EXTENSIONS
                    .iter()
                    .any(|accept_ext| *accept_ext == ext);
                if !is_recognized_extension {
                    return false;
                }
            }
        }

        // recognized extension => check first few bytes ("magic bytes")

        let mut buf = [0u8; 3];
        let read_success = input.read_exact(&mut buf);
        if read_success.is_err() {
            return false;
        }

        buf.as_slice() == Self::MAGIC_BYTES
    }

    fn get_reader(
        &self,
        path: &str,
        input: T,
    ) -> Result<Box<dyn crate::api::Reader>, Box<dyn Error>> {
        let file = AndiChromParser::parse(path, input)?;
        Ok(Box::new(AndiChromReader::new(file)))
    }
}

pub struct AndiChromParser {}

pub struct AndiChromReader {
    file: AndiChromFile,
}

impl AndiChromReader {
    pub fn new(file: AndiChromFile) -> Self {
        AndiChromReader { file }
    }

    fn read_root(&self, path: &str) -> Result<Node, Box<dyn Error>> {
        let path = Path::new(path);
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
            "Dataset Date/Time Stamp",
            &admin_data.dataset_date_time_stamp,
            &mut parameters,
        );
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
            "Detection Method Comments",
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
        let data = match &raw_data.raw_data_retention {
            Some(x_values) => {
                // x values present
                let y_values = &raw_data.ordinate_values;
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
                let y_values = &raw_data.ordinate_values;
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

        Ok(Node {
            name: "Raw Data".to_owned(),
            parameters,
            data,
            metadata: Vec::new(),
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
            .peaks
            .as_ref()
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
        path_segments = match path_segments[..] {
            // "/"
            ["", ""] => path_segments.split_at(2).0.to_vec(),
            // ""
            [""] => path_segments.split_at(1).0.to_vec(),
            _ => path_segments,
        };
        // map segments to indices, expected segment structure is "n-some optional name"
        let mut indices: Vec<usize> = vec![];
        for seg in path_segments {
            let idx_str = seg
                .split_once("-")
                .ok_or(AndiError::new(&format!("Illegal node path: {}", path)))?
                .0;
            let idx = idx_str.parse::<usize>()?;
            indices.push(idx);
        }

        Ok(indices)
    }
}

impl Reader for AndiChromReader {
    fn read(&self, path: &str) -> Result<Node, Box<dyn Error>> {
        let path_indices = Self::convert_path_to_node_indices(path)?;
        match path_indices[..] {
            [] => {
                // "", "/"
                return self.read_root(path);
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

impl<T: Seek + Read + 'static> Parser<T> for AndiChromParser {
    type R = AndiChromFile;

    fn parse(name: &str, input: T) -> Result<Self::R, Box<dyn std::error::Error>> {
        let input_seek_read = Box::new(input);
        let mut reader = netcdf3::FileReader::open_seek_read(name, input_seek_read)?;

        AndiChromFile::new(&mut reader)
    }
}

#[derive(Debug)]
pub struct AndiChromFile {
    pub admin_data: AndiChromAdminData,
    pub sample_description: AndiChromSampleDescription,
    pub detection_method: AndiChromDetectionMethod,
    pub raw_data: AndiChromRawData,
    pub peak_processing_results: AndiChromPeakProcessingResults,
    pub non_standard_variables: Vec<String>,
    pub non_standard_attributes: Vec<String>,
}

impl AndiChromFile {
    pub fn new(mut reader: &mut netcdf3::FileReader) -> Result<Self, Box<dyn std::error::Error>> {
        let admin_data = AndiChromAdminData::new(&mut reader)?;
        let sample_description = AndiChromSampleDescription::new(&mut reader)?;
        let detection_method = AndiChromDetectionMethod::new(&mut reader)?;
        let raw_data = AndiChromRawData::new(&mut reader)?;
        let peak_processing_results = AndiChromPeakProcessingResults::new(
            &mut reader,
            &raw_data.retention_unit,
            detection_method.detector_unit.as_deref(),
        )?;

        Ok(AndiChromFile {
            admin_data,
            sample_description,
            detection_method,
            raw_data,
            peak_processing_results,
            // TODO: read
            non_standard_variables: vec![],
            // TODO: read
            non_standard_attributes: vec![],
        })
    }
}

#[derive(Debug)]
pub struct AndiChromAdminData {
    pub dataset_completeness: AndiDatasetCompleteness, // required
    pub protocol_template_revision: String,            // required
    pub netcdf_revision: String,                       // required
    pub languages: Option<String>,
    pub administrative_comments: Option<String>,
    pub dataset_origin: Option<String>,
    pub dataset_owner: Option<String>,
    pub dataset_date_time_stamp: Option<String>,
    pub injection_date_time_stamp: String, // required
    pub experiment_title: Option<String>,
    pub operator_name: Option<String>,
    pub separation_experiment_type: Option<String>,
    pub company_method_name: Option<String>,
    pub company_method_id: Option<String>,
    pub pre_experiment_program_name: Option<String>,
    pub post_experiment_program_name: Option<String>,
    pub source_file_reference: Option<String>,
    pub error_log: Vec<String>,
}

impl AndiChromAdminData {
    pub fn new(reader: &mut netcdf3::FileReader) -> Result<Self, Box<dyn Error>> {
        let dataset_completeness_attr = reader
            .data_set()
            .get_global_attr_as_string("dataset_completeness")
            .ok_or(AndiError::new("Missing dataset_completeness attribute."))?;
        let dataset_completeness = AndiDatasetCompleteness::from_str(&dataset_completeness_attr)?;
        let protocol_template_revision = reader
            .data_set()
            .get_global_attr_as_string("aia_template_revision")
            .ok_or(AndiError::new("Missing aia_template_revision attribute."))?;
        let netcdf_revision = reader
            .data_set()
            .get_global_attr_as_string("netcdf_revision")
            .ok_or(AndiError::new("Missing netcdf_revision attribute."))?;
        let languages = reader.data_set().get_global_attr_as_string("languages");
        let administrative_comments = reader
            .data_set()
            .get_global_attr_as_string("administrative_comments");
        let dataset_origin = reader
            .data_set()
            .get_global_attr_as_string("dataset_origin");
        let dataset_owner = reader.data_set().get_global_attr_as_string("dataset_owner");
        let dataset_date_time_stamp = reader
            .data_set()
            .get_global_attr_as_string("dataset_date_time_stamp");
        let injection_date_time_stamp = reader
            .data_set()
            .get_global_attr_as_string("injection_date_time_stamp")
            .ok_or(AndiError::new(
                "Missing injection_date_time_stamp attribute.",
            ))?;
        let experiment_title = reader
            .data_set()
            .get_global_attr_as_string("experiment_title");
        let operator_name = reader.data_set().get_global_attr_as_string("operator_name");
        let separation_experiment_type = reader
            .data_set()
            .get_global_attr_as_string("separation_experiment_type");
        let company_method_name = reader
            .data_set()
            .get_global_attr_as_string("company_method_name");
        let company_method_id = reader
            .data_set()
            .get_global_attr_as_string("company_method_id");
        let pre_experiment_program_name = reader
            .data_set()
            .get_global_attr_as_string("pre_experiment_program_name");
        let post_experiment_program_name = reader
            .data_set()
            .get_global_attr_as_string("post_experiment_program_name");
        let source_file_reference = reader
            .data_set()
            .get_global_attr_as_string("source_file_reference");
        let error_log = read_multi_string_var(reader, "error_log")?;

        Ok(Self {
            dataset_completeness,
            protocol_template_revision,
            netcdf_revision,
            languages,
            administrative_comments,
            dataset_origin,
            dataset_owner,
            dataset_date_time_stamp,
            injection_date_time_stamp,
            experiment_title,
            operator_name,
            separation_experiment_type,
            company_method_name,
            company_method_id,
            pre_experiment_program_name,
            post_experiment_program_name,
            source_file_reference,
            error_log,
        })
    }
}

fn read_scalar_var_f32(
    reader: &mut netcdf3::FileReader,
    var_name: &str,
) -> Result<Option<f32>, AndiError> {
    let var = reader.data_set().get_var(var_name);
    match var {
        Some(var) => {
            if var.len() != 1 {
                return Err(AndiError::new(&format!("{} not scalar", var_name)));
            }
            if var.data_type() != DataType::F32 {
                return Err(AndiError::new(&format!(
                    "{} unexpected data type: {}",
                    var_name,
                    var.data_type()
                )));
            }
            let val = reader.read_var_f32(var_name).unwrap()[0];
            Ok(Some(val))
        }
        None => Ok(None),
    }
}

#[derive(Debug)]
pub struct AndiChromSampleDescription {
    pub sample_id_comments: Option<String>,
    pub sample_id: Option<String>,
    pub sample_name: Option<String>,
    pub sample_type: Option<String>,
    pub sample_injection_volume: Option<f32>, // in ml
    pub sample_amount: Option<f32>,           // in mg
}

impl AndiChromSampleDescription {
    pub fn new(reader: &mut netcdf3::FileReader) -> Result<Self, AndiError> {
        let sample_id_comments = reader
            .data_set()
            .get_global_attr_as_string("sample_id_comments");
        let sample_id = reader.data_set().get_global_attr_as_string("sample_id");
        let sample_name = reader.data_set().get_global_attr_as_string("sample_name");
        let sample_type = reader.data_set().get_global_attr_as_string("sample_type");
        // TODO: if present in sample data, always stored as global attribute of either type string or float
        let sample_injection_volume = read_scalar_var_f32(reader, "sample_injection_volume")?;
        // TODO: if present in sample data, always stored as global attribute of either type string or float
        let sample_amount = read_scalar_var_f32(reader, "sample_amount")?;

        Ok(Self {
            sample_id_comments,
            sample_id,
            sample_name,
            sample_type,
            sample_injection_volume,
            sample_amount,
        })
    }
}

#[derive(Debug)]
pub struct AndiChromDetectionMethod {
    pub detection_method_table_name: Option<String>,
    pub detector_method_comments: Option<String>,
    pub detection_method_name: Option<String>,
    pub detector_name: Option<String>,
    pub detector_maximum_value: Option<f32>,
    pub detector_minimum_value: Option<f32>,
    pub detector_unit: Option<String>,
}

impl AndiChromDetectionMethod {
    pub fn new(reader: &mut netcdf3::FileReader) -> Result<Self, AndiError> {
        let detection_method_table_name = reader
            .data_set()
            .get_global_attr_as_string("detection_method_table_name");
        // "detector_method_comment" or "detector_method_comments"?
        // => sample files use "detector_method_comments"
        let detector_method_comments = reader
            .data_set()
            .get_global_attr_as_string("detector_method_comments");
        let detection_method_name = reader
            .data_set()
            .get_global_attr_as_string("detection_method_name");
        let detector_name = reader.data_set().get_global_attr_as_string("detector_name");
        let detector_maximum_value = read_scalar_var_f32(reader, "detector_maximum_value")?;
        let detector_minimum_value = read_scalar_var_f32(reader, "detector_minimum_value")?;
        let detector_unit = reader.data_set().get_global_attr_as_string("detector_unit");

        Ok(Self {
            detection_method_table_name,
            detector_method_comments,
            detection_method_name,
            detector_name,
            detector_maximum_value,
            detector_minimum_value,
            detector_unit,
        })
    }
}

#[derive(Debug)]
pub struct AndiChromRawData {
    pub point_number: i32, // required
    pub raw_data_table_name: Option<String>,
    pub retention_unit: String,               // required
    pub actual_run_time_length: f32,          // required
    pub actual_sampling_interval: f32,        // required
    pub actual_delay_time: f32,               // required
    pub ordinate_values: Vec<f32>,            // required
    pub uniform_sampling_flag: bool,          // required?, default: true
    pub raw_data_retention: Option<Vec<f32>>, // required if uniformSamplingFlag==false
    pub autosampler_position: Option<String>,
}

impl AndiChromRawData {
    pub fn new(reader: &mut netcdf3::FileReader) -> Result<Self, Box<dyn Error>> {
        let point_number_dim = reader
            .data_set()
            .get_dim("point_number")
            .ok_or(AndiError::new("Missing dataset_completeness dimension."))?;
        // TODO: usize?
        let point_number = point_number_dim.size() as i32;
        let raw_data_table_name = reader
            .data_set()
            .get_global_attr_as_string("raw_data_table_name");
        let retention_unit = reader
            .data_set()
            .get_global_attr_as_string("retention_unit")
            .ok_or(AndiError::new("Missing retention_unit attribute."))?;
        let actual_run_time_length = read_scalar_var_f32(reader, "actual_run_time_length")?
            .ok_or(AndiError::new("Missing actual_run_time_length variable."))?;
        let actual_sampling_interval = read_scalar_var_f32(reader, "actual_sampling_interval")?
            .ok_or(AndiError::new("Missing actual_sampling_interval variable."))?;
        let actual_delay_time = read_scalar_var_f32(reader, "actual_delay_time")?
            .ok_or(AndiError::new("Missing actual_delay_time variable."))?;
        // TODO: lazy load values
        let ordinate_values = reader
            .read_var("ordinate_values")?
            .get_f32()
            .ok_or(AndiError::new("Missing ordinate_values variable."))?
            .to_owned();

        let mut uniform_sampling_flag_attr = reader
            .data_set()
            .get_var_attr("ordinate_values", "uniform_sampling_flag");
        if uniform_sampling_flag_attr.is_none() {
            uniform_sampling_flag_attr = reader.data_set().get_global_attr("uniform_sampling_flag");
        }
        let uniform_sampling_flag = match uniform_sampling_flag_attr {
            Some(attr) => attr.get_as_string().unwrap_or("Y".to_owned()) == "Y",
            None => true,
        };
        // TODO: lazy load values
        let raw_data_retention = match uniform_sampling_flag {
            true => None,
            false => Some(
                reader
                    .read_var("raw_data_retention")?
                    .get_f32()
                    .ok_or(AndiError::new("Missing raw_data_retention variable."))?
                    .to_owned(),
            ),
        };
        let ordinate_values_var = reader.data_set().get_var("ordinate_values");
        let autosampler_position = match ordinate_values_var {
            None => None,
            Some(var) => var.get_attr_as_string("autosampler_position"),
        };

        Ok(Self {
            point_number,
            raw_data_table_name,
            retention_unit,
            actual_run_time_length,
            actual_sampling_interval,
            actual_delay_time,
            ordinate_values,
            uniform_sampling_flag,
            raw_data_retention,
            autosampler_position,
        })
    }
}

#[derive(Debug)]
pub struct AndiChromPeakProcessingResults {
    pub peak_number: i32,
    pub peak_processing_results_table_name: Option<String>,
    pub peak_processing_results_comments: Option<String>,
    pub peak_processing_method_name: Option<String>,
    pub peak_processing_date_time_stamp: Option<String>,
    pub peak_amount_unit: Option<String>,
    pub peaks: Option<Vec<AndiChromPeak>>,
}

impl AndiChromPeakProcessingResults {
    pub fn new(
        reader: &mut netcdf3::FileReader,
        peak_retention_unit: &str,
        detector_unit: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        let peak_number_dim = reader.data_set().get_dim("peak_number");
        let peak_number = match peak_number_dim {
            // TODO: usize?
            Some(dim) => dim.size() as i32,
            None => 0,
        };
        let peak_processing_results_table_name = reader
            .data_set()
            .get_global_attr_as_string("peak_processing_results_table_name");
        let peak_processing_results_comments = reader
            .data_set()
            .get_global_attr_as_string("peak_processing_results_comments");
        let peak_processing_method_name = reader
            .data_set()
            .get_global_attr_as_string("peak_processing_method_name");
        let peak_processing_date_time_stamp = reader
            .data_set()
            .get_global_attr_as_string("peak_processing_date_time_stamp");
        let peak_amount_unit = reader
            .data_set()
            .get_global_attr_as_string("peak_amount_unit");
        let peaks = Self::read_peaks(
            reader,
            peak_number,
            peak_retention_unit,
            peak_amount_unit.as_deref(),
            detector_unit,
        )?;

        Ok(Self {
            peak_number,
            peak_processing_results_table_name,
            peak_processing_results_comments,
            peak_processing_method_name,
            peak_processing_date_time_stamp,
            peak_amount_unit,
            peaks,
        })
    }

    fn read_peaks(
        reader: &mut netcdf3::FileReader,
        peak_number: i32,
        peak_retention_unit: &str,
        peak_amount_unit: Option<&str>,
        detector_unit: Option<&str>,
    ) -> Result<Option<Vec<AndiChromPeak>>, Box<dyn Error>> {
        if peak_number <= 0 {
            return Ok(None);
        }

        // As the netcdf3 library (currently) does not support indexed reads,
        // read underlying arrays as a whole and populate peak here instead of using a dedicated new().

        let peak_retention_time_var = read_optional_var(reader, "peak_retention_time")?;
        let peak_name_var = read_optional_var(reader, "peak_name")?;
        let peak_amount_var = read_optional_var(reader, "peak_amount")?;
        let peak_start_time_var = read_optional_var(reader, "peak_start_time")?;
        let peak_end_time_var = read_optional_var(reader, "peak_end_time")?;
        let peak_width_var = read_optional_var(reader, "peak_width")?;
        let peak_area_var = read_optional_var(reader, "peak_area")?;
        let peak_area_percent_var = read_optional_var(reader, "peak_area_percent")?;
        let peak_height_var = read_optional_var(reader, "peak_height")?;
        let peak_height_percent_var = read_optional_var(reader, "peak_height_percent")?;
        let baseline_start_time_var = read_optional_var(reader, "baseline_start_time")?;
        let baseline_start_value_var = read_optional_var(reader, "baseline_start_value")?;
        let baseline_stop_time_var = read_optional_var(reader, "baseline_stop_time")?;
        let baseline_stop_value_var = read_optional_var(reader, "baseline_stop_value")?;
        let peak_start_detection_code_var = read_optional_var(reader, "peak_start_detection_code")?;
        let peak_stop_detection_code_var = read_optional_var(reader, "peak_stop_detection_code")?;
        let retention_index_var = read_optional_var(reader, "retention_index")?;
        let migration_time_var = read_optional_var(reader, "migration_time")?;
        let peak_asymmetry_var = read_optional_var(reader, "peak_asymmetry")?;
        let peak_efficiency_var = read_optional_var(reader, "peak_efficiency")?;
        let mass_on_column_var = read_optional_var(reader, "mass_on_column")?;
        let manually_reintegrated_peaks_var =
            read_optional_var(reader, "manually_reintegrated_peaks")?;

        let mut peaks = Vec::<AndiChromPeak>::new();
        for i in 0..peak_number as usize {
            let peak_retention_time = read_index_from_var_f32(&peak_retention_time_var, i)?;
            let peak_name = read_index_from_var_2d_string(&peak_name_var, i)?;
            let peak_amount = read_index_from_var_f32(&peak_amount_var, i)?;
            let peak_start_time = read_index_from_var_f32(&peak_start_time_var, i)?;
            let peak_end_time = read_index_from_var_f32(&peak_end_time_var, i)?;
            let peak_width = read_index_from_var_f32(&peak_width_var, i)?;
            let peak_area = read_index_from_var_f32(&peak_area_var, i)?;
            let peak_area_percent = read_index_from_var_f32(&peak_area_percent_var, i)?;
            let peak_height = read_index_from_var_f32(&peak_height_var, i)?;
            let peak_height_percent = read_index_from_var_f32(&peak_height_percent_var, i)?;
            let baseline_start_time = read_index_from_var_f32(&baseline_start_time_var, i)?;
            let baseline_start_value = read_index_from_var_f32(&baseline_start_value_var, i)?;
            let baseline_stop_time = read_index_from_var_f32(&baseline_stop_time_var, i)?;
            let baseline_stop_value = read_index_from_var_f32(&baseline_stop_value_var, i)?;
            let peak_start_detection_code =
                read_index_from_var_2d_string(&peak_start_detection_code_var, i)?;
            let peak_stop_detection_code =
                read_index_from_var_2d_string(&peak_stop_detection_code_var, i)?;
            let retention_index = read_index_from_var_f32(&retention_index_var, i)?;
            let migration_time = read_index_from_var_f32(&migration_time_var, i)?;
            let peak_asymmetry = read_index_from_var_f32(&peak_asymmetry_var, i)?;
            let peak_efficiency = read_index_from_var_f32(&peak_efficiency_var, i)?;
            let mass_on_column = read_index_from_var_f32(&mass_on_column_var, i)?;

            let manually_reintegrated_peaks = read_index_from_slice(
                manually_reintegrated_peaks_var
                    .as_ref()
                    .map(|(_, _, v)| v.get_i16())
                    .flatten(),
                manually_reintegrated_peaks_var
                    .as_ref()
                    .map(|(name, _, _)| *name)
                    .unwrap_or_default(),
                i,
            )?
            .map(|reint| reint != &0)
            .unwrap_or(false);

            let peak = AndiChromPeak {
                peak_retention_time,
                peak_name,
                peak_amount,
                peak_start_time,
                peak_end_time,
                peak_width,
                peak_area,
                peak_area_percent,
                peak_height,
                peak_height_percent,
                baseline_start_time,
                baseline_start_value,
                baseline_stop_time,
                baseline_stop_value,
                peak_start_detection_code,
                peak_stop_detection_code,
                retention_index,
                migration_time,
                peak_asymmetry,
                peak_efficiency,
                mass_on_column,
                manually_reintegrated_peaks,

                peak_retention_unit: peak_retention_unit.to_owned(),
                peak_amount_unit: peak_amount_unit.map(|x| x.to_owned()),
                detector_unit: detector_unit.map(|x| x.to_owned()),
            };
            peaks.push(peak);
        }

        Ok(Some(peaks))
    }
}

#[derive(Debug)]
pub struct AndiChromPeak {
    pub peak_retention_time: Option<f32>,
    pub peak_name: Option<String>,
    pub peak_amount: Option<f32>,
    pub peak_start_time: Option<f32>,
    pub peak_end_time: Option<f32>,
    pub peak_width: Option<f32>,
    pub peak_area: Option<f32>,
    pub peak_area_percent: Option<f32>,
    pub peak_height: Option<f32>,
    pub peak_height_percent: Option<f32>,
    pub baseline_start_time: Option<f32>,
    pub baseline_start_value: Option<f32>,
    pub baseline_stop_time: Option<f32>,
    pub baseline_stop_value: Option<f32>,
    pub peak_start_detection_code: Option<String>,
    pub peak_stop_detection_code: Option<String>,
    pub retention_index: Option<f32>,
    pub migration_time: Option<f32>,
    pub peak_asymmetry: Option<f32>,
    pub peak_efficiency: Option<f32>,
    pub mass_on_column: Option<f32>,
    pub manually_reintegrated_peaks: bool,

    pub peak_retention_unit: String,
    pub peak_amount_unit: Option<String>,
    pub detector_unit: Option<String>,
}

// TODO: needed?
// pub struct AndiNonStandardVariables {}

// pub struct AndiNonStandardAttributes {}
