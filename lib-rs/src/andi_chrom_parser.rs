use super::andi_utils::{
    read_index_from_slice, read_index_from_var_2d_string, read_index_from_var_f32,
    read_multi_string_var, read_optional_var,
};
use crate::{
    andi::{AndiDatasetCompleteness, AndiError},
    api::Parser,
};
use netcdf3::DataType;
use std::{
    error::Error,
    io::{Read, Seek},
    str::FromStr,
};

pub struct AndiChromParser {}

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
