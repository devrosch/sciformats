use std::str::FromStr;

use netcdf3::DataType;

use crate::{
    andi::{AndiDatasetCompleteness, AndiError},
    api::{self, SciReader},
    FileWrapper,
};

pub struct AndiChromReader {}

// impl SciReader<AndiChromFile> for AndiChromReader {
//     fn read(input: Box<dyn api::SeekRead>) -> Result<AndiChromFile, Box<dyn std::error::Error>> {
//         netcdf3::FileReader::open_seek_read("input_file_name", input);
//         todo!()
//     }
// }

impl SciReader<FileWrapper, AndiChromFile> for AndiChromReader {
    fn read(name: &str, input: FileWrapper) -> Result<AndiChromFile, Box<dyn std::error::Error>> {
        let input_seek_read = Box::new(input);
        let mut reader = netcdf3::FileReader::open_seek_read(name, input_seek_read)?;

        let admin_data = AndiChromAdminData::new(&reader)?;
        let sample_description = AndiChromSampleDescription::new(&mut reader)?;
        let detection_method = AndiChromDetectionMethod::new(&mut reader)?;
        todo!()
    }
}

pub struct AndiChromFile {
    pub admin_data: AndiChromAdminData,
    pub sample_description: AndiChromSampleDescription,
    pub detection_method: AndiChromDetectionMethod,
    pub raw_data: AndiChromRawData,
    pub peak_processing_data: AndiChromPeakProcessingResults,
    pub non_standard_variables: Vec<String>,
    pub non_standard_attributes: Vec<String>,
}

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
    pub fn new(reader: &netcdf3::FileReader) -> Result<AndiChromAdminData, AndiError> {
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

        Ok(AndiChromAdminData {
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
            error_log: vec![], // TODO: read error_log
        })

        // TODO: read error_log

        // let var = reader.data_set().get_var("error_log").unwrap();
        // let dims = var.get_dims();
        // let dim_0 = dims.get(0).unwrap();
        // let dim_1 = dims.get(1).unwrap();
        // var.use_dim("dim_name");
        // let error_log_var = reader.read_var("error_log");
        // let err_log = error_log_var.unwrap().get_u8().unwrap();

        // Variable errorLog = file.findVariable(file.getRootGroup(), "error_log");
        // if (errorLog == null) {
        //     return null;
        // }
        // List<String> errorLogEntries = new ArrayList<>();
        // int numberOfEntries = errorLog.getDimension(0).getLength();
        // for (int i = 0; i < numberOfEntries; i++) {
        //     String entry = readMultipleStringVariable(file, "error_log", i);
        //     errorLogEntries.add(entry);
        // }
        // return errorLogEntries;

        // this.errorLog = readErrorLog(file);
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

pub struct AndiChromSampleDescription {
    pub sample_id_comments: Option<String>,
    pub sample_id: Option<String>,
    pub sample_name: Option<String>,
    pub sample_type: Option<String>,
    pub sample_injection_volume: Option<f32>, // in ml
    pub sample_amount: Option<f32>,           // in mg
}

impl AndiChromSampleDescription {
    pub fn new(reader: &mut netcdf3::FileReader) -> Result<AndiChromSampleDescription, AndiError> {
        let sample_id_comments = reader
            .data_set()
            .get_global_attr_as_string("sample_id_comments");
        let sample_id = reader.data_set().get_global_attr_as_string("sample_id");
        let sample_name = reader.data_set().get_global_attr_as_string("sample_name");
        let sample_type = reader.data_set().get_global_attr_as_string("sample_type");
        let sample_injection_volume = read_scalar_var_f32(reader, "sample_injection_volume")?;
        let sample_amount = read_scalar_var_f32(reader, "sample_amount")?;

        Ok(AndiChromSampleDescription {
            sample_id_comments,
            sample_id,
            sample_name,
            sample_type,
            sample_injection_volume,
            sample_amount,
        })
    }
}

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
    pub fn new(reader: &mut netcdf3::FileReader) -> Result<AndiChromDetectionMethod, AndiError> {
        let detection_method_table_name = reader
            .data_set()
            .get_global_attr_as_string("detection_method_table_name");
        // TODO: detector_method_comment or detector_method_comments?
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

        Ok(AndiChromDetectionMethod {
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

pub struct AndiChromRawData {
    point_number: i32, // required
    raw_data_table_name: String,
    retention_unit: String,        // required
    actual_run_time_length: f32,   // required
    actual_sampling_interval: f32, // required
    actual_delay_time: f32,        // required
    ordinate_values: Vec<f32>,     // required
    uniform_sampling_flag: bool,   // required?, default: true
    raw_data_retention: Vec<f32>,  // required if uniformSamplingFlag==false
    autosampler_position: String,
}

pub struct AndiChromPeakProcessingResults {
    peak_number: i32,
    peak_processing_results_table_name: String,
    peak_processing_results_comments: String,
    peak_processing_method_name: String,
    peak_processing_date_time_stamp: String,
    peak_amount_unit: String,
    peaks: Vec<AndiChromPeak>,
}

// TODO: needed?
// pub struct AndiNonStandardVariables {}

// pub struct AndiNonStandardAttributes {}

pub struct AndiChromPeak {
    peak_retention_time: f32,
    peak_name: String,
    peak_amount: f32,
    peak_start_time: f32,
    peak_end_time: f32,
    peak_width: f32,
    peak_area: f32,
    peak_area_percent: f32,
    peak_height: f32,
    peak_height_percent: f32,
    baseline_start_time: f32,
    baseline_start_value: f32,
    baseline_stop_time: f32,
    baseline_stop_value: f32,
    peak_start_detection_code: String,
    peak_stop_detection_code: String,
    retention_index: f32,
    migration_time: f32,
    peak_asymmetry: f32,
    peak_efficiency: f32,
    mass_on_column: f32,
    manually_reintegrated_peak: bool,

    peak_retention_unit: String,
    peak_amount_unit: String,
    detector_unit: String,
}
