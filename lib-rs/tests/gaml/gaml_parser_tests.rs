use std::str::FromStr;

use super::{open_file, GAML_SAMPLE_FILE};
use chrono::{DateTime, FixedOffset};
use sf_rs::{
    api::Parser,
    gaml::gaml_parser::{
        Byteorder, Experiment, Format, GamlParser, Integrity, Link, Parameter, Technique, Trace,
        Units, Valueorder,
    },
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
    assert_eq!(1, experiment0.traces.len());
    parse_trace00_succeeds(&experiment0.traces[0]);

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
    assert_eq!(2, experiment1.traces.len());
    parse_trace10_succeeds(&experiment1.traces[0]);
    parse_trace11_succeeds(&experiment1.traces[1]);
}

fn parse_trace00_succeeds(trace: &Trace) {
    assert_eq!(Some("Trace 0/0 name".to_owned()), trace.name);
    assert_eq!(Technique::Unknown, trace.technique);
    assert_eq!(1, trace.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Trace 0/0 parameter group 0".to_owned()),
            name: "Trace 0/0 parameter name 0".to_owned(),
            label: Some("Trace 0/0 parameter label 0".to_owned()),
            alias: Some("Trace 0/0 parameter alias 0".to_owned()),
            value: Some("Trace 0/0 parameter value 0".to_owned()),
        },
        trace.parameters[0]
    );
    assert_eq!(1, trace.coordinates.len());

    let coordinates0 = &trace.coordinates[0];
    assert_eq!(
        Some("Coordinates 0/0/0 label".to_owned()),
        coordinates0.label
    );
    assert_eq!(
        Some("coordinates000-linkid".to_owned()),
        coordinates0.linkid
    );
    assert_eq!(Units::Microns, coordinates0.units);
    assert_eq!(Some(Valueorder::Unspecified), coordinates0.valueorder);
    assert_eq!(1, coordinates0.links.len());
    assert_eq!(
        Link {
            linkref: "coordinates000-linkref".to_owned()
        },
        coordinates0.links[0]
    );
    assert_eq!(1, coordinates0.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Coordinates 0/0/0 parameter group 0".to_owned()),
            name: "Coordinates 0/0/0 parameter name 0".to_owned(),
            label: Some("Coordinates 0/0/0 parameter label 0".to_owned()),
            alias: Some("Coordinates 0/0/0 parameter alias 0".to_owned()),
            value: Some("Coordinates 0/0/0 parameter value 0".to_owned()),
        },
        coordinates0.parameters[0]
    );
    let coordinates0_values = &coordinates0.values;
    assert_eq!(Byteorder::Intel, coordinates0_values.byteorder);
    assert_eq!(Format::Float32, coordinates0_values.format);
    assert_eq!(Some(2), coordinates0_values.numvalues);
    assert_eq!(vec![1.0, 2.0], coordinates0_values.get_data().unwrap());
}

fn parse_trace10_succeeds(trace: &Trace) {
    assert_eq!(Some("Trace 1/0 name".to_owned()), trace.name);
    assert_eq!(Technique::Unknown, trace.technique);
    assert_eq!(1, trace.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Trace 1/0 parameter group 0".to_owned()),
            name: "Trace 1/0 parameter name 0".to_owned(),
            label: Some("Trace 1/0 parameter label 0".to_owned()),
            alias: Some("Trace 1/0 parameter alias 0".to_owned()),
            value: Some("Trace 1/0 parameter value 0".to_owned()),
        },
        trace.parameters[0]
    );
    assert_eq!(1, trace.coordinates.len());

    let coordinates0 = &trace.coordinates[0];
    assert_eq!(
        Some("Coordinates 1/0/0 label".to_owned()),
        coordinates0.label
    );
    assert_eq!(
        Some("coordinates100-linkid".to_owned()),
        coordinates0.linkid
    );
    assert_eq!(Units::Microns, coordinates0.units);
    assert_eq!(Some(Valueorder::Unspecified), coordinates0.valueorder);
    assert_eq!(1, coordinates0.links.len());
    assert_eq!(
        Link {
            linkref: "coordinates100-linkref".to_owned()
        },
        coordinates0.links[0]
    );
    assert_eq!(1, coordinates0.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Coordinates 1/0/0 parameter group 0".to_owned()),
            name: "Coordinates 1/0/0 parameter name 0".to_owned(),
            label: Some("Coordinates 1/0/0 parameter label 0".to_owned()),
            alias: Some("Coordinates 1/0/0 parameter alias 0".to_owned()),
            value: Some("Coordinates 1/0/0 parameter value 0".to_owned()),
        },
        coordinates0.parameters[0]
    );
    let coordinates0_values = &coordinates0.values;
    assert_eq!(Byteorder::Intel, coordinates0_values.byteorder);
    assert_eq!(Format::Float32, coordinates0_values.format);
    assert_eq!(Some(2), coordinates0_values.numvalues);
    assert_eq!(vec![1.0, 2.0], coordinates0_values.get_data().unwrap());
}

fn parse_trace11_succeeds(trace: &Trace) {
    assert_eq!(Some("Trace 1/1 name".to_owned()), trace.name);
    assert_eq!(Technique::Unknown, trace.technique);
    assert_eq!(1, trace.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Trace 1/1 parameter group 0".to_owned()),
            name: "Trace 1/1 parameter name 0".to_owned(),
            label: Some("Trace 1/1 parameter label 0".to_owned()),
            alias: Some("Trace 1/1 parameter alias 0".to_owned()),
            value: Some("Trace 1/1 parameter value 0".to_owned()),
        },
        trace.parameters[0]
    );
    assert_eq!(1, trace.coordinates.len());

    let coordinates0 = &trace.coordinates[0];
    assert_eq!(
        Some("Coordinates 1/1/0 label".to_owned()),
        coordinates0.label
    );
    assert_eq!(
        Some("coordinates110-linkid".to_owned()),
        coordinates0.linkid
    );
    assert_eq!(Units::Microns, coordinates0.units);
    assert_eq!(Some(Valueorder::Unspecified), coordinates0.valueorder);
    assert_eq!(1, coordinates0.links.len());
    assert_eq!(
        Link {
            linkref: "coordinates110-linkref".to_owned()
        },
        coordinates0.links[0]
    );
    assert_eq!(1, coordinates0.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Coordinates 1/1/0 parameter group 0".to_owned()),
            name: "Coordinates 1/1/0 parameter name 0".to_owned(),
            label: Some("Coordinates 1/1/0 parameter label 0".to_owned()),
            alias: Some("Coordinates 1/1/0 parameter alias 0".to_owned()),
            value: Some("Coordinates 1/1/0 parameter value 0".to_owned()),
        },
        coordinates0.parameters[0]
    );
    let coordinates0_values = &coordinates0.values;
    assert_eq!(Byteorder::Intel, coordinates0_values.byteorder);
    assert_eq!(Format::Float32, coordinates0_values.format);
    assert_eq!(Some(2), coordinates0_values.numvalues);
    assert_eq!(vec![1.0, 2.0], coordinates0_values.get_data().unwrap());
}
