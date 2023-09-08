use sf_rs::{andi_chrom::AndiChromParser, api::Parser, andi::AndiDatasetCompleteness};

use std::{
    fs::File,
    path::PathBuf, str::FromStr,
};

#[test]
fn test_andi_chrom_parsing_succeeds() {
    assert_eq!(5, 5);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/andi_chrom_valid.cdf");
    let file = File::open(&path).unwrap();

    let chrom = AndiChromParser::parse(path.to_str().unwrap(), file).unwrap();

    let admin_data = chrom.admin_data;
    assert_eq!(AndiDatasetCompleteness::from_str("C1+C2").unwrap(), admin_data.dataset_completeness);
    assert_eq!("1.0", admin_data.protocol_template_revision);
    assert_eq!("2.0", admin_data.netcdf_revision);
    assert_eq!("English", admin_data.languages.unwrap());
    assert_eq!("dummy admin comment", admin_data.administrative_comments.unwrap());
    assert_eq!("sf_rs", admin_data.dataset_origin.unwrap());
    assert_eq!("Robert", admin_data.dataset_owner.unwrap());
    assert_eq!("20230908200501+0200", admin_data.dataset_date_time_stamp.unwrap());
    assert_eq!("20230908200501+0200", admin_data.injection_date_time_stamp);
    assert_eq!("sf_rs sample file", admin_data.experiment_title.unwrap());
    assert_eq!("Rob", admin_data.operator_name.unwrap());
    assert_eq!("liquid chromatography", admin_data.separation_experiment_type.unwrap());
    assert_eq!("dummy company method 1", admin_data.company_method_name.unwrap());
    assert_eq!("1", admin_data.company_method_id.unwrap());
    assert_eq!("dummy pre exp prog name", admin_data.pre_experiment_program_name.unwrap());
    assert_eq!("dummy post exp prog name", admin_data.post_experiment_program_name.unwrap());
    assert_eq!("dummy source file reference", admin_data.source_file_reference.unwrap());
    let error_log = admin_data.error_log;
    assert_eq!(2, error_log.len());
    assert_eq!("error 1", error_log.get(0).unwrap());
    assert_eq!("error 2", error_log.get(1).unwrap());

    let sample_description = chrom.sample_description;
    assert_eq!("dummy sample id comments", sample_description.sample_id_comments.unwrap());
    assert_eq!("12345", sample_description.sample_id.unwrap());
    assert_eq!("dummy sample name", sample_description.sample_name.unwrap());
    assert_eq!("test", sample_description.sample_type.unwrap());
    // TODO: present in sample data as global attribute of type float
    // assert_eq!(1.0, sample_description.sample_injection_volume.unwrap());
    // TODO: present in sample data as global attribute of type float
    // assert_eq!(2.2, sample_description.sample_amount.unwrap());
    
    let detection_method = chrom.detection_method;
    assert_eq!("dummy method table name", detection_method.detection_method_table_name.unwrap());
    assert_eq!("dummy detector method comments", detection_method.detector_method_comments.unwrap());
    assert_eq!("dummy detection method 1", detection_method.detection_method_name.unwrap());
    assert_eq!("dummy detector name", detection_method.detector_name.unwrap());
    assert_eq!(999999.0, detection_method.detector_maximum_value.unwrap());
    assert_eq!(1.0, detection_method.detector_minimum_value.unwrap());
    assert_eq!("au", detection_method.detector_unit.unwrap());

    // TODO: add more tests

    // println!("{:?}", chrom);
}
