use crate::api::SeekBufRead;
use quick_xml::{
    events::{BytesStart, Event},
    name::QName,
    Reader,
};
use std::{cell::RefCell, collections::HashMap, error::Error, fmt, io::BufRead, rc::Rc, str, vec};

#[derive(Debug)]
pub struct SfXmlError {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl SfXmlError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.into(),
            source: None,
        }
    }

    pub fn from_source(source: impl Into<Box<dyn Error>>, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: Some(source.into()),
        }
    }

    pub(super) fn into_inner(self) -> (String, Option<Box<dyn Error>>) {
        (self.message, self.source)
    }
}

impl Error for SfXmlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|b| b.as_ref())
    }
}

impl fmt::Display for SfXmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<quick_xml::Error> for SfXmlError {
    fn from(value: quick_xml::Error) -> Self {
        Self::from_source(value, "Structural error parsing XML.")
    }
}

impl From<std::io::Error> for SfXmlError {
    fn from(value: std::io::Error) -> Self {
        Self::from_source(value, "I/O error parsing XML.")
    }
}

pub(super) struct BufEvent<'buf> {
    pub event: Event<'buf>,
    pub buf: &'buf mut Vec<u8>,
}

impl<'buf> BufEvent<'buf> {
    pub fn new(event: Event<'buf>, buf: &'buf mut Vec<u8>) -> BufEvent<'buf> {
        Self { event, buf }
    }
}

type ElemConstructor<'f, 'buf, R, T, E> =
    &'f (dyn Fn(BufEvent<'buf>, &mut Reader<R>) -> Result<(T, BufEvent<'buf>), E>);

type ElemConstructorRc<'f, 'buf, T, E> = &'f (dyn Fn(
    BufEvent<'buf>,
    Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
) -> Result<(T, BufEvent<'buf>), E>);

type ElemConstructorCore<'f, 'buf, Reader, T, E> =
    &'f mut (dyn Fn(BufEvent<'buf>, &mut Reader) -> Result<(T, BufEvent<'buf>), E>);

pub(super) enum XmlTagStart<'buf> {
    Start(HashMap<QName<'buf>, std::borrow::Cow<'buf, str>>),
    Empty(HashMap<QName<'buf>, std::borrow::Cow<'buf, str>>),
}

impl<'buf> XmlTagStart<'buf> {
    pub fn get_req_attr(&self, name: &str) -> Result<String, SfXmlError> {
        match self {
            Self::Start(attributes) | Self::Empty(attributes) => Ok(attributes
                .get(&QName(name.as_bytes()))
                .ok_or(SfXmlError::new(&format!("Missing attribute: {:?}", name)))?
                .clone()
                .into_owned()),
        }
    }

    pub fn parse_req_attr<T, E: Error + 'static>(
        &self,
        name: &str,
        parse_fn: &(dyn Fn(&str) -> Result<T, E>),
        context: &str,
    ) -> Result<T, SfXmlError> {
        let value = self.get_req_attr(name)?;
        Self::parse_attr_and_map_err(&value, name, parse_fn, context)
    }

    pub fn get_opt_attr(&self, name: &str) -> Option<String> {
        match self {
            Self::Start(attributes) | Self::Empty(attributes) => attributes
                .get(&QName(name.as_bytes()))
                .map(|value| value.clone().into_owned()),
        }
    }
    pub fn parse_opt_attr<T, E: Error + 'static>(
        &self,
        name: &str,
        parse_fn: &(dyn Fn(&str) -> Result<T, E>),
        context: &str,
    ) -> Result<Option<T>, SfXmlError> {
        self.get_opt_attr(name)
            .map(|value| Self::parse_attr_and_map_err(&value, name, parse_fn, context))
            .transpose()
    }

    fn parse_attr_and_map_err<T, E: Error + 'static>(
        value: &str,
        name: &str,
        parse_fn: &(dyn Fn(&str) -> Result<T, E>),
        context: &str,
    ) -> Result<T, SfXmlError> {
        parse_fn(value).map_err(|e| {
            SfXmlError::from_source(
                e,
                format!(
                    "Error parsing {}. Unexpected {} attribute: {}",
                    context, name, &value
                ),
            )
        })
    }
}

pub(super) fn skip_xml_decl<'buf, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'buf mut Vec<u8>,
) -> Result<BufEvent<'buf>, SfXmlError> {
    let buf_event = skip_whitespace(reader, buf)?;
    match &buf_event.event {
        Event::Decl(_) => skip_whitespace(reader, buf_event.buf),
        _ => Ok(buf_event),
    }
}

