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

use super::{ANDI_CHROM_QUIRKS, ANDI_CHROM_VALID, ANDI_NON_CDF_DUMMY, open_file};
use sciformats::{
    andi::{AndiDatasetCompleteness, andi_chrom_parser::AndiChromParser},
    api::Parser,
};
use std::str::FromStr;

fn assert_eq_f32(left: f32, right: f32) {
    let max = left.max(right);
    let epsilon = f32::EPSILON * max;
    assert!(f32::abs(left - right) <= epsilon)
}

#[test]
fn andi_chrom_parse_valid_succeeds() {
    let (path, file) = open_file(ANDI_CHROM_VALID);
    let chrom = AndiChromParser::parse(&path, file).unwrap();

    let admin_data = &chrom.admin_data;
    assert_eq!(
        AndiDatasetCompleteness::from_str("C1+C2").unwrap(),
        admin_data.dataset_completeness
    );
    assert_eq!("1.0", admin_data.protocol_template_revision);
    assert_eq!("2.0", admin_data.netcdf_revision);
    assert_eq!("English", admin_data.languages.as_ref().unwrap());
    assert_eq!(
        "dummy admin comment",
        admin_data.administrative_comments.as_ref().unwrap()
    );
    assert_eq!("sciformats", admin_data.dataset_origin.as_ref().unwrap());
    assert_eq!("Robert", admin_data.dataset_owner.as_ref().unwrap());
    assert_eq!(
        "20230908200501+0200",
        admin_data.dataset_date_time_stamp.as_ref().unwrap()
    );
    assert_eq!("20230908200501+0200", admin_data.injection_date_time_stamp);
    assert_eq!(
        "sciformats sample file",
        admin_data.experiment_title.as_ref().unwrap()
    );
    assert_eq!("Rob", admin_data.operator_name.as_ref().unwrap());
    assert_eq!(
        "liquid chromatography",
        admin_data.separation_experiment_type.as_ref().unwrap()
    );
    assert_eq!(
        "dummy company method 1",
        admin_data.company_method_name.as_ref().unwrap()
    );
    assert_eq!("1", admin_data.company_method_id.as_ref().unwrap());
    assert_eq!(
        "dummy pre exp prog name",
        admin_data.pre_experiment_program_name.as_ref().unwrap()
    );
    assert_eq!(
        "dummy post exp prog name",
        admin_data.post_experiment_program_name.as_ref().unwrap()
    );
    assert_eq!(
        "dummy source file reference",
        admin_data.source_file_reference.as_ref().unwrap()
    );
    let error_log = &admin_data.error_log;
    assert_eq!(2, error_log.len());
    assert_eq!("error 1", error_log.get(0).unwrap());
    assert_eq!("error 2", error_log.get(1).unwrap());

    let sample_description = &chrom.sample_description;
    assert_eq!(
        "dummy sample id comments",
        sample_description.sample_id_comments.as_ref().unwrap()
    );
    assert_eq!("12345", sample_description.sample_id.as_ref().unwrap());
    assert_eq!(
        "dummy sample name",
        sample_description.sample_name.as_ref().unwrap()
    );
    assert_eq!("test", sample_description.sample_type.as_ref().unwrap());
    assert_eq!(1.0, sample_description.sample_injection_volume.unwrap());
    assert_eq!(2.2, sample_description.sample_amount.unwrap());

    let detection_method = &chrom.detection_method;
    assert_eq!(
        "dummy method table name",
        detection_method
            .detection_method_table_name
            .as_ref()
            .unwrap()
    );
    assert_eq!(
        "dummy detector method comments",
        detection_method.detector_method_comments.as_ref().unwrap()
    );
    assert_eq!(
        "dummy detection method 1",
        detection_method.detection_method_name.as_ref().unwrap()
    );
    assert_eq!(
        "dummy detector name",
        detection_method.detector_name.as_ref().unwrap()
    );
    assert_eq_f32(999999.0, detection_method.detector_maximum_value.unwrap());
    assert_eq_f32(1.0, detection_method.detector_minimum_value.unwrap());
    assert_eq!("au", detection_method.detector_unit.as_ref().unwrap());

    let raw_data = &chrom.raw_data;
    assert_eq!(10, raw_data.point_number);
    assert_eq!(
        "dummy raw data table name",
        raw_data.raw_data_table_name.as_ref().unwrap()
    );
    assert_eq!("seconds", raw_data.retention_unit);
    assert_eq_f32(100.0, raw_data.actual_run_time_length);
    assert_eq_f32(10.0, raw_data.actual_sampling_interval);
    assert_eq_f32(0.0, raw_data.actual_delay_time);
    assert_eq!(
        vec![
            10000f32, 111111.1, 10000f32, 122222.2, 10000f32, 133333.3, 10000f32, 10000f32,
            10000f32, 10000f32,
        ],
        raw_data.get_ordinate_values().unwrap()
    );
    assert_eq!(true, raw_data.uniform_sampling_flag);
    assert!(raw_data.get_raw_data_retention().unwrap().is_none());
    assert_eq!("1:2", raw_data.autosampler_position.as_ref().unwrap());

    let peak_processing_results = &chrom.peak_processing_results;
    assert_eq!(3, peak_processing_results.peak_number);
    assert_eq!(
        "dummy pp res table name",
        peak_processing_results
            .peak_processing_results_table_name
            .as_ref()
            .unwrap()
    );
    assert_eq!(
        "dummy pp res comments",
        peak_processing_results
            .peak_processing_results_comments
            .as_ref()
            .unwrap()
    );
    assert_eq!(
        "dummy pp method name",
        peak_processing_results
            .peak_processing_method_name
            .as_ref()
            .unwrap()
    );
    assert_eq!(
        "20230908201502+0200",
        peak_processing_results
            .peak_processing_date_time_stamp
            .as_ref()
            .unwrap()
    );
    assert_eq!(
        "ppm",
        peak_processing_results.peak_amount_unit.as_ref().unwrap()
    );
    assert_eq!(
        3,
        peak_processing_results.get_peaks().unwrap().unwrap().len()
    );

    let peaks = peak_processing_results.get_peaks().unwrap().unwrap();

    let peak_0 = peaks.get(0).unwrap();
    assert_eq_f32(10.111, peak_0.peak_retention_time.unwrap());
    assert_eq!("ref", peak_0.peak_name.as_ref().unwrap());
    assert_eq_f32(110.1111, peak_0.peak_amount.unwrap());
    assert_eq_f32(8.0, peak_0.peak_start_time.unwrap());
    assert_eq_f32(12.0, peak_0.peak_end_time.unwrap());
    assert!(peak_0.peak_width.is_none());
    assert_eq_f32(111.0, peak_0.peak_area.unwrap());
    assert!(peak_0.peak_area_percent.is_none());
    assert_eq_f32(111111.1, peak_0.peak_height.unwrap());
    assert!(peak_0.peak_height_percent.is_none());
    assert!(peak_0.baseline_start_time.is_none());
    assert_eq_f32(5.0, peak_0.baseline_start_value.unwrap());
    assert!(peak_0.baseline_stop_time.is_none());
    assert_eq_f32(7.0, peak_0.baseline_stop_value.unwrap());
    assert_eq!("A", peak_0.peak_start_detection_code.as_ref().unwrap());
    assert_eq!("X", peak_0.peak_stop_detection_code.as_ref().unwrap());
    assert!(peak_0.retention_index.is_none());
    assert!(peak_0.migration_time.is_none());
    assert!(peak_0.peak_asymmetry.is_none());
    assert!(peak_0.peak_efficiency.is_none());
    assert!(peak_0.mass_on_column.is_none());
    assert_eq!(false, peak_0.manually_reintegrated_peaks);
    assert_eq!("seconds", peak_0.peak_retention_unit);
    assert_eq!("ppm", peak_0.peak_amount_unit.as_ref().unwrap());
    assert_eq!("au", peak_0.detector_unit.as_ref().unwrap());

    // skip peak 1

    let peak_2 = peaks.get(2).unwrap();
    assert_eq_f32(50.333, peak_2.peak_retention_time.unwrap());
    assert_eq!("peak name 2", peak_2.peak_name.as_ref().unwrap());
    assert_eq_f32(330.333, peak_2.peak_amount.unwrap());
    assert_eq_f32(48.0, peak_2.peak_start_time.unwrap());
    assert_eq_f32(52.0, peak_2.peak_end_time.unwrap());
    assert!(peak_2.peak_width.is_none());
    assert_eq_f32(333.0, peak_2.peak_area.unwrap());
    assert!(peak_2.peak_area_percent.is_none());
    assert_eq_f32(133333.3, peak_2.peak_height.unwrap());
    assert!(peak_2.peak_height_percent.is_none());
    assert!(peak_2.baseline_start_time.is_none());
    assert_eq_f32(7.0, peak_2.baseline_start_value.unwrap());
    assert!(peak_2.baseline_stop_time.is_none());
    assert_eq_f32(5.0, peak_2.baseline_stop_value.unwrap());
    assert_eq!("C", peak_2.peak_start_detection_code.as_ref().unwrap());
    assert_eq!("Z", peak_2.peak_stop_detection_code.as_ref().unwrap());
    assert!(peak_2.retention_index.is_none());
    assert!(peak_2.migration_time.is_none());
    assert!(peak_2.peak_asymmetry.is_none());
    assert!(peak_2.peak_efficiency.is_none());
    assert!(peak_2.mass_on_column.is_none());
    assert_eq!(false, peak_2.manually_reintegrated_peaks);
    assert_eq!("seconds", peak_2.peak_retention_unit);
    assert_eq!("ppm", peak_2.peak_amount_unit.as_ref().unwrap());
    assert_eq!("au", peak_2.detector_unit.as_ref().unwrap());
}

