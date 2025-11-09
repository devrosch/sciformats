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

use super::{
    andi_enums::{AndiMsExperimentType, AndiMsScanFunction},
    andi_ms_parser::{AndiMsFile, AndiMsInstrumentComponent, AndiMsRawDataPerScan},
};
use crate::{
    api::{Column, Node, Parameter, PointXy, Reader, Table, Value},
    common::SfError,
    utils::convert_path_to_node_indices,
};
use std::{collections::HashMap, path::Path};

pub struct AndiMsReader {
    path: String,
    file: AndiMsFile,
}

impl Reader for AndiMsReader {
    fn read(&self, path: &str) -> Result<Node, SfError> {
        let path_indices = convert_path_to_node_indices(path)?;
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
            // TODO: add mass-time mapping values as sub node if applicable
            [5, n] => self.read_raw_data_per_scan(n),
            [5, n, 0] => self.read_library_data_per_scan(n),
            [6] => self.read_scan_groups(),
            [6, n] => self.read_scan_group(n),
            _ => Err(SfError::new(&format!("Illegal node path: {}", path)).into()),
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

    fn read_root(&self) -> Result<Node, SfError> {
        let path = Path::new(&self.path);
        let file_name = path.file_name().map_or("", |f| f.to_str().unwrap_or(""));
        let mut child_node_names = vec![
            "Admin Data".to_owned(),
            "Instrument Components".to_owned(),
            "Sample Data".to_owned(),
            "Test Data".to_owned(),
            "Raw Data Global".to_owned(),
            "Raw Data Scans".to_owned(),
        ];
        if self.file.test_data.scan_function == AndiMsScanFunction::Sid {
            child_node_names.push("Scan Groups".to_owned());
        }
        Ok(Node {
            name: file_name.to_owned(),
            parameters: vec![],
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
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

    fn read_admin_data(&self) -> Result<Node, SfError> {
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

    fn read_instrument_components(&self) -> Result<Node, SfError> {
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

    fn read_instrument_component(&self, index: usize) -> Result<Node, SfError> {
        let components = &self.file.instrument_data.instrument_components;
        let component = components.get(index).ok_or(SfError::new(&format!(
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
        Self::push_opt_str("Instrument ID", &component.instrument_id, &mut parameters);
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
            "Instrument Serial No",
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
            parameters,
            data: Vec::new(),
            metadata: Vec::new(),
            table: None,
            child_node_names: Vec::new(),
        })
    }

    fn read_sample_data(&self) -> Result<Node, SfError> {
        let sample_data = &self.file.sample_data;

        let mut parameters: Vec<Parameter> = Vec::new();
        Self::push_opt_str("Sample Owner", &sample_data.sample_owner, &mut parameters);
        Self::push_opt_str(
            "Sample Receipt Date/Time Stamp",
            &sample_data.sample_receipt_date_time_stamp,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample Internal ID",
            &sample_data.sample_internal_id,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample Internal ID",
            &sample_data.sample_internal_id,
            &mut parameters,
        );
        Self::push_opt_str(
            "Sample External ID",
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

    fn read_test_data(&self) -> Result<Node, SfError> {
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

    fn read_raw_data_global(&self) -> Result<Node, SfError> {
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

    fn read_raw_data_scans(&self) -> Result<Node, SfError> {
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
    fn read_raw_data_per_scan(&self, index: usize) -> Result<Node, SfError> {
        let scans = &self.file.raw_data_scans.raw_data_per_scan_list;
        let scan = scans.get(index).ok_or(SfError::new(&format!(
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
        Self::push_opt_f64("Inter Scan Time", &scan.inter_scan_time, &mut parameters);
        Self::push_opt_f64("Resolution", &scan.resolution, &mut parameters);

        let raw_data_global = &self.file.raw_data_global;
        let x_values = match (raw_data_global.has_masses, raw_data_global.has_times) {
            (true, _) => scan.get_mass_axis_values()?.ok_or(SfError::new(&format!(
                "Could not find m/z values for scan at index: {}",
                index
            )))?,
            (_, true) => scan.get_time_axis_values()?.ok_or(SfError::new(&format!(
                "Could not find time values for scan at index: {}",
                index
            )))?,
            _ => {
                return Err(SfError::new(&format!(
                    "Could not find m/z or time values for scan at index: {}",
                    index
                )))?;
            }
        };
        let y_values = scan
            .get_intensity_axis_values()?
            .ok_or(SfError::new(&format!(
                "Could not find intensity values for scan at index: {}",
                index
            )))?;
        if x_values.len() != y_values.len() {
            return Err(SfError::new(&format!(
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
            return Err(SfError::new(&format!(
                "Mismatch of flag index and value lengths for scan at index: {}",
                index
            )))?;
        }
        let mut flagged_peaks: Vec<f64> = vec![];
        for i in flagged_peak_indices {
            let peak = x_values.get(i as usize).ok_or(SfError::new(&format!(
                "Illegal flag index {} for scan at index: {}",
                i, index
            )))?;
            flagged_peaks.push(*peak);
        }

        // TODO: make table None if no rows are present
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

        let mut child_node_names: Vec<String> = vec![];
        if self.file.admin_data.experiment_type == AndiMsExperimentType::LibraryMassSpectrum {
            child_node_names.push("Library Data".to_owned());
        }

        // TODO: add mass-time mapping values as sub node if applicable

        Ok(Node {
            name,
            parameters,
            data,
            metadata,
            table: Some(table),
            child_node_names,
        })
    }

    fn read_library_data_per_scan(&self, index: usize) -> Result<Node, SfError> {
        let library_data = &self
            .file
            .library_data
            .as_ref()
            .ok_or(SfError::new("No library data found."))?;
        let scan_lib_data = library_data
            .library_data_per_scan
            .get(index)
            .ok_or(SfError::new(&format!(
                "Illegal path. Library data per scan not found for index: {}",
                index
            )))?;

        let name = "Library Data".to_owned();

        let mut parameters: Vec<Parameter> = vec![];
        parameters.push(Parameter::from_str_i32(
            "Scan Number",
            scan_lib_data.scan_number,
        ));
        Self::push_opt_str("Entry Name", &scan_lib_data.entry_name, &mut parameters);
        Self::push_opt_str("Entry ID", &scan_lib_data.entry_id, &mut parameters);
        Self::push_opt_i32("Entry Number", &scan_lib_data.entry_number, &mut parameters);
        Self::push_opt_str(
            "Source Data File Reference",
            &scan_lib_data.source_data_file_reference,
            &mut parameters,
        );
        Self::push_opt_str("CAS Name", &scan_lib_data.cas_name, &mut parameters);
        Self::push_opt_str("Other Name 0", &scan_lib_data.other_name_0, &mut parameters);
        Self::push_opt_str("Other Name 1", &scan_lib_data.other_name_1, &mut parameters);
        Self::push_opt_str("Other Name 2", &scan_lib_data.other_name_2, &mut parameters);
        Self::push_opt_str("Other Name 3", &scan_lib_data.other_name_3, &mut parameters);
        Self::push_opt_i32("CAS Number", &scan_lib_data.cas_number, &mut parameters);
        Self::push_opt_str(
            "Chemical Formula",
            &scan_lib_data.chemical_formula,
            &mut parameters,
        );
        Self::push_opt_str(
            "Wiswesser Notation",
            &scan_lib_data.wiswesser_notation,
            &mut parameters,
        );
        Self::push_opt_str(
            "SMILES Notation",
            &scan_lib_data.smiles_notation,
            &mut parameters,
        );
        Self::push_opt_str(
            "Molfile Reference Name",
            &scan_lib_data.molfile_reference_name,
            &mut parameters,
        );
        Self::push_opt_str(
            "Other Structure Notation",
            &scan_lib_data.other_structure_notation,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Retention Index",
            &scan_lib_data.retention_index,
            &mut parameters,
        );
        Self::push_opt_str(
            "Retention Index Type",
            &scan_lib_data.retention_index_type,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Absolute Retention Time",
            &scan_lib_data.absolute_retention_time,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Relative Retention",
            &scan_lib_data.relative_retention,
            &mut parameters,
        );
        Self::push_opt_str(
            "Retention Reference Name",
            &scan_lib_data.retention_reference_name,
            &mut parameters,
        );
        Self::push_opt_i32(
            "Retention Reference CAS Number",
            &scan_lib_data.retention_reference_cas_number,
            &mut parameters,
        );
        Self::push_opt_f32(
            "Melting Point",
            &scan_lib_data.melting_point,
            &mut parameters,
        );
        Self::push_opt_f32(
            "Boiling Point",
            &scan_lib_data.boiling_point,
            &mut parameters,
        );
        Self::push_opt_f64(
            "Chemical Mass",
            &scan_lib_data.chemical_mass,
            &mut parameters,
        );
        Self::push_opt_i32("Nominal Mass", &scan_lib_data.nominal_mass, &mut parameters);
        Self::push_opt_f64(
            "Accurate Mass",
            &scan_lib_data.accurate_mass,
            &mut parameters,
        );
        Self::push_opt_str(
            "Other Information",
            &scan_lib_data.other_information,
            &mut parameters,
        );

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: vec![],
        })
    }

    pub fn read_scan_groups(&self) -> Result<Node, SfError> {
        let scan_groups = &self
            .file
            .scan_groups
            .as_ref()
            .ok_or(SfError::new("Illegal path. No scan groups found."))?;

        let child_node_names: Vec<String> = scan_groups
            .raw_data_per_scan_groups
            .iter()
            .map(|scan_group| format!("Scan Group {}", scan_group.group_number))
            .collect();

        Ok(Node {
            name: "Scan Groups".to_owned(),
            parameters: vec![],
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
        })
    }

    pub fn read_scan_group(&self, n: usize) -> Result<Node, SfError> {
        let scan_groups = &self
            .file
            .scan_groups
            .as_ref()
            .ok_or(SfError::new("Illegal path. No scan groups found."))?;
        let scan_group = &scan_groups
            .raw_data_per_scan_groups
            .get(n)
            .ok_or(SfError::new(&format!(
                "Illegal path. No scan group found for index: {}",
                n
            )))?;

        let name = format!("Scan Group {}", n);
        let parameters: Vec<Parameter> = vec![
            Parameter::from_str_i32("Group Number", scan_group.group_number),
            Parameter::from_str_i32(
                "Number Of Masses In Group",
                scan_group.number_of_masses_in_group,
            ),
            Parameter::from_str_i32("Starting Scan Number", scan_group.starting_scan_number),
        ];

        let masses = scan_group.get_group_masses()?;
        let sampling_times = scan_group.get_group_sampling_times()?;
        let delay_times = scan_group.get_group_delay_times()?;

        // TODO: make table None if no rows are present
        let mut table = Table {
            column_names: vec![Column::new("mass", "M/Z")],
            rows: vec![],
        };
        if sampling_times.is_some() {
            table
                .column_names
                .push(Column::new("sampling_time", "Sampling Time"));
        }
        if delay_times.is_some() {
            table
                .column_names
                .push(Column::new("delay_time", "Delay Time"));
        }

        for (i, mass) in masses.iter().enumerate() {
            let mut row = HashMap::new();
            row.insert("mass".to_owned(), Value::F64(*mass));
            if let Some(samplings) = &sampling_times {
                let sampling = samplings.get(i).ok_or(SfError::new(&format!(
                    "In scan_group {} could not find sampling time at index: {}",
                    n, i
                )))?;
                row.insert("sampling_time".to_owned(), Value::F64(*sampling));
            }
            if let Some(delays) = &delay_times {
                let delay = delays.get(i).ok_or(SfError::new(&format!(
                    "In scan_group {} could not find delay time at index: {}",
                    n, i
                )))?;
                row.insert("delay_time".to_owned(), Value::F64(*delay));
            }
            table.rows.push(row);
        }

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: Some(table),
            child_node_names: vec![],
        })
    }

    fn read_error_log(&self) -> Result<Node, SfError> {
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