pub(super) fn read_start_or_empty<'buf, R>(
    tag: &[u8],
    reader: &Reader<R>,
    buf_event: &'buf BufEvent<'buf>,
) -> Result<XmlTagStart<'buf>, SfXmlError> {
    match &buf_event.event {
        Event::Start(bytes) => {
            check_matches_tag_name(tag, bytes)?;
            Ok(XmlTagStart::Start(read_attributes(bytes, reader)))
        }
        Event::Empty(bytes) => {
            check_matches_tag_name(tag, bytes)?;
            Ok(XmlTagStart::Empty(read_attributes(bytes, reader)))
        }
        _ => Err(SfXmlError::new(&format!(
            "Unexpected event instead of start of {}: {:?}",
            str::from_utf8(tag).unwrap_or_default(),
            buf_event.event
        )))?,
    }
}

pub(super) fn read_start<'buf, R>(
    tag: &[u8],
    reader: &Reader<R>,
    buf_event: &'buf BufEvent<'buf>,
) -> Result<XmlTagStart<'buf>, SfXmlError> {
    match read_start_or_empty(tag, reader, buf_event) {
        Ok(XmlTagStart::Start(attr)) => Ok(XmlTagStart::Start(attr)),
        Ok(XmlTagStart::Empty(_)) => Err(SfXmlError::new(&format!(
            "Empty XML tag instead of start tag found for: {}",
            str::from_utf8(tag).unwrap_or_default()
        ))),
        Err(e) => Err(e),
    }
}

pub(super) fn read_empty<'buf, R>(
    tag: &[u8],
    reader: &Reader<R>,
    buf_event: &'buf BufEvent<'buf>,
) -> Result<XmlTagStart<'buf>, SfXmlError> {
    match read_start_or_empty(tag, reader, buf_event) {
        Ok(XmlTagStart::Start(_)) => Err(SfXmlError::new(&format!(
            "Start XML tag instead of empty tag found for: {}",
            str::from_utf8(tag).unwrap_or_default()
        )))?,
        Ok(XmlTagStart::Empty(attr)) => Ok(XmlTagStart::Empty(attr)),
        Err(e) => Err(e),
    }
}

pub(super) fn read_next_event<'buf, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'buf mut Vec<u8>,
) -> Result<BufEvent<'buf>, SfXmlError> {
    let raw_next = reader.read_event_into(buf)?;
    Ok(BufEvent::new(raw_next.into_owned(), buf))
}

pub(super) fn skip_whitespace<'buf, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'buf mut Vec<u8>,
) -> Result<BufEvent<'buf>, SfXmlError> {
    let event = loop {
        match reader.read_event_into(buf)? {
            Event::Text(bytes) => {
                if !bytes.unescape()?.trim().is_empty() {
                    break Event::Text(bytes);
                }
            }
            Event::Comment(_) => (),
            other_event => break other_event,
        }
    };
    Ok(BufEvent::new(event.into_owned(), buf))
}

pub(super) fn next_non_whitespace<'buf, R: BufRead>(
    next: BufEvent<'buf>,
    reader: &mut Reader<R>,
) -> Result<BufEvent<'buf>, SfXmlError> {
    let is_ws = match &next.event {
        Event::Text(bytes) => bytes.unescape()?.trim().is_empty(),
        Event::Comment(_) => true,
        _ => false,
    };

    match is_ws {
        true => skip_whitespace(reader, next.buf),
        false => Ok(next),
    }
}

pub(super) fn next_non_whitespace_rc(
    next: BufEvent<'_>,
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
) -> Result<BufEvent<'_>, SfXmlError> {
    let mut reader = reader_ref.borrow_mut();
    next_non_whitespace(next, &mut reader)
}

pub(super) fn read_value<'buf, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'buf mut Vec<u8>,
) -> Result<(String, BufEvent<'buf>), SfXmlError> {
    let mut value = String::new();
    let next = loop {
        let event = match reader.read_event_into(buf)? {
            Event::Text(bytes) => {
                value += &bytes.unescape()?;
                None
            }
            Event::Comment(_) => None,
            any_other => Some(any_other),
        };

        if let Some(e) = event {
            break e;
        }
    };

    Ok((value, BufEvent::new(next.into_owned(), buf)))
}

