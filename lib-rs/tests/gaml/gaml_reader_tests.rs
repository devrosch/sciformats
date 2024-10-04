use super::{open_file, GAML_SAMPLE_FILE};
use sf_rs::{
    api::{Column, Parameter, Parser, PointXy, Reader, Table, Value},
    gaml::{gaml_parser::GamlParser, gaml_reader::GamlReader},
};
use std::collections::HashMap;

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
        Parameter::from_str_str("Collectdate", "2024-05-31T09:17:00Z"),
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
    assert_eq!(1, experiment0.child_node_names.len());
    parse_trace00_succeeds(reader);

    let experiment1 = reader.read("/1").unwrap();
    assert_eq!("Experiment 1, Experiment 1 name", experiment1.name);
    let experiment1_parameters = &experiment1.parameters;
    assert_eq!(5, experiment1_parameters.len());
    assert_eq!(
        Parameter::from_str_str("Name", "Experiment 1 name"),
        experiment1_parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Collectdate", "2024-05-31T09:18:00Z"),
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
    assert_eq!(2, experiment1.child_node_names.len());
    parse_trace10_succeeds(reader);
    parse_trace11_succeeds(reader);
}

fn parse_trace00_succeeds(reader: &GamlReader) {
    let trace = reader.read("/0/0").unwrap();
    assert_eq!("Trace 0, Trace 0/0 name", trace.name);
    let parameters = &trace.parameters;
    assert_eq!(3, parameters.len());
    assert_eq!(
        Parameter::from_str_str("Name", "Trace 0/0 name"),
        parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Technique", "UNKNOWN"),
        parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Trace 0/0 parameter name 0 (group=Trace 0/0 parameter group 0, label=Trace 0/0 parameter label 0, alias=Trace 0/0 parameter alias 0)", "Trace 0/0 parameter value 0"),
        parameters[2]
    );
    assert_eq!(Vec::<PointXy>::new(), trace.data);
    assert_eq!(Vec::<(String, String)>::new(), trace.metadata);
    assert_eq!(None, trace.table);
    assert_eq!(2, trace.child_node_names.len());
    assert_eq!(
        "XYData 0, 0 (Coordinates 0/0/0 label=1 MICRONS)",
        trace.child_node_names[0]
    );
    assert_eq!(
        "AltXYData 0, 0, 0 (Coordinates 0/0/0 label=1 MICRONS)",
        trace.child_node_names[1]
    );
    parse_xydata000_succeeds(reader);
    parse_xydata001_succeeds(reader);
}

fn parse_trace10_succeeds(reader: &GamlReader) {
    let trace = reader.read("/1/0").unwrap();
    assert_eq!("Trace 0, Trace 1/0 name", trace.name);
    let parameters = &trace.parameters;
    assert_eq!(3, parameters.len());
    assert_eq!(
        Parameter::from_str_str("Name", "Trace 1/0 name"),
        parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Technique", "UNKNOWN"),
        parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Trace 1/0 parameter name 0 (group=Trace 1/0 parameter group 0, label=Trace 1/0 parameter label 0, alias=Trace 1/0 parameter alias 0)", "Trace 1/0 parameter value 0"),
        parameters[2]
    );
    assert_eq!(Vec::<PointXy>::new(), trace.data);
    assert_eq!(Vec::<(String, String)>::new(), trace.metadata);
    assert_eq!(None, trace.table);
    assert_eq!(2, trace.child_node_names.len());
    assert_eq!(
        "XYData 0, 0 (Coordinates 1/0/0 label=1 MICRONS)",
        trace.child_node_names[0]
    );
    assert_eq!(
        "XYData 0, 1 (Coordinates 1/0/0 label=1 MICRONS)",
        trace.child_node_names[1]
    );
    parse_xydata100_succeeds(reader);
    parse_xydata101_succeeds(reader);
}

