mod io;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use crate::io::open_file;
use sf_rs::{
    andi::{
        andi_enums::{
            AndiMsDataFormat, AndiMsDetectorType, AndiMsExperimentType, AndiMsIntensityAxisUnit,
            AndiMsIonizationMethod, AndiMsIonizationPolarity, AndiMsMassAxisUnit,
            AndiMsMassSpectrometerInlet, AndiMsResolutionType, AndiMsSampleState,
            AndiMsScanDirection, AndiMsScanFunction, AndiMsScanLaw, AndiMsSeparationMethod,
            AndiMsTimeAxisUnit,
        },
        andi_ms_parser::AndiMsParser,
        AndiDatasetCompleteness,
    },
    api::Parser,
};
use std::str::FromStr;
use wasm_bindgen_test::wasm_bindgen_test;

const ANDI_MS_LIBRARY: &str = "andi_ms_library.cdf";

// fn assert_eq_f32(left: f32, right: f32) {
//     let max = left.max(right);
//     let epsilon = f32::EPSILON * max;
//     assert!(f32::abs(left - right) <= epsilon)
// }

fn assert_blank_len(s: &str, size: usize) {
    assert_eq!(size, s.len());
    assert!(s.chars().all(|c| c == ' '));
}

#[wasm_bindgen_test]
#[test]
fn andi_ms_parse_library_file_succeeds() {
    let (path, file) = open_file(ANDI_MS_LIBRARY);
    let ms = AndiMsParser::parse(&path, file).unwrap();

    let admin_data = &ms.admin_data;
    assert_eq!(
        AndiDatasetCompleteness::from_str("C1+C2").unwrap(),
        admin_data.dataset_completeness
    );
    assert_eq!("1.0", admin_data.ms_template_revision);
    assert_eq!(None, admin_data.administrative_comments);
    assert_eq!(None, admin_data.dataset_origin);
    assert_eq!(None, admin_data.dataset_owner);
    assert_eq!(None, admin_data.experiment_title);
    assert_eq!("20231029185100+0100", admin_data.experiment_date_time_stamp);
    assert_eq!(
        AndiMsExperimentType::LibraryMassSpectrum,
        admin_data.experiment_type
    );
    assert_eq!(None, admin_data.experiment_x_ref_0);
    assert_eq!(None, admin_data.experiment_x_ref_1);
    assert_eq!(None, admin_data.experiment_x_ref_2);
    assert_eq!(None, admin_data.experiment_x_ref_3);
    assert_eq!(
        "20231029185100+0100",
        admin_data.netcdf_file_date_time_stamp
    );
    assert_eq!("2.0", admin_data.netcdf_revision);
    assert_eq!(None, admin_data.operator_name);
    assert_eq!(
        Some("Dummy Source File Reference".to_owned()),
        admin_data.source_file_reference
    );
    assert_eq!(None, admin_data.source_file_format);
    assert_eq!(None, admin_data.source_file_date_time_stamp);
    assert_eq!(None, admin_data.external_file_ref_0);
    assert_eq!(None, admin_data.external_file_ref_1);
    assert_eq!(None, admin_data.external_file_ref_2);
    assert_eq!(None, admin_data.external_file_ref_3);
    assert_eq!("English", admin_data.languages);
    assert_eq!(None, admin_data.number_of_times_processed);
    assert_eq!(None, admin_data.number_of_times_calibrated);
    assert_eq!(None, admin_data.calibration_history_0);
    assert_eq!(None, admin_data.calibration_history_1);
    assert_eq!(None, admin_data.calibration_history_2);
    assert_eq!(None, admin_data.calibration_history_3);
    assert_eq!(None, admin_data.pre_experiment_program_name);
    assert_eq!(None, admin_data.post_experiment_program_name);
    assert_eq!(1, admin_data.error_log.len());
    assert_eq!(
        "                                                               ",
        admin_data.error_log.get(0).unwrap()
    );
    assert_eq!(None, admin_data.instrument_number);

    let instrument_data = &ms.instrument_data;
    assert_eq!(0, instrument_data.instrument_components.len());

    let sample_data = &ms.sample_data;
    assert_eq!(None, sample_data.sample_owner);
    assert_eq!(None, sample_data.sample_receipt_date_time_stamp);
    assert_eq!(None, sample_data.sample_internal_id);
    assert_eq!(None, sample_data.sample_external_id);
    assert_eq!(None, sample_data.sample_procedure_name);
    assert_eq!(None, sample_data.sample_prep_procedure);
    assert_eq!(AndiMsSampleState::OtherState, sample_data.sample_state);
    assert_eq!(None, sample_data.sample_matrix);
    assert_eq!(None, sample_data.sample_storage);
    assert_eq!(None, sample_data.sample_disposal);
    assert_eq!(None, sample_data.sample_history);
    assert_eq!(None, sample_data.sample_prep_comments);
    assert_eq!(None, sample_data.sample_comments);
    assert_eq!(None, sample_data.sample_manual_handling);

    let test_data = &ms.test_data;
    assert_eq!(
        AndiMsSeparationMethod::None,
        test_data.separation_experiment_type
    );
    assert_eq!(
        AndiMsMassSpectrometerInlet::Direct,
        test_data.mass_spectrometer_inlet
    );
    assert_eq!(None, test_data.mass_spectrometer_inlet_temperature);
    assert_eq!(AndiMsIonizationMethod::Ei, test_data.ionization_mode);
    assert_eq!(
        AndiMsIonizationPolarity::Plus,
        test_data.ionization_polarity
    );
    assert_eq!(None, test_data.electron_energy);
    assert_eq!(None, test_data.laser_wavelength);
    assert_eq!(None, test_data.reagent_gas);
    assert_eq!(None, test_data.reagent_gas_pressure);
    assert_eq!(None, test_data.fab_type);
    assert_eq!(None, test_data.fab_matrix);
    assert_eq!(None, test_data.source_temperature);
    assert_eq!(None, test_data.filament_current);
    assert_eq!(None, test_data.emission_current);
    assert_eq!(None, test_data.accelerating_potential);
    assert_eq!(AndiMsDetectorType::Em, test_data.detector_type);
    assert_eq!(None, test_data.detector_potential);
    assert_eq!(AndiMsResolutionType::Constant, test_data.resolution_type);
    assert_eq!(None, test_data.resolution_method);
    assert_eq!(AndiMsScanFunction::Scan, test_data.scan_function);
    assert_eq!(AndiMsScanDirection::Up, test_data.scan_direction);
    assert_eq!(AndiMsScanLaw::Linear, test_data.scan_law);
    assert_eq!(None, test_data.scan_time);
    assert_eq!(None, test_data.mass_calibration_file_name);
    assert_eq!(None, test_data.external_reference_file_name);
    assert_eq!(None, test_data.internal_reference_file_name);
    assert_eq!(None, test_data.instrument_parameter_comments);

    let raw_data_global = &ms.raw_data_global;
    assert_eq!(3, raw_data_global.scan_number);
    assert_eq!(true, raw_data_global.has_masses);
    assert_eq!(false, raw_data_global.has_times);
    assert_eq!(1.0, raw_data_global.mass_axis_scale_factor);
    assert_eq!(1.0, raw_data_global.time_axis_scale_factor);
    assert_eq!(1.0, raw_data_global.intensity_axis_scale_factor);
    assert_eq!(0.0, raw_data_global.intensity_axis_offset);
    assert_eq!(AndiMsMassAxisUnit::Mz, raw_data_global.mass_axis_units);
    assert_eq!(AndiMsTimeAxisUnit::Seconds, raw_data_global.time_axis_units);
    assert_eq!(
        AndiMsIntensityAxisUnit::Arbitrary,
        raw_data_global.intensity_axis_units
    );
    assert_eq!(
        AndiMsIntensityAxisUnit::Arbitrary,
        raw_data_global.total_intensity_units
    );
    assert_eq!(
        AndiMsDataFormat::Short,
        raw_data_global.mass_axis_data_format
    );
    assert_eq!(
        AndiMsDataFormat::Short,
        raw_data_global.time_axis_data_format
    );
    assert_eq!(
        AndiMsDataFormat::Short,
        raw_data_global.intensity_axis_data_format
    );
    assert_eq!(None, raw_data_global.mass_axis_label);
    assert_eq!(None, raw_data_global.time_axis_label);
    assert_eq!(None, raw_data_global.intensity_axis_label);
    assert_eq!(None, raw_data_global.mass_axis_global_range_min);
    assert_eq!(None, raw_data_global.mass_axis_global_range_max);
    assert_eq!(None, raw_data_global.time_axis_global_range_min);
    assert_eq!(None, raw_data_global.time_axis_global_range_max);
    assert_eq!(None, raw_data_global.intensity_axis_global_range_min);
    assert_eq!(None, raw_data_global.intensity_axis_global_range_max);
    assert_eq!(None, raw_data_global.calibrated_mass_range_min);
    assert_eq!(None, raw_data_global.calibrated_mass_range_max);
    assert_eq!(None, raw_data_global.actual_run_time);
    assert_eq!(None, raw_data_global.actual_delay_time);
    assert_eq!(true, raw_data_global.uniform_sampling_flag);
    assert_eq!(None, raw_data_global.comments);

    let raw_data_scans = &ms.raw_data_scans;
    assert_eq!(3, raw_data_scans.raw_data_per_scan_list.len());

    let raw_data_scan_0 = &raw_data_scans.raw_data_per_scan_list[0];
    assert_eq!(0, raw_data_scan_0.scan_number);
    assert_eq!(0, raw_data_scan_0.actual_scan_number);
    assert_eq!(2, raw_data_scan_0.number_of_points);
    assert_eq!(
        vec![16f64, 32f64],
        raw_data_scan_0.get_mass_axis_values().unwrap().unwrap()
    );
    assert_eq!(None, raw_data_scan_0.get_time_axis_values().unwrap());
    assert_eq!(
        vec![100f64, 200f64],
        raw_data_scan_0
            .get_intensity_axis_values()
            .unwrap()
            .unwrap()
    );
    assert_eq!(0, raw_data_scan_0.number_of_flags);
    assert!(raw_data_scan_0.get_flagged_peaks().unwrap().is_empty());
    assert!(raw_data_scan_0.get_flag_values().unwrap().is_empty());
    assert_eq!(None, raw_data_scan_0.total_intensity);
    assert_eq!(None, raw_data_scan_0.a_d_sampling_rate);
    assert_eq!(None, raw_data_scan_0.a_d_coaddition_factor);
    assert_eq!(None, raw_data_scan_0.scan_acquisition_time);
    assert_eq!(None, raw_data_scan_0.scan_duration);
    assert_eq!(16f64, raw_data_scan_0.mass_range_min.unwrap());
    assert_eq!(32f64, raw_data_scan_0.mass_range_max.unwrap());
    assert_eq!(None, raw_data_scan_0.time_range_min);
    assert_eq!(None, raw_data_scan_0.time_range_max);
    assert_eq!(None, raw_data_scan_0.inter_scan_time);
    assert_eq!(None, raw_data_scan_0.resolution);

    let raw_data_scan_2 = &raw_data_scans.raw_data_per_scan_list[2];
    assert_eq!(2, raw_data_scan_2.scan_number);
    assert_eq!(2, raw_data_scan_2.actual_scan_number);
    assert_eq!(3, raw_data_scan_2.number_of_points);
    assert_eq!(
        vec![1f64, 35f64, 36f64],
        raw_data_scan_2.get_mass_axis_values().unwrap().unwrap()
    );
    assert_eq!(None, raw_data_scan_2.get_time_axis_values().unwrap());
    assert_eq!(
        vec![50f64, 20f64, 10f64],
        raw_data_scan_2
            .get_intensity_axis_values()
            .unwrap()
            .unwrap()
    );
    assert_eq!(0, raw_data_scan_2.number_of_flags);
    assert!(raw_data_scan_2.get_flagged_peaks().unwrap().is_empty());
    assert!(raw_data_scan_2.get_flag_values().unwrap().is_empty());
    assert_eq!(None, raw_data_scan_2.total_intensity);
    assert_eq!(None, raw_data_scan_2.a_d_sampling_rate);
    assert_eq!(None, raw_data_scan_2.a_d_coaddition_factor);
    assert_eq!(None, raw_data_scan_2.scan_acquisition_time);
    assert_eq!(None, raw_data_scan_2.scan_duration);
    assert_eq!(1f64, raw_data_scan_2.mass_range_min.unwrap());
    assert_eq!(36f64, raw_data_scan_2.mass_range_max.unwrap());
    assert_eq!(None, raw_data_scan_2.time_range_min);
    assert_eq!(None, raw_data_scan_2.time_range_max);
    assert_eq!(None, raw_data_scan_2.inter_scan_time);
    assert_eq!(None, raw_data_scan_2.resolution);

    let library_data = &ms.library_data.unwrap();
    assert_eq!(3, library_data.library_data_per_scan.len());

    let library_data_scan_0 = &library_data.library_data_per_scan[0];
    assert_eq!(0, library_data_scan_0.scan_number);
    assert_eq!(
        "Entry Name 0",
        library_data_scan_0.entry_name.as_ref().unwrap()
    );
    assert_blank_len(library_data_scan_0.entry_id.as_ref().unwrap(), 31);
    assert_eq!(0, library_data_scan_0.entry_number.unwrap());
    assert_blank_len(
        library_data_scan_0
            .source_data_file_reference
            .as_ref()
            .unwrap(),
        31,
    );
    assert_blank_len(library_data_scan_0.cas_name.as_ref().unwrap(), 254);
    assert_eq!(
        "Other Name 0 0",
        library_data_scan_0.other_name_0.as_ref().unwrap()
    );
    assert_eq!(
        "Other Name 1 0",
        library_data_scan_0.other_name_1.as_ref().unwrap()
    );
    assert_blank_len(library_data_scan_0.other_name_2.as_ref().unwrap(), 254);
    assert_blank_len(library_data_scan_0.other_name_3.as_ref().unwrap(), 254);
    assert_eq!(12345, library_data_scan_0.cas_number.unwrap());
    assert_eq!("O2", library_data_scan_0.chemical_formula.as_ref().unwrap());
    assert_blank_len(
        library_data_scan_0.wiswesser_notation.as_ref().unwrap(),
        127,
    );
    assert_eq!("O=O", library_data_scan_0.smiles_notation.as_ref().unwrap());
    assert_eq!(None, library_data_scan_0.molfile_reference_name);
    assert_blank_len(
        library_data_scan_0
            .other_structure_notation
            .as_ref()
            .unwrap(),
        127,
    );
    assert_eq!(None, library_data_scan_0.retention_index);
    assert_blank_len(
        library_data_scan_0.retention_index_type.as_ref().unwrap(),
        31,
    );
    assert_eq!(None, library_data_scan_0.absolute_retention_time);
    assert_eq!(None, library_data_scan_0.relative_retention);
    assert_blank_len(
        library_data_scan_0
            .retention_reference_name
            .as_ref()
            .unwrap(),
        127,
    );
    assert_eq!(None, library_data_scan_0.retention_reference_cas_number);
    assert_eq!(None, library_data_scan_0.melting_point);
    assert_eq!(None, library_data_scan_0.boiling_point);
    assert_eq!(None, library_data_scan_0.chemical_mass);
    assert_eq!(32, library_data_scan_0.nominal_mass.unwrap());
    assert_eq!(None, library_data_scan_0.accurate_mass);
    assert_blank_len(library_data_scan_0.other_information.as_ref().unwrap(), 254);

    let library_data_scan_2 = &library_data.library_data_per_scan[2];
    assert_eq!(2, library_data_scan_2.scan_number);
    assert_eq!(
        "Entry Name 2",
        library_data_scan_2.entry_name.as_ref().unwrap()
    );
    assert_blank_len(library_data_scan_2.entry_id.as_ref().unwrap(), 31);
    assert_eq!(2, library_data_scan_2.entry_number.unwrap());
    assert_blank_len(
        library_data_scan_2
            .source_data_file_reference
            .as_ref()
            .unwrap(),
        31,
    );
    assert_blank_len(library_data_scan_2.cas_name.as_ref().unwrap(), 254);
    assert_eq!(
        "Other Name 0 2",
        library_data_scan_2.other_name_0.as_ref().unwrap()
    );
    assert_eq!(
        "Other Name 1 2",
        library_data_scan_2.other_name_1.as_ref().unwrap()
    );
    assert_blank_len(library_data_scan_2.other_name_2.as_ref().unwrap(), 254);
    assert_blank_len(library_data_scan_2.other_name_3.as_ref().unwrap(), 254);
    assert_eq!(1234567, library_data_scan_2.cas_number.unwrap());
    assert_eq!(
        "HCl",
        library_data_scan_2.chemical_formula.as_ref().unwrap()
    );
    assert_blank_len(
        library_data_scan_2.wiswesser_notation.as_ref().unwrap(),
        127,
    );
    assert_eq!("Cl", library_data_scan_2.smiles_notation.as_ref().unwrap());
    assert_eq!(None, library_data_scan_2.molfile_reference_name);
    assert_blank_len(
        library_data_scan_2
            .other_structure_notation
            .as_ref()
            .unwrap(),
        127,
    );
    assert_eq!(None, library_data_scan_2.retention_index);
    assert_blank_len(
        library_data_scan_2.retention_index_type.as_ref().unwrap(),
        31,
    );
    assert_eq!(None, library_data_scan_2.absolute_retention_time);
    assert_eq!(None, library_data_scan_2.relative_retention);
    assert_blank_len(
        library_data_scan_2
            .retention_reference_name
            .as_ref()
            .unwrap(),
        127,
    );
    assert_eq!(None, library_data_scan_2.retention_reference_cas_number);
    assert_eq!(None, library_data_scan_2.melting_point);
    assert_eq!(None, library_data_scan_2.boiling_point);
    assert_eq!(None, library_data_scan_2.chemical_mass);
    assert_eq!(36, library_data_scan_2.nominal_mass.unwrap());
    assert_eq!(None, library_data_scan_2.accurate_mass);
    assert_blank_len(library_data_scan_2.other_information.as_ref().unwrap(), 254);

    assert!(ms.scan_groups.is_none());

    // TODO: add tests for non standard variables and attributes once available
}
