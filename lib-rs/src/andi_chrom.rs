use std::collections::BTreeSet;

use crate::{api::{self, SciReader}, andi::AndiCategory};

struct AndiChromReader {}

impl SciReader<AndiChromFile> for AndiChromReader {
    fn read(input: Box<dyn api::SeekRead>) -> Result<AndiChromFile, Box<dyn std::error::Error>> {
        todo!()
    }
}

struct AndiChromFile {
  pub admin_data: AndiChromAdminData,
  pub sample_description: AndiChromSampleDescription,
  pub detection_method: AndiChromDetectionMethod,
  pub raw_data: AndiChromRawData,
  pub peak_processing_data: AndiChromPeakProcessingResults,
  pub non_standard_variables: AndiNonStandardVariables,
  pub non_standard_attributes: AndiNonStandardAttributes,
}

struct AndiChromAdminData {
  dataset_completeness: BTreeSet<AndiCategory>, // required
  protocol_template_revision: String,           // required
  netcdf_revision: String,                      // required
  languages: String,
  administrative_comments: String,
  dataset_origin: String,
  dataset_owner: String,
  dataset_date_time_stamp: String,
  injection_date_time_stamp: String,            // required
  experiment_title: String,
  operator_name: String,
  separation_experiment_type: String,
  company_method_name: String,
  company_method_id: String,
  pre_expt_program_name: String,
  post_expt_program_name: String,
  source_file_reference: String,
  error_log: Vec<String>,
}

struct AndiChromSampleDescription {}

struct AndiChromDetectionMethod {}

struct AndiChromRawData {}

struct AndiChromPeakProcessingResults {}

struct AndiNonStandardVariables {}

struct AndiNonStandardAttributes {}
