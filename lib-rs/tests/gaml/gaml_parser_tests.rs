use super::{open_file, GAML_SAMPLE_FILE};
use sf_rs::{
    api::Parser,
    gaml::gaml_parser::{GamlParser, Integrity, Parameter},
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
}
