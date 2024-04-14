use super::{GamlError, SeekBufRead};
use quick_xml::{
    events::{BytesStart, Event},
    name::QName,
    Reader,
};
use std::{cell::RefCell, collections::HashMap, io::BufRead, rc::Rc, str, vec};

fn check_matches_tag_name(tag: &[u8], bytes_start: &BytesStart<'_>) -> Result<(), GamlError> {
    if bytes_start.name().as_ref() != tag {
        Err(GamlError::new(&format!(
            "Unexpected tag instead of \"{}\": {:?}",
            str::from_utf8(tag).unwrap_or_default(),
            str::from_utf8(bytes_start.name().as_ref())
        )))
    } else {
        Ok(())
    }
}

pub(super) fn read_start<'b>(
    tag: &[u8],
    event: &'b Event<'b>,
) -> Result<&'b BytesStart<'b>, GamlError> {
    if let Event::Start(bytes_start) = event {
        check_matches_tag_name(tag, bytes_start)?;
        Ok(bytes_start)
    } else {
        Err(GamlError::new(&format!("Unexpected event: {:?}", event)))
    }
}

pub(super) fn read_empty<'b>(
    tag: &[u8],
    event: &'b Event<'b>,
) -> Result<&'b BytesStart<'b>, GamlError> {
    if let Event::Empty(bytes_start) = event {
        check_matches_tag_name(tag, bytes_start)?;
        Ok(bytes_start)
    } else {
        Err(GamlError::new(&format!("Unexpected event: {:?}", event)))
    }
}

pub(super) fn read_start_or_empty<'b>(
    tag: &[u8],
    event: &'b Event<'b>,
) -> Result<(&'b BytesStart<'b>, bool), GamlError> {
    match event {
        Event::Start(bytes_start) => {
            check_matches_tag_name(tag, bytes_start)?;
            Ok((bytes_start, false))
        }
        Event::Empty(bytes_start) => {
            check_matches_tag_name(tag, bytes_start)?;
            Ok((bytes_start, true))
        }
        _ => Err(GamlError::new(&format!("Unexpected event: {:?}", event))),
    }
}

pub(super) fn get_attributes<'a, R>(
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

pub(super) fn get_req_attr<'a>(
    name: &str,
    attr_map: &HashMap<QName<'a>, std::borrow::Cow<'a, str>>,
) -> Result<String, GamlError> {
    Ok(attr_map
        .get(&QName(name.as_bytes()))
        .ok_or(GamlError::new(&format!("Missing attribute: {:?}", name)))?
        .clone()
        .into_owned())
}

pub(super) fn get_opt_attr<'a>(
    name: &str,
    attr_map: &HashMap<QName<'a>, std::borrow::Cow<'a, str>>,
) -> Option<String> {
    attr_map
        .get(&QName(name.as_bytes()))
        .map(|name| name.clone().into_owned())
}

pub(super) fn skip_xml_decl<'b, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'b mut Vec<u8>,
) -> Result<Event<'b>, GamlError> {
    let event = skip_whitespace(reader, buf).map(|e| e.into_owned())?;
    match &event {
        Event::Decl(_) => skip_whitespace(reader, buf),
        _ => Ok(event),
    }
}

pub(super) fn skip_whitespace<R: BufRead>(
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

pub(super) fn next_non_whitespace<'r, R: BufRead>(
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

pub(super) fn read_value<'b, R: BufRead>(
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

pub(super) fn read_value_pos<'b, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'b mut Vec<u8>,
) -> Result<(u64, u64, Event<'b>), GamlError> {
    let start_pos = reader.buffer_position() as u64;
    let mut end_pos = start_pos;
    let next = loop {
        let event = match reader.read_event_into(buf)? {
            Event::Text(_) => {
                end_pos = reader.buffer_position() as u64;
                None
            }
            Event::Comment(_) => None,
            any_other => Some(any_other.into_owned()),
        };

        if let Some(e) = event {
            break e;
        }
    };

    Ok((start_pos, end_pos, next))
}

pub(super) fn check_end(tag_name: &[u8], event: &Event<'_>) -> Result<(), GamlError> {
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

type ElemConstructor<'f, R, T> =
    &'f (dyn Fn(&Event<'_>, &mut Reader<R>, &mut Vec<u8>) -> Result<T, GamlError>);

pub(super) fn read_req_elem<'buf, R: BufRead, T>(
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

pub(super) fn read_opt_elem<'e, R: BufRead, T>(
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

pub(super) fn read_sequence<'e, R: BufRead, T>(
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

type ElemConstructorRc<'f, T> = &'f (dyn Fn(
    &Event<'_>,
    Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    &mut Vec<u8>,
) -> Result<T, GamlError>);

// todo: avoid code duplication with read_sequence
pub(super) fn read_sequence_rc<'e, T>(
    tag_name: &[u8],
    mut next: Event<'e>,
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    buf: &mut Vec<u8>,
    constructor: ElemConstructorRc<T>,
) -> Result<(Vec<T>, Event<'e>), GamlError> {
    let mut ret = vec![];
    loop {
        match &next {
            Event::Start(bytes) | Event::Empty(bytes) => {
                let name = bytes.name().as_ref().to_owned();
                if name == tag_name {
                    let elem = constructor(&next, Rc::clone(&reader_ref), buf)?;
                    ret.push(elem);
                    let mut reader = reader_ref.borrow_mut();
                    next = skip_whitespace(&mut reader, buf)?;
                } else {
                    return Ok((ret, next));
                }
            }
            _ => return Ok((ret, next)),
        }
    }
}
