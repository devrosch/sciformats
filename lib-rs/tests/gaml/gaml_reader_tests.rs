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
    assert_eq!(2, root.child_node_names.len());
    assert_eq!("Experiment 0, Experiment 0 name", root.child_node_names[0]);
    assert_eq!("Experiment 1, Experiment 1 name", root.child_node_names[1]);

    parse_experiments_succeeds(&reader);
}

fn parse_experiments_succeeds(reader: &GamlReader) {
    let experiment0 = reader.read("/0").unwrap();
    assert_eq!("Experiment 0, Experiment 0 name", experiment0.name);
    let experiment0_parameters = &experiment0.parameters;
    assert_eq!(5, experiment0_parameters.len());
    assert_eq!(
        Parameter::from_str_str("Name", "Experiment 0 name"),
        experiment0_parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Collectdate", "2024-05-31T09:17:00+00:00"),
        experiment0_parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Experiment 0 parameter name 0 (group=Experiment 0 parameter group 0, label=Experiment 0 parameter label 0, alias=Experiment 0 parameter alias 0)", "Experiment 0 parameter value 0"),
        experiment0_parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("Experiment 0 parameter name 1 (group=Experiment 0 parameter group 1, label=Experiment 0 parameter label 1, alias=Experiment 0 parameter alias 1)", "Experiment 0 parameter value 1"),
        experiment0_parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("Experiment 0 parameter name 2 (group=Experiment 0 parameter group 1, label=Experiment 0 parameter label 2, alias=Experiment 0 parameter alias 2)", "Experiment 0 parameter value 2"),
        experiment0_parameters[4]
    );
    assert_eq!(Vec::<PointXy>::new(), experiment0.data);
    assert_eq!(Vec::<(String, String)>::new(), experiment0.metadata);
    assert_eq!(None, experiment0.table);
    // todo:
    // assert_eq!(0, experiment0.child_node_names.len());

    let experiment1 = reader.read("/1").unwrap();
    assert_eq!("Experiment 1, Experiment 1 name", experiment1.name);
    let experiment1_parameters = &experiment1.parameters;
    assert_eq!(5, experiment1_parameters.len());
    assert_eq!(
        Parameter::from_str_str("Name", "Experiment 1 name"),
        experiment1_parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Collectdate", "2024-05-31T09:18:00+00:00"),
        experiment1_parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Experiment 1 parameter name 0 (group=Experiment 1 parameter group 0, label=Experiment 1 parameter label 0, alias=Experiment 1 parameter alias 0)", "Experiment 1 parameter value 0"),
        experiment1_parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("Experiment 1 parameter name 1 (group=Experiment 1 parameter group 1, label=Experiment 1 parameter label 1, alias=Experiment 1 parameter alias 1)", "Experiment 1 parameter value 1"),
        experiment1_parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("Experiment 1 parameter name 2 (group=Experiment 1 parameter group 1, label=Experiment 1 parameter label 2, alias=Experiment 1 parameter alias 2)", "Experiment 1 parameter value 2"),
        experiment1_parameters[4]
    );
    assert_eq!(Vec::<PointXy>::new(), experiment1.data);
    assert_eq!(Vec::<(String, String)>::new(), experiment1.metadata);
    assert_eq!(None, experiment1.table);
    // todo:
    // assert_eq!(0, experiment1.child_node_names.len());
}
