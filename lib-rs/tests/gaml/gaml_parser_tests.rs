use std::str::FromStr;

use super::{open_file, GAML_SAMPLE_FILE};
use chrono::{DateTime, FixedOffset};
use sf_rs::{
    api::Parser,
    gaml::gaml_parser::{Experiment, GamlParser, Integrity, Parameter},
};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
#[test]
fn gaml_parse_valid_succeeds() {
    let (path, file) = open_file(GAML_SAMPLE_FILE);
    let gaml = GamlParser::parse(&path, file).unwrap();

    assert_eq!("1.20", gaml.version);
    assert_eq!(Some("Gaml Test File".to_owned()), gaml.name);
    assert_eq!(
        Some(Integrity {
            algorithm: "SHA1".to_owned(),
            value: "03cfd743661f07975fa2f1220c5194cbaff48451".to_owned()
        }),
        gaml.integrity
    );

    let gaml_parameters = gaml.parameters;
    assert_eq!(3, gaml_parameters.len());
    assert_eq!(
        Parameter {
            group: Some("GAML parameter group 0".to_owned()),
            name: "GAML parameter name 0".to_owned(),
            label: Some("GAML parameter label 0".to_owned()),
            alias: Some("GAML parameter alias 0".to_owned()),
            value: Some("GAML parameter value 0".to_owned()),
        },
        gaml_parameters[0]
    );
    assert_eq!(
        Parameter {
            group: Some("GAML parameter group 0".to_owned()),
            name: "GAML parameter name 1".to_owned(),
            label: Some("GAML parameter label 1".to_owned()),
            alias: Some("GAML parameter alias 1".to_owned()),
            value: Some("GAML parameter value 1".to_owned()),
        },
        gaml_parameters[1]
    );
    assert_eq!(
        Parameter {
            group: Some("GAML parameter group 1".to_owned()),
            name: "GAML parameter name 2".to_owned(),
            label: Some("GAML parameter label 2".to_owned()),
            alias: Some("GAML parameter alias 2".to_owned()),
            value: Some("GAML parameter value 2".to_owned()),
        },
        gaml_parameters[2]
    );

    parse_experiments_succeeds(&gaml.experiments);
}

fn parse_experiments_succeeds(experiments: &[Experiment]) {
    assert_eq!(2, experiments.len());
    let experiment0 = &experiments[0];
    assert_eq!(Some("Experiment 0 name".to_owned()), experiment0.name);
    assert_eq!(
        Some(DateTime::<FixedOffset>::from_str("2024-05-31T09:17:00Z").unwrap()),
        experiment0.collectdate
    );
    assert_eq!(3, experiment0.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Experiment 0 parameter group 0".to_owned()),
            name: "Experiment 0 parameter name 0".to_owned(),
            label: Some("Experiment 0 parameter label 0".to_owned()),
            alias: Some("Experiment 0 parameter alias 0".to_owned()),
            value: Some("Experiment 0 parameter value 0".to_owned()),
        },
        experiment0.parameters[0]
    );
    assert_eq!(
        Parameter {
            group: Some("Experiment 0 parameter group 1".to_owned()),
            name: "Experiment 0 parameter name 1".to_owned(),
            label: Some("Experiment 0 parameter label 1".to_owned()),
            alias: Some("Experiment 0 parameter alias 1".to_owned()),
            value: Some("Experiment 0 parameter value 1".to_owned()),
        },
        experiment0.parameters[1]
    );
    assert_eq!(
        Parameter {
            group: Some("Experiment 0 parameter group 1".to_owned()),
            name: "Experiment 0 parameter name 2".to_owned(),
            label: Some("Experiment 0 parameter label 2".to_owned()),
            alias: Some("Experiment 0 parameter alias 2".to_owned()),
            value: Some("Experiment 0 parameter value 2".to_owned()),
        },
        experiment0.parameters[2]
    );

    let experiment1 = &experiments[1];
    assert_eq!(Some("Experiment 1 name".to_owned()), experiment1.name);
    assert_eq!(
        Some(DateTime::<FixedOffset>::from_str("2024-05-31T09:18:00Z").unwrap()),
        experiment1.collectdate
    );
    assert_eq!(3, experiment1.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Experiment 1 parameter group 0".to_owned()),
            name: "Experiment 1 parameter name 0".to_owned(),
            label: Some("Experiment 1 parameter label 0".to_owned()),
            alias: Some("Experiment 1 parameter alias 0".to_owned()),
            value: Some("Experiment 1 parameter value 0".to_owned()),
        },
        experiment1.parameters[0]
    );
    assert_eq!(
        Parameter {
            group: Some("Experiment 1 parameter group 1".to_owned()),
            name: "Experiment 1 parameter name 1".to_owned(),
            label: Some("Experiment 1 parameter label 1".to_owned()),
            alias: Some("Experiment 1 parameter alias 1".to_owned()),
            value: Some("Experiment 1 parameter value 1".to_owned()),
        },
        experiment1.parameters[1]
    );
    assert_eq!(
        Parameter {
            group: Some("Experiment 1 parameter group 1".to_owned()),
            name: "Experiment 1 parameter name 2".to_owned(),
            label: Some("Experiment 1 parameter label 2".to_owned()),
            alias: Some("Experiment 1 parameter alias 2".to_owned()),
            value: Some("Experiment 1 parameter value 2".to_owned()),
        },
        experiment1.parameters[2]
    );
}
