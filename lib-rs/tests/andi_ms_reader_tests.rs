mod io;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use std::collections::HashMap;

use crate::io::open_file;
use sf_rs::{
    andi::{
        andi_enums::{
            AndiMsDataFormat, AndiMsDetectorType, AndiMsExperimentType, AndiMsFlagValue,
            AndiMsIntensityAxisUnit, AndiMsIonizationMethod, AndiMsIonizationPolarity,
            AndiMsMassAxisUnit, AndiMsMassSpectrometerInlet, AndiMsResolutionType,
            AndiMsSampleState, AndiMsScanDirection, AndiMsScanFunction, AndiMsScanLaw,
            AndiMsSeparationMethod, AndiMsTimeAxisUnit,
        },
        andi_ms_parser::AndiMsParser,
        andi_ms_reader::AndiMsReader,
    },
    api::{Column, Parameter, Parser, PointXy, Reader, Value},
};
use wasm_bindgen_test::wasm_bindgen_test;

const ANDI_MS_CENTROID_FILE_PATH: &str = "andi_ms_centroid.cdf";
const ANDI_MS_CONTINUUM_FILE_PATH: &str = "andi_ms_continuum.cdf";

#[wasm_bindgen_test]
#[test]
fn andi_ms_centroid_read_succeeds() {
    let (path, file) = open_file(ANDI_MS_CENTROID_FILE_PATH);
    let ms = AndiMsParser::parse(&path, file).unwrap();
    let reader = AndiMsReader::new(&path, ms);

    let root = &reader.read("/").unwrap();
    assert_eq!(ANDI_MS_CENTROID_FILE_PATH, root.name);
    assert!(root.parameters.is_empty());
    assert!(root.data.is_empty());
    assert!(root.metadata.is_empty());
    assert_eq!(None, root.table);
    assert_eq!(
        vec![
            "Admin Data",
            "Instrument Components",
            "Sample Data",
            "Test Data",
            "Raw Data Global",
            "Raw Data Scans",
        ],
        root.child_node_names
    );

    let admin_data = &reader.read("/0").unwrap();
    assert_eq!("Admin Data", admin_data.name);
    assert_eq!(
        Parameter::from_str_str("Dataset Completeness", "C1+C2"),
        admin_data.parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("MS Template Revision", "1.0.1"),
        admin_data.parameters[1]
    );
    // no administrative_comment
    assert_eq!(
        Parameter::from_str_str("Dataset Origin", "Dummy dataset origin"),
        admin_data.parameters[2]
    );
    // no dataset_owner
    assert_eq!(
        Parameter::from_str_str("Experiment Title", "Dummy experiment title"),
        admin_data.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("Experiment Date/Time Stamp", "20231029185100+0100"),
        admin_data.parameters[4]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Experiment Type",
            AndiMsExperimentType::CentroidedMassSpectrum.to_string()
        ),
        admin_data.parameters[5]
    );
    // no experiment_x_ref_0
    // no experiment_x_ref_1
    // no experiment_x_ref_2
    // no experiment_x_ref_3
    assert_eq!(
        Parameter::from_str_str("NetCDF File Date/Time Stamp", "20231029185100+0100"),
        admin_data.parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("NetCDF Revision", "2.3.2"),
        admin_data.parameters[7]
    );
    // no operator_name
    // no source_file_reference
    // no source_file_format
    // no source_file_date_time_stamp
    assert_eq!(
        Parameter::from_str_str("Source File Reference", "Dummy source file reference"),
        admin_data.parameters[8]
    );
    // no external_file_ref_0
    // no external_file_ref_1
    // no external_file_ref_2
    // no external_file_ref_3
    assert_eq!(
        Parameter::from_str_str("Languages", "English"),
        admin_data.parameters[9]
    );
    // no number_of_times_processed
    // no number_of_times_calibrated
    // no calibration_history_0
    // no calibration_history_1
    // no calibration_history_2
    // no calibration_history_3
    // no pre_experiment_program_name
    // no post_experiment_program_name
    assert_eq!(
        Parameter::from_str_i32("Instrument Number", 1),
        admin_data.parameters[10]
    );
    assert!(admin_data.data.is_empty());
    assert!(admin_data.metadata.is_empty());
    assert_eq!(None, admin_data.table);
    assert_eq!(vec!["Error Log"], admin_data.child_node_names);

    let error_log = &reader.read("/0-admin data/0-error_log").unwrap();
    assert_eq!("Error Log", error_log.name);
    assert!(error_log.data.is_empty());
    assert!(error_log.metadata.is_empty());
    let error_log_table = error_log.table.as_ref().unwrap();
    assert_eq!(1, error_log_table.column_names.len());
    assert_eq!(
        Column::new("message", "Message"),
        error_log_table.column_names[0]
    );
    assert_eq!(1, error_log_table.rows.len());
    assert_eq!(1, error_log_table.rows[0].len());
    assert_eq!(
        &Value::String("Dummy error 1".to_owned()),
        error_log_table.rows[0].get("message").unwrap()
    );
    assert!(error_log.child_node_names.is_empty());

    let instrument_data = &reader.read("/1").unwrap();
    assert_eq!("Instrument Components", instrument_data.name);
    assert!(instrument_data.parameters.is_empty());
    assert!(instrument_data.data.is_empty());
    assert!(instrument_data.metadata.is_empty());
    assert_eq!(None, instrument_data.table);
    assert_eq!(
        vec!["Dummy instrument id (Dummy instrument name)"],
        instrument_data.child_node_names
    );

    let instrument_component_0 = &reader.read("/1/0").unwrap();
    assert_eq!(
        "Dummy instrument id (Dummy instrument name)",
        instrument_component_0.name
    );
    assert_eq!(10, instrument_component_0.parameters.len());
    assert_eq!(
        Parameter::from_str_str("Instrument Name", "Dummy instrument name"),
        instrument_component_0.parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Instrument ID", "Dummy instrument id"),
        instrument_component_0.parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Instrument Manufacturer", "Dummy instrument mfr"),
        instrument_component_0.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("Instrument Model", "Dummy instrument model"),
        instrument_component_0.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("Instrument Serial No", "Dummy instrument serial no"),
        instrument_component_0.parameters[4]
    );
    assert_eq!(
        Parameter::from_str_str("Instrument Comments", "Dummy instrument comments"),
        instrument_component_0.parameters[5]
    );
    assert_eq!(
        Parameter::from_str_str("Instrument Software Version", "Dummy instrument sw version"),
        instrument_component_0.parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Instrument Firmware Version", "Dummy instrument fw version"),
        instrument_component_0.parameters[7]
    );
    assert_eq!(
        Parameter::from_str_str("Instrument OS Version", "Dummy instrument os version"),
        instrument_component_0.parameters[8]
    );
    assert_eq!(
        Parameter::from_str_str("Instrument App Version", "Dummy instrument app version"),
        instrument_component_0.parameters[9]
    );
    assert!(instrument_component_0.data.is_empty());
    assert!(instrument_component_0.metadata.is_empty());
    assert_eq!(None, instrument_component_0.table);
    assert!(instrument_component_0.child_node_names.is_empty());

    let sample_data = &reader.read("/2").unwrap();
    assert_eq!("Sample Data", sample_data.name);
    assert_eq!(1, sample_data.parameters.len());
    assert_eq!(
        Parameter::from_str_str("Sample State", AndiMsSampleState::OtherState.to_string()),
        sample_data.parameters[0]
    );
    assert!(sample_data.data.is_empty());
    assert!(sample_data.metadata.is_empty());
    assert_eq!(None, sample_data.table);
    assert!(sample_data.child_node_names.is_empty());

    let test_data = &reader.read("/3").unwrap();
    assert_eq!("Test Data", test_data.name);
    assert_eq!(14, test_data.parameters.len());
    assert_eq!(
        Parameter::from_str_str(
            "Separation Experiment Type",
            AndiMsSeparationMethod::None.to_string()
        ),
        test_data.parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Mass Spectrometer Inlet",
            AndiMsMassSpectrometerInlet::Capillary.to_string()
        ),
        test_data.parameters[1]
    );
    // no mass_spectrometer_inlet_temperature
    assert_eq!(
        Parameter::from_str_str("Ionization Mode", AndiMsIonizationMethod::Ei.to_string()),
        test_data.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Ionization Polarity",
            AndiMsIonizationPolarity::Plus.to_string()
        ),
        test_data.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_f32("Electron Energy", 0f32),
        test_data.parameters[4]
    );
    //no laser_wavelength
    assert_eq!(
        Parameter::from_str_str("Reagent Gas", "Dummy reagent gas"),
        test_data.parameters[5]
    );
    // no reagent_gas_pressure
    // no fab_type
    // no fab_matrix
    // no source_temperature
    // no filament_current
    // no emission_current
    assert_eq!(
        Parameter::from_str_f32("Accelerating Potential", 1000f32),
        test_data.parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Detector Type", AndiMsDetectorType::Em.to_string()),
        test_data.parameters[7]
    );
    // no detector_potential
    // no detector_entrance_potential
    assert_eq!(
        Parameter::from_str_str(
            "Resolution Type",
            AndiMsResolutionType::Proportional.to_string()
        ),
        test_data.parameters[8]
    );
    assert_eq!(
        Parameter::from_str_str("Resolution Method", "Dummy test resolution method"),
        test_data.parameters[9]
    );
    assert_eq!(
        Parameter::from_str_str("Scan Function", AndiMsScanFunction::Scan.to_string()),
        test_data.parameters[10]
    );
    assert_eq!(
        Parameter::from_str_str("Scan Direction", AndiMsScanDirection::Down.to_string()),
        test_data.parameters[11]
    );
    assert_eq!(
        Parameter::from_str_str("Scan Law", AndiMsScanLaw::Exponential.to_string()),
        test_data.parameters[12]
    );
    assert_eq!(
        Parameter::from_str_f32("Scan Time", 1.2f32),
        test_data.parameters[13]
    );
    // no mass_calibration_file_name
    // no external_reference_file_name
    // no internal_reference_file_name
    // no instrument_parameter_comments
    assert!(test_data.data.is_empty());
    assert!(test_data.metadata.is_empty());
    assert_eq!(None, test_data.table);
    assert!(test_data.child_node_names.is_empty());

    let raw_data_global = &reader.read("/4").unwrap();
    assert_eq!("Raw Data Global", raw_data_global.name);
    assert_eq!(17, raw_data_global.parameters.len());
    assert_eq!(
        Parameter::from_str_i32("Scan Number", 2),
        raw_data_global.parameters[0]
    );
    // no starting_scan_number
    assert_eq!(
        Parameter::from_str_bool("Has Masses", true),
        raw_data_global.parameters[1]
    );
    assert_eq!(
        Parameter::from_str_bool("Has Times", true),
        raw_data_global.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_f64("Mass Axis Scale Factor", 1f64),
        raw_data_global.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_f64("Time Axis Scale Factor", 1f64),
        raw_data_global.parameters[4]
    );
    assert_eq!(
        Parameter::from_str_f64("Intensity Axis Scale Factor", 1f64),
        raw_data_global.parameters[5]
    );
    assert_eq!(
        Parameter::from_str_f64("Intensity Axis Offset", 0f64),
        raw_data_global.parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Mass Axis Units", AndiMsMassAxisUnit::Mz.to_string()),
        raw_data_global.parameters[7]
    );
    assert_eq!(
        Parameter::from_str_str("Time Axis Units", AndiMsTimeAxisUnit::Arbitrary.to_string()),
        raw_data_global.parameters[8]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Intensity Axis Units",
            AndiMsIntensityAxisUnit::Arbitrary.to_string()
        ),
        raw_data_global.parameters[9]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Total Intensity Units",
            AndiMsIntensityAxisUnit::Arbitrary.to_string()
        ),
        raw_data_global.parameters[10]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Mass Axis Data Format",
            AndiMsDataFormat::Double.to_string()
        ),
        raw_data_global.parameters[11]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Time Axis Data Format",
            AndiMsDataFormat::Double.to_string()
        ),
        raw_data_global.parameters[12]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Intensity Axis Data Format",
            AndiMsDataFormat::Float.to_string()
        ),
        raw_data_global.parameters[13]
    );
    // no mass_axis_label
    // no time_axis_label
    // no intensity_axis_label
    assert_eq!(
        Parameter::from_str_f64("Mass Axis Global Range Min", 2.0f64),
        raw_data_global.parameters[14]
    );
    assert_eq!(
        Parameter::from_str_f64("Mass Axis Global Range Max", 100.0f64),
        raw_data_global.parameters[15]
    );
    // no time_axis_global_range_min
    // no time_axis_global_range_max
    // no intensity_axis_global_range_min
    // no intensity_axis_global_range_max
    // no calibrated_mass_range_min
    // no calibrated_mass_range_max
    // no actual_run_time
    // no actual_delay_time
    assert_eq!(
        Parameter::from_str_bool("Uniform Sampling Flag", true),
        raw_data_global.parameters[16]
    );
    // no comments
    assert!(raw_data_global.data.is_empty());
    assert!(raw_data_global.metadata.is_empty());
    assert_eq!(None, raw_data_global.table);
    assert!(raw_data_global.child_node_names.is_empty());

    let raw_data_scans = &reader.read("/5").unwrap();
    assert_eq!("Raw Data Scans", raw_data_scans.name);
    assert!(raw_data_scans.parameters.is_empty());
    assert!(raw_data_scans.data.is_empty());
    assert!(raw_data_scans.metadata.is_empty());
    assert_eq!(None, raw_data_scans.table);
    assert_eq!(
        vec![
            "99 (m/z: 20-400, t: 0.5-0.7)",
            "100 (m/z: 20-400, t: 0.6-0.8)"
        ],
        raw_data_scans.child_node_names
    );

    let raw_data_scan_0 = &reader.read("/5/0").unwrap();
    assert_eq!("99 (m/z: 20-400, t: 0.5-0.7)", raw_data_scan_0.name);
    assert_eq!(16, raw_data_scan_0.parameters.len());
    assert_eq!(
        Parameter::from_str_str(
            "Resolution Type",
            AndiMsResolutionType::Proportional.to_string()
        ),
        raw_data_scan_0.parameters[0]
    );
    assert_eq!(
        Parameter::from_str_i32("Scan Number", 0),
        raw_data_scan_0.parameters[1]
    );
    assert_eq!(
        Parameter::from_str_i32("Actual Scan Number", 99),
        raw_data_scan_0.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_i32("Number Of Points", 4),
        raw_data_scan_0.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_i32("Number Of Flags", 2),
        raw_data_scan_0.parameters[4]
    );
    assert_eq!(
        Parameter::from_str_f64("Total Intensity", 50f64),
        raw_data_scan_0.parameters[5]
    );
    assert_eq!(
        Parameter::from_str_f64("A/D Sampling Rate", 400000f64),
        raw_data_scan_0.parameters[6]
    );
    assert_eq!(
        Parameter::from_str_i32("A/D Coaddition Factor", 3),
        raw_data_scan_0.parameters[7]
    );
    assert_eq!(
        Parameter::from_str_f64("Scan Acquisition Time", 456f64),
        raw_data_scan_0.parameters[8]
    );
    assert_eq!(
        Parameter::from_str_f64("Scan Duration", 12345f64),
        raw_data_scan_0.parameters[9]
    );
    assert_eq!(
        Parameter::from_str_f64("Mass Range Min", 20f64),
        raw_data_scan_0.parameters[10]
    );
    assert_eq!(
        Parameter::from_str_f64("Mass Range Max", 400f64),
        raw_data_scan_0.parameters[11]
    );
    assert_eq!(
        Parameter::from_str_f64("Time Range Min", 0.5f64),
        raw_data_scan_0.parameters[12]
    );
    assert_eq!(
        Parameter::from_str_f64("Time Range Max", 0.7f64),
        raw_data_scan_0.parameters[13]
    );
    assert_eq!(
        Parameter::from_str_f64("Inter Scan Time", 0.123f64),
        raw_data_scan_0.parameters[14]
    );
    assert_eq!(
        Parameter::from_str_f64("Resolution", 100f64),
        raw_data_scan_0.parameters[15]
    );
    assert_eq!(
        vec![
            PointXy::new(20.01, 1100.0),
            PointXy::new(150.02, 2200.0),
            PointXy::new(250.03, 3300.0),
            PointXy::new(399.99, 4400.0)
        ],
        raw_data_scan_0.data
    );
    assert_eq!(
        vec![
            ("x.unit".to_owned(), "M/Z".to_owned()),
            ("y.unit".to_owned(), "Arbitrary Intensity Units".to_owned()),
            ("plot.style".to_owned(), "sticks".to_owned())
        ],
        raw_data_scan_0.metadata
    );
    let raw_data_scan_0_table = raw_data_scan_0.table.as_ref().unwrap();
    assert_eq!(
        vec![
            Column::new("peak", "Peak m/z"),
            Column::new("flags", "Flags")
        ],
        raw_data_scan_0_table.column_names
    );
    assert_eq!(2, raw_data_scan_0_table.rows.len());
    let raw_data_scan_0_table_row_0_expected: HashMap<String, Value> = HashMap::from([
        ("peak".to_owned(), Value::F64(150.02)),
        (
            "flags".to_owned(),
            Value::String(AndiMsFlagValue::Exception.to_string()),
        ),
    ]);
    assert_eq!(
        raw_data_scan_0_table_row_0_expected.len(),
        raw_data_scan_0_table.rows[0].len()
    );
    for (key, value) in raw_data_scan_0_table_row_0_expected {
        assert_eq!(&value, raw_data_scan_0_table.rows[0].get(&key).unwrap());
    }
    let raw_data_scan_0_table_row_1_expected: HashMap<String, Value> = HashMap::from([
        ("peak".to_owned(), Value::F64(250.03)),
        (
            "flags".to_owned(),
            Value::String(AndiMsFlagValue::Saturated.to_string()),
        ),
    ]);
    assert_eq!(
        raw_data_scan_0_table_row_1_expected.len(),
        raw_data_scan_0_table.rows[1].len()
    );
    for (key, value) in raw_data_scan_0_table_row_1_expected {
        assert_eq!(&value, raw_data_scan_0_table.rows[1].get(&key).unwrap());
    }
    assert!(raw_data_scan_0.child_node_names.is_empty());

    // library_data
    assert!(&reader.read("/5/0/0").is_err());

    // scan_groups
    assert!(&reader.read("/6").is_err());

    // // TODO: add tests for non standard variables and attributes once available
}