fn parse_trace11_succeeds(reader: &GamlReader) {
    let trace = reader.read("/1/1").unwrap();
    assert_eq!("Trace 1, Trace 1/1 name", trace.name);
    let parameters = &trace.parameters;
    assert_eq!(3, parameters.len());
    assert_eq!(
        Parameter::from_str_str("Name", "Trace 1/1 name"),
        parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Technique", "UNKNOWN"),
        parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Trace 1/1 parameter name 0 (group=Trace 1/1 parameter group 0, label=Trace 1/1 parameter label 0, alias=Trace 1/1 parameter alias 0)", "Trace 1/1 parameter value 0"),
        parameters[2]
    );
    assert_eq!(Vec::<PointXy>::new(), trace.data);
    assert_eq!(Vec::<(String, String)>::new(), trace.metadata);
    assert_eq!(None, trace.table);
    assert_eq!(1, trace.child_node_names.len());
    assert_eq!("XYData 0, 0", trace.child_node_names[0]);
    parse_xydata110_succeeds(reader);
}

fn parse_xydata000_succeeds(reader: &GamlReader) {
    let xy_data = reader.read("/0/0/0").unwrap();
    assert_eq!(
        "XYData 0, 0 (Coordinates 0/0/0 label=1 MICRONS)",
        xy_data.name
    );
    let parameters = &xy_data.parameters;
    assert_eq!(25, parameters.len());
    assert_eq!(
        Parameter::from_str_str("Xdata units", "MINUTES"),
        parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata label", "Xdata 0/0/0 label"),
        parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata linkid", "xdata000-linkid"),
        parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata valueorder", "UNSPECIFIED"),
        parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata units", "MICRONS"),
        parameters[4]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata label", "Ydata 0/0/0/0 label"),
        parameters[5]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 units", "MICRONS"),
        parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 label", "Coordinates 0/0/0 label"),
        parameters[7]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 linkid", "coordinates000-linkid"),
        parameters[8]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 valueorder", "UNSPECIFIED"),
        parameters[9]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata linkref", "coordinates000-linkid"),
        parameters[10]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 link linkref", "xdata000-linkid"),
        parameters[11]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 link linkref", "altxdata0000-linkid"),
        parameters[12]
    );
    assert_eq!(Parameter::from_str_str("Xdata Xdata 0/0/0 parameter name 0 (group=Xdata 0/0/0 parameter group 0, label=Xdata 0/0/0 parameter label 0, alias=Xdata 0/0/0 parameter alias 0)", "Xdata 0/0/0 parameter value 0"), parameters[13]);
    assert_eq!(Parameter::from_str_str("Ydata Ydata 0/0/0/0 parameter name 0 (group=Ydata 0/0/0/0 parameter group 0, label=Ydata 0/0/0/0 parameter label 0, alias=Ydata 0/0/0/0 parameter alias 0)", "Ydata 0/0/0/0 parameter value 0"), parameters[14]);
    assert_eq!(Parameter::from_str_str("Coordinate 0 Coordinates 0/0/0 parameter name 0 (group=Coordinates 0/0/0 parameter group 0, label=Coordinates 0/0/0 parameter label 0, alias=Coordinates 0/0/0 parameter alias 0)", "Coordinates 0/0/0 parameter value 0"), parameters[15]);
    assert_eq!(
        Parameter::from_str_str("Xdata values format", "FLOAT32"),
        parameters[16]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata values byteorder", "INTEL"),
        parameters[17]
    );
    assert_eq!(
        Parameter::from_str_u64("Xdata values numvalues", 2),
        parameters[18]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata values format", "FLOAT32"),
        parameters[19]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata values byteorder", "INTEL"),
        parameters[20]
    );
    assert_eq!(
        Parameter::from_str_u64("Ydata values numvalues", 2),
        parameters[21]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 values format", "FLOAT32"),
        parameters[22]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 values byteorder", "INTEL"),
        parameters[23]
    );
    assert_eq!(
        Parameter::from_str_u64("Coordinate 0 values numvalues", 1),
        parameters[24]
    );
    assert_eq!(
        vec![PointXy::new(1.0, 1.0,), PointXy::new(2.0, 2.0,)],
        xy_data.data
    );
    assert_eq!(
        vec![
            ("x.label".to_owned(), "Xdata 0/0/0 label".to_owned()),
            ("x.unit".to_owned(), "MINUTES".to_owned()),
            ("y.label".to_owned(), "Ydata 0/0/0/0 label".to_owned()),
            ("y.unit".to_owned(), "MICRONS".to_owned())
        ],
        xy_data.metadata
    );
    assert_eq!(None, xy_data.table);

    assert_eq!(1, xy_data.child_node_names.len());
    parse_peaktables0000_succeeds(reader);
}

