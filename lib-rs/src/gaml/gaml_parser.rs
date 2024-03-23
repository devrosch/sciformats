use super::gaml_utils::{
    check_end, get_attributes, get_opt_attr, get_req_attr, read_opt_elem, read_start, read_value,
    skip_whitespace, skip_xml_decl,
};
use super::GamlError;
use crate::api::Parser;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::io::{BufRead, BufReader, Read, Seek};
use std::str;

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
}

impl Gaml {
    const TAG: &'static [u8] = b"GAML";

    pub fn new<R: BufRead>(_name: &str, mut reader: Reader<R>) -> Result<Self, GamlError> {
        let mut buf = Vec::new();

        // skip <?xml> element
        let start_event = skip_xml_decl(&mut reader, &mut buf)?;

        let start = read_start(Self::TAG, &start_event)?;
        let attr_map = get_attributes(start, &reader);
        let version = get_req_attr("version", &attr_map)?;
        let name = get_opt_attr("name", &attr_map);

        let mut next_event = skip_whitespace(&mut reader, &mut buf)?;
        let integrity = read_opt_elem(b"integrity", &next_event, &mut reader, &Integrity::new)?;

        let param_0 = Parameter::new(
            &skip_whitespace(&mut reader, &mut buf)?,
            &mut reader,
            &mut buf,
        )?;
        let param_1 = Parameter::new(
            &skip_whitespace(&mut reader, &mut buf)?,
            &mut reader,
            &mut buf,
        )?;
        let param_2 = Parameter::new(
            &skip_whitespace(&mut reader, &mut buf)?,
            &mut reader,
            &mut buf,
        )?;
        let parameters: Vec<Parameter> = vec![param_0, param_1, param_2];

        next_event = skip_whitespace(&mut reader, &mut buf)?;
        check_end(Self::TAG, &next_event)?;

        Ok(Self {
            version,
            name,
            integrity,
            parameters,
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

    pub fn new<R: BufRead>(event: &Event<'_>, reader: &mut Reader<R>) -> Result<Self, GamlError> {
        let start = read_start(Self::TAG, event)?;

        let attr_map = get_attributes(start, reader);
        let algorithm = get_req_attr("algorithm", &attr_map)?;

        let mut buf = Vec::new();
        let (value, next_elem) = read_value(reader, &mut buf)?;
        check_end(Self::TAG, &next_elem)?;

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

    pub fn new<'e, 'b: 'e, R: BufRead>(
        event: &'e Event<'b>,
        reader: &mut Reader<R>,
        buf: &'b mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        let start = read_start(Self::TAG, event)?;

        let attr_map = get_attributes(start, reader);
        let group = get_opt_attr("group", &attr_map);
        let name = get_req_attr("name", &attr_map)?;
        let label = get_opt_attr("label", &attr_map);
        let alias = get_opt_attr("alias", &attr_map);

        // let mut buf = Vec::new();
        let (value, next_elem) = read_value(reader, buf)?;

        check_end(Self::TAG, &next_elem)?;

        Ok(Parameter {
            group,
            name,
            label,
            alias,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parsing_simple_gaml_succeeds() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
                        <GAML version="1.20" name="Gaml test file">
                            <integrity algorithm="SHA1">03cfd743661f07975fa2f1220c5194cbaff48451</integrity>
                            <parameter name="parameter0" label="Parameter label 0" group="Parameter group 0">Parameter value 0</parameter>
                            <parameter name="parameter1" label="Parameter label 1" group="Parameter group 1">Parameter value 1</parameter>
                            <parameter name="parameter2" label="Parameter label 2" group="Parameter group 2">Parameter value 2</parameter>
                        </GAML>"#;
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
    }
}