#[wasm_bindgen_test]
#[test]
fn andi_ms_continuum_read_succeeds() {
    let (path, file) = open_file(ANDI_MS_CONTINUUM_FILE_PATH);
    let ms = AndiMsParser::parse(&path, file).unwrap();
    let reader = AndiMsReader::new(&path, ms);

    let root = &reader.read("/").unwrap();
    assert_eq!(ANDI_MS_CONTINUUM_FILE_PATH, root.name);
    assert!(root.parameters.is_empty());
    assert!(root.data.is_empty());
    assert!(root.metadata.is_empty());
    assert_eq!(None, root.table);
    assert_eq!(
        vec![
            "Admin Data",
            "Instrument Components",
            "Sample Data",
            "Test Data",
            "Raw Data Global",
            "Raw Data Scans",
        ],
        root.child_node_names
    );

    let admin_data = &reader.read("/0").unwrap();
    assert_eq!("Admin Data", admin_data.name);
    assert_eq!(9, admin_data.parameters.len());
    assert_eq!(
        Parameter::from_str_str("Dataset Completeness", "C1+C2"),
        admin_data.parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("MS Template Revision", "1.0.1"),
        admin_data.parameters[1]
    );
    // no administrative_comment
    assert_eq!(
        Parameter::from_str_str("Dataset Origin", "Dummy dataset origin"),
        admin_data.parameters[2]
    );
    // no dataset_owner
    // no experiment_title
    assert_eq!(
        Parameter::from_str_str("Experiment Date/Time Stamp", "20231029185100+0100"),
        admin_data.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Experiment Type",
            AndiMsExperimentType::ContinuumMassSpectrum.to_string()
        ),
        admin_data.parameters[4]
    );
    // no experiment_x_ref_0
    // no experiment_x_ref_1
    // no experiment_x_ref_2
    // no experiment_x_ref_3
    assert_eq!(
        Parameter::from_str_str("NetCDF File Date/Time Stamp", "20231029185100+0100"),
        admin_data.parameters[5]
    );
    assert_eq!(
        Parameter::from_str_str("NetCDF Revision", "2.3.2"),
        admin_data.parameters[6]
    );
    // no operator_name
    // no source_file_reference
    assert_eq!(
        Parameter::from_str_str("Source File Reference", "Dummy source file reference"),
        admin_data.parameters[7]
    );
    // no source_file_format
    // no source_file_date_time_stamp
    // no external_file_ref_0
    // no external_file_ref_1
    // no external_file_ref_2
    // no external_file_ref_3
    assert_eq!(
        Parameter::from_str_str("Languages", "English"),
        admin_data.parameters[8]
    );
    // no number_of_times_processed
    // no number_of_times_calibrated
    // no calibration_history_0
    // no calibration_history_1
    // no calibration_history_2
    // no calibration_history_3
    // no pre_experiment_program_name
    // no post_experiment_program_name
    // no instrument_number
    assert!(admin_data.data.is_empty());
    assert!(admin_data.metadata.is_empty());
    assert_eq!(None, admin_data.table);
    assert_eq!(vec!["Error Log"], admin_data.child_node_names);

    let error_log = &reader.read("/0-admin data/0-error_log").unwrap();
    assert_eq!("Error Log", error_log.name);
    assert!(error_log.data.is_empty());
    assert!(error_log.metadata.is_empty());
    let error_log_table = error_log.table.as_ref().unwrap();
    assert_eq!(1, error_log_table.column_names.len());
    assert_eq!(
        Column::new("message", "Message"),
        error_log_table.column_names[0]
    );
    assert_eq!(1, error_log_table.rows.len());
    assert_eq!(1, error_log_table.rows[0].len());
    assert_eq!(
        &Value::String("Dummy error 1".to_owned()),
        error_log_table.rows[0].get("message").unwrap()
    );
    assert!(error_log.child_node_names.is_empty());

    let instrument_data = &reader.read("/1").unwrap();
    assert_eq!("Instrument Components", instrument_data.name);
    assert!(instrument_data.parameters.is_empty());
    assert!(instrument_data.data.is_empty());
    assert!(instrument_data.metadata.is_empty());
    assert_eq!(None, instrument_data.table);
    assert!(instrument_data.child_node_names.is_empty());

    let sample_data = &reader.read("/2").unwrap();
    assert_eq!("Sample Data", sample_data.name);
    assert_eq!(1, sample_data.parameters.len());
    assert_eq!(
        Parameter::from_str_str("Sample State", AndiMsSampleState::OtherState.to_string()),
        sample_data.parameters[0]
    );
    assert!(sample_data.data.is_empty());
    assert!(sample_data.metadata.is_empty());
    assert_eq!(None, sample_data.table);
    assert!(sample_data.child_node_names.is_empty());

    let test_data = &reader.read("/3").unwrap();
    assert_eq!("Test Data", test_data.name);
    assert_eq!(9, test_data.parameters.len());
    assert_eq!(
        Parameter::from_str_str(
            "Separation Experiment Type",
            AndiMsSeparationMethod::None.to_string()
        ),
        test_data.parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Mass Spectrometer Inlet",
            AndiMsMassSpectrometerInlet::Direct.to_string()
        ),
        test_data.parameters[1]
    );
    // no mass_spectrometer_inlet_temperature
    assert_eq!(
        Parameter::from_str_str("Ionization Mode", AndiMsIonizationMethod::Ei.to_string()),
        test_data.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Ionization Polarity",
            AndiMsIonizationPolarity::Plus.to_string()
        ),
        test_data.parameters[3]
    );
    // no electron_energy
    // no laser_wavelength
    // no reagent_gas
    // no reagent_gas_pressure
    // no fab_type
    // no fab_matrix
    // no source_temperature
    // no filament_current
    // no emission_current
    // no accelerating_potential
    assert_eq!(
        Parameter::from_str_str("Detector Type", AndiMsDetectorType::Em.to_string()),
        test_data.parameters[4]
    );
    // no detector_potential
    // no detector_entrance_potential
    assert_eq!(
        Parameter::from_str_str(
            "Resolution Type",
            AndiMsResolutionType::Constant.to_string()
        ),
        test_data.parameters[5]
    );
    // no resolution_method
    assert_eq!(
        Parameter::from_str_str("Scan Function", AndiMsScanFunction::Scan.to_string()),
        test_data.parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Scan Direction", AndiMsScanDirection::Up.to_string()),
        test_data.parameters[7]
    );
    assert_eq!(
        Parameter::from_str_str("Scan Law", AndiMsScanLaw::Linear.to_string()),
        test_data.parameters[8]
    );
    // no scan_time
    // no mass_calibration_file_name
    // no external_reference_file_name
    // no internal_reference_file_name
    // no instrument_parameter_comments
    assert!(test_data.data.is_empty());
    assert!(test_data.metadata.is_empty());
    assert_eq!(None, test_data.table);
    assert!(test_data.child_node_names.is_empty());

    let raw_data_global = &reader.read("/4").unwrap();
    assert_eq!("Raw Data Global", raw_data_global.name);
    assert_eq!(15, raw_data_global.parameters.len());
    assert_eq!(
        Parameter::from_str_i32("Scan Number", 2),
        raw_data_global.parameters[0]
    );
    // no starting_scan_number
    assert_eq!(
        Parameter::from_str_bool("Has Masses", true),
        raw_data_global.parameters[1]
    );
    assert_eq!(
        Parameter::from_str_bool("Has Times", false),
        raw_data_global.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_f64("Mass Axis Scale Factor", 1f64),
        raw_data_global.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_f64("Time Axis Scale Factor", 1f64),
        raw_data_global.parameters[4]
    );
    assert_eq!(
        Parameter::from_str_f64("Intensity Axis Scale Factor", 1f64),
        raw_data_global.parameters[5]
    );
    assert_eq!(
        Parameter::from_str_f64("Intensity Axis Offset", 0f64),
        raw_data_global.parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Mass Axis Units", AndiMsMassAxisUnit::Mz.to_string()),
        raw_data_global.parameters[7]
    );
    assert_eq!(
        Parameter::from_str_str("Time Axis Units", AndiMsTimeAxisUnit::Seconds.to_string()),
        raw_data_global.parameters[8]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Intensity Axis Units",
            AndiMsIntensityAxisUnit::Arbitrary.to_string()
        ),
        raw_data_global.parameters[9]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Total Intensity Units",
            AndiMsIntensityAxisUnit::Arbitrary.to_string()
        ),
        raw_data_global.parameters[10]
    );
    assert_eq!(
        Parameter::from_str_str("Mass Axis Data Format", AndiMsDataFormat::Float.to_string()),
        raw_data_global.parameters[11]
    );
    assert_eq!(
        Parameter::from_str_str("Time Axis Data Format", AndiMsDataFormat::Float.to_string()),
        raw_data_global.parameters[12]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Intensity Axis Data Format",
            AndiMsDataFormat::Float.to_string()
        ),
        raw_data_global.parameters[13]
    );
    // no mass_axis_label
    // no time_axis_label
    // no intensity_axis_label
    // no mass_axis_global_range_min
    // no mass_axis_global_range_max
    // no time_axis_global_range_min
    // no time_axis_global_range_max
    // no intensity_axis_global_range_min
    // no intensity_axis_global_range_max
    // no calibrated_mass_range_min
    // no calibrated_mass_range_max
    // no actual_run_time
    // no actual_delay_time
    assert_eq!(
        Parameter::from_str_bool("Uniform Sampling Flag", true),
        raw_data_global.parameters[14]
    );
    // no comments
    assert!(raw_data_global.data.is_empty());
    assert!(raw_data_global.metadata.is_empty());
    assert_eq!(None, raw_data_global.table);
    assert!(raw_data_global.child_node_names.is_empty());

    let raw_data_scans = &reader.read("/5").unwrap();
    assert_eq!("Raw Data Scans", raw_data_scans.name);
    assert!(raw_data_scans.parameters.is_empty());
    assert!(raw_data_scans.data.is_empty());
    assert!(raw_data_scans.metadata.is_empty());
    assert_eq!(None, raw_data_scans.table);
    assert_eq!(
        vec!["0 (m/z: 35-35.9)", "1 (m/z: 35-35.9)"],
        raw_data_scans.child_node_names
    );

    let raw_data_scan_0 = &reader.read("/5/0").unwrap();
    assert_eq!("0 (m/z: 35-35.9)", raw_data_scan_0.name);
    assert_eq!(8, raw_data_scan_0.parameters.len());
    assert_eq!(
        Parameter::from_str_str(
            "Resolution Type",
            AndiMsResolutionType::Constant.to_string()
        ),
        raw_data_scan_0.parameters[0]
    );
    assert_eq!(
        Parameter::from_str_i32("Scan Number", 0),
        raw_data_scan_0.parameters[1]
    );
    assert_eq!(
        Parameter::from_str_i32("Actual Scan Number", 0),
        raw_data_scan_0.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_i32("Number Of Points", 10),
        raw_data_scan_0.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_i32("Number Of Flags", 0),
        raw_data_scan_0.parameters[4]
    );
    // no total_intensity
    // no a_d_sampling_rate
    // no a_d_coaddition_factor
    assert_eq!(
        Parameter::from_str_f64("Scan Acquisition Time", 0.1f64),
        raw_data_scan_0.parameters[5]
    );
    // no scan_duration
    assert_eq!(
        Parameter::from_str_f64("Mass Range Min", 35f64),
        raw_data_scan_0.parameters[6]
    );
    assert_eq!(
        Parameter::from_str_f64("Mass Range Max", 35.9f64),
        raw_data_scan_0.parameters[7]
    );
    // no time_range_min
    // no time_range_max
    // no inter_scan_time
    // no resolution
    assert_eq!(
        vec![
            PointXy::new(35.0f32 as f64, 1.0e-03f32 as f64),
            PointXy::new(35.1f32 as f64, 1.1e-03f32 as f64),
            PointXy::new(35.2f32 as f64, 1.0e-03f32 as f64),
            PointXy::new(35.3f32 as f64, 1.0e-01f32 as f64),
            PointXy::new(35.4f32 as f64, 1.0e+01f32 as f64),
            PointXy::new(35.5f32 as f64, 1.0e+03f32 as f64),
            PointXy::new(35.6f32 as f64, 1.0e+01f32 as f64),
            PointXy::new(35.7f32 as f64, 1.0e-01f32 as f64),
            PointXy::new(35.8f32 as f64, 1.0e-03f32 as f64),
            PointXy::new(35.9f32 as f64, 1.1e-03f32 as f64),
        ],
        raw_data_scan_0.data
    );
    assert_eq!(
        vec![
            ("x.unit".to_owned(), "M/Z".to_owned()),
            ("y.unit".to_owned(), "Arbitrary Intensity Units".to_owned()),
        ],
        raw_data_scan_0.metadata
    );
    let raw_data_scan_0_table = raw_data_scan_0.table.as_ref().unwrap();
    assert_eq!(
        vec![
            Column::new("peak", "Peak m/z"),
            Column::new("flags", "Flags")
        ],
        raw_data_scan_0_table.column_names
    );
    assert!(raw_data_scan_0_table.rows.is_empty());
    assert!(raw_data_scan_0.child_node_names.is_empty());

    // library_data
    assert!(&reader.read("/5/0/0").is_err());

    // scan_groups
    assert!(&reader.read("/6").is_err());

    // // TODO: add tests for non standard variables and attributes once available
}

#[wasm_bindgen_test]
#[test]
fn andi_ms_read_illegal_node_path_fails() {
    let (path, file) = open_file(ANDI_MS_CENTROID_FILE_PATH);
    let ms = AndiMsParser::parse(&path, file).unwrap();
    let reader = AndiMsReader::new(&path, ms);

    let illegal_path_data = reader.read("/7");
    assert!(illegal_path_data.is_err());
}
