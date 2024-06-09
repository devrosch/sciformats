use std::str::FromStr;

use super::{open_file, GAML_SAMPLE_FILE};
use chrono::{DateTime, FixedOffset};
use sf_rs::{
    api::Parser,
    gaml::gaml_parser::{
        Byteorder, Experiment, Format, GamlParser, Integrity, Link, Parameter, Peaktable,
        Technique, Trace, Units, Valueorder, Version, Xdata,
    },
};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
#[test]
fn gaml_parse_valid_succeeds() {
    let (path, file) = open_file(GAML_SAMPLE_FILE);
    let gaml = GamlParser::parse(&path, file).unwrap();

    assert_eq!(Version::Version1_20, gaml.version);
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
    assert_eq!(2, coordinates0.links.len());
    assert_eq!(
        Link {
            linkref: "xdata000-linkid".to_owned()
        },
        coordinates0.links[0]
    );
    assert_eq!(
        Link {
            linkref: "altxdata0000-linkid".to_owned()
        },
        coordinates0.links[1]
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
    assert_eq!(Some(1), coordinates0_values.numvalues);
    assert_eq!(vec![1.0], coordinates0_values.get_data().unwrap());

    assert_eq!(1, trace.x_data.len());
    parse_xdata000_succeeds(&trace.x_data[0]);
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
            linkref: "xdata100-linkid".to_owned()
        },
        coordinates0.links[0]
    );
    assert!(coordinates0.parameters.is_empty());
    let coordinates0_values = &coordinates0.values;
    assert_eq!(Byteorder::Intel, coordinates0_values.byteorder);
    assert_eq!(Format::Float32, coordinates0_values.format);
    assert_eq!(Some(1), coordinates0_values.numvalues);
    assert_eq!(vec![1.0], coordinates0_values.get_data().unwrap());
    assert_eq!(1, trace.x_data.len());
    parse_xdata100_succeeds(&trace.x_data[0]);
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
    assert_eq!(1, trace.x_data.len());
    parse_xdata110_succeeds(&trace.x_data[0]);
}

fn parse_xdata000_succeeds(x_data: &Xdata) {
    assert_eq!(Units::Minutes, x_data.units);
    assert_eq!(Some("Xdata 0/0/0 label".to_owned()), x_data.label);
    assert_eq!(Some("xdata000-linkid".to_owned()), x_data.linkid);
    assert_eq!(Some(Valueorder::Unspecified), x_data.valueorder);
    assert_eq!(
        Link {
            linkref: "coordinates000-linkid".to_owned()
        },
        x_data.links[0]
    );
    assert_eq!(1, x_data.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Xdata 0/0/0 parameter group 0".to_owned()),
            name: "Xdata 0/0/0 parameter name 0".to_owned(),
            label: Some("Xdata 0/0/0 parameter label 0".to_owned()),
            alias: Some("Xdata 0/0/0 parameter alias 0".to_owned()),
            value: Some("Xdata 0/0/0 parameter value 0".to_owned()),
        },
        x_data.parameters[0]
    );
    let x_data_values = &x_data.values;
    assert_eq!(Byteorder::Intel, x_data_values.byteorder);
    assert_eq!(Format::Float32, x_data_values.format);
    assert_eq!(Some(2), x_data_values.numvalues);
    assert_eq!(vec![1.0, 2.0], x_data_values.get_data().unwrap());

    assert_eq!(1, x_data.alt_x_data.len());
    let alt_x_data = &x_data.alt_x_data[0];
    assert_eq!(Units::Centimeters, alt_x_data.units);
    assert_eq!(Some("altXdata 0/0/0/0 label".to_owned()), alt_x_data.label);
    assert_eq!(Some("altxdata0000-linkid".to_owned()), alt_x_data.linkid);
    assert_eq!(Some(Valueorder::Unspecified), alt_x_data.valueorder);
    assert_eq!(
        Link {
            linkref: "coordinates000-linkid".to_owned()
        },
        alt_x_data.links[0]
    );
    assert_eq!(1, alt_x_data.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("altXdata 0/0/0/0 parameter group 0".to_owned()),
            name: "altXdata 0/0/0/0 parameter name 0".to_owned(),
            label: Some("altXdata 0/0/0/0 parameter label 0".to_owned()),
            alias: Some("altXdata 0/0/0/0 parameter alias 0".to_owned()),
            value: Some("altXdata 0/0/0/0 parameter value 0".to_owned()),
        },
        alt_x_data.parameters[0]
    );
    let alt_x_data_values = &alt_x_data.values;
    assert_eq!(Byteorder::Intel, alt_x_data_values.byteorder);
    assert_eq!(Format::Float32, alt_x_data_values.format);
    assert_eq!(Some(2), alt_x_data_values.numvalues);
    assert_eq!(vec![1.0, 2.0], alt_x_data_values.get_data().unwrap());

    assert_eq!(1, x_data.y_data.len());
    let y_data0 = &x_data.y_data[0];
    assert_eq!(Units::Microns, y_data0.units);
    assert_eq!(Some("Ydata 0/0/0/0 label".to_owned()), y_data0.label);
    assert_eq!(1, y_data0.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Ydata 0/0/0/0 parameter group 0".to_owned()),
            name: "Ydata 0/0/0/0 parameter name 0".to_owned(),
            label: Some("Ydata 0/0/0/0 parameter label 0".to_owned()),
            alias: Some("Ydata 0/0/0/0 parameter alias 0".to_owned()),
            value: Some("Ydata 0/0/0/0 parameter value 0".to_owned()),
        },
        y_data0.parameters[0]
    );
    let y_data0_values = &y_data0.values;
    assert_eq!(Byteorder::Intel, y_data0_values.byteorder);
    assert_eq!(Format::Float32, y_data0_values.format);
    assert_eq!(Some(2), y_data0_values.numvalues);
    assert_eq!(vec![1.0, 2.0], y_data0_values.get_data().unwrap());

    assert_eq!(1, y_data0.peaktables.len());
    parse_peaktables0000_succeeds(&y_data0.peaktables[0]);
}

