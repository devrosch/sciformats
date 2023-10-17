use netcdf3::Variable;

use super::andi_enums::{
    AndiMsDataFormat, AndiMsDetectorType, AndiMsExperimentType, AndiMsIntensityAxisUnit,
    AndiMsIonizationMethod, AndiMsIonizationPolarity, AndiMsMassAxisUnit,
    AndiMsMassSpectrometerInlet, AndiMsResolutionType, AndiMsSampleState, AndiMsScanDirection,
    AndiMsScanFunction, AndiMsScanLaw, AndiMsSeparationMethod, AndiMsTimeAxisUnit,
};
use super::andi_utils::{
    read_enum_from_global_str_attr, read_global_attr_f64, read_global_attr_i16,
    read_global_f32_attr, read_global_i32_attr, read_global_str_attr, read_multi_string_var,
    trim_zeros_in_place,
};
use super::{AndiDatasetCompleteness, AndiError};
use crate::api::Parser;
use std::{
    error::Error,
    io::{Read, Seek},
    str::FromStr,
};

pub struct AndiMsParser {}

impl<T: Seek + Read + 'static> Parser<T> for AndiMsParser {
    type R = AndiMsFile;

    fn parse(name: &str, input: T) -> Result<Self::R, Box<dyn std::error::Error>> {
        let input_seek_read = Box::new(input);
        let reader = netcdf3::FileReader::open_seek_read(name, input_seek_read)?;

        AndiMsFile::new(reader)
    }
}

#[derive(Debug)]
pub struct AndiMsFile {
    pub admin_data: AndiMsAdminData,
    pub instrument_data: AndiMsInstrumentData,
    pub sample_data: AndiMsSampleData,
    pub test_data: AndiMsTestData,
    // pub sample_description: AndiMsSampleDescription,
    // pub detection_method: AndiMsDetectionMethod,
    // pub raw_data: AndiMsRawData,
    // pub peak_processing_results: AndiMsPeakProcessingResults,
    // pub non_standard_variables: Vec<String>,
    // pub non_standard_attributes: Vec<String>,
}

impl AndiMsFile {
    pub fn new(mut reader: netcdf3::FileReader) -> Result<Self, Box<dyn std::error::Error>> {
        let admin_data = AndiMsAdminData::new(&mut reader)?;
        let instrument_data = AndiMsInstrumentData::new(&mut reader, admin_data.instrument_number)?;
        let sample_data = AndiMsSampleData::new(&reader)?;
        let test_data = AndiMsTestData::new(&reader)?;
        // let sample_description = AndiMsSampleDescription::new(&mut reader)?;
        // let detection_method = AndiMsDetectionMethod::new(&mut reader)?;

        // let reader_ref: Rc<RefCell<netcdf3::FileReader>> = Rc::new(RefCell::new(reader));

        // let raw_data = AndiMsRawData::new(Rc::clone(&reader_ref))?;
        // let peak_processing_results = AndiMsPeakProcessingResults::new(
        //     reader_ref,
        //     &raw_data.retention_unit,
        //     detection_method.detector_unit.as_deref(),
        // )?;

        Ok(Self {
            admin_data,
            instrument_data,
            sample_data,
            test_data,
            // sample_description,
            // detection_method,
            // raw_data,
            // peak_processing_results,
            // // TODO: read
            // non_standard_variables: vec![],
            // // TODO: read
            // non_standard_attributes: vec![],
        })
    }
}

#[derive(Debug)]
pub struct AndiMsAdminData {
    pub dataset_completeness: AndiDatasetCompleteness, // required
    pub ms_template_revision: String,                  // required
    pub administrative_comments: Option<String>,
    pub dataset_origin: Option<String>,
    pub dataset_owner: Option<String>,
    pub experiment_title: Option<String>,
    pub experiment_date_time_stamp: String, // required
    pub experiment_type: AndiMsExperimentType,
    pub experiment_x_ref_0: Option<String>,
    pub experiment_x_ref_1: Option<String>,
    pub experiment_x_ref_2: Option<String>,
    pub experiment_x_ref_3: Option<String>,
    pub netcdf_file_date_time_stamp: String, // required
    pub netcdf_revision: String,             // required
    pub operator_name: Option<String>,
    pub source_file_reference: Option<String>,
    pub source_file_format: Option<String>,
    pub source_file_date_time_stamp: Option<String>,
    pub external_file_ref_0: Option<String>,
    pub external_file_ref_1: Option<String>,
    pub external_file_ref_2: Option<String>,
    pub external_file_ref_3: Option<String>,
    pub languages: String, // required
    // TODO: really i32?
    pub number_of_times_processed: Option<i32>,
    pub number_of_times_calibrated: Option<i32>,
    pub calibration_history_0: Option<String>,
    pub calibration_history_1: Option<String>,
    pub calibration_history_2: Option<String>,
    pub calibration_history_3: Option<String>,
    pub pre_experiment_program_name: Option<String>,
    pub post_experiment_program_name: Option<String>,
    pub error_log: Vec<String>,
    // TODO: really i32?
    pub instrument_number: Option<i32>,
}

