use super::{open_file, GAML_SAMPLE_FILE};
use sf_rs::{
    api::{Parameter, Parser, PointXy, Reader},
    gaml::{gaml_parser::GamlParser, gaml_reader::GamlReader},
};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
#[test]
fn gaml_parse_valid_succeeds() {
    let (path, file) = open_file(GAML_SAMPLE_FILE);
    let gaml = GamlParser::parse(&path, file).unwrap();
    let reader = GamlReader::new(&path, gaml);

    let root = &reader.read("/").unwrap();

    assert_eq!(GAML_SAMPLE_FILE, &root.name);
    let root_parameters = &root.parameters;
    assert_eq!(6, root_parameters.len());

    assert_eq!(
        Parameter::from_str_str("Version", "1.20"),
        root_parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Name", "Gaml Test File"),
        root_parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str(
            "Integrity (algorithm=SHA1)",
            "03cfd743661f07975fa2f1220c5194cbaff48451"
        ),
        root_parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("GAML parameter name 0 (group=GAML parameter group 0, label=GAML parameter label 0, alias=GAML parameter alias 0)", "GAML parameter value 0"),
        root_parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("GAML parameter name 1 (group=GAML parameter group 0, label=GAML parameter label 1, alias=GAML parameter alias 1)", "GAML parameter value 1"),
        root_parameters[4]
    );
    assert_eq!(
        Parameter::from_str_str("GAML parameter name 2 (group=GAML parameter group 1, label=GAML parameter label 2, alias=GAML parameter alias 2)", "GAML parameter value 2"),
        root_parameters[5]
    );

    assert_eq!(Vec::<PointXy>::new(), root.data);
    assert_eq!(Vec::<(String, String)>::new(), root.metadata);
    assert_eq!(None, root.table);
}
