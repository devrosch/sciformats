mod io;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use crate::io::open_file;
use sf_rs::{
    andi_chrom_parser::AndiChromParser,
    andi_chrom_reader::AndiChromReader,
    api::{Parser, Reader},
};
use wasm_bindgen_test::wasm_bindgen_test;

fn assert_eq_f64(left: f64, right: f64) {
    let max = left.max(right);
    let epsilon = f32::EPSILON as f64 * max;
    assert!(f64::abs(left - right) <= epsilon)
}

const ANDI_CHROM_VALID_FILE_PATH: &str = "andi_chrom_valid.cdf";

#[wasm_bindgen_test]
#[test]
fn andi_chrom_read_valid_succeeds() {
    let (path, file) = open_file(ANDI_CHROM_VALID_FILE_PATH);
    let chrom = AndiChromParser::parse(&path, file).unwrap();
    let reader = AndiChromReader::new(&path, chrom);

    let root = &reader.read("/").unwrap();
    assert_eq!(ANDI_CHROM_VALID_FILE_PATH, root.name);
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
        ("Dataset Completeness".to_owned(), "C1+C2".to_owned()),
        admin_data.parameters[0]
    );
    assert_eq!(
        ("Protocol Template Revision".to_owned(), "1.0".to_owned()),
        admin_data.parameters[1]
    );
    assert_eq!(
        ("NetCDF Revision".to_owned(), "2.0".to_owned()),
        admin_data.parameters[2]
    );
    assert_eq!(
        ("Languages".to_owned(), "English".to_owned()),
        admin_data.parameters[3]
    );
    assert_eq!(
        (
            "Administrative Comments".to_owned(),
            "dummy admin comment".to_owned()
        ),
        admin_data.parameters[4]
    );
    assert_eq!(
        ("Dataset Origin".to_owned(), "sf_rs".to_owned()),
        admin_data.parameters[5]
    );
    assert_eq!(
        ("Dataset Owner".to_owned(), "Robert".to_owned()),
        admin_data.parameters[6]
    );
    assert_eq!(
        (
            "Dataset Date/Time Stamp".to_owned(),
            "20230908200501+0200".to_owned()
        ),
        admin_data.parameters[7]
    );
    assert_eq!(
        (
            "Injection Date/Time Stamp".to_owned(),
            "20230908200501+0200".to_owned()
        ),
        admin_data.parameters[8]
    );
    assert_eq!(
        (
            "Experiment Title".to_owned(),
            "sf_rs sample file".to_owned()
        ),
        admin_data.parameters[9]
    );
    assert_eq!(
        ("Operator Name".to_owned(), "Rob".to_owned()),
        admin_data.parameters[10]
    );
    assert_eq!(
        (
            "Separation Experiment Type".to_owned(),
            "liquid chromatography".to_owned()
        ),
        admin_data.parameters[11]
    );
    assert_eq!(
        (
            "Company Method Name".to_owned(),
            "dummy company method 1".to_owned()
        ),
        admin_data.parameters[12]
    );
    assert_eq!(
        ("Company Method ID".to_owned(), "1".to_owned()),
        admin_data.parameters[13]
    );
    assert_eq!(
        (
            "Pre Experiment Program Name".to_owned(),
            "dummy pre exp prog name".to_owned()
        ),
        admin_data.parameters[14]
    );
    assert_eq!(
        (
            "Post Experiment Program Name".to_owned(),
            "dummy post exp prog name".to_owned()
        ),
        admin_data.parameters[15]
    );
    assert_eq!(
        (
            "Source File Reference".to_owned(),
            "dummy source file reference".to_owned()
        ),
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
        ("message".to_owned(), "Message".to_owned()),
        error_log_table.column_names[0]
    );
    assert_eq!(1, error_log_table.rows[0].len());
    assert_eq!("error 1", error_log_table.rows[0].get("message").unwrap());
    assert_eq!(1, error_log_table.rows[1].len());
    assert_eq!("error 2", error_log_table.rows[1].get("message").unwrap());
    assert!(error_log.child_node_names.is_empty());

    let sample_description = &reader.read("/1- some name").unwrap();
    assert_eq!("Sample Description", sample_description.name);
    assert_eq!(
        (
            "Sample ID Comments".to_owned(),
            "dummy sample id comments".to_owned()
        ),
        sample_description.parameters[0]
    );
    assert_eq!(
        ("Sample ID".to_owned(), "12345".to_owned()),
        sample_description.parameters[1]
    );
    assert_eq!(
        ("Sample Name".to_owned(), "dummy sample name".to_owned()),
        sample_description.parameters[2]
    );
    assert_eq!(
        ("Sample Type".to_owned(), "test".to_owned()),
        sample_description.parameters[3]
    );
    assert_eq!(
        ("Sample Injection Volume".to_owned(), "1".to_owned()),
        sample_description.parameters[4]
    );
    assert_eq!(
        ("Sample Amount".to_owned(), "2.2".to_owned()),
        sample_description.parameters[5]
    );
    assert!(sample_description.data.is_empty());
    assert!(sample_description.metadata.is_empty());
    assert_eq!(None, sample_description.table);
    assert!(sample_description.child_node_names.is_empty());

    let detection_method = &reader.read("/2").unwrap();
    assert_eq!(
        (
            "Detection Method Table Name".to_owned(),
            "dummy method table name".to_owned()
        ),
        detection_method.parameters[0]
    );
    assert_eq!(
        (
            "Detector Method Comments".to_owned(),
            "dummy detector method comments".to_owned()
        ),
        detection_method.parameters[1]
    );
    assert_eq!(
        (
            "Detection Method Name".to_owned(),
            "dummy detection method 1".to_owned()
        ),
        detection_method.parameters[2]
    );
    assert_eq!(
        ("Detector Name".to_owned(), "dummy detector name".to_owned()),
        detection_method.parameters[3]
    );
    assert_eq!(
        ("Detector Maximum Value".to_owned(), 999999.0f32.to_string()),
        detection_method.parameters[4]
    );
    assert_eq!(
        ("Detector Minimum Value".to_owned(), 1.0f32.to_string()),
        detection_method.parameters[5]
    );
    assert_eq!(
        ("Detector Unit".to_owned(), "au".to_owned()),
        detection_method.parameters[6]
    );
    assert!(detection_method.data.is_empty());
    assert!(detection_method.metadata.is_empty());
    assert_eq!(None, detection_method.table);
    assert!(detection_method.child_node_names.is_empty());

    let raw_data = &reader.read("/3").unwrap();
    assert_eq!(
        ("Point Number".to_owned(), 10.to_string()),
        raw_data.parameters[0]
    );
    assert_eq!(
        (
            "Raw Data Table Name".to_owned(),
            "dummy raw data table name".to_owned()
        ),
        raw_data.parameters[1]
    );
    assert_eq!(
        ("Retention Unit".to_owned(), "seconds".to_owned()),
        raw_data.parameters[2]
    );
    assert_eq!(
        ("Actual Run Time Length".to_owned(), "100".to_owned()),
        raw_data.parameters[3]
    );
    assert_eq!(
        ("Actual Sampling Interval".to_owned(), "10".to_owned()),
        raw_data.parameters[4]
    );
    assert_eq!(
        ("Actual Delay Time".to_owned(), "0".to_owned()),
        raw_data.parameters[5]
    );
    assert_eq!(
        ("Uniform Sampling Flag".to_owned(), "true".to_owned()),
        raw_data.parameters[6]
    );
    assert_eq!(
        ("Autosampler Position".to_owned(), "1:2".to_owned()),
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
        assert_eq_f64(expect_data[i].0, raw_data.data[i].0);
        assert_eq_f64(expect_data[i].1, raw_data.data[i].1);
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
        ("Peak Number".to_owned(), "3".to_owned()),
        peak_processing_results.parameters[0]
    );
    assert_eq!(
        (
            "Peak Processing Results Table Name".to_owned(),
            "dummy pp res table name".to_owned()
        ),
        peak_processing_results.parameters[1]
    );

    assert_eq!(
        (
            "Peak Processing Results Comments".to_owned(),
            "dummy pp res comments".to_owned()
        ),
        peak_processing_results.parameters[2]
    );
    assert_eq!(
        (
            "Peak Processing Method Name".to_owned(),
            "dummy pp method name".to_owned()
        ),
        peak_processing_results.parameters[3]
    );
    assert_eq!(
        (
            "Peak Processing Date Time Stamp".to_owned(),
            "20230908201502+0200".to_owned()
        ),
        peak_processing_results.parameters[4]
    );
    assert_eq!(
        ("Peak Amount Unit".to_owned(), "ppm".to_owned()),
        peak_processing_results.parameters[5]
    );

    let column_names = &peak_processing_results.table.as_ref().unwrap().column_names;
    assert_eq!(15, column_names.len());
    assert_eq!(
        (
            "peak_retention_time".to_owned(),
            "Peak Retention Time".to_owned()
        ),
        column_names[0]
    );
    assert_eq!(
        ("peak_name".to_owned(), "Peak Name".to_owned()),
        column_names[1]
    );
    assert_eq!(
        ("peak_amount".to_owned(), "Peak Amount".to_owned()),
        column_names[2]
    );
    assert_eq!(
        ("peak_start_time".to_owned(), "Peak Start Time".to_owned()),
        column_names[3]
    );
    assert_eq!(
        ("peak_end_time".to_owned(), "Peak End Time".to_owned()),
        column_names[4]
    );
    assert_eq!(
        ("peak_area".to_owned(), "Peak Area".to_owned()),
        column_names[5]
    );
    assert_eq!(
        ("peak_height".to_owned(), "Peak Height".to_owned()),
        column_names[6]
    );
    assert_eq!(
        (
            "baseline_start_value".to_owned(),
            "Baseline Start Value".to_owned()
        ),
        column_names[7]
    );
    assert_eq!(
        (
            "baseline_stop_value".to_owned(),
            "Baseline Stop Value".to_owned()
        ),
        column_names[8]
    );
    assert_eq!(
        (
            "peak_start_detection_code".to_owned(),
            "Peak Start Detection Code".to_owned()
        ),
        column_names[9]
    );
    assert_eq!(
        (
            "peak_stop_detection_code".to_owned(),
            "Peak Stop Detection Code".to_owned()
        ),
        column_names[10]
    );
    assert_eq!(
        (
            "manually_reintegrated_peaks".to_owned(),
            "Manually Reintegrated Peak".to_owned()
        ),
        column_names[11]
    );
    assert_eq!(
        (
            "peak_retention_unit".to_owned(),
            "Peak Retention Unit".to_owned()
        ),
        column_names[12]
    );
    assert_eq!(
        ("peak_amount_unit".to_owned(), "Peak Amount Unit".to_owned()),
        column_names[13]
    );
    assert_eq!(
        ("detector_unit".to_owned(), "Detector Unit".to_owned()),
        column_names[14]
    );

    assert_eq!(
        3,
        peak_processing_results.table.as_ref().unwrap().rows.len()
    );

    let peak_0 = &peak_processing_results.table.as_ref().unwrap().rows[0];
    assert_eq!(15, peak_0.keys().len());
    assert_eq!("10.111", peak_0["peak_retention_time"]);
    assert_eq!("ref", peak_0["peak_name"]);
    assert_eq!("110.1111", peak_0["peak_amount"]);
    assert_eq!("8", peak_0["peak_start_time"]);
    assert_eq!("12", peak_0["peak_end_time"]);
    assert_eq!("111", peak_0["peak_area"]);
    assert_eq!("111111.1", peak_0["peak_height"]);
    assert_eq!("5", peak_0["baseline_start_value"]);
    assert_eq!("7", peak_0["baseline_stop_value"]);
    assert_eq!("A", peak_0["peak_start_detection_code"]);
    assert_eq!("X", peak_0["peak_stop_detection_code"]);
    assert_eq!("false", peak_0["manually_reintegrated_peaks"]);
    assert_eq!("seconds", peak_0["peak_retention_unit"]);
    assert_eq!("ppm", peak_0["peak_amount_unit"]);
    assert_eq!("au", peak_0["detector_unit"]);

    // skip peak 1

    let peak_2 = &peak_processing_results.table.as_ref().unwrap().rows[2];
    assert_eq!(15, peak_2.keys().len());
    assert_eq!("50.333", peak_2["peak_retention_time"]);
    assert_eq!("peak name 2", peak_2["peak_name"]);
    assert_eq!("330.333", peak_2["peak_amount"]);
    assert_eq!("48", peak_2["peak_start_time"]);
    assert_eq!("52", peak_2["peak_end_time"]);
    assert_eq!("333", peak_2["peak_area"]);
    assert_eq!("133333.3", peak_2["peak_height"]);
    assert_eq!("7", peak_2["baseline_start_value"]);
    assert_eq!("5", peak_2["baseline_stop_value"]);
    assert_eq!("C", peak_2["peak_start_detection_code"]);
    assert_eq!("Z", peak_2["peak_stop_detection_code"]);
    assert_eq!("false", peak_2["manually_reintegrated_peaks"]);
    assert_eq!("seconds", peak_2["peak_retention_unit"]);
    assert_eq!("ppm", peak_2["peak_amount_unit"]);
    assert_eq!("au", peak_2["detector_unit"]);

    // TODO: add tests for non standard variables and attributes once available
}

#[wasm_bindgen_test]
#[test]
fn andi_chrom_read_quirks() {
    let (path, file) = open_file("andi_chrom_quirks.cdf");
    let chrom = AndiChromParser::parse(&path, file).unwrap();
    let reader = AndiChromReader::new(&path, chrom);

    let raw_data = reader.read("/3");
    assert!(raw_data.is_ok());
}
