use super::{gaml_parser::Parameter, GamlError};
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

pub fn next_non_whitespace<'r, R: BufRead + 'r>(
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
    let value = match reader.read_event_into(buf) {
        Ok(Event::Text(e)) => Ok(e.unescape()?.into_owned()),
        Ok(e) => Err(GamlError::new(&format!("Unexpected event: {:?}", &e))),
        Err(e) => Err(GamlError::from_source(e, "Error reading GAML.")),
    }?;

    let ret = (value, reader.read_event_into(buf).map(|e| e.into_owned())?);

    Ok(ret)
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

pub fn read_req_elem<'reader, 'buf, 'f, R: BufRead + 'buf, T>(
    tag_name: &[u8],
    next_event: Event<'_>,
    reader: &'reader mut Reader<R>,
    buf: &'buf mut Vec<u8>,
    constructor: ElemConstructor<R, T>,
) -> Result<(T, Event<'buf>), GamlError> {
    match next_event {
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

pub fn read_opt_elem<'buf, 'reader, 'f, R: BufRead + 'buf, T>(
    tag_name: &[u8],
    next_event: Event<'buf>,
    reader: &'reader mut Reader<R>,
    buf: &'buf mut Vec<u8>,
    constructor: ElemConstructor<'f, R, T>,
) -> Result<(Option<T>, Event<'buf>), GamlError> {
    match next_event {
        Event::Start(e) => {
            let name = e.name().as_ref().to_owned();
            if name == tag_name {
                let (elem, next) =
                    read_req_elem(tag_name, Event::Start(e), reader, buf, constructor)?;
                Ok((Some(elem), next))
            } else {
                Ok((None, Event::Start(e)))
            }
        }
        e => Ok((None, e)),
    }
}

pub fn read_params<'e, 'r: 'e, R: BufRead + 'r>(
    tag_name: &[u8],
    mut next_event: Event<'e>,
    reader: &'r mut Reader<R>,
) -> Result<(Vec<Parameter>, Event<'r>), GamlError> {
    let mut ret = vec![];
    let mut buf = vec![];
    loop {
        match next_event {
            Event::Start(ref bytes) => {
                let name = bytes.name().as_ref().to_owned();
                if name == tag_name {
                    let param = Parameter::new(&next_event, reader, &mut buf)?;
                    ret.push(param);
                    next_event = skip_whitespace(reader, &mut buf)?;
                } else {
                    return Ok((ret, Event::Start(bytes.to_owned())));
                }
            }
            any_other => return Ok((ret, any_other.into_owned())),
        }
    }
}