fn parse_xydata001_succeeds(reader: &GamlReader) {
    let xy_data = reader.read("/0/0/1").unwrap();
    assert_eq!(
        "AltXYData 0, 0, 0 (Coordinates 0/0/0 label=1 MICRONS)",
        xy_data.name
    );
    let parameters = &xy_data.parameters;
    assert_eq!(25, parameters.len());
    assert_eq!(
        Parameter::from_str_str("AltXdata units", "CENTIMETERS"),
        parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("AltXdata label", "altXdata 0/0/0/0 label"),
        parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("AltXdata linkid", "altxdata0000-linkid"),
        parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("AltXdata valueorder", "UNSPECIFIED"),
        parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata units", "MICRONS"),
        parameters[4]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata label", "Ydata 0/0/0/0 label"),
        parameters[5]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 units", "MICRONS"),
        parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 label", "Coordinates 0/0/0 label"),
        parameters[7]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 linkid", "coordinates000-linkid"),
        parameters[8]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 valueorder", "UNSPECIFIED"),
        parameters[9]
    );
    assert_eq!(
        Parameter::from_str_str("AltXdata linkref", "coordinates000-linkid"),
        parameters[10]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 link linkref", "xdata000-linkid"),
        parameters[11]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 link linkref", "altxdata0000-linkid"),
        parameters[12]
    );
    assert_eq!(Parameter::from_str_str("AltXdata altXdata 0/0/0/0 parameter name 0 (group=altXdata 0/0/0/0 parameter group 0, label=altXdata 0/0/0/0 parameter label 0, alias=altXdata 0/0/0/0 parameter alias 0)", "altXdata 0/0/0/0 parameter value 0"), parameters[13]);
    assert_eq!(Parameter::from_str_str("Ydata Ydata 0/0/0/0 parameter name 0 (group=Ydata 0/0/0/0 parameter group 0, label=Ydata 0/0/0/0 parameter label 0, alias=Ydata 0/0/0/0 parameter alias 0)", "Ydata 0/0/0/0 parameter value 0"), parameters[14]);
    assert_eq!(Parameter::from_str_str("Coordinate 0 Coordinates 0/0/0 parameter name 0 (group=Coordinates 0/0/0 parameter group 0, label=Coordinates 0/0/0 parameter label 0, alias=Coordinates 0/0/0 parameter alias 0)", "Coordinates 0/0/0 parameter value 0"), parameters[15]);
    assert_eq!(
        Parameter::from_str_str("AltXdata values format", "FLOAT32"),
        parameters[16]
    );
    assert_eq!(
        Parameter::from_str_str("AltXdata values byteorder", "INTEL"),
        parameters[17]
    );
    assert_eq!(
        Parameter::from_str_u64("AltXdata values numvalues", 2),
        parameters[18]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata values format", "FLOAT32"),
        parameters[19]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata values byteorder", "INTEL"),
        parameters[20]
    );
    assert_eq!(
        Parameter::from_str_u64("Ydata values numvalues", 2),
        parameters[21]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 values format", "FLOAT32"),
        parameters[22]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 values byteorder", "INTEL"),
        parameters[23]
    );
    assert_eq!(
        Parameter::from_str_u64("Coordinate 0 values numvalues", 1),
        parameters[24]
    );
    assert_eq!(
        vec![PointXy::new(1.0, 1.0,), PointXy::new(2.0, 2.0,)],
        xy_data.data
    );
    assert_eq!(
        vec![
            ("x.label".to_owned(), "altXdata 0/0/0/0 label".to_owned()),
            ("x.unit".to_owned(), "CENTIMETERS".to_owned()),
            ("y.label".to_owned(), "Ydata 0/0/0/0 label".to_owned()),
            ("y.unit".to_owned(), "MICRONS".to_owned())
        ],
        xy_data.metadata
    );
    assert_eq!(None, xy_data.table);
    assert!(xy_data.child_node_names.is_empty());
}