pub(super) fn read_value_pos<'buf, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'buf mut Vec<u8>,
) -> Result<(u64, u64, BufEvent<'buf>), SfXmlError> {
    let start_pos = reader.buffer_position() as u64;
    let mut end_pos = start_pos;
    let next = loop {
        let event = match reader.read_event_into(buf)? {
            Event::Text(_) => {
                end_pos = reader.buffer_position() as u64;
                None
            }
            Event::Comment(_) => None,
            any_other => Some(any_other),
        };

        if let Some(e) = event {
            break e;
        }
    };

    Ok((start_pos, end_pos, BufEvent::new(next.into_owned(), buf)))
}

pub(super) fn consume_end<'buf, R: BufRead>(
    tag_name: &[u8],
    reader: &mut Reader<R>,
    next: BufEvent<'buf>,
) -> Result<BufEvent<'buf>, SfXmlError> {
    let next = next_non_whitespace(next, reader)?;
    match &next.event {
        Event::End(bytes) => {
            if bytes.name().as_ref() == tag_name {
                let next_raw = reader.read_event_into(next.buf)?.into_owned();
                Ok(BufEvent::new(next_raw, next.buf))
            } else {
                Err(SfXmlError::new(&format!(
                    "Unexpected end tag for \"{}\": {}",
                    str::from_utf8(tag_name).unwrap_or_default(),
                    std::str::from_utf8(bytes.name().as_ref()).unwrap_or_default(),
                )))?
            }
        }
        e => Err(SfXmlError::new(&format!(
            "Unexpected event instead of end tag for \"{}\": {:?}",
            str::from_utf8(tag_name).unwrap_or_default(),
            e
        )))?,
    }
}

pub(super) fn consume_end_rc<'buf>(
    tag_name: &[u8],
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    next: BufEvent<'buf>,
) -> Result<BufEvent<'buf>, SfXmlError> {
    let mut reader = reader_ref.borrow_mut();
    consume_end(tag_name, &mut reader, next)
}

#[allow(dead_code)]
pub(super) fn read_req_elem<'buf, R: BufRead, T, E: Error + From<SfXmlError>>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader: &mut Reader<R>,
    constructor: ElemConstructor<'_, 'buf, R, T, E>,
) -> Result<(T, BufEvent<'buf>), E> {
    let next = next_non_whitespace(next, reader)?;
    read_req_elem_core(tag_name, next, &mut |e| constructor(e, reader))
}

pub(super) fn read_req_elem_rc<'buf, T, E: Error + From<SfXmlError>>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    constructor: ElemConstructorRc<'_, 'buf, T, E>,
) -> Result<(T, BufEvent<'buf>), E> {
    let next = next_non_whitespace_rc(next, Rc::clone(&reader_ref))?;
    read_req_elem_core(tag_name, next, &mut |e| {
        constructor(e, Rc::clone(&reader_ref))
    })
}

pub(super) fn read_opt_elem<'buf, R: BufRead, T, E: Error + From<SfXmlError>>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    mut reader: &mut Reader<R>,
    constructor: ElemConstructor<'_, 'buf, R, T, E>,
) -> Result<(Option<T>, BufEvent<'buf>), E> {
    read_opt_elem_core(
        tag_name,
        next,
        &mut reader,
        &mut |e, r| next_non_whitespace(e, r).map_err(|e| e.into()),
        &mut |e, r| constructor(e, r),
    )
}

pub(super) fn read_opt_elem_rc<'buf, T, E: Error + From<SfXmlError>>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    mut reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    constructor: ElemConstructorRc<'_, 'buf, T, E>,
) -> Result<(Option<T>, BufEvent<'buf>), E> {
    read_opt_elem_core(
        tag_name,
        next,
        &mut reader_ref,
        &mut |e, r| next_non_whitespace_rc(e, Rc::clone(r)).map_err(|e| e.into()),
        &mut |e, r| constructor(e, Rc::clone(r)),
    )
}

pub(super) fn read_sequence<'buf, R: BufRead, T, E: Error + From<SfXmlError>>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    mut reader: &mut Reader<R>,
    constructor: ElemConstructor<'_, 'buf, R, T, E>,
) -> Result<(Vec<T>, BufEvent<'buf>), E> {
    read_sequence_core(
        tag_name,
        next,
        &mut reader,
        &mut |e, r| next_non_whitespace(e, r).map_err(|e| e.into()),
        &mut |e, r| constructor(e, r),
    )
}

