use super::gaml_utils::{
    check_end, get_attributes, get_opt_attr, get_req_attr, read_start, read_value, skip_whitespace,
};
use super::GamlError;
use crate::api::Parser;
use crate::gaml::gaml_utils::read_opt_elem;
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
    pub fn new<R: BufRead>(_name: &str, mut reader: Reader<R>) -> Result<Self, GamlError> {
        const TAG: &[u8] = b"GAML";
        let mut buf = Vec::new();

        // skip <?xml> element
        let _e0 = reader.read_event_into(&mut buf);
        let _e1 = reader.read_event_into(&mut buf);

        let start_tag = read_start(TAG, &mut reader, &mut buf)?;
        let attr_map = get_attributes(&start_tag, &reader);
        let version = get_req_attr("version", &attr_map)?;
        let name = get_opt_attr("name", &attr_map);

        let mut next_event = skip_whitespace(&mut reader, &mut buf)?;
        let integrity = read_opt_elem(b"integrity", &next_event, &mut reader, &Integrity::new)?;

        let param_0 = Parameter::new(skip_whitespace(&mut reader, &mut buf)?, &mut reader)?;
        let param_1 = Parameter::new(skip_whitespace(&mut reader, &mut buf)?, &mut reader)?;
        let param_2 = Parameter::new(skip_whitespace(&mut reader, &mut buf)?, &mut reader)?;
        let parameters: Vec<Parameter> = vec![param_0, param_1, param_2];

        next_event = skip_whitespace(&mut reader, &mut buf)?;
        check_end(TAG, &next_event)?;

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
    pub fn new<R: BufRead>(event: &Event<'_>, reader: &mut Reader<R>) -> Result<Self, GamlError> {
        const TAG: &[u8] = b"integrity";
        match event {
            Event::Start(e) => match e.name().as_ref() {
                TAG => {
                    let attr_map = get_attributes(e, reader);
                    let algorithm = get_req_attr("algorithm", &attr_map)?;
                    let mut buf = Vec::new();
                    let (value, next_elem) = read_value(reader, &mut buf)?;
                    check_end(TAG, &next_elem)?;
                    Ok(Self { algorithm, value })
                }
                tag_name => Err(GamlError::new(&format!(
                    "Unexpected tag instead of \"{}\": {:?}",
                    str::from_utf8(TAG).unwrap(),
                    str::from_utf8(tag_name)
                ))),
            },
            e => Err(GamlError::new(&format!("Unexpected event: {:?}", &e))),
        }
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
    pub fn new<R: BufRead>(event: Event<'_>, reader: &mut Reader<R>) -> Result<Self, GamlError> {
        const TAG: &[u8] = b"parameter";
        match event {
            Event::Start(e) => match e.name().as_ref() {
                TAG => {
                    let attr_map = get_attributes(&e, reader);
                    let group = get_opt_attr("group", &attr_map);
                    let name = get_req_attr("name", &attr_map)?;
                    let label = get_opt_attr("label", &attr_map);
                    let alias = get_opt_attr("alias", &attr_map);

                    let mut buf = Vec::new();
                    let (value, next_elem) = read_value(reader, &mut buf)?;

                    check_end(TAG, &next_elem)?;

                    Ok(Parameter {
                        group,
                        name,
                        label,
                        alias,
                        value,
                    })
                }
                tag_name => Err(GamlError::new(&format!(
                    "Unexpected tag instead of \"{}\": {:?}",
                    str::from_utf8(TAG).unwrap(),
                    str::from_utf8(tag_name)
                ))),
            },
            e => Err(GamlError::new(&format!("Unexpected event: {:?}", &e))),
        }
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