fn parse_xydata100_succeeds(reader: &GamlReader) {
    let xy_data = reader.read("/1/0/0").unwrap();
    assert_eq!(
        "XYData 0, 0 (Coordinates 1/0/0 label=1 MICRONS)",
        xy_data.name
    );
    let parameters = &xy_data.parameters;
    assert_eq!(21, parameters.len());
    assert_eq!(
        Parameter::from_str_str("Xdata units", "MASSCHARGERATIO"),
        parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata label", "Xdata 1/0/0 label"),
        parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata linkid", "xdata100-linkid"),
        parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata valueorder", "UNSPECIFIED"),
        parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata units", "MILLIVOLTS"),
        parameters[4]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata label", "Ydata 1/0/0/0 label"),
        parameters[5]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 units", "MICRONS"),
        parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 label", "Coordinates 1/0/0 label"),
        parameters[7]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 linkid", "coordinates100-linkid"),
        parameters[8]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 valueorder", "UNSPECIFIED"),
        parameters[9]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata linkref", "coordinates100-linkid"),
        parameters[10]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 link linkref", "xdata100-linkid"),
        parameters[11]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata values format", "FLOAT32"),
        parameters[12]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata values byteorder", "INTEL"),
        parameters[13]
    );
    assert_eq!(
        Parameter::from_str_u64("Xdata values numvalues", 2),
        parameters[14]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata values format", "FLOAT32"),
        parameters[15]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata values byteorder", "INTEL"),
        parameters[16]
    );
    assert_eq!(
        Parameter::from_str_u64("Ydata values numvalues", 2),
        parameters[17]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 values format", "FLOAT32"),
        parameters[18]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 values byteorder", "INTEL"),
        parameters[19]
    );
    assert_eq!(
        Parameter::from_str_u64("Coordinate 0 values numvalues", 1),
        parameters[20]
    );
    assert_eq!(
        vec![PointXy::new(1.0, 1.0,), PointXy::new(2.0, 2.0,)],
        xy_data.data
    );
    assert_eq!(
        vec![
            ("x.label".to_owned(), "Xdata 1/0/0 label".to_owned()),
            ("x.unit".to_owned(), "MASSCHARGERATIO".to_owned()),
            ("y.label".to_owned(), "Ydata 1/0/0/0 label".to_owned()),
            ("y.unit".to_owned(), "MILLIVOLTS".to_owned()),
            ("plot.style".to_owned(), "sticks".to_owned()),
        ],
        xy_data.metadata
    );
    assert_eq!(None, xy_data.table);
    assert!(xy_data.child_node_names.is_empty());
}

fn parse_xydata101_succeeds(reader: &GamlReader) {
    let xy_data = reader.read("/1/0/1").unwrap();
    assert_eq!(
        "XYData 0, 1 (Coordinates 1/0/0 label=1 MICRONS)",
        xy_data.name
    );
    let parameters = &xy_data.parameters;
    assert_eq!(21, parameters.len());
    assert_eq!(
        Parameter::from_str_str("Xdata units", "MASSCHARGERATIO"),
        parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata label", "Xdata 1/0/0 label"),
        parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata linkid", "xdata100-linkid"),
        parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata valueorder", "UNSPECIFIED"),
        parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata units", "MILLIVOLTS"),
        parameters[4]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata label", "Ydata 1/0/0/1 label"),
        parameters[5]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 units", "MICRONS"),
        parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 label", "Coordinates 1/0/0 label"),
        parameters[7]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 linkid", "coordinates100-linkid"),
        parameters[8]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 valueorder", "UNSPECIFIED"),
        parameters[9]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata linkref", "coordinates100-linkid"),
        parameters[10]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 link linkref", "xdata100-linkid"),
        parameters[11]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata values format", "FLOAT32"),
        parameters[12]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata values byteorder", "INTEL"),
        parameters[13]
    );
    assert_eq!(
        Parameter::from_str_u64("Xdata values numvalues", 2),
        parameters[14]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata values format", "FLOAT32"),
        parameters[15]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata values byteorder", "INTEL"),
        parameters[16]
    );
    assert_eq!(
        Parameter::from_str_u64("Ydata values numvalues", 2),
        parameters[17]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 values format", "FLOAT32"),
        parameters[18]
    );
    assert_eq!(
        Parameter::from_str_str("Coordinate 0 values byteorder", "INTEL"),
        parameters[19]
    );
    assert_eq!(
        Parameter::from_str_u64("Coordinate 0 values numvalues", 1),
        parameters[20]
    );
    assert_eq!(
        vec![PointXy::new(1.0, 1.0,), PointXy::new(2.0, 2.0,)],
        xy_data.data
    );
    assert_eq!(
        vec![
            ("x.label".to_owned(), "Xdata 1/0/0 label".to_owned()),
            ("x.unit".to_owned(), "MASSCHARGERATIO".to_owned()),
            ("y.label".to_owned(), "Ydata 1/0/0/1 label".to_owned()),
            ("y.unit".to_owned(), "MILLIVOLTS".to_owned()),
            ("plot.style".to_owned(), "sticks".to_owned()),
        ],
        xy_data.metadata
    );
    assert_eq!(None, xy_data.table);
    assert!(xy_data.child_node_names.is_empty());
}