fn parse_xdata100_succeeds(x_data: &Xdata) {
    assert_eq!(Units::Masschargeratio, x_data.units);
    assert_eq!(Some("Xdata 1/0/0 label".to_owned()), x_data.label);
    assert_eq!(Some("xdata100-linkid".to_owned()), x_data.linkid);
    assert_eq!(Some(Valueorder::Unspecified), x_data.valueorder);
    assert_eq!(
        Link {
            linkref: "coordinates100-linkid".to_owned()
        },
        x_data.links[0]
    );
    assert!(x_data.parameters.is_empty());
    let x_data_values = &x_data.values;
    assert_eq!(Byteorder::Intel, x_data_values.byteorder);
    assert_eq!(Format::Float32, x_data_values.format);
    assert_eq!(Some(2), x_data_values.numvalues);
    assert_eq!(vec![1.0, 2.0], x_data_values.get_data().unwrap());
    assert!(x_data.alt_x_data.is_empty());

    assert_eq!(2, x_data.y_data.len());
    let y_data0 = &x_data.y_data[0];
    assert_eq!(Units::Millivolts, y_data0.units);
    assert_eq!(Some("Ydata 1/0/0/0 label".to_owned()), y_data0.label);
    assert!(y_data0.parameters.is_empty());
    let y_data0_values = &y_data0.values;
    assert_eq!(Byteorder::Intel, y_data0_values.byteorder);
    assert_eq!(Format::Float32, y_data0_values.format);
    assert_eq!(Some(2), y_data0_values.numvalues);
    assert_eq!(vec![1.0, 2.0], y_data0_values.get_data().unwrap());
    assert!(y_data0.peaktables.is_empty());

    let y_data1 = &x_data.y_data[1];
    assert_eq!(Units::Millivolts, y_data1.units);
    assert_eq!(Some("Ydata 1/0/0/1 label".to_owned()), y_data1.label);
    assert!(y_data1.parameters.is_empty());
    let y_data1_values = &y_data1.values;
    assert_eq!(Byteorder::Intel, y_data1_values.byteorder);
    assert_eq!(Format::Float32, y_data1_values.format);
    assert_eq!(Some(2), y_data1_values.numvalues);
    assert_eq!(vec![1.0, 2.0], y_data1_values.get_data().unwrap());
    assert!(y_data1.peaktables.is_empty());
}