impl AndiMsAdminData {
    pub fn new(reader: &mut netcdf3::FileReader) -> Result<Self, Box<dyn Error>> {
        let dataset_completeness_attr = read_global_str_attr(reader, "dataset_completeness")
            .ok_or(AndiError::new("Missing dataset_completeness attribute."))?;
        let dataset_completeness = AndiDatasetCompleteness::from_str(&dataset_completeness_attr)?;
        let ms_template_revision = read_global_str_attr(reader, "ms_template_revision")
            .ok_or(AndiError::new("Missing ms_template_revision attribute."))?;
        let administrative_comments = read_global_str_attr(reader, "administrative_comments");
        let dataset_origin = read_global_str_attr(reader, "dataset_origin");
        let dataset_owner = read_global_str_attr(reader, "dataset_owner");
        let experiment_title = read_global_str_attr(reader, "experiment_title");
        let experiment_date_time_stamp = read_global_str_attr(reader, "experiment_date_time_stamp")
            .ok_or(AndiError::new(
                "Missing experiment_date_time_stamp attribute.",
            ))?;
        // let experiment_type = read_global_str_attr(reader, "experiment_type")
        //     .map_or(Ok(AndiMsExperimentType::default()), |s| {
        //         AndiMsExperimentType::from_str(&s)
        //     })?;
        let experiment_type = read_global_str_attr(reader, "experiment_type").map_or(
            Ok(AndiMsExperimentType::default()),
            |s| {
                AndiMsExperimentType::from_str(&s).or(Err(AndiError::new(&format!(
                    "Illegal experiment_type: {}",
                    s
                ))))
            },
        )?;
        let experiment_x_ref_0 = read_global_str_attr(reader, "experiment_x_ref_0");
        let experiment_x_ref_1 = read_global_str_attr(reader, "experiment_x_ref_1");
        let experiment_x_ref_2 = read_global_str_attr(reader, "experiment_x_ref_2");
        let experiment_x_ref_3 = read_global_str_attr(reader, "experiment_x_ref_3");
        let netcdf_file_date_time_stamp =
            read_global_str_attr(reader, "netcdf_file_date_time_stamp").ok_or(AndiError::new(
                "Missing netcdf_file_date_time_stamp attribute.",
            ))?;
        let netcdf_revision = read_global_str_attr(reader, "netcdf_revision")
            .ok_or(AndiError::new("Missing netcdf_revision attribute."))?;
        let operator_name = read_global_str_attr(reader, "operator_name");
        let source_file_reference = read_global_str_attr(reader, "source_file_reference");
        let source_file_format = read_global_str_attr(reader, "source_file_format");
        let source_file_date_time_stamp =
            read_global_str_attr(reader, "source_file_date_time_stamp");
        let external_file_ref_0 = read_global_str_attr(reader, "external_file_ref_0");
        let external_file_ref_1 = read_global_str_attr(reader, "external_file_ref_1");
        let external_file_ref_2 = read_global_str_attr(reader, "external_file_ref_2");
        let external_file_ref_3 = read_global_str_attr(reader, "external_file_ref_3");
        let languages = read_global_str_attr(reader, "languages")
            .ok_or(AndiError::new("Missing languages attribute."))?;
        let number_of_times_processed = read_global_i32_attr(reader, "number_of_times_processed")?;
        let number_of_times_calibrated =
            read_global_i32_attr(reader, "number_of_times_calibrated")?;
        let calibration_history_0 = read_global_str_attr(reader, "calibration_history_0");
        let calibration_history_1 = read_global_str_attr(reader, "calibration_history_1");
        let calibration_history_2 = read_global_str_attr(reader, "calibration_history_2");
        let calibration_history_3 = read_global_str_attr(reader, "calibration_history_3");
        let pre_experiment_program_name =
            read_global_str_attr(reader, "pre_experiment_program_name");
        let post_experiment_program_name =
            read_global_str_attr(reader, "post_experiment_program_name");
        let error_log = read_multi_string_var(reader, "error_log")?;
        // TODO: really i32?
        let instrument_number = reader
            .data_set()
            .get_dim("instrument_number")
            .map(|dim| dim.size() as i32);

        // TODO: continue

        Ok(Self {
            dataset_completeness,
            ms_template_revision,
            administrative_comments,
            dataset_origin,
            dataset_owner,
            experiment_title,
            experiment_date_time_stamp,
            experiment_type,
            experiment_x_ref_0,
            experiment_x_ref_1,
            experiment_x_ref_2,
            experiment_x_ref_3,
            netcdf_file_date_time_stamp,
            netcdf_revision,
            operator_name,
            source_file_reference,
            source_file_format,
            source_file_date_time_stamp,
            external_file_ref_0,
            external_file_ref_1,
            external_file_ref_2,
            external_file_ref_3,
            languages,
            number_of_times_processed,
            number_of_times_calibrated,
            calibration_history_0,
            calibration_history_1,
            calibration_history_2,
            calibration_history_3,
            pre_experiment_program_name,
            post_experiment_program_name,
            error_log,
            instrument_number,
        })
    }
}

