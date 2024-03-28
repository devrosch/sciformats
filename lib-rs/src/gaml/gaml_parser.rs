use super::gaml_utils::{
    check_end, get_attributes, get_opt_attr, get_req_attr, next_non_whitespace, read_opt_elem,
    read_sequence, read_start, read_value, skip_whitespace, skip_xml_decl,
};
use super::GamlError;
use crate::api::Parser;
use chrono::{DateTime, FixedOffset};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::io::{BufRead, BufReader, Read, Seek};
use std::str::{self, FromStr};
use strum::EnumString;

pub struct GamlParser {}

impl<T: Seek + Read + 'static> Parser<T> for GamlParser {
    type R = Gaml;
    type E = GamlError;

    fn parse(name: &str, input: T) -> Result<Self::R, Self::E> {
        let buf_reader = BufReader::new(input);
        let reader = Reader::from_reader(buf_reader);
        Self::R::new(name, reader)
    }
}

pub struct Gaml {
    // Attributes
    pub version: String,
    pub name: Option<String>,
    // Elements
    pub integrity: Option<Integrity>,
    pub parameters: Vec<Parameter>,
    pub experiments: Vec<Experiment>,
}

impl Gaml {
    const TAG: &'static [u8] = b"GAML";

    pub fn new<R: BufRead>(_name: &str, mut reader: Reader<R>) -> Result<Self, GamlError> {
        let mut buf = Vec::new();

        // skip <?xml> element
        let next = skip_xml_decl(&mut reader, &mut buf)?;

        // attributes
        let start = read_start(Self::TAG, &next)?;
        let attr_map = get_attributes(start, &reader);
        let version = get_req_attr("version", &attr_map)?;
        let name = get_opt_attr("name", &attr_map);
        let next = skip_whitespace(&mut reader, &mut buf)?;

        // nested elements
        let (integrity, next) =
            read_opt_elem(b"integrity", next, &mut reader, &mut buf, &Integrity::new)?;
        let next = next_non_whitespace(next, &mut reader, &mut buf)?;
        let (parameters, next) =
            read_sequence(b"parameter", next, &mut reader, &mut buf, &Parameter::new)?;
        let next = next_non_whitespace(next, &mut reader, &mut buf)?;
        let (experiments, next) =
            read_sequence(b"experiment", next, &mut reader, &mut buf, &Experiment::new)?;
        let next = next_non_whitespace(next, &mut reader, &mut buf)?;

        check_end(Self::TAG, &next)?;

        Ok(Self {
            version,
            name,
            integrity,
            parameters,
            experiments,
        })
    }
}

pub struct Integrity {
    // Attributes
    pub algorithm: String,
    // Content
    pub value: String,
}

impl Integrity {
    const TAG: &'static [u8] = b"integrity";

    pub fn new<R: BufRead>(
        event: &Event<'_>,
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        let start = read_start(Self::TAG, event)?;

        // attributes
        let attr_map = get_attributes(start, reader);
        let algorithm = get_req_attr("algorithm", &attr_map)?;

        // value
        let (value, next) = read_value(reader, buf)?;

        check_end(Self::TAG, &next)?;

        Ok(Self { algorithm, value })
    }
}

pub struct Parameter {
    // Attributes
    pub group: Option<String>,
    pub name: String,
    pub label: Option<String>,
    pub alias: Option<String>,
    // Content
    pub value: String,
}

impl Parameter {
    const TAG: &'static [u8] = b"parameter";

    pub fn new<R: BufRead>(
        event: &Event<'_>,
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        let start = read_start(Self::TAG, event)?;

        // attributes
        let attr_map = get_attributes(start, reader);
        let group = get_opt_attr("group", &attr_map);
        let name = get_req_attr("name", &attr_map)?;
        let label = get_opt_attr("label", &attr_map);
        let alias = get_opt_attr("alias", &attr_map);

        // value
        let (value, next) = read_value(reader, buf)?;

        check_end(Self::TAG, &next)?;

        Ok(Parameter {
            group,
            name,
            label,
            alias,
            value,
        })
    }
}