fn parse_xdata110_succeeds(x_data: &Xdata) {
    assert_eq!(Units::Masschargeratio, x_data.units);
    assert_eq!(Some("Xdata 1/1/0 label".to_owned()), x_data.label);
    assert_eq!(Some("xdata110-linkid".to_owned()), x_data.linkid);
    assert_eq!(Some(Valueorder::Unspecified), x_data.valueorder);
    assert!(x_data.parameters.is_empty());
    let x_data_values = &x_data.values;
    assert_eq!(Byteorder::Intel, x_data_values.byteorder);
    assert_eq!(Format::Float32, x_data_values.format);
    assert_eq!(Some(2), x_data_values.numvalues);
    assert_eq!(vec![1.0, 2.0], x_data_values.get_data().unwrap());
    assert!(x_data.alt_x_data.is_empty());

    assert_eq!(1, x_data.y_data.len());
    let y_data0 = &x_data.y_data[0];
    assert_eq!(Units::Millivolts, y_data0.units);
    assert_eq!(Some("Ydata 1/1/0/0 label".to_owned()), y_data0.label);
    assert!(y_data0.parameters.is_empty());
    let y_data0_values = &y_data0.values;
    assert_eq!(Byteorder::Intel, y_data0_values.byteorder);
    assert_eq!(Format::Float32, y_data0_values.format);
    assert_eq!(Some(2), y_data0_values.numvalues);
    assert_eq!(vec![1.0, 2.0], y_data0_values.get_data().unwrap());
    assert!(y_data0.peaktables.is_empty());
}

fn parse_peaktables0000_succeeds(peaktable: &Peaktable) {
    assert_eq!(Some("Peaktable 0/0/0/0/0 name".to_owned()), peaktable.name);
    assert_eq!(1, peaktable.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Peaktable 0/0/0/0/0 parameter group 0".to_owned()),
            name: "Peaktable 0/0/0/0/0 parameter name 0".to_owned(),
            label: Some("Peaktable 0/0/0/0/0 parameter label 0".to_owned()),
            alias: Some("Peaktable 0/0/0/0/0 parameter alias 0".to_owned()),
            value: Some("Peaktable 0/0/0/0/0 parameter value 0".to_owned()),
        },
        peaktable.parameters[0]
    );

    assert_eq!(1, peaktable.peaks.len());
    let peak0 = &peaktable.peaks[0];
    assert_eq!(1, peak0.number);
    assert_eq!(Some("Peak 0/0/0/0/0/0 group".to_owned()), peak0.group);
    assert_eq!(Some("Peak 0/0/0/0/0/0 name".to_owned()), peak0.name);
    assert_eq!(1, peak0.parameters.len());
    assert_eq!(
        Parameter {
            group: Some("Peak 0/0/0/0/0/0 parameter group 0".to_owned()),
            name: "Peak 0/0/0/0/0/0 parameter name 0".to_owned(),
            label: Some("Peak 0/0/0/0/0/0 parameter label 0".to_owned()),
            alias: Some("Peak 0/0/0/0/0/0 parameter alias 0".to_owned()),
            value: Some("Peak 0/0/0/0/0/0 parameter value 0".to_owned()),
        },
        peak0.parameters[0]
    );
    assert_eq!(0.1, peak0.peak_x_value);
    assert_eq!(100.0, peak0.peak_y_value);

    let baseline0 = peak0.baseline.as_ref().unwrap();
    assert!(baseline0.parameters.is_empty());
    assert_eq!(1.1, baseline0.start_x_value);
    assert_eq!(11.1, baseline0.start_y_value);
    assert_eq!(2.2, baseline0.end_x_value);
    assert_eq!(22.2, baseline0.end_y_value);
    let basecurve = baseline0.basecurve.as_ref().unwrap();
    assert_eq!(2, basecurve.base_x_data.len());
    assert_eq!(Byteorder::Intel, basecurve.base_x_data[0].byteorder);
    assert_eq!(Format::Float32, basecurve.base_x_data[0].format);
    assert_eq!(Some(2), basecurve.base_x_data[0].numvalues);
    assert_eq!(Byteorder::Intel, basecurve.base_x_data[1].byteorder);
    assert_eq!(Format::Float32, basecurve.base_x_data[1].format);
    assert_eq!(Some(2), basecurve.base_x_data[1].numvalues);
    assert_eq!(vec![1.0, 2.0, 1.0, 2.0], basecurve.get_x_data().unwrap());
    assert_eq!(2, basecurve.base_y_data.len());
    assert_eq!(Byteorder::Intel, basecurve.base_y_data[0].byteorder);
    assert_eq!(Format::Float32, basecurve.base_y_data[0].format);
    assert_eq!(Some(2), basecurve.base_y_data[0].numvalues);
    assert_eq!(Byteorder::Intel, basecurve.base_y_data[1].byteorder);
    assert_eq!(Format::Float32, basecurve.base_y_data[1].format);
    assert_eq!(Some(2), basecurve.base_y_data[1].numvalues);
    assert_eq!(vec![1.0, 2.0, 1.0, 2.0], basecurve.get_y_data().unwrap());
}