#[derive(Debug)]
pub struct AndiMsInstrumentComponent {
    // pub instrument_number: i32, // not necessary
    pub instrument_name: Option<String>,
    pub instrument_id: Option<String>,
    pub instrument_mfr: Option<String>,
    pub instrument_model: Option<String>,
    pub instrument_serial_no: Option<String>,
    pub instrument_comments: Option<String>,
    pub instrument_sw_version: Option<String>,
    pub instrument_fw_version: Option<String>,
    pub instrument_os_version: Option<String>,
    pub instrument_app_version: Option<String>,
}

#[derive(Debug)]
pub struct AndiMsInstrumentData {
    pub instrument_components: Vec<AndiMsInstrumentComponent>,
}

impl AndiMsInstrumentData {
    pub fn new(
        reader: &mut netcdf3::FileReader,
        instrument_number: Option<i32>,
    ) -> Result<Self, Box<dyn Error>> {
        if instrument_number.is_none() {
            return Ok(Self {
                instrument_components: vec![],
            });
        }

        let instrument_names = read_multi_string_var(reader, "instrument_name")?;
        let instrument_ids = read_multi_string_var(reader, "instrument_id")?;
        let instrument_mfrs = read_multi_string_var(reader, "instrument_mfr")?;
        let instrument_models = read_multi_string_var(reader, "instrument_model")?;
        let instrument_serial_nos = read_multi_string_var(reader, "instrument_serial_no")?;
        let instrument_comments_list = read_multi_string_var(reader, "instrument_comments")?;
        let instrument_sw_versions = read_multi_string_var(reader, "instrument_sw_version")?;
        let instrument_fw_versions = read_multi_string_var(reader, "instrument_fw_version")?;
        let instrument_os_versions = read_multi_string_var(reader, "instrument_os_version")?;
        let instrument_app_versions = read_multi_string_var(reader, "instrument_app_version")?;

        fn get_value_from_index<T: AsRef<str>>(
            items: &[T],
            var_name: &str,
            index: usize,
        ) -> Result<Option<String>, AndiError> {
            if items.is_empty() {
                return Ok(None);
            }
            match items.get(index) {
                Some(item) => Ok(Some(item.as_ref().to_owned())),
                None => Err(AndiError::new(&format!(
                    "Missing element for {} at index: {}",
                    var_name, index
                ))),
            }
        }

        let mut instrument_components: Vec<AndiMsInstrumentComponent> = vec![];
        for i in 0..instrument_number.unwrap() as usize {
            let instrument_name = get_value_from_index(&instrument_names, "instrument_name", i)?;
            let instrument_id = get_value_from_index(&instrument_ids, "instrument_id", i)?;
            let instrument_mfr = get_value_from_index(&instrument_mfrs, "instrument_mfr", i)?;
            let instrument_model = get_value_from_index(&instrument_models, "instrument_model", i)?;
            let instrument_serial_no =
                get_value_from_index(&instrument_serial_nos, "instrument_serial_no", i)?;
            let instrument_comments =
                get_value_from_index(&instrument_comments_list, "instrument_comments", i)?;
            let instrument_sw_version =
                get_value_from_index(&instrument_sw_versions, "instrument_sw_version", i)?;
            let instrument_fw_version =
                get_value_from_index(&instrument_fw_versions, "instrument_fw_version", i)?;
            let instrument_os_version =
                get_value_from_index(&instrument_os_versions, "instrument_os_version", i)?;
            let instrument_app_version =
                get_value_from_index(&instrument_app_versions, "instrument_app_version", i)?;

            instrument_components.push(AndiMsInstrumentComponent {
                instrument_name,
                instrument_id,
                instrument_mfr,
                instrument_model,
                instrument_serial_no,
                instrument_comments,
                instrument_sw_version,
                instrument_fw_version,
                instrument_os_version,
                instrument_app_version,
            })
        }

        Ok(Self {
            instrument_components,
        })
    }
}