fn parse_xydata110_succeeds(reader: &GamlReader) {
    let xy_data = reader.read("/1/1/0").unwrap();
    assert_eq!("XYData 0, 0", xy_data.name);
    let parameters = &xy_data.parameters;
    assert_eq!(12, parameters.len());
    assert_eq!(
        Parameter::from_str_str("Xdata units", "MASSCHARGERATIO"),
        parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata label", "Xdata 1/1/0 label"),
        parameters[1]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata linkid", "xdata110-linkid"),
        parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata valueorder", "UNSPECIFIED"),
        parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata units", "MILLIVOLTS"),
        parameters[4]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata label", "Ydata 1/1/0/0 label"),
        parameters[5]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata values format", "FLOAT32"),
        parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("Xdata values byteorder", "INTEL"),
        parameters[7]
    );
    assert_eq!(
        Parameter::from_str_u64("Xdata values numvalues", 2),
        parameters[8]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata values format", "FLOAT32"),
        parameters[9]
    );
    assert_eq!(
        Parameter::from_str_str("Ydata values byteorder", "INTEL"),
        parameters[10]
    );
    assert_eq!(
        Parameter::from_str_u64("Ydata values numvalues", 2),
        parameters[11]
    );
    assert_eq!(
        vec![PointXy::new(1.0, 1.0,), PointXy::new(2.0, 2.0,)],
        xy_data.data
    );
    assert_eq!(
        vec![
            ("x.label".to_owned(), "Xdata 1/1/0 label".to_owned()),
            ("x.unit".to_owned(), "MASSCHARGERATIO".to_owned()),
            ("y.label".to_owned(), "Ydata 1/1/0/0 label".to_owned()),
            ("y.unit".to_owned(), "MILLIVOLTS".to_owned()),
            ("plot.style".to_owned(), "sticks".to_owned()),
        ],
        xy_data.metadata
    );
    assert_eq!(None, xy_data.table);
    assert!(xy_data.child_node_names.is_empty());
}

