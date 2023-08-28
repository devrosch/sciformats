use crate::api::{self, SciReader};

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

struct AndiChromAdminData {}

struct AndiChromSampleDescription {}

struct AndiChromDetectionMethod {}

struct AndiChromRawData {}

struct AndiChromPeakProcessingResults {}

struct AndiNonStandardVariables {}

struct AndiNonStandardAttributes {}