#[derive(Debug)]
pub struct AndiMsSampleData {
    pub sample_owner: Option<String>,
    pub sample_receipt_date_time_stamp: Option<String>,
    pub sample_internal_id: Option<String>,
    pub sample_external_id: Option<String>,
    pub sample_procedure_name: Option<String>,
    pub sample_prep_procedure: Option<String>,
    pub sample_state: AndiMsSampleState,
    pub sample_matrix: Option<String>,
    pub sample_storage: Option<String>,
    pub sample_disposal: Option<String>,
    pub sample_history: Option<String>,
    pub sample_prep_comments: Option<String>,
    pub sample_comments: Option<String>,
    pub sample_manual_handling: Option<String>,
}

impl AndiMsSampleData {
    pub fn new(reader: &netcdf3::FileReader) -> Result<Self, Box<dyn Error>> {
        let sample_owner = read_global_str_attr(reader, "sample_owner");
        let sample_receipt_date_time_stamp =
            read_global_str_attr(reader, "sample_receipt_date_time_stamp");
        let sample_internal_id = read_global_str_attr(reader, "sample_internal_id");
        let sample_external_id = read_global_str_attr(reader, "sample_external_id");
        let sample_procedure_name = read_global_str_attr(reader, "sample_procedure_name");
        let sample_prep_procedure = read_global_str_attr(reader, "sample_prep_procedure");
        // let sample_state = read_global_str_attr(reader, "sample_state")
        //     .map_or(Ok(AndiMsSampleState::default()), |s| {
        //         AndiMsSampleState::from_str(&s)
        //     })?;
        let sample_state = read_global_str_attr(reader, "sample_state").map_or(
            Ok(AndiMsSampleState::default()),
            |s| {
                AndiMsSampleState::from_str(&s)
                    .or(Err(AndiError::new(&format!("Illegal sample_state: {}", s))))
            },
        )?;
        let sample_matrix = read_global_str_attr(reader, "sample_matrix");
        let sample_storage = read_global_str_attr(reader, "sample_storage");
        let sample_disposal = read_global_str_attr(reader, "sample_disposal");
        let sample_history = read_global_str_attr(reader, "sample_history");
        let sample_prep_comments = read_global_str_attr(reader, "sample_prep_comments");
        let sample_comments = read_global_str_attr(reader, "sample_comments");
        let sample_manual_handling = read_global_str_attr(reader, "sample_manual_handling");

        Ok(Self {
            sample_owner,
            sample_receipt_date_time_stamp,
            sample_internal_id,
            sample_external_id,
            sample_procedure_name,
            sample_prep_procedure,
            sample_state,
            sample_matrix,
            sample_storage,
            sample_disposal,
            sample_history,
            sample_prep_comments,
            sample_comments,
            sample_manual_handling,
        })
    }
}

