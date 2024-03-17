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
    // TODO: more efficient way than to call e.into_owned()?
    let mut event: Result<Event<'_>, quick_xml::Error> =
        reader.read_event_into(buf).map(|e| e.into_owned());

    let is_ws = match &event {
        Err(_) => false,
        Ok(Event::Text(ws)) => ws.unescape()?.trim().is_empty(),
        Ok(_) => false,
    };

    if is_ws {
        event = reader.read_event_into(buf);
    }

    event.map_err(|e| GamlError::from_source(e, "Error skipping whitespace."))
}
