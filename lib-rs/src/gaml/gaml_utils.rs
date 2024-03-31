use super::GamlError;
use quick_xml::{
    events::{BytesStart, Event},
    name::QName,
    Reader,
};
use std::{collections::HashMap, io::BufRead, str, vec};

pub fn read_start<'b>(tag: &[u8], event: &'b Event<'b>) -> Result<&'b BytesStart<'b>, GamlError> {
    if let Event::Start(e) = event {
        if e.name().as_ref() != tag {
            return Err(GamlError::new(&format!(
                "Unexpected tag instead of \"{}\": {:?}",
                str::from_utf8(tag).unwrap_or_default(),
                str::from_utf8(e.name().as_ref())
            )));
        }
        Ok(e)
    } else {
        Err(GamlError::new(&format!("Unexpected event: {:?}", event)))
    }
}

pub fn read_empty<'b>(tag: &[u8], event: &'b Event<'b>) -> Result<&'b BytesStart<'b>, GamlError> {
    if let Event::Empty(e) = event {
        if e.name().as_ref() != tag {
            return Err(GamlError::new(&format!(
                "Unexpected tag instead of \"{}\": {:?}",
                str::from_utf8(tag).unwrap_or_default(),
                str::from_utf8(e.name().as_ref())
            )));
        }
        Ok(e)
    } else {
        Err(GamlError::new(&format!("Unexpected event: {:?}", event)))
    }
}

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

pub fn skip_xml_decl<'b, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'b mut Vec<u8>,
) -> Result<Event<'b>, GamlError> {
    let event = skip_whitespace(reader, buf).map(|e| e.into_owned())?;
    match &event {
        Event::Decl(_) => skip_whitespace(reader, buf),
        _ => Ok(event),
    }
}

pub fn skip_whitespace<R: BufRead>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
) -> Result<Event<'static>, GamlError> {
    loop {
        let event: Event<'_> = reader.read_event_into(buf)?;
        match event {
            Event::Text(bytes) => {
                if !bytes.unescape()?.trim().is_empty() {
                    // TODO: more efficient way than to call into_owned()?
                    return Ok(Event::Text(bytes.into_owned()));
                };
            }
            Event::Comment(_) => (),
            // TODO: more efficient way than to call into_owned()?
            any_other => return Ok(any_other.into_owned()),
        }
    }
}

pub fn next_non_whitespace<'r, R: BufRead>(
    event: Event<'r>,
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
) -> Result<Event<'r>, GamlError> {
    let is_ws = match &event {
        Event::Text(bytes) => bytes.unescape()?.trim().is_empty(),
        Event::Comment(_) => true,
        _ => false,
    };

    match is_ws {
        true => skip_whitespace(reader, buf),
        false => Ok(event),
    }
}

pub fn read_value<'b, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'b mut Vec<u8>,
) -> Result<(String, Event<'b>), GamlError> {
    let mut value = String::new();
    let next = loop {
        let event = match reader.read_event_into(buf)? {
            Event::Text(bytes) => {
                value += &bytes.unescape()?;
                None
            }
            Event::Comment(_) => None,
            any_other => Some(any_other.into_owned()),
        };

        if let Some(e) = event {
            break e;
        }
    };

    Ok((value, next))
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

type ElemConstructor<'f, R, T> =
    &'f (dyn Fn(&Event<'_>, &mut Reader<R>, &mut Vec<u8>) -> Result<T, GamlError>);

pub fn read_req_elem<'buf, R: BufRead, T>(
    tag_name: &[u8],
    next: Event<'_>,
    reader: &mut Reader<R>,
    buf: &'buf mut Vec<u8>,
    constructor: ElemConstructor<R, T>,
) -> Result<(T, Event<'buf>), GamlError> {
    match next {
        Event::Start(e) => {
            let name = e.name().as_ref().to_owned();
            if name == tag_name {
                let evt = Event::Start(e);
                let elem = constructor(&evt, reader, buf)?;
                let next = reader.read_event_into(buf)?;
                Ok((elem, next))
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

pub fn read_opt_elem<'e, R: BufRead, T>(
    tag_name: &[u8],
    next: Event<'e>,
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    constructor: ElemConstructor<R, T>,
) -> Result<(Option<T>, Event<'e>), GamlError> {
    match next {
        Event::Start(e) => {
            let name = e.name().as_ref().to_owned();
            if name == tag_name {
                let (elem, next) =
                    read_req_elem(tag_name, Event::Start(e), reader, buf, constructor)?;
                // todo: avoid into_owned()?
                Ok((Some(elem), next.into_owned()))
            } else {
                Ok((None, Event::Start(e)))
            }
        }
        e => Ok((None, e)),
    }
}

pub fn read_sequence<'e, R: BufRead, T>(
    tag_name: &[u8],
    mut next: Event<'e>,
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    constructor: ElemConstructor<R, T>,
) -> Result<(Vec<T>, Event<'e>), GamlError> {
    let mut ret = vec![];
    loop {
        match &next {
            Event::Start(bytes) | Event::Empty(bytes) => {
                let name = bytes.name().as_ref().to_owned();
                if name == tag_name {
                    let elem = constructor(&next, reader, buf)?;
                    ret.push(elem);
                    next = skip_whitespace(reader, buf)?;
                } else {
                    return Ok((ret, next));
                }
            }
            _ => return Ok((ret, next)),
        }
    }
}