pub struct Experiment {
    // Attributes
    pub name: Option<String>,
    // Elements
    pub collectdate: Option<DateTime<FixedOffset>>,
    pub parameters: Vec<Parameter>,
    pub traces: Vec<Trace>,
}

impl Experiment {
    const TAG: &'static [u8] = b"experiment";

    pub fn new<R: BufRead>(
        event: &Event<'_>,
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        let start = read_start(Self::TAG, event)?;

        // attributes
        let attr_map = get_attributes(start, reader);
        let name = get_opt_attr("name", &attr_map);

        // nested elements
        let next = skip_whitespace(reader, buf)?;
        let (datetime, next) = read_opt_elem(b"collectdate", next, reader, buf, &Collectdate::new)?;
        let collectdate = match datetime {
            None => None,
            Some(dt) => Some(DateTime::parse_from_rfc3339(&dt.value)?),
        };
        let next = next_non_whitespace(next, reader, buf)?;
        let (parameters, next) = read_sequence(b"parameter", next, reader, buf, &Parameter::new)?;
        let next = next_non_whitespace(next, reader, buf)?;
        let (traces, next) = read_sequence(b"trace", next, reader, buf, &Trace::new)?;
        let next = next_non_whitespace(next, reader, buf)?;

        check_end(Self::TAG, &next)?;

        Ok(Self {
            name,
            collectdate,
            parameters,
            traces,
        })
    }
}

struct Collectdate {
    pub value: String,
}

impl Collectdate {
    const TAG: &'static [u8] = b"collectdate";

    pub fn new<R: BufRead>(
        event: &Event<'_>,
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        read_start(Self::TAG, event)?;
        // value
        let (value, next) = read_value(reader, buf)?;
        check_end(Self::TAG, &next)?;

        Ok(Self {
            value: value.trim().into(),
        })
    }
}

#[derive(EnumString)]
pub enum Technique {
    #[strum(serialize = "ATOMIC")]
    Atomic,
    #[strum(serialize = "CHROM")]
    Chrom,
    #[strum(serialize = "FLUOR")]
    Fluor,
    #[strum(serialize = "IR")]
    Ir,
    #[strum(serialize = "MS")]
    Ms,
    #[strum(serialize = "NIR")]
    Nir,
    #[strum(serialize = "NMR")]
    Nmr,
    #[strum(serialize = "PDA")]
    Pda,
    #[strum(serialize = "PARTICLE")]
    Particle,
    #[strum(serialize = "POLAR")]
    Polar,
    #[strum(serialize = "RAMAN")]
    Raman,
    #[strum(serialize = "THERMAL")]
    Thermal,
    #[strum(serialize = "UNKNOWN")]
    Unknown,
    #[strum(serialize = "UVVIS")]
    Uvvis,
    #[strum(serialize = "XRAY")]
    Xray,
}

pub struct Trace {
    // Attributes
    pub name: Option<String>,
    pub technique: Technique,
    // Elements
    pub parameters: Vec<Parameter>,
    // todo:
    // coordinates
    // xdata
}

impl Trace {
    const TAG: &'static [u8] = b"trace";