#[test]
fn andi_chrom_parse_invalid_fails() {
    let (path, file) = open_file(ANDI_NON_CDF_DUMMY);
    let chrom = AndiChromParser::parse(&path, file);

    assert!(chrom.is_err());
}

#[test]
fn andi_chrom_parse_quirks() {
    let (path, file) = open_file(ANDI_CHROM_QUIRKS);
    let chrom = AndiChromParser::parse(&path, file).unwrap();

    assert_eq!("au", chrom.detection_method.detector_unit.unwrap());
    assert_eq!("seconds", chrom.raw_data.retention_unit);
    assert_eq!(
        "ppm",
        chrom.peak_processing_results.peak_amount_unit.unwrap()
    );
}

#[test]
fn andi_chrom_file_prints_debug_info() {
    let (path, file) = open_file(ANDI_CHROM_VALID);
    let chrom = AndiChromParser::parse(&path, file).unwrap();

    let chrom_debug_info = format!("{:?}", chrom);
    assert!(chrom_debug_info.contains("AndiChromFile"));
    assert!(chrom_debug_info.contains("AndiChromSampleDescription"));
    assert!(chrom_debug_info.contains("AndiChromDetectionMethod"));
    assert!(chrom_debug_info.contains("AndiChromRawData"));
    assert!(chrom_debug_info.contains("AndiChromPeakProcessingResults"));

    let peaks = chrom.peak_processing_results.get_peaks().unwrap().unwrap();
    let peaks_debug_info = format!("{:?}", peaks);
    assert!(peaks_debug_info.contains("AndiChromPeak"));
}
