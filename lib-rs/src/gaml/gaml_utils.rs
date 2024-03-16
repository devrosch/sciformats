use quick_xml::{
    events::{BytesStart, Event},
    name::QName,
    Reader,
};
use std::{collections::HashMap, io::BufRead};

use super::GamlError;

pub fn get_attributes<'a, R>(
    bytes_start: &'a BytesStart<'a>,
    reader: &Reader<R>,
) -> HashMap<QName<'a>, std::borrow::Cow<'a, str>> {
    bytes_start
        .attributes()
        .filter(|a| a.is_ok())
        .map(|a| {
            let attr = a.unwrap();
            (
                attr.key,
                attr.decode_and_unescape_value(reader).unwrap_or_default(),
            )
        })
        .collect::<HashMap<_, _>>()
}

pub fn get_req_attr<'a>(
    name: &str,
    attr_map: &HashMap<QName<'a>, std::borrow::Cow<'a, str>>,
) -> Result<String, GamlError> {
    Ok(attr_map
        .get(&QName(name.as_bytes()))
        .ok_or(GamlError::new(&format!("Missing attribute: {:?}", name)))?
        .clone()
        .into_owned())
}

pub fn get_opt_attr<'a>(
    name: &str,
    attr_map: &HashMap<QName<'a>, std::borrow::Cow<'a, str>>,
) -> Option<String> {
    attr_map
        .get(&QName(name.as_bytes()))
        .map(|name| name.clone().into_owned())
}

pub fn skip_whitespace<'b, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'b mut Vec<u8>,
) -> Result<Event<'b>, GamlError> {
    let event: Result<Event<'_>, quick_xml::Error> = reader.read_event_into(buf);
    // skip whitespace
    match event {
        Err(e) => Err(GamlError::from_source(e, "Error skipping whitespace.")),
        Ok(Event::Text(ws)) => match ws.unescape()?.trim().is_empty() {
            true => Ok(()),
            false => Err(GamlError::new(&format!(
                "Unexpected text instead of whitespace: {:?}",
                &ws
            ))),
        },
        Ok(e) => Err(GamlError::new(&format!(
            "Unexpected event instead of whitespace: {:?}",
            &e
        ))),
    }?;
    // return event following the whitespace
    reader
        .read_event_into(buf)
        .map_err(|e| GamlError::from_source(e, "Error reading next event after whitespace."))
}