fn parse_peaktables0000_succeeds(reader: &GamlReader) {
    let peaktable = reader.read("/0/0/0/0").unwrap();
    assert_eq!("Peaktable 0, Peaktable 0/0/0/0/0 name", peaktable.name);
    let parameters = &peaktable.parameters;
    assert_eq!(3, parameters.len());
    assert_eq!(
        Parameter::from_str_str("Peaktable name", "Peaktable 0/0/0/0/0 name"),
        parameters[0]
    );
    assert_eq!(Parameter::from_str_str("Peaktable Peaktable 0/0/0/0/0 parameter name 0 (group=Peaktable 0/0/0/0/0 parameter group 0, label=Peaktable 0/0/0/0/0 parameter label 0, alias=Peaktable 0/0/0/0/0 parameter alias 0)", "Peaktable 0/0/0/0/0 parameter value 0"), parameters[1]);
    assert_eq!(Parameter::from_str_str("Peak 0 number 1 Peak 0/0/0/0/0/0 parameter name 0 (group=Peak 0/0/0/0/0/0 parameter group 0, label=Peak 0/0/0/0/0/0 parameter label 0, alias=Peak 0/0/0/0/0/0 parameter alias 0)", "Peak 0/0/0/0/0/0 parameter value 0"), parameters[2]);
    assert!(peaktable.metadata.is_empty());
    assert_eq!(
        Some(Table {
            column_names: vec![
                Column::new("number", "Number"),
                Column::new("group", "Group"),
                Column::new("name", "Name"),
                Column::new("peak_x_value", "peakXvalue"),
                Column::new("peak_y_value", "peakYvalue"),
                Column::new("baseline_start_x_value", "Baseline Start X Value"),
                Column::new("baseline_start_y_value", "Baseline Start Y Value"),
                Column::new("baseline_end_x_value", "Baseline End X Value"),
                Column::new("baseline_end_y_value", "Baseline End Y Value"),
            ],
            rows: vec![HashMap::from([
                ("number".to_owned(), Value::U64(1)),
                (
                    "group".to_owned(),
                    Value::String("Peak 0/0/0/0/0/0 group".to_owned())
                ),
                (
                    "name".to_owned(),
                    Value::String("Peak 0/0/0/0/0/0 name".to_owned())
                ),
                ("peak_x_value".to_owned(), Value::F64(0.1)),
                ("peak_y_value".to_owned(), Value::F64(100.0)),
                ("baseline_start_x_value".to_owned(), Value::F64(1.1)),
                ("baseline_start_y_value".to_owned(), Value::F64(11.1)),
                ("baseline_end_x_value".to_owned(), Value::F64(2.2)),
                ("baseline_end_y_value".to_owned(), Value::F64(22.2)),
            ])]
        }),
        peaktable.table
    );
    assert_eq!(1, peaktable.child_node_names.len());
    assert_eq!("Basecurve Peak 0, number 1", peaktable.child_node_names[0]);

    let basecurve0 = reader.read("/0/0/0/0/0").unwrap();
    assert_eq!("Basecurve Peak 0, number 1", basecurve0.name);
    assert_eq!(12, basecurve0.parameters.len());
    let basecurve0_parameters = &basecurve0.parameters;
    assert_eq!(
        Parameter::from_str_str("BaseXdata values 0 format", "FLOAT32"),
        basecurve0_parameters[0]
    );
    assert_eq!(
        Parameter::from_str_str("BaseXdata values 0 byteorder", "INTEL"),
        basecurve0_parameters[1]
    );
    assert_eq!(
        Parameter::from_str_u64("BaseXdata values 0 numvalues", 2),
        basecurve0_parameters[2]
    );
    assert_eq!(
        Parameter::from_str_str("BaseXdata values 1 format", "FLOAT32"),
        basecurve0_parameters[3]
    );
    assert_eq!(
        Parameter::from_str_str("BaseXdata values 1 byteorder", "INTEL"),
        basecurve0_parameters[4]
    );
    assert_eq!(
        Parameter::from_str_u64("BaseXdata values 1 numvalues", 2),
        basecurve0_parameters[5]
    );
    assert_eq!(
        Parameter::from_str_str("BaseYdata values 0 format", "FLOAT32"),
        basecurve0_parameters[6]
    );
    assert_eq!(
        Parameter::from_str_str("BaseYdata values 0 byteorder", "INTEL"),
        basecurve0_parameters[7]
    );
    assert_eq!(
        Parameter::from_str_u64("BaseYdata values 0 numvalues", 2),
        basecurve0_parameters[8]
    );
    assert_eq!(
        Parameter::from_str_str("BaseYdata values 1 format", "FLOAT32"),
        basecurve0_parameters[9]
    );
    assert_eq!(
        Parameter::from_str_str("BaseYdata values 1 byteorder", "INTEL"),
        basecurve0_parameters[10]
    );
    assert_eq!(
        Parameter::from_str_u64("BaseYdata values 1 numvalues", 2),
        basecurve0_parameters[11]
    );
    assert!(basecurve0.child_node_names.is_empty());
}