#[derive(Debug)]
pub struct AndiMsTestData {
    pub separation_experiment_type: AndiMsSeparationMethod,
    pub mass_spectrometer_inlet: AndiMsMassSpectrometerInlet,
    /// in °C
    pub mass_spectrometer_inlet_temperature: Option<f32>,
    pub ionization_mode: AndiMsIonizationMethod,
    pub ionization_polarity: AndiMsIonizationPolarity,
    /// in V
    pub electron_energy: Option<f32>,
    /// in nm
    pub laser_wavelength: Option<f32>,
    pub reagent_gas: Option<String>,
    /// in ???
    pub reagent_gas_pressure: Option<f32>,
    pub fab_type: Option<String>,
    pub fab_matrix: Option<String>,
    /// in °C
    pub source_temperature: Option<f32>,
    /// in A
    pub filament_current: Option<f32>,
    /// in µA
    pub emission_current: Option<f32>,
    /// in V
    pub accelerating_potential: Option<f32>,
    pub detector_type: AndiMsDetectorType,
    /// in V
    pub detector_potential: Option<f32>,
    /// in V
    pub detector_entrance_potential: Option<f32>,
    pub resolution_type: AndiMsResolutionType,
    pub resolution_method: Option<String>,
    pub scan_function: AndiMsScanFunction,
    pub scan_direction: AndiMsScanDirection,
    pub scan_law: AndiMsScanLaw,
    /// in s
    pub scan_time: Option<f32>,
    pub mass_calibration_file_name: Option<String>,
    pub external_reference_file_name: Option<String>,
    pub internal_reference_file_name: Option<String>,
    pub instrument_parameter_comments: Option<String>,
}

impl AndiMsTestData {
    pub fn new(reader: &netcdf3::FileReader) -> Result<Self, Box<dyn Error>> {
        let separation_experiment_type = read_enum_from_global_str_attr::<AndiMsSeparationMethod>(
            reader,
            "test_separation_type",
        )?;
        let mass_spectrometer_inlet =
            read_enum_from_global_str_attr::<AndiMsMassSpectrometerInlet>(reader, "test_ms_inlet")?;
        let mass_spectrometer_inlet_temperature =
            read_global_f32_attr(reader, "test_ms_inlet_temperature")?;
        let ionization_mode = read_enum_from_global_str_attr::<AndiMsIonizationMethod>(
            reader,
            "test_ionization_mode",
        )?;
        let ionization_polarity = read_enum_from_global_str_attr::<AndiMsIonizationPolarity>(
            reader,
            "test_ionization_polarity",
        )?;
        let electron_energy = read_global_f32_attr(reader, "test_electron_energy")?;
        let laser_wavelength = read_global_f32_attr(reader, "test_laser_wavelength")?;
        let reagent_gas = read_global_str_attr(reader, "test_reagent_gas");
        let reagent_gas_pressure = read_global_f32_attr(reader, "test_reagent_gas_pressure")?;
        let fab_type = read_global_str_attr(reader, "test_fab_type");
        let fab_matrix = read_global_str_attr(reader, "test_fab_matrix");
        let source_temperature = read_global_f32_attr(reader, "test_source_temperature")?;
        let filament_current = read_global_f32_attr(reader, "test_filament_current")?;
        let emission_current = read_global_f32_attr(reader, "test_emission_current")?;
        let accelerating_potential = read_global_f32_attr(reader, "test_accelerating_potential")?;
        let detector_type =
            read_enum_from_global_str_attr::<AndiMsDetectorType>(reader, "test_detector_type")?;
        let detector_potential = read_global_f32_attr(reader, "test_detector_potential")?;
        let detector_entrance_potential =
            read_global_f32_attr(reader, "test_detector_entrance_potential")?;
        let resolution_type =
            read_enum_from_global_str_attr::<AndiMsResolutionType>(reader, "test_resolution_type")?;
        let resolution_method = read_global_str_attr(reader, "test_resolution_method");
        let scan_function =
            read_enum_from_global_str_attr::<AndiMsScanFunction>(reader, "test_scan_function")?;
        let scan_direction =
            read_enum_from_global_str_attr::<AndiMsScanDirection>(reader, "test_scan_direction")?;
        let scan_law = read_enum_from_global_str_attr::<AndiMsScanLaw>(reader, "test_scan_law")?;
        let scan_time = read_global_f32_attr(reader, "test_scan_time")?;
        let mass_calibration_file_name = read_global_str_attr(reader, "mass_calibration_file");
        let external_reference_file_name =
            read_global_str_attr(reader, "test_external_reference_file");
        let internal_reference_file_name =
            read_global_str_attr(reader, "test_internal_reference_file");
        let instrument_parameter_comments = read_global_str_attr(reader, "test_comments");

        Ok(Self {
            separation_experiment_type,
            mass_spectrometer_inlet,
            mass_spectrometer_inlet_temperature,
            ionization_mode,
            ionization_polarity,
            electron_energy,
            laser_wavelength,
            reagent_gas,
            reagent_gas_pressure,
            fab_type,
            fab_matrix,
            source_temperature,
            filament_current,
            emission_current,
            accelerating_potential,
            detector_type,
            detector_potential,
            detector_entrance_potential,
            resolution_type,
            resolution_method,
            scan_function,
            scan_direction,
            scan_law,
            scan_time,
            mass_calibration_file_name,
            external_reference_file_name,
            internal_reference_file_name,
            instrument_parameter_comments,
        })
    }
}

