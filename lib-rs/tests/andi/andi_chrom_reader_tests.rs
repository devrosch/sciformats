use super::{open_file, ANDI_CHROM_QUIRKS, ANDI_CHROM_VALID};
use sf_rs::{
    andi::{andi_chrom_parser::AndiChromParser, andi_chrom_reader::AndiChromReader},
    api::{Column, Parameter, Parser, Reader, Value},
};

fn assert_eq_f64(left: f64, right: f64) {
    let max = left.max(right);
    let epsilon = f32::EPSILON as f64 * max;
    assert!(f64::abs(left - right) <= epsilon)
}

#[test]
fn andi_chrom_read_valid_succeeds() {
    let (path, file) = open_file(ANDI_CHROM_VALID);
    let chrom = AndiChromParser::parse(&path, file).unwrap();
    let reader = AndiChromReader::new(&path, chrom);

    let root = &reader.read("/").unwrap();
    assert_eq!(ANDI_CHROM_VALID, root.name);
    assert!(root.parameters.is_empty());
    assert!(root.data.is_empty());
    assert!(root.metadata.is_empty());
    assert_eq!(None, root.table);
    assert_eq!(
        vec![
            "Admin Data",
            "Sample Description",
            "Detection Method",
            "Raw Data",
            "Peak Processing Results"
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
        Parameter::from_str_str("Protocol Template Revision", "1.0"),
        admin_data.parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("NetCDF Revision", "2.0"),
        admin_data.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("Languages", "English"),
        admin_data.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("Administrative Comments", "dummy admin comment"),
        admin_data.parameters[4]
    );
    assert_eq!(
        Parameter::from_str_str("Dataset Origin", "sf_rs"),
        admin_data.parameters[5]
    );
    assert_eq!(
        Parameter::from_str_str("Dataset Owner", "Robert"),
        admin_data.parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Dataset Date/Time Stamp", "20230908200501+0200"),
        admin_data.parameters[7]
    );
    assert_eq!(
        Parameter::from_str_str("Injection Date/Time Stamp", "20230908200501+0200"),
        admin_data.parameters[8]
    );
    assert_eq!(
        Parameter::from_str_str("Experiment Title", "sf_rs sample file"),
        admin_data.parameters[9]
    );
    assert_eq!(
        Parameter::from_str_str("Operator Name", "Rob"),
        admin_data.parameters[10]
    );
    assert_eq!(
        Parameter::from_str_str("Separation Experiment Type", "liquid chromatography"),
        admin_data.parameters[11]
    );
    assert_eq!(
        Parameter::from_str_str("Company Method Name", "dummy company method 1"),
        admin_data.parameters[12]
    );
    assert_eq!(
        Parameter::from_str_str("Company Method ID", "1"),
        admin_data.parameters[13]
    );
    assert_eq!(
        Parameter::from_str_str("Pre Experiment Program Name", "dummy pre exp prog name"),
        admin_data.parameters[14]
    );
    assert_eq!(
        Parameter::from_str_str("Post Experiment Program Name", "dummy post exp prog name"),
        admin_data.parameters[15]
    );
    assert_eq!(
        Parameter::from_str_str("Source File Reference", "dummy source file reference"),
        admin_data.parameters[16]
    );
    assert!(admin_data.data.is_empty());
    assert!(admin_data.metadata.is_empty());
    assert_eq!(None, admin_data.table);
    assert_eq!(vec!["Error Log",], admin_data.child_node_names);

    let error_log = &reader.read("/0-root name/0-error_log").unwrap();
    assert_eq!("Error Log", error_log.name);
    assert!(error_log.data.is_empty());
    assert!(error_log.metadata.is_empty());
    let error_log_table = error_log.table.as_ref().unwrap();
    assert_eq!(1, error_log_table.column_names.len());
    assert_eq!(
        Column::new("message", "Message"),
        error_log_table.column_names[0]
    );
    assert_eq!(1, error_log_table.rows[0].len());
    assert_eq!(
        &Value::String("error 1".to_owned()),
        error_log_table.rows[0].get("message").unwrap()
    );
    assert_eq!(1, error_log_table.rows[1].len());
    assert_eq!(
        &Value::String("error 2".to_owned()),
        error_log_table.rows[1].get("message").unwrap()
    );
    assert!(error_log.child_node_names.is_empty());

    let sample_description = &reader.read("/1- some name").unwrap();
    assert_eq!("Sample Description", sample_description.name);
    assert_eq!(
        Parameter::from_str_str("Sample ID Comments", "dummy sample id comments"),
        sample_description.parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Sample ID", "12345"),
        sample_description.parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Sample Name", "dummy sample name"),
        sample_description.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("Sample Type", "test"),
        sample_description.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_f32("Sample Injection Volume", 1.0f32),
        sample_description.parameters[4]
    );
    assert_eq!(
        Parameter::from_str_f32("Sample Amount", 2.2f32),
        sample_description.parameters[5]
    );
    assert!(sample_description.data.is_empty());
    assert!(sample_description.metadata.is_empty());
    assert_eq!(None, sample_description.table);
    assert!(sample_description.child_node_names.is_empty());

    let detection_method = &reader.read("/2").unwrap();
    assert_eq!(
        Parameter::from_str_str("Detection Method Table Name", "dummy method table name"),
        detection_method.parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Detector Method Comments", "dummy detector method comments"),
        detection_method.parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Detection Method Name", "dummy detection method 1"),
        detection_method.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("Detector Name", "dummy detector name"),
        detection_method.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_f32("Detector Maximum Value", 999999.0f32),
        detection_method.parameters[4]
    );
    assert_eq!(
        Parameter::from_str_f32("Detector Minimum Value", 1.0f32),
        detection_method.parameters[5]
    );
    assert_eq!(
        Parameter::from_str_str("Detector Unit", "au"),
        detection_method.parameters[6]
    );
    assert!(detection_method.data.is_empty());
    assert!(detection_method.metadata.is_empty());
    assert_eq!(None, detection_method.table);
    assert!(detection_method.child_node_names.is_empty());

    let raw_data = &reader.read("/3").unwrap();
    assert_eq!(
        Parameter::from_str_i32("Point Number", 10i32),
        raw_data.parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Raw Data Table Name", "dummy raw data table name"),
        raw_data.parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Retention Unit", "seconds"),
        raw_data.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_f32("Actual Run Time Length", 100f32),
        raw_data.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_f32("Actual Sampling Interval", 10f32),
        raw_data.parameters[4]
    );
    assert_eq!(
        Parameter::from_str_f32("Actual Delay Time", 0f32),
        raw_data.parameters[5]
    );
    assert_eq!(
        Parameter::from_str_bool("Uniform Sampling Flag", true),
        raw_data.parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Autosampler Position", "1:2"),
        raw_data.parameters[7]
    );
    assert_eq!(10, raw_data.data.len());
    let expect_data = vec![
        (0f64, 10000f64),
        (10f64, 111111.1f64),
        (20f64, 10000f64),
        (30f64, 122222.2f64),
        (40f64, 10000f64),
        (50f64, 133333.3f64),
        (60f64, 10000f64),
        (70f64, 10000f64),
        (80f64, 10000f64),
        (90f64, 10000f64),
    ];
    for i in 0..10 {
        assert_eq_f64(expect_data[i].0, raw_data.data[i].x);
        assert_eq_f64(expect_data[i].1, raw_data.data[i].y);
    }
    assert_eq!(2, raw_data.metadata.len());
    assert_eq!(
        ("x.unit".to_owned(), "seconds".to_owned()),
        raw_data.metadata[0]
    );
    assert_eq!(("y.unit".to_owned(), "au".to_owned()), raw_data.metadata[1]);
    assert_eq!(None, raw_data.table);
    assert!(raw_data.child_node_names.is_empty());

    let peak_processing_results = &reader.read("/4").unwrap();
    assert_eq!(
        Parameter::from_str_i32("Peak Number", 3),
        peak_processing_results.parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Peak Processing Results Table Name",
            "dummy pp res table name"
        ),
        peak_processing_results.parameters[1]
    );

    assert_eq!(
        Parameter::from_str_str("Peak Processing Results Comments", "dummy pp res comments"),
        peak_processing_results.parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("Peak Processing Method Name", "dummy pp method name"),
        peak_processing_results.parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("Peak Processing Date Time Stamp", "20230908201502+0200"),
        peak_processing_results.parameters[4]
    );
    assert_eq!(
        Parameter::from_str_str("Peak Amount Unit", "ppm"),
        peak_processing_results.parameters[5]
    );

    let column_names = &peak_processing_results.table.as_ref().unwrap().column_names;
    assert_eq!(15, column_names.len());
    assert_eq!(
        Column::new("peak_retention_time", "Peak Retention Time"),
        column_names[0]
    );
    assert_eq!(Column::new("peak_name", "Peak Name"), column_names[1]);
    assert_eq!(Column::new("peak_amount", "Peak Amount"), column_names[2]);
    assert_eq!(
        Column::new("peak_start_time", "Peak Start Time"),
        column_names[3]
    );
    assert_eq!(
        Column::new("peak_end_time", "Peak End Time"),
        column_names[4]
    );
    assert_eq!(Column::new("peak_area", "Peak Area"), column_names[5]);
    assert_eq!(Column::new("peak_height", "Peak Height"), column_names[6]);
    assert_eq!(
        Column::new("baseline_start_value", "Baseline Start Value"),
        column_names[7]
    );
    assert_eq!(
        Column::new("baseline_stop_value", "Baseline Stop Value"),
        column_names[8]
    );
    assert_eq!(
        Column::new("peak_start_detection_code", "Peak Start Detection Code"),
        column_names[9]
    );
    assert_eq!(
        Column::new("peak_stop_detection_code", "Peak Stop Detection Code"),
        column_names[10]
    );
    assert_eq!(
        Column::new("manually_reintegrated_peaks", "Manually Reintegrated Peak"),
        column_names[11]
    );
    assert_eq!(
        Column::new("peak_retention_unit", "Peak Retention Unit"),
        column_names[12]
    );
    assert_eq!(
        Column::new("peak_amount_unit", "Peak Amount Unit"),
        column_names[13]
    );
    assert_eq!(
        Column::new("detector_unit", "Detector Unit"),
        column_names[14]
    );

    assert_eq!(
        3,
        peak_processing_results.table.as_ref().unwrap().rows.len()
    );

    let peak_0 = &peak_processing_results.table.as_ref().unwrap().rows[0];
    assert_eq!(15, peak_0.keys().len());
    assert_eq!(Value::F32(10.111), peak_0["peak_retention_time"]);
    assert_eq!(Value::String("ref".to_owned()), peak_0["peak_name"]);
    assert_eq!(Value::F32(110.1111), peak_0["peak_amount"]);
    assert_eq!(Value::F32(8.0), peak_0["peak_start_time"]);
    assert_eq!(Value::F32(12.0), peak_0["peak_end_time"]);
    assert_eq!(Value::F32(111.0), peak_0["peak_area"]);
    assert_eq!(Value::F32(111111.1), peak_0["peak_height"]);
    assert_eq!(Value::F32(5.0), peak_0["baseline_start_value"]);
    assert_eq!(Value::F32(7.0), peak_0["baseline_stop_value"]);
    assert_eq!(
        Value::String("A".to_owned()),
        peak_0["peak_start_detection_code"]
    );
    assert_eq!(
        Value::String("X".to_owned()),
        peak_0["peak_stop_detection_code"]
    );
    assert_eq!(Value::Bool(false), peak_0["manually_reintegrated_peaks"]);
    assert_eq!(
        Value::String("seconds".to_owned()),
        peak_0["peak_retention_unit"]
    );
    assert_eq!(Value::String("ppm".to_owned()), peak_0["peak_amount_unit"]);
    assert_eq!(Value::String("au".to_owned()), peak_0["detector_unit"]);

    // skip peak 1

    let peak_2 = &peak_processing_results.table.as_ref().unwrap().rows[2];
    assert_eq!(15, peak_2.keys().len());
    assert_eq!(Value::F32(50.333), peak_2["peak_retention_time"]);
    assert_eq!(Value::String("peak name 2".to_owned()), peak_2["peak_name"]);
    assert_eq!(Value::F32(330.333), peak_2["peak_amount"]);
    assert_eq!(Value::F32(48.0), peak_2["peak_start_time"]);
    assert_eq!(Value::F32(52.0), peak_2["peak_end_time"]);
    assert_eq!(Value::F32(333.0), peak_2["peak_area"]);
    assert_eq!(Value::F32(133333.3), peak_2["peak_height"]);
    assert_eq!(Value::F32(7.0), peak_2["baseline_start_value"]);
    assert_eq!(Value::F32(5.0), peak_2["baseline_stop_value"]);
    assert_eq!(
        Value::String("C".to_owned()),
        peak_2["peak_start_detection_code"]
    );
    assert_eq!(
        Value::String("Z".to_owned()),
        peak_2["peak_stop_detection_code"]
    );
    assert_eq!(Value::Bool(false), peak_2["manually_reintegrated_peaks"]);
    assert_eq!(
        Value::String("seconds".to_owned()),
        peak_2["peak_retention_unit"]
    );
    assert_eq!(Value::String("ppm".to_owned()), peak_2["peak_amount_unit"]);
    assert_eq!(Value::String("au".to_owned()), peak_2["detector_unit"]);

    // TODO: add tests for non standard variables and attributes once available
}

#[test]
fn andi_chrom_read_quirks() {
    let (path, file) = open_file(ANDI_CHROM_QUIRKS);
    let chrom = AndiChromParser::parse(&path, file).unwrap();
    let reader = AndiChromReader::new(&path, chrom);

    let raw_data = reader.read("/3");
    assert!(raw_data.is_ok());
}

#[test]
fn andi_chrom_read_illegal_node_path_fails() {
    let (path, file) = open_file(ANDI_CHROM_VALID);
    let chrom = AndiChromParser::parse(&path, file).unwrap();
    let reader = AndiChromReader::new(&path, chrom);

    let illegal_path_data = reader.read("/5");
    assert!(illegal_path_data.is_err());
}
