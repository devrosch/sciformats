use super::{
    andi_enums::AndiMsExperimentType,
    andi_ms_parser::{AndiMsFile, AndiMsInstrumentComponent, AndiMsRawDataPerScan},
};
use crate::{
    andi::AndiError,
    api::{Column, Node, Parameter, PointXy, Reader, Table, Value},
};
use std::{collections::HashMap, error::Error, path::Path};
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_family = "wasm")]
use wasm_bindgen::JsError;

#[wasm_bindgen]
pub struct AndiMsReader {
    path: String,
    file: AndiMsFile,
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
impl AndiMsReader {
    #[wasm_bindgen(js_name = read)]
    pub fn js_read(&self, path: &str) -> Result<Node, JsError> {
        let read_result = Reader::read(self, path);
        match read_result {
            Ok(node) => Ok(node),
            Err(error) => Err(JsError::new(&error.to_string())),
        }
    }
}

impl Reader for AndiMsReader {
    fn read(&self, path: &str) -> Result<Node, Box<dyn Error>> {
        let path_indices = Self::convert_path_to_node_indices(path)?;
        match path_indices[..] {
            [] => self.read_root(), // "", "/"
            [0] => self.read_admin_data(),
            [0, 0] => self.read_error_log(),
            [1] => self.read_instrument_components(),
            [1, n] => self.read_instrument_component(n),
            [2] => self.read_sample_data(),
            [3] => self.read_test_data(),
            [4] => self.read_raw_data_global(),
            [5] => self.read_raw_data_scans(),
            [5, n] => self.read_raw_data_per_scan(n),
            // TODO: add mass-time mapping values as sub node if applicable
            // TODO: add mappings for library data and scan groups
            // [3] => self.read_raw_data(),
            // [4] => self.read_peak_processing_results(),
            _ => Err(AndiError::new(&format!("Illegal node path: {}", path)).into()),
        }
    }
}

impl AndiMsReader {
    pub fn new(path: &str, file: AndiMsFile) -> Self {
        AndiMsReader {
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

    fn push_opt_str(key: &str, val: &Option<String>, vec: &mut Vec<Parameter>) {
        if let Some(v) = val {
            vec.push(Parameter::from_str_str(key, v))
        }
    }

    fn push_opt_i32(key: &str, val: &Option<i32>, vec: &mut Vec<Parameter>) {
        if let Some(v) = val {
            vec.push(Parameter::from_str_i32(key, *v))
        }
    }

    fn push_opt_f32(key: &str, val: &Option<f32>, vec: &mut Vec<Parameter>) {
        if let Some(v) = val {
            vec.push(Parameter::from_str_f32(key, *v))
        }
    }

    fn push_opt_f64(key: &str, val: &Option<f64>, vec: &mut Vec<Parameter>) {
        if let Some(v) = val {
            vec.push(Parameter::from_str_f64(key, *v))
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
            "MS Template Revision",
            &admin_data.ms_template_revision,
        ));
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
            "Experiment Title",
            &admin_data.experiment_title,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_str(
            "Experiment Date/Time Stamp",
            &admin_data.experiment_date_time_stamp,
        ));
        parameters.push(Parameter::from_str_str(
            "Experiment Type",
            admin_data.experiment_type.to_string(),
        ));
        Self::push_opt_str(
            "Experiment X Ref 0",
            &admin_data.experiment_x_ref_0,
            &mut parameters,
        );
        Self::push_opt_str(
            "Experiment X Ref 1",
            &admin_data.experiment_x_ref_1,
            &mut parameters,
        );
        Self::push_opt_str(
            "Experiment X Ref 2",
            &admin_data.experiment_x_ref_2,
            &mut parameters,
        );
        Self::push_opt_str(
            "Experiment X Ref 3",
            &admin_data.experiment_x_ref_3,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_str(
            "NetCDF File Date/Time Stamp",
            &admin_data.netcdf_file_date_time_stamp,
        ));
        parameters.push(Parameter::from_str_str(
            "NetCDF Revision",
            &admin_data.netcdf_revision,
        ));
        Self::push_opt_str("Operator Name", &admin_data.operator_name, &mut parameters);
        Self::push_opt_str(
            "Source File Reference",
            &admin_data.source_file_reference,
            &mut parameters,
        );
        Self::push_opt_str(
            "Source File Format",
            &admin_data.source_file_format,
            &mut parameters,
        );
        Self::push_opt_str(
            "Source File Date/Time Stamp",
            &admin_data.source_file_date_time_stamp,
            &mut parameters,
        );
        Self::push_opt_str(
            "External File Ref 0",
            &admin_data.external_file_ref_0,
            &mut parameters,
        );
        Self::push_opt_str(
            "External File Ref 1",
            &admin_data.external_file_ref_1,
            &mut parameters,
        );
        Self::push_opt_str(
            "External File Ref 2",
            &admin_data.external_file_ref_2,
            &mut parameters,
        );
        Self::push_opt_str(
            "External File Ref 3",
            &admin_data.external_file_ref_3,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_str("Languages", &admin_data.languages));
        Self::push_opt_i32(
            "Number Of Times Processed",
            &admin_data.number_of_times_processed,
            &mut parameters,
        );
        Self::push_opt_i32(
            "Number Of Times Calibrated",
            &admin_data.number_of_times_calibrated,
            &mut parameters,
        );
        Self::push_opt_str(
            "Calibration History 0",
            &admin_data.calibration_history_0,
            &mut parameters,
        );
        Self::push_opt_str(
            "Calibration History 1",
            &admin_data.calibration_history_1,
            &mut parameters,
        );
        Self::push_opt_str(
            "Calibration History 2",
            &admin_data.calibration_history_2,
            &mut parameters,
        );
        Self::push_opt_str(
            "Calibration History 3",
            &admin_data.calibration_history_3,
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
        Self::push_opt_i32(
            "Instrument Number",
            &admin_data.instrument_number,
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

    fn generate_instrument_component_name(
        index: usize,
        component: &AndiMsInstrumentComponent,
    ) -> String {
        let instrument_id = &component.instrument_id;
        let instrument_name = &component.instrument_name;
        match (instrument_id, instrument_name) {
            (Some(id), None) => id.to_owned(),
            (None, Some(name)) => name.to_owned(),
            (Some(id), Some(name)) => format!("{} ({})", id, name),
            _ => index.to_string(),
        }
    }

    fn read_instrument_components(&self) -> Result<Node, Box<dyn Error>> {
        let components = &self.file.instrument_data.instrument_components;
        let child_node_names: Vec<String> = components
            .iter()
            .enumerate()
            .map(|(index, component)| Self::generate_instrument_component_name(index, component))
            .collect();

        Ok(Node {
            name: "Instrument Components".to_owned(),
            parameters: Vec::new(),
            data: Vec::new(),
            metadata: Vec::new(),
            table: None,
            child_node_names,
        })
    }

    fn read_instrument_component(&self, index: usize) -> Result<Node, Box<dyn Error>> {
        let components = &self.file.instrument_data.instrument_components;
        let component = components.get(index).ok_or(AndiError::new(&format!(
            "Illegal path. Instrument component not found for index: {}",
            index
        )))?;

        let name = Self::generate_instrument_component_name(index, component);
        let mut parameters: Vec<Parameter> = vec![];
        Self::push_opt_str(
            "Instrument Name",
            &component.instrument_name,
            &mut parameters,
        );
        Self::push_opt_str("Instrument Id", &component.instrument_id, &mut parameters);
        Self::push_opt_str(
            "Instrument Manufacturer",
            &component.instrument_mfr,
            &mut parameters,
        );
        Self::push_opt_str(
            "Instrument Model",
            &component.instrument_model,
            &mut parameters,
        );
        Self::push_opt_str(
            "Instrument Srial No",
            &component.instrument_serial_no,
            &mut parameters,
        );
        Self::push_opt_str(
            "Instrument Comments",
            &component.instrument_comments,
            &mut parameters,
        );
        Self::push_opt_str(
            "Instrument Software Version",
            &component.instrument_sw_version,
            &mut parameters,
        );
        Self::push_opt_str(
            "Instrument Firmware Version",
            &component.instrument_fw_version,
            &mut parameters,
        );
        Self::push_opt_str(
            "Instrument OS Version",
            &component.instrument_os_version,
            &mut parameters,
        );
        Self::push_opt_str(
            "Instrument App Version",
            &component.instrument_app_version,
            &mut parameters,
        );

        Ok(Node {
            name,
            parameters: Vec::new(),
            data: Vec::new(),
            metadata: Vec::new(),
            table: None,
            child_node_names: Vec::new(),
        })
    }

    fn read_sample_data(&self) -> Result<Node, Box<dyn Error>> {
        let sample_data = &self.file.sample_data;

        let mut parameters: Vec<Parameter> = Vec::new();
        Self::push_opt_str("Sample Owner", &sample_data.sample_owner, &mut parameters);
        Self::push_opt_str(
            "Sample Receipt Date/Time Stamp",
            &sample_data.sample_receipt_date_time_stamp,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample Internal Id",
            &sample_data.sample_internal_id,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample Internal Id",
            &sample_data.sample_internal_id,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample External Id",
            &sample_data.sample_external_id,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample Procedure Name",
            &sample_data.sample_procedure_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample Preparation Procedure",
            &sample_data.sample_prep_comments,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_str(
            "Sample State",
            sample_data.sample_state.to_string(),
        ));
        Self::push_opt_str("Sample Matrix", &sample_data.sample_matrix, &mut parameters);
        Self::push_opt_str(
            "Sample Storage",
            &sample_data.sample_storage,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample Disposal",
            &sample_data.sample_disposal,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample History",
            &sample_data.sample_history,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample Preparation Comments",
            &sample_data.sample_prep_comments,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample Comments",
            &sample_data.sample_comments,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample Manual Handling",
            &sample_data.sample_manual_handling,
            &mut parameters,
        );

        Ok(Node {
            name: "Sample Data".to_owned(),
            parameters,
            data: Vec::new(),
            metadata: Vec::new(),
            table: None,
            child_node_names: Vec::new(),
        })
    }

    fn read_test_data(&self) -> Result<Node, Box<dyn Error>> {
        let test_data = &self.file.test_data;

        let mut parameters: Vec<Parameter> = Vec::new();
        parameters.push(Parameter::from_str_str(
            "Separation Experiment Type",
            test_data.separation_experiment_type.to_string(),
        ));
        parameters.push(Parameter::from_str_str(
            "Mass Spectrometer Inlet",
            test_data.mass_spectrometer_inlet.to_string(),
        ));
        Self::push_opt_f32(
            "Mass Spectrometer Inlet Temperature",
            &test_data.mass_spectrometer_inlet_temperature,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_str(
            "Ionization Mode",
            test_data.ionization_mode.to_string(),
        ));
        parameters.push(Parameter::from_str_str(
            "Ionization Polarity",
            test_data.ionization_polarity.to_string(),
        ));
        Self::push_opt_f32(
            "Electron Energy",
            &test_data.electron_energy,
            &mut parameters,
        );
        Self::push_opt_f32(
            "Laser Wavelength",
            &test_data.laser_wavelength,
            &mut parameters,
        );
        Self::push_opt_str("Reagent Gas", &test_data.reagent_gas, &mut parameters);
        Self::push_opt_str("Fab Type", &test_data.fab_type, &mut parameters);
        Self::push_opt_str("Fab Matrix", &test_data.fab_matrix, &mut parameters);
        Self::push_opt_f32(
            "Source Temperature",
            &test_data.source_temperature,
            &mut parameters,
        );
        Self::push_opt_f32(
            "Filament Current",
            &test_data.filament_current,
            &mut parameters,
        );
        Self::push_opt_f32(
            "Emission Current",
            &test_data.emission_current,
            &mut parameters,
        );
        Self::push_opt_f32(
            "Accelerating Potential",
            &test_data.accelerating_potential,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_str(
            "Detector Type",
            test_data.detector_type.to_string(),
        ));
        Self::push_opt_f32(
            "Detector Potential",
            &test_data.detector_potential,
            &mut parameters,
        );
        Self::push_opt_f32(
            "Detector Entrance Potential",
            &test_data.detector_entrance_potential,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_str(
            "Resolution Type",
            test_data.resolution_type.to_string(),
        ));
        Self::push_opt_str(
            "Resolution Method",
            &test_data.resolution_method,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_str(
            "Scan Function",
            test_data.scan_function.to_string(),
        ));
        parameters.push(Parameter::from_str_str(
            "Scan Direction",
            test_data.scan_direction.to_string(),
        ));
        parameters.push(Parameter::from_str_str(
            "Scan Law",
            test_data.scan_law.to_string(),
        ));
        Self::push_opt_f32("Scan Time", &test_data.scan_time, &mut parameters);
        Self::push_opt_str(
            "Mass Calibration File Name",
            &test_data.mass_calibration_file_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "External Reference File Name",
            &test_data.external_reference_file_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Internal Reference File Name",
            &test_data.internal_reference_file_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Instrument Parameter Comments",
            &test_data.instrument_parameter_comments,
            &mut parameters,
        );

        Ok(Node {
            name: "Test Data".to_owned(),
            parameters,
            data: Vec::new(),
            metadata: Vec::new(),
            table: None,
            child_node_names: Vec::new(),
        })
    }

    fn read_raw_data_global(&self) -> Result<Node, Box<dyn Error>> {
        let raw_data_global = &self.file.raw_data_global;

        let mut parameters: Vec<Parameter> = Vec::new();
        parameters.push(Parameter::from_str_i32(
            "Scan Number",
            raw_data_global.scan_number,
        ));
        Self::push_opt_i32(
            "Starting Scan Number",
            &raw_data_global.starting_scan_number,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_bool(
            "Has Masses",
            raw_data_global.has_masses,
        ));
        parameters.push(Parameter::from_str_bool(
            "Has Times",
            raw_data_global.has_times,
        ));
        parameters.push(Parameter::from_str_f64(
            "Mass Axis Scale Factor",
            raw_data_global.mass_axis_scale_factor,
        ));
        parameters.push(Parameter::from_str_f64(
            "Time Axis Scale Factor",
            raw_data_global.time_axis_scale_factor,
        ));
        parameters.push(Parameter::from_str_f64(
            "Intensity Axis Scale Factor",
            raw_data_global.intensity_axis_scale_factor,
        ));
        parameters.push(Parameter::from_str_f64(
            "Intensity Axis Offset",
            raw_data_global.intensity_axis_offset,
        ));
        parameters.push(Parameter::from_str_str(
            "Mass Axis Units",
            raw_data_global.mass_axis_units.to_string(),
        ));
        parameters.push(Parameter::from_str_str(
            "Time Axis Units",
            raw_data_global.time_axis_units.to_string(),
        ));
        parameters.push(Parameter::from_str_str(
            "Intensity Axis Units",
            raw_data_global.intensity_axis_units.to_string(),
        ));
        parameters.push(Parameter::from_str_str(
            "Total Intensity Units",
            raw_data_global.total_intensity_units.to_string(),
        ));
        parameters.push(Parameter::from_str_str(
            "Mass Axis Data Format",
            raw_data_global.mass_axis_data_format.to_string(),
        ));
        parameters.push(Parameter::from_str_str(
            "Time Axis Data Format",
            raw_data_global.time_axis_data_format.to_string(),
        ));
        parameters.push(Parameter::from_str_str(
            "Intensity Axis Data Format",
            raw_data_global.intensity_axis_data_format.to_string(),
        ));
        Self::push_opt_str(
            "Mass Axis Label",
            &raw_data_global.mass_axis_label,
            &mut parameters,
        );
        Self::push_opt_str(
            "Time Axis Label",
            &raw_data_global.time_axis_label,
            &mut parameters,
        );
        Self::push_opt_str(
            "Intensity Axis Label",
            &raw_data_global.intensity_axis_label,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Mass Axis Global Range Min",
            &raw_data_global.mass_axis_global_range_min,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Mass Axis Global Range Max",
            &raw_data_global.mass_axis_global_range_max,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Time Axis Global Range Min",
            &raw_data_global.time_axis_global_range_min,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Time Axis Global Range Max",
            &raw_data_global.time_axis_global_range_max,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Intensity Axis Global Range Min",
            &raw_data_global.intensity_axis_global_range_min,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Intensity Axis Global Range Max",
            &raw_data_global.intensity_axis_global_range_max,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Calibrated Mass Range Min",
            &raw_data_global.calibrated_mass_range_min,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Calibrated Mass Global Range Max",
            &raw_data_global.calibrated_mass_range_max,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Actual Run Time",
            &raw_data_global.actual_run_time,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Actual Delay Time",
            &raw_data_global.actual_delay_time,
            &mut parameters,
        );
        parameters.push(Parameter::from_str_bool(
            "Uniform Sampling Flag",
            raw_data_global.uniform_sampling_flag,
        ));
        Self::push_opt_str("Comments", &raw_data_global.comments, &mut parameters);

        Ok(Node {
            name: "Raw Data Global".to_owned(),
            parameters,
            data: Vec::new(),
            metadata: Vec::new(),
            table: None,
            child_node_names: Vec::new(),
        })
    }

    fn generate_scan_name(scan: &AndiMsRawDataPerScan) -> String {
        let actual_scan_number = &scan.actual_scan_number;
        let mass_range_min = &scan.mass_range_min;
        let mass_range_max = &scan.mass_range_max;
        let time_range_min = &scan.time_range_min;
        let time_range_max = &scan.time_range_max;
        match (
            mass_range_min,
            mass_range_max,
            time_range_min,
            time_range_max,
        ) {
            (Some(m_min), Some(m_max), Some(t_min), Some(t_max)) => format!(
                "{} (m/z: {}-{}, t: {}-{})",
                actual_scan_number, m_min, m_max, t_min, t_max
            ),
            (Some(m_min), Some(m_max), _, _) => {
                format!("{} (m/z: {}-{})", actual_scan_number, m_min, m_max)
            }
            (_, _, Some(t_min), Some(t_max)) => {
                format!("{} (t: {}-{})", actual_scan_number, t_min, t_max)
            }
            _ => actual_scan_number.to_string(),
        }
    }

    fn read_raw_data_scans(&self) -> Result<Node, Box<dyn Error>> {
        let scans = &self.file.raw_data_scans.raw_data_per_scan_list;
        let child_node_names: Vec<String> = scans.iter().map(Self::generate_scan_name).collect();

        Ok(Node {
            name: "Raw Data Scans".to_owned(),
            parameters: Vec::new(),
            data: Vec::new(),
            metadata: Vec::new(),
            table: None,
            child_node_names,
        })
    }

    #[allow(clippy::vec_init_then_push)]
    fn read_raw_data_per_scan(&self, index: usize) -> Result<Node, Box<dyn Error>> {
        let scans = &self.file.raw_data_scans.raw_data_per_scan_list;
        let scan = scans.get(index).ok_or(AndiError::new(&format!(
            "Illegal path. Raw data per scan not found for index: {}",
            index
        )))?;

        let name = Self::generate_scan_name(scan);

        let mut parameters: Vec<Parameter> = vec![];
        parameters.push(Parameter::from_str_str(
            "Resolution Type",
            scan.resolution_type.to_string(),
        ));
        parameters.push(Parameter::from_str_i32("Scan Number", scan.scan_number));
        parameters.push(Parameter::from_str_i32(
            "Actual Scan Number",
            scan.actual_scan_number,
        ));
        parameters.push(Parameter::from_str_i32(
            "Number Of Points",
            scan.number_of_points,
        ));
        parameters.push(Parameter::from_str_i32(
            "Number Of Flags",
            scan.number_of_flags,
        ));
        Self::push_opt_f64("Total Intensity", &scan.total_intensity, &mut parameters);
        Self::push_opt_f64(
            "A/D Sampling Rate",
            &scan.a_d_sampling_rate,
            &mut parameters,
        );
        Self::push_opt_i32(
            "A/D Coaddition Factor",
            &scan.a_d_coaddition_factor.map(|v| v as i32),
            &mut parameters,
        );
        Self::push_opt_f64(
            "Scan Acquisition Time",
            &scan.scan_acquisition_time,
            &mut parameters,
        );
        Self::push_opt_f64("Scan Duration", &scan.scan_duration, &mut parameters);
        Self::push_opt_f64("Mass Range Min", &scan.mass_range_min, &mut parameters);
        Self::push_opt_f64("Mass Range Max", &scan.mass_range_max, &mut parameters);
        Self::push_opt_f64("Time Range Min", &scan.time_range_min, &mut parameters);
        Self::push_opt_f64("Time Range Max", &scan.time_range_max, &mut parameters);
        Self::push_opt_f64("Inter scan time", &scan.inter_scan_time, &mut parameters);
        Self::push_opt_f64("Resolution", &scan.resolution, &mut parameters);

        let raw_data_global = &self.file.raw_data_global;
        let x_values = match (raw_data_global.has_masses, raw_data_global.has_times) {
            (true, _) => scan.get_mass_axis_values()?.ok_or(AndiError::new(&format!(
                "Could not find m/z values for scan at index: {}",
                index
            )))?,
            (_, true) => scan.get_time_axis_values()?.ok_or(AndiError::new(&format!(
                "Could not find time values for scan at index: {}",
                index
            )))?,
            _ => {
                return Err(AndiError::new(&format!(
                    "Could not find m/z or time values for scan at index: {}",
                    index
                )))?
            }
        };
        let y_values = scan
            .get_intensity_axis_values()?
            .ok_or(AndiError::new(&format!(
                "Could not find intensity values for scan at index: {}",
                index
            )))?;
        if x_values.len() != y_values.len() {
            return Err(AndiError::new(&format!(
                "Mismatch of x and y value lengths for scan at index: {}",
                index
            )))?;
        }
        let data: Vec<PointXy> = x_values
            .iter()
            .zip(y_values.iter())
            .map(|(x, y)| PointXy::new(*x, *y))
            .collect();

        let mut metadata = Vec::<(String, String)>::new();
        if raw_data_global.has_masses {
            if let Some(label) = &raw_data_global.mass_axis_label {
                metadata.push(("x.label".to_owned(), label.to_owned()));
            }
            metadata.push((
                "x.unit".to_owned(),
                raw_data_global.mass_axis_units.to_string(),
            ));
        } else if raw_data_global.has_times {
            if let Some(label) = &raw_data_global.time_axis_label {
                metadata.push(("x.label".to_owned(), label.to_owned()));
            }
            metadata.push((
                "x.unit".to_owned(),
                raw_data_global.time_axis_units.to_string(),
            ));
        }
        if let Some(label) = &raw_data_global.intensity_axis_label {
            metadata.push(("y.label".to_owned(), label.to_owned()));
        }
        metadata.push((
            "y.unit".to_owned(),
            raw_data_global.intensity_axis_units.to_string(),
        ));
        if self.file.admin_data.experiment_type == AndiMsExperimentType::CentroidedMassSpectrum
            || self.file.admin_data.experiment_type == AndiMsExperimentType::LibraryMassSpectrum
        {
            metadata.push(("plot.style".to_owned(), "sticks".to_owned()));
        }

        let flagged_peak_indices = scan.get_flagged_peak_indices()?;
        let flag_values = scan.get_flag_values()?;
        if flagged_peak_indices.len() != flag_values.len() {
            return Err(AndiError::new(&format!(
                "Mismatch of flag index and value lengths for scan at index: {}",
                index
            )))?;
        }
        let mut flagged_peaks: Vec<f64> = vec![];
        for i in flagged_peak_indices {
            let peak = x_values.get(i as usize).ok_or(AndiError::new(&format!(
                "Illegal flag index {} for scan at index: {}",
                i, index
            )))?;
            flagged_peaks.push(*peak);
        }

        let mut table = Table {
            column_names: vec![
                Column::new("peak", "Peak m/z"),
                Column::new("flags", "Flags"),
            ],
            rows: vec![],
        };

        for (i, flags) in flag_values.iter().enumerate() {
            let mut row = HashMap::new();
            let peak = flagged_peaks[i];
            row.insert("peak".to_owned(), Value::F64(peak));
            let flag_string = flags
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            row.insert("flags".to_owned(), Value::String(flag_string));
            table.rows.push(row);
        }

        // TODO: add mass-time mapping values as sub node if applicable

        Ok(Node {
            name,
            parameters,
            data,
            metadata,
            table: Some(table),
            child_node_names: Vec::new(),
        })
    }

    // fn read_raw_data(&self) -> Result<Node, Box<dyn Error>> {
    //     let raw_data = &self.file.raw_data;

    //     let mut parameters: Vec<Parameter> = Vec::new();
    //     parameters.push(Parameter {
    //         key: "Point Number".into(),
    //         value: Value::I32(raw_data.point_number),
    //     });
    //     Self::push_opt_str(
    //         "Raw Data Table Name",
    //         &raw_data.raw_data_table_name,
    //         &mut parameters,
    //     );
    //     parameters.push(Parameter::from_str_str(
    //         "Retention Unit",
    //         &raw_data.retention_unit,
    //     ));
    //     parameters.push(Parameter::from_str_f32(
    //         "Actual Run Time Length",
    //         raw_data.actual_run_time_length,
    //     ));
    //     parameters.push(Parameter::from_str_f32(
    //         "Actual Sampling Interval",
    //         raw_data.actual_sampling_interval,
    //     ));
    //     parameters.push(Parameter::from_str_f32(
    //         "Actual Delay Time",
    //         raw_data.actual_delay_time,
    //     ));
    //     parameters.push(Parameter::from_str_bool(
    //         "Uniform Sampling Flag",
    //         raw_data.uniform_sampling_flag,
    //     ));
    //     Self::push_opt_str(
    //         "Autosampler Position",
    //         &raw_data.autosampler_position,
    //         &mut parameters,
    //     );

    //     // map to xy pairs
    //     let raw_data_retention = raw_data.get_raw_data_retention()?;
    //     let data = match &raw_data_retention {
    //         Some(x_values) => {
    //             // x values present
    //             let y_values = &raw_data.get_ordinate_values()?;
    //             if x_values.len() != y_values.len() {
    //                 return Err(Box::new(AndiError::new(
    //                     "Numbers of ordinate and retention values do not match.",
    //                 )));
    //             }
    //             let xy_values: Vec<PointXy> = x_values
    //                 .iter()
    //                 .enumerate()
    //                 .map(|(i, &x)| {
    //                     PointXy::new(x as f64, y_values.get(i).unwrap().to_owned() as f64)
    //                 })
    //                 .collect();
    //             xy_values
    //         }
    //         None => {
    //             // x values need to be calculated
    //             let actual_delay_time = raw_data.actual_delay_time as f64;
    //             let actual_sampling_interval = raw_data.actual_sampling_interval as f64;
    //             let y_values = &raw_data.get_ordinate_values()?;
    //             let xy_values: Vec<PointXy> = y_values
    //                 .iter()
    //                 .enumerate()
    //                 .map(|(i, &y)| {
    //                     // spec is ambigious, could be i or (i+1)
    //                     let x = actual_delay_time + i as f64 * actual_sampling_interval;
    //                     PointXy::new(x, y as f64)
    //                 })
    //                 .collect();
    //             xy_values
    //         }
    //     };

    //     let mut metadata: Vec<(String, String)> = vec![];
    //     metadata.push(("x.unit".to_owned(), raw_data.retention_unit.to_owned()));
    //     let y_unit = &self.file.detection_method.detector_unit;
    //     if let Some(y_unit) = y_unit {
    //         metadata.push(("y.unit".to_owned(), y_unit.to_owned()));
    //     }

    //     Ok(Node {
    //         name: "Raw Data".to_owned(),
    //         parameters,
    //         data,
    //         metadata,
    //         table: None,
    //         child_node_names: Vec::new(),
    //     })
    // }

    // fn read_peak_processing_results(&self) -> Result<Node, Box<dyn Error>> {
    //     let peak_processing_results = &self.file.peak_processing_results;

    //     let mut parameters: Vec<Parameter> = Vec::new();
    //     parameters.push(Parameter::from_str_i32(
    //         "Peak Number",
    //         peak_processing_results.peak_number,
    //     ));
    //     Self::push_opt_str(
    //         "Peak Processing Results Table Name",
    //         &peak_processing_results.peak_processing_results_table_name,
    //         &mut parameters,
    //     );
    //     Self::push_opt_str(
    //         "Peak Processing Results Comments",
    //         &peak_processing_results.peak_processing_results_comments,
    //         &mut parameters,
    //     );
    //     Self::push_opt_str(
    //         "Peak Processing Method Name",
    //         &peak_processing_results.peak_processing_method_name,
    //         &mut parameters,
    //     );
    //     Self::push_opt_str(
    //         "Peak Processing Date Time Stamp",
    //         &peak_processing_results.peak_processing_date_time_stamp,
    //         &mut parameters,
    //     );
    //     Self::push_opt_str(
    //         "Peak Amount Unit",
    //         &peak_processing_results.peak_amount_unit,
    //         &mut parameters,
    //     );

    //     let table = match &peak_processing_results.peak_number {
    //         0 => None,
    //         _ => Some(self.read_peaks(peak_processing_results.peak_number as usize)?),
    //     };

    //     Ok(Node {
    //         name: "Peak Processing Results".to_owned(),
    //         parameters,
    //         data: Vec::new(),
    //         metadata: Vec::new(),
    //         table,
    //         child_node_names: Vec::new(),
    //     })
    // }

    // fn read_peaks(&self, num_peaks: usize) -> Result<Table, Box<dyn Error>> {
    //     let peaks = self
    //         .file
    //         .peak_processing_results
    //         .get_peaks()?
    //         // .peaks
    //         // .as_ref()
    //         .ok_or(AndiError::new(&format!(
    //             "No peaks found but peak_number paramater not zero: {}",
    //             num_peaks
    //         )))?;

    //     // table columns
    //     let mut column_names: Vec<Column> = vec![];
    //     if peaks.iter().any(|p| p.peak_retention_time.is_some()) {
    //         column_names.push(Column::new("peak_retention_time", "Peak Retention Time"));
    //     }
    //     if peaks.iter().any(|p| p.peak_name.is_some()) {
    //         column_names.push(Column::new("peak_name", "Peak Name"));
    //     }
    //     if peaks.iter().any(|p| p.peak_amount.is_some()) {
    //         column_names.push(Column::new("peak_amount", "Peak Amount"));
    //     }
    //     if peaks.iter().any(|p| p.peak_start_time.is_some()) {
    //         column_names.push(Column::new("peak_start_time", "Peak Start Time"));
    //     }
    //     if peaks.iter().any(|p| p.peak_end_time.is_some()) {
    //         column_names.push(Column::new("peak_end_time", "Peak End Time"));
    //     }
    //     if peaks.iter().any(|p| p.peak_width.is_some()) {
    //         column_names.push(Column::new("peak_width", "Peak Width"));
    //     }
    //     if peaks.iter().any(|p| p.peak_area.is_some()) {
    //         column_names.push(Column::new("peak_area", "Peak Area"));
    //     }
    //     if peaks.iter().any(|p| p.peak_area_percent.is_some()) {
    //         column_names.push(Column::new("peak_area_percent", "Peak Area Percent"));
    //     }
    //     if peaks.iter().any(|p| p.peak_height.is_some()) {
    //         column_names.push(Column::new("peak_height", "Peak Height"));
    //     }
    //     if peaks.iter().any(|p| p.peak_height_percent.is_some()) {
    //         column_names.push(Column::new("peak_height_percent", "Peak Height Percent"));
    //     }
    //     if peaks.iter().any(|p| p.baseline_start_time.is_some()) {
    //         column_names.push(Column::new("baseline_start_time", "Baseline Start Time"));
    //     }
    //     if peaks.iter().any(|p| p.baseline_start_value.is_some()) {
    //         column_names.push(Column::new("baseline_start_value", "Baseline Start Value"));
    //     }
    //     if peaks.iter().any(|p| p.baseline_stop_time.is_some()) {
    //         column_names.push(Column::new("baseline_stop_time", "Baseline Stop Time"));
    //     }
    //     if peaks.iter().any(|p| p.baseline_stop_value.is_some()) {
    //         column_names.push(Column::new("baseline_stop_value", "Baseline Stop Value"));
    //     }
    //     if peaks.iter().any(|p| p.peak_start_detection_code.is_some()) {
    //         column_names.push(Column::new(
    //             "peak_start_detection_code",
    //             "Peak Start Detection Code",
    //         ));
    //     }
    //     if peaks.iter().any(|p| p.peak_stop_detection_code.is_some()) {
    //         column_names.push(Column::new(
    //             "peak_stop_detection_code",
    //             "Peak Stop Detection Code",
    //         ));
    //     }
    //     if peaks.iter().any(|p| p.retention_index.is_some()) {
    //         column_names.push(Column::new("retention_index", "Retention Index"));
    //     }
    //     if peaks.iter().any(|p| p.migration_time.is_some()) {
    //         column_names.push(Column::new("migration_time", "Migration Time"));
    //     }
    //     if peaks.iter().any(|p| p.peak_asymmetry.is_some()) {
    //         column_names.push(Column::new("peak_asymmetry", "Peak Asymmetry"));
    //     }
    //     if peaks.iter().any(|p| p.peak_efficiency.is_some()) {
    //         column_names.push(Column::new("peak_efficiency", "Peak Efficiency"));
    //     }
    //     if peaks.iter().any(|p| p.mass_on_column.is_some()) {
    //         column_names.push(Column::new("mass_on_column", "Mass On Column"));
    //     }
    //     column_names.push(Column::new(
    //         "manually_reintegrated_peaks",
    //         "Manually Reintegrated Peak",
    //     ));
    //     column_names.push(Column::new("peak_retention_unit", "Peak Retention Unit"));
    //     if peaks.iter().any(|p| p.peak_amount_unit.is_some()) {
    //         column_names.push(Column::new("peak_amount_unit", "Peak Amount Unit"));
    //     }
    //     if peaks.iter().any(|p| p.detector_unit.is_some()) {
    //         column_names.push(Column::new("detector_unit", "Detector Unit"));
    //     }

    //     // table rows
    //     let mut rows: Vec<HashMap<String, Value>> = vec![];
    //     for peak in peaks {
    //         let mut row: HashMap<String, Value> = HashMap::new();
    //         if let Some(val) = peak.peak_retention_time {
    //             row.insert("peak_retention_time".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.peak_name {
    //             row.insert("peak_name".into(), Value::String(val));
    //         }
    //         if let Some(val) = peak.peak_amount {
    //             row.insert("peak_amount".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.peak_start_time {
    //             row.insert("peak_start_time".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.peak_end_time {
    //             row.insert("peak_end_time".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.peak_width {
    //             row.insert("peak_width".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.peak_area {
    //             row.insert("peak_area".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.peak_area_percent {
    //             row.insert("peak_area_percent".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.peak_height {
    //             row.insert("peak_height".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.peak_height_percent {
    //             row.insert("peak_height_percent".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.baseline_start_time {
    //             row.insert("baseline_start_time".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.baseline_start_value {
    //             row.insert("baseline_start_value".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.baseline_stop_time {
    //             row.insert("baseline_stop_time".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.baseline_stop_value {
    //             row.insert("baseline_stop_value".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.peak_start_detection_code {
    //             row.insert("peak_start_detection_code".into(), Value::String(val));
    //         }
    //         if let Some(val) = peak.peak_stop_detection_code {
    //             row.insert("peak_stop_detection_code".into(), Value::String(val));
    //         }
    //         if let Some(val) = peak.retention_index {
    //             row.insert("retention_index".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.migration_time {
    //             row.insert("migration_time".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.peak_asymmetry {
    //             row.insert("peak_asymmetry".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.peak_efficiency {
    //             row.insert("peak_efficiency".into(), Value::F32(val));
    //         }
    //         if let Some(val) = peak.mass_on_column {
    //             row.insert("mass_on_column".into(), Value::F32(val));
    //         }
    //         row.insert(
    //             "manually_reintegrated_peaks".into(),
    //             Value::Bool(peak.manually_reintegrated_peaks),
    //         );
    //         row.insert(
    //             "peak_retention_unit".into(),
    //             Value::String(peak.peak_retention_unit),
    //         );
    //         if let Some(val) = peak.peak_amount_unit {
    //             row.insert("peak_amount_unit".into(), Value::String(val));
    //         }
    //         if let Some(val) = peak.detector_unit {
    //             row.insert("detector_unit".into(), Value::String(val));
    //         }

    //         rows.push(row);
    //     }

    //     Ok(Table { column_names, rows })
    // }

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
            let idx_str = seg.split_once('-').map_or(seg, |p| p.0);
            let idx = idx_str.parse::<usize>()?;
            indices.push(idx);
        }

        Ok(indices)
    }
}

// TODO: needed?
// pub struct AndiNonStandardVariables {}

// pub struct AndiNonStandardAttributes {}