pub struct AndiMsRawDataGlobal {
    /// 32 bit
    pub scan_number: i32,
    /// 32 bit
    pub starting_scan_number: Option<i32>,
    pub has_masses: bool,
    pub has_times: bool,
    /// default: 1.0
    pub mass_axis_scale_factor: f64,
    /// default: 1.0
    pub time_axis_scale_factor: f64,
    /// default: 1.0
    pub intensity_axis_scale_factor: f64,
    /// default: 0.0
    pub intensity_axis_offset: f64,
    pub mass_axis_units: AndiMsMassAxisUnit,
    pub time_axis_units: AndiMsTimeAxisUnit,
    pub intensity_axis_units: AndiMsIntensityAxisUnit,
    pub total_intensity_units: AndiMsIntensityAxisUnit,
    pub mass_axis_data_format: AndiMsDataFormat,
    pub time_axis_data_format: AndiMsDataFormat,
    pub intensity_axis_data_format: AndiMsDataFormat,
    pub mass_axis_label: Option<String>,
    pub time_axis_label: Option<String>,
    pub intensity_axis_label: Option<String>,
    pub mass_axis_global_range_min: Option<f64>,
    pub mass_axis_global_range_max: Option<f64>,
    pub time_axis_global_range_min: Option<f64>,
    pub time_axis_global_range_max: Option<f64>,
    pub intensity_axis_global_range_min: Option<f64>,
    pub intensity_axis_global_range_max: Option<f64>,
    pub calibrated_mass_range_min: Option<f64>,
    pub calibrated_mass_range_max: Option<f64>,
    pub actual_run_time: Option<f64>,
    pub actual_delay_time: Option<f64>,
    pub uniform_sampling_flag: bool,
    pub comments: Option<String>,
}

pub fn read_var_attr_f64(
    var_opt: Option<&Variable>,
    attr_name: &str,
) -> Result<Option<f64>, AndiError> {
    match var_opt.and_then(|var| var.get_attr_f64(attr_name)) {
        None | Some([]) => Ok(None),
        Some([val]) => Ok(Some(val.to_owned())),
        Some([..]) => {
            // unwrap is safe here as this can only match if var exists
            let var_name = var_opt.unwrap().name();
            Err(AndiError::new(&format!(
                "More than one element found in {} attribute for variable {}.",
                attr_name, var_name
            )))
        }
    }
}

pub fn read_var_attr_str(var_opt: Option<&Variable>, attr_name: &str) -> Option<String> {
    var_opt
        .and_then(|var| var.get_attr_as_string(attr_name))
        .map(|mut s| {
            trim_zeros_in_place(&mut s);
            s
        })
}