    pub fn new<R: BufRead>(
        event: &Event<'_>,
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        let start = read_start(Self::TAG, event)?;

        // attributes
        let attr_map = get_attributes(start, reader);
        let name = get_opt_attr("name", &attr_map);
        let technique_str = get_req_attr("technique", &attr_map)?;
        let technique = Technique::from_str(&technique_str).map_err(|e| {
            GamlError::from_source(e, format!("Unexpected technique: {}", &technique_str))
        })?;

        // nested elements
        let next = skip_whitespace(reader, buf)?;
        let (parameters, next) = read_sequence(b"parameter", next, reader, buf, &Parameter::new)?;
        let next = next_non_whitespace(next, reader, buf)?;

        // todo: read sequences of coordinates, xdata

        check_end(Self::TAG, &next)?;

        Ok(Self {
            name,
            technique,
            parameters,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use super::*;
    use std::io::Cursor;

    #[test]
    fn parsing_simple_gaml_succeeds() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml test file\">\n
                            <integrity algorithm=\"SHA1\">03cfd743661f07975fa2f1220c5194cbaff48451</integrity>\n
                            <parameter name=\"parameter0\" label=\"Parameter label 0\" group=\"Parameter group 0\">Parameter value 0</parameter>\n
                            <!-- A comment -->
                            <parameter name=\"parameter1\" label=\"Parameter label 1\" group=\"Parameter group 1\">\
                            <!-- A comment -->Parameter <!-- A comment -->value 1<!-- A comment --></parameter>\
                            <parameter name=\"parameter2\" label=\"Parameter label 2\" group=\"Parameter group 2\">Parameter value 2</parameter>\n
                            <experiment name=\"Experiment name\">
                                <collectdate>2024-03-27T06:46:00Z</collectdate>
                                <parameter name=\"exp-parameter0\" label=\"Experiment parameter label 0\">Experiment parameter value 0</parameter>
                                <trace name=\"Trace 0\" technique=\"UNKNOWN\">
                                    <parameter name=\"trace-parameter0\" label=\"Trace parameter 0\" group=\"Trace 0 group\">Parameter value 0</parameter>
                                    <parameter name=\"trace-parameter1\" label=\"Trace parameter 1\" group=\"Trace 1 group\">Parameter value 1</parameter>
                                </trace>
                            </experiment>
                        </GAML>";
        let cursor = Cursor::new(xml);

        let gaml = GamlParser::parse("test.gaml", cursor).unwrap();

        assert_eq!("1.20", gaml.version);
        assert_eq!(Some("Gaml test file".into()), gaml.name);
        let integrity = &gaml.integrity.unwrap();
        assert_eq!("SHA1", integrity.algorithm);
        let parameters = &gaml.parameters;
        assert_eq!(3, parameters.len());
        assert_eq!("parameter0", &parameters[0].name);
        assert_eq!(Some("Parameter label 0".into()), parameters[0].label);
        assert_eq!(Some("Parameter group 0".into()), parameters[0].group);
        assert_eq!("Parameter value 0", &parameters[0].value);
        assert_eq!("parameter1", &parameters[1].name);
        assert_eq!(Some("Parameter label 1".into()), parameters[1].label);
        assert_eq!(Some("Parameter group 1".into()), parameters[1].group);
        assert_eq!("Parameter value 1", &parameters[1].value);
        assert_eq!("parameter2", &parameters[2].name);
        assert_eq!(Some("Parameter label 2".into()), parameters[2].label);
        assert_eq!(Some("Parameter group 2".into()), parameters[2].group);
        assert_eq!("Parameter value 2", &parameters[2].value);
        let experiments = &gaml.experiments;
        assert_eq!(1, experiments.len());
        let date = NaiveDate::from_ymd_opt(2024, 03, 27).unwrap();
        let time = NaiveTime::from_hms_opt(06, 46, 0).unwrap();
        assert_eq!(
            DateTime::<FixedOffset>::from_naive_utc_and_offset(
                NaiveDateTime::new(date, time),
                FixedOffset::east_opt(0).unwrap()
            ),
            experiments[0].collectdate.unwrap()
        );
        let experiment_parameters = &experiments[0].parameters;
        assert_eq!(1, experiment_parameters.len());
        assert_eq!("exp-parameter0", &experiment_parameters[0].name);
        assert_eq!(
            Some("Experiment parameter label 0".into()),
            experiment_parameters[0].label
        );
        assert_eq!(None, experiment_parameters[0].group);
        assert_eq!(
            "Experiment parameter value 0",
            &experiment_parameters[0].value
        );
    }
}