pub(super) fn read_sequence_rc<'buf, T, E: Error + From<SfXmlError>>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    mut reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    constructor: ElemConstructorRc<'_, 'buf, T, E>,
) -> Result<(Vec<T>, BufEvent<'buf>), E> {
    read_sequence_core(
        tag_name,
        next,
        &mut reader_ref,
        &mut |e, r| next_non_whitespace_rc(e, Rc::clone(r)).map_err(|e| e.into()),
        &mut |e, r| constructor(e, Rc::clone(r)),
    )
}

pub(super) fn read_req_elem_value<'buf, R: BufRead>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader: &mut Reader<R>,
) -> Result<(String, BufEvent<'buf>), SfXmlError> {
    let next = next_non_whitespace(next, reader)?;
    read_start(tag_name, reader, &next)?;
    let (value, next) = read_value(reader, next.buf)?;
    let next = consume_end(tag_name, reader, next)?;

    Ok((value, next))
}

pub(super) fn read_req_elem_value_f64<'buf, R: BufRead>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader: &mut Reader<R>,
) -> Result<(f64, BufEvent<'buf>), SfXmlError> {
    let (value, next) = read_req_elem_value(tag_name, next, reader)?;
    let value_f64 = value.parse::<f64>().map_err(|e| {
        let tag = String::from_utf8_lossy(tag_name);
        SfXmlError::from_source(e, format!("Illegal value for {}: {}", tag, value))
    })?;

    Ok((value_f64, next))
}

// -------------------------------------------------------------
// private
// -------------------------------------------------------------

fn check_matches_tag_name(tag: &[u8], bytes_start: &BytesStart<'_>) -> Result<(), SfXmlError> {
    if bytes_start.name().as_ref() != tag {
        Err(SfXmlError::new(&format!(
            "Unexpected tag instead of \"{}\": {:?}",
            str::from_utf8(tag).unwrap_or_default(),
            str::from_utf8(bytes_start.name().as_ref())
        )))
    } else {
        Ok(())
    }
}

fn read_attributes<'buf, R>(
    bytes_start: &'buf BytesStart<'buf>,
    reader: &Reader<R>,
) -> HashMap<QName<'buf>, std::borrow::Cow<'buf, str>> {
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

fn read_req_elem_core<'buf, T, E: Error + From<SfXmlError>>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    wrapped_constructor: &mut dyn FnMut(BufEvent<'buf>) -> Result<(T, BufEvent<'buf>), E>,
) -> Result<(T, BufEvent<'buf>), E> {
    match &next.event {
        Event::Start(bytes) => {
            if bytes.name().as_ref() == tag_name {
                wrapped_constructor(next)
            } else {
                Err(SfXmlError::new(&format!(
                    "Unexpected start tag: {:?}",
                    std::str::from_utf8(bytes.name().as_ref())
                )))?
            }
        }
        e => Err(SfXmlError::new(&format!(
            "Unexpected event instead of start tag: {:?}",
            &e
        )))?,
    }
}

fn read_opt_elem_core<'buf, T, Reader, E: Error>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader: &mut Reader,
    next_non_ws: &mut dyn Fn(BufEvent<'buf>, &mut Reader) -> Result<BufEvent<'buf>, E>,
    wrapped_constructor: ElemConstructorCore<'_, 'buf, Reader, T, E>,
) -> Result<(Option<T>, BufEvent<'buf>), E> {
    let next = next_non_ws(next, reader)?;
    match &next.event {
        Event::Start(bytes) => {
            if bytes.name().as_ref() == tag_name {
                let (elem, next) = wrapped_constructor(next, reader)?;
                Ok((Some(elem), next))
            } else {
                Ok((None, next))
            }
        }
        _ => Ok((None, next)),
    }
}

fn read_sequence_core<'buf, T, Reader, E: Error>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader: &mut Reader,
    next_non_ws: &mut dyn Fn(BufEvent<'buf>, &mut Reader) -> Result<BufEvent<'buf>, E>,
    wrapped_constructor: ElemConstructorCore<'_, 'buf, Reader, T, E>,
) -> Result<(Vec<T>, BufEvent<'buf>), E> {
    let mut next = next_non_ws(next, reader)?;
    let mut ret = vec![];
    loop {
        match &next.event {
            Event::Start(bytes) | Event::Empty(bytes) => {
                if bytes.name().as_ref() == tag_name {
                    let res = wrapped_constructor(next, reader)?;
                    ret.push(res.0);
                    next = next_non_ws(res.1, reader)?;
                } else {
                    return Ok((ret, next));
                }
            }
            _ => return Ok((ret, next)),
        }
    }
}