impl AndiMsRawDataGlobal {
    pub fn new(reader: &netcdf3::FileReader) -> Result<Self, Box<dyn Error>> {
        let scan_number_dim = reader
            .data_set()
            .get_dim("scan_number")
            .ok_or(AndiError::new("Missing scan_number dimension."))?;
        // TODO: usize?
        let scan_number = scan_number_dim.size() as i32;
        let starting_scan_number = read_global_i32_attr(reader, "starting_scan_number")?;
        let mass_values_var = reader.data_set().get_var("mass_values");
        let has_masses = mass_values_var.is_some();
        let time_values_var = reader.data_set().get_var("time_values");
        let has_times = time_values_var.is_some();
        let mass_axis_scale_factor =
            read_var_attr_f64(mass_values_var, "scale_factor")?.unwrap_or(1f64);
        let time_axis_scale_factor =
            read_var_attr_f64(time_values_var, "scale_factor")?.unwrap_or(1f64);
        let intensity_values_var = reader.data_set().get_var("intensity_values");
        let intensity_axis_scale_factor =
            read_var_attr_f64(intensity_values_var, "scale_factor")?.unwrap_or(1f64);
        let intensity_axis_offset =
            read_var_attr_f64(intensity_values_var, "add_offset")?.unwrap_or(0f64);
        let mass_axis_units = match read_var_attr_str(mass_values_var, "units") {
            None => AndiMsMassAxisUnit::default(),
            Some(s) => AndiMsMassAxisUnit::from_str(&s)?,
        };
        let time_axis_units = match read_var_attr_str(time_values_var, "units") {
            None => AndiMsTimeAxisUnit::default(),
            Some(s) => AndiMsTimeAxisUnit::from_str(&s)?,
        };
        let intensity_axis_units = match read_var_attr_str(intensity_values_var, "units") {
            None => AndiMsIntensityAxisUnit::default(),
            Some(s) => AndiMsIntensityAxisUnit::from_str(&s)?,
        };
        let total_intensity_var = reader.data_set().get_var("total_intensity");
        let total_intensity_units = match read_var_attr_str(total_intensity_var, "units") {
            None => AndiMsIntensityAxisUnit::default(),
            Some(s) => AndiMsIntensityAxisUnit::from_str(&s)?,
        };
        let mass_axis_data_format = match read_global_str_attr(reader, "raw_data_mass_format") {
            None => AndiMsDataFormat::Short,
            Some(s) => AndiMsDataFormat::from_str(&s)?,
        };
        let time_axis_data_format = match read_global_str_attr(reader, "raw_data_time_format") {
            None => AndiMsDataFormat::Short,
            Some(s) => AndiMsDataFormat::from_str(&s)?,
        };
        let intensity_axis_data_format =
            match read_global_str_attr(reader, "raw_data_intensity_format") {
                None => AndiMsDataFormat::Long,
                Some(s) => AndiMsDataFormat::from_str(&s)?,
            };
        let mass_axis_label = read_var_attr_str(mass_values_var, "long_name");
        let time_axis_label = read_var_attr_str(time_values_var, "long_name");
        let intensity_axis_label = read_var_attr_str(intensity_values_var, "long_name");
        let mass_axis_global_range_min = read_global_attr_f64(reader, "global_mass_min")?;
        let mass_axis_global_range_max = read_global_attr_f64(reader, "global_mass_max")?;
        let time_axis_global_range_min = read_global_attr_f64(reader, "global_time_min")?;
        let time_axis_global_range_max = read_global_attr_f64(reader, "global_time_max")?;
        let intensity_axis_global_range_min = read_global_attr_f64(reader, "global_intensity_min")?;
        let intensity_axis_global_range_max = read_global_attr_f64(reader, "global_intensity_max")?;
        let calibrated_mass_range_min = read_global_attr_f64(reader, "calibrated_mass_min")?;
        let calibrated_mass_range_max = read_global_attr_f64(reader, "calibrated_mass_max")?;
        let actual_run_time = read_global_attr_f64(reader, "actual_run_time_length")?;
        let actual_delay_time = read_global_attr_f64(reader, "actual_delay_time")?;
        let uniform_sampling_flag = read_global_attr_i16(reader, "raw_data_uniform_sampling_flag")?
            .map_or(true, |v| v != 0);
        let comments = read_global_str_attr(reader, "raw_data_comments");

        Ok(Self {
            scan_number,
            starting_scan_number,
            has_masses,
            has_times,
            mass_axis_scale_factor,
            time_axis_scale_factor,
            intensity_axis_scale_factor,
            intensity_axis_offset,
            mass_axis_units,
            time_axis_units,
            intensity_axis_units,
            total_intensity_units,
            mass_axis_data_format,
            time_axis_data_format,
            intensity_axis_data_format,
            mass_axis_label,
            time_axis_label,
            intensity_axis_label,
            mass_axis_global_range_min,
            mass_axis_global_range_max,
            time_axis_global_range_min,
            time_axis_global_range_max,
            intensity_axis_global_range_min,
            intensity_axis_global_range_max,
            calibrated_mass_range_min,
            calibrated_mass_range_max,
            actual_run_time,
            actual_delay_time,
            uniform_sampling_flag,
            comments,
        })
    }
}

// TODO: needed?
// pub struct AndiNonStandardVariables {}

// pub struct AndiNonStandardAttributes {}
