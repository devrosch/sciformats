use super::GamlError;
use quick_xml::{
    events::{BytesStart, Event},
    name::QName,
    Reader,
};
use std::{collections::HashMap, io::BufRead};

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

pub fn read_value<'b, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'b mut Vec<u8>,
) -> Result<(String, Event<'b>), GamlError> {
    let value = match reader.read_event_into(buf) {
        Ok(Event::Text(e)) => Ok(e.unescape()?.into_owned()),
        Ok(e) => Err(GamlError::new(&format!("Unexpected event: {:?}", &e))),
        Err(e) => Err(GamlError::from_source(e, "Error reading GAML.")),
    }?;

    let ret = (value, reader.read_event_into(buf).map(|e| e.into_owned())?);

    Ok(ret)
}

pub fn read_start<'b, R: BufRead>(
    tag_name: &[u8],
    reader: &mut Reader<R>,
    buf: &'b mut Vec<u8>,
) -> Result<BytesStart<'b>, GamlError> {
    // TODO: make efficient
    let event = reader.read_event_into(buf).map(|e| e.into_owned())?;
    match event {
        Event::Start(e) => {
            let name = e.name().as_ref().to_owned();
            if name == tag_name {
                Ok(e)
            } else {
                Err(GamlError::new(&format!(
                    "Unexpected start tag: {:?}",
                    std::str::from_utf8(&name)
                )))
            }
        }
        e => Err(GamlError::new(&format!(
            "Unexpected event instead of start tag: {:?}",
            &e
        ))),
    }
}

pub fn check_end(tag_name: &[u8], event: &Event<'_>) -> Result<(), GamlError> {
    match event {
        Event::End(e) => {
            let name = e.name().as_ref().to_owned();
            if name == tag_name {
                Ok(())
            } else {
                Err(GamlError::new(&format!(
                    "Unexpected end tag: {:?}",
                    std::str::from_utf8(&name)
                )))
            }
        }
        e => Err(GamlError::new(&format!(
            "Unexpected event instead of end tag: {:?}",
            &e
        ))),
    }
}

pub fn consume_end<R: BufRead>(
    tag_name: &[u8],
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
) -> Result<(), GamlError> {
    let event = reader.read_event_into(buf)?;
    check_end(tag_name, &event)
}

pub fn read_req_elem<'b, R: BufRead, T>(
    tag_name: &[u8],
    next_event: &Event<'b>,
    reader: &mut Reader<R>,
    constructor: &dyn Fn(&Event<'_>, &mut Reader<R>) -> Result<T, GamlError>,
) -> Result<T, GamlError> {
    match next_event {
        Event::Start(e) => {
            let name = e.name().as_ref().to_owned();
            if name == tag_name {
                let elem = constructor(next_event, reader)?;
                Ok(elem)
            } else {
                Err(GamlError::new(&format!(
                    "Unexpected start tag: {:?}",
                    std::str::from_utf8(&name)
                )))
            }
        }
        e => Err(GamlError::new(&format!(
            "Unexpected event instead of start tag: {:?}",
            &e
        ))),
    }
}

pub fn read_opt_elem<'b, R: BufRead, T>(
    tag_name: &[u8],
    next_event: &Event<'b>,
    reader: &mut Reader<R>,
    constructor: &dyn Fn(&Event<'_>, &mut Reader<R>) -> Result<T, GamlError>,
) -> Result<Option<T>, GamlError> {
    match next_event {
        Event::Start(e) => {
            let name = e.name().as_ref().to_owned();
            if name == tag_name {
                Ok(Some(read_req_elem(
                    tag_name,
                    next_event,
                    reader,
                    constructor,
                )?))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}
