use super::{gaml_parser::Values, GamlError, SeekBufRead};
use crate::api::Parameter;
use quick_xml::{
    events::{BytesStart, Event},
    name::QName,
    Reader,
};
use std::{cell::RefCell, collections::HashMap, error::Error, io::BufRead, rc::Rc, str, vec};

pub(super) struct BufEvent<'buf> {
    pub event: Event<'buf>,
    pub buf: &'buf mut Vec<u8>,
}

impl<'buf> BufEvent<'buf> {
    pub fn new(event: Event<'buf>, buf: &'buf mut Vec<u8>) -> BufEvent<'buf> {
        Self { event, buf }
    }
}

pub(super) enum XmlTagStart<'buf> {
    Start(HashMap<QName<'buf>, std::borrow::Cow<'buf, str>>),
    Empty(HashMap<QName<'buf>, std::borrow::Cow<'buf, str>>),
}

impl<'buf> XmlTagStart<'buf> {
    pub fn get_req_attr(&self, name: &str) -> Result<String, GamlError> {
        match self {
            Self::Start(attributes) | Self::Empty(attributes) => Ok(attributes
                .get(&QName(name.as_bytes()))
                .ok_or(GamlError::new(&format!("Missing attribute: {:?}", name)))?
                .clone()
                .into_owned()),
        }
    }

    pub fn parse_req_attr<T, E: Error + 'static>(
        &self,
        name: &str,
        parse_fn: &(dyn Fn(&str) -> Result<T, E>),
        context: &str,
    ) -> Result<T, GamlError> {
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
    ) -> Result<Option<T>, GamlError> {
        self.get_opt_attr(name)
            .map(|value| Self::parse_attr_and_map_err(&value, name, parse_fn, context))
            .transpose()
    }

    fn parse_attr_and_map_err<T, E: Error + 'static>(
        value: &str,
        name: &str,
        parse_fn: &(dyn Fn(&str) -> Result<T, E>),
        context: &str,
    ) -> Result<T, GamlError> {
        parse_fn(value).map_err(|e| {
            GamlError::from_source(
                e,
                format!(
                    "Error parsing {}. Unexpected {} attribute: {}",
                    context, name, &value
                ),
            )
        })
    }
}

pub(super) fn read_start<'buf, R>(
    tag: &[u8],
    reader: &Reader<R>,
    buf_event: &'buf BufEvent<'buf>,
) -> Result<XmlTagStart<'buf>, GamlError> {
    match read_start_or_empty(tag, reader, buf_event) {
        Ok(XmlTagStart::Start(attr)) => Ok(XmlTagStart::Start(attr)),
        Ok(XmlTagStart::Empty(_)) => Err(GamlError::new(&format!(
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
) -> Result<XmlTagStart<'buf>, GamlError> {
    match read_start_or_empty(tag, reader, buf_event) {
        Ok(XmlTagStart::Start(_)) => Err(GamlError::new(&format!(
            "Start XML tag instead of empty tag found for: {}",
            str::from_utf8(tag).unwrap_or_default()
        ))),
        Ok(XmlTagStart::Empty(attr)) => Ok(XmlTagStart::Empty(attr)),
        Err(e) => Err(e),
    }
}

pub(super) fn read_start_or_empty<'buf, R>(
    tag: &[u8],
    reader: &Reader<R>,
    buf_event: &'buf BufEvent<'buf>,
) -> Result<XmlTagStart<'buf>, GamlError> {
    match &buf_event.event {
        Event::Start(bytes_start) => {
            check_matches_tag_name(tag, bytes_start)?;
            Ok(XmlTagStart::Start(read_attributes(bytes_start, reader)))
        }
        Event::Empty(bytes_start) => {
            check_matches_tag_name(tag, bytes_start)?;
            Ok(XmlTagStart::Empty(read_attributes(bytes_start, reader)))
        }
        _ => Err(GamlError::new(&format!(
            "Unexpected event instead of start of {}: {:?}",
            str::from_utf8(tag).unwrap_or_default(),
            buf_event.event
        ))),
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

pub(super) fn skip_xml_decl<'buf, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'buf mut Vec<u8>,
) -> Result<BufEvent<'buf>, GamlError> {
    let buf_event = skip_whitespace(reader, buf)?;
    match &buf_event.event {
        Event::Decl(_) => skip_whitespace(reader, buf_event.buf),
        _ => Ok(buf_event),
    }
}

pub(super) fn skip_whitespace<'buf, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'buf mut Vec<u8>,
) -> Result<BufEvent<'buf>, GamlError> {
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
) -> Result<BufEvent<'buf>, GamlError> {
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

pub(super) fn read_value<'buf, R: BufRead>(
    reader: &mut Reader<R>,
    buf: &'buf mut Vec<u8>,
) -> Result<(String, BufEvent<'buf>), GamlError> {
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
) -> Result<(u64, u64, BufEvent<'buf>), GamlError> {
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
) -> Result<BufEvent<'buf>, GamlError> {
    let next = next_non_whitespace(next, reader)?;
    match &next.event {
        Event::End(e) => {
            if e.name().as_ref() == tag_name {
                let next_raw = reader.read_event_into(next.buf)?.into_owned();
                Ok(BufEvent::new(next_raw, next.buf))
            } else {
                Err(GamlError::new(&format!(
                    "Unexpected end tag for \"{}\": {}",
                    str::from_utf8(tag_name).unwrap_or_default(),
                    std::str::from_utf8(e.name().as_ref()).unwrap_or_default(),
                )))
            }
        }
        e => Err(GamlError::new(&format!(
            "Unexpected event instead of end tag for \"{}\": {:?}",
            str::from_utf8(tag_name).unwrap_or_default(),
            e
        ))),
    }
}

pub(super) fn consume_end_rc<'buf>(
    tag_name: &[u8],
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    next: BufEvent<'buf>,
) -> Result<BufEvent<'buf>, GamlError> {
    let mut reader = reader_ref.borrow_mut();
    consume_end(tag_name, &mut reader, next)
}

type ElemConstructor<'f, 'buf, R, T> =
    &'f (dyn Fn(BufEvent<'buf>, &mut Reader<R>) -> Result<(T, BufEvent<'buf>), GamlError>);

pub(super) fn read_req_elem<'buf, R: BufRead, T>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader: &mut Reader<R>,
    constructor: ElemConstructor<'_, 'buf, R, T>,
) -> Result<(T, BufEvent<'buf>), GamlError> {
    let next = next_non_whitespace(next, reader)?;
    match &next.event {
        Event::Start(e) => {
            if e.name().as_ref() == tag_name {
                Ok(constructor(next, reader)?)
            } else {
                Err(GamlError::new(&format!(
                    "Unexpected start tag: {:?}",
                    std::str::from_utf8(e.name().as_ref())
                )))
            }
        }
        e => Err(GamlError::new(&format!(
            "Unexpected event instead of start tag: {:?}",
            &e
        ))),
    }
}

pub(super) fn read_opt_elem<'buf, R: BufRead, T>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader: &mut Reader<R>,
    constructor: ElemConstructor<'_, 'buf, R, T>,
) -> Result<(Option<T>, BufEvent<'buf>), GamlError> {
    let next = next_non_whitespace(next, reader)?;
    match &next.event {
        Event::Start(e) => {
            if e.name().as_ref() == tag_name {
                let (elem, next) = read_req_elem(tag_name, next, reader, constructor)?;
                Ok((Some(elem), next))
            } else {
                Ok((None, next))
            }
        }
        _ => Ok((None, next)),
    }
}

pub(super) fn read_sequence<'buf, R: BufRead, T>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader: &mut Reader<R>,
    constructor: ElemConstructor<'_, 'buf, R, T>,
) -> Result<(Vec<T>, BufEvent<'buf>), GamlError> {
    let mut next = next_non_whitespace(next, reader)?;
    let mut ret = vec![];
    loop {
        match &next.event {
            Event::Start(bytes) | Event::Empty(bytes) => {
                if bytes.name().as_ref() == tag_name {
                    let res = constructor(next, reader)?;
                    ret.push(res.0);
                    next = next_non_whitespace(res.1, reader)?;
                } else {
                    return Ok((ret, next));
                }
            }
            _ => return Ok((ret, next)),
        }
    }
}

type ElemConstructorRc<'f, 'buf, T> = &'f (dyn Fn(
    BufEvent<'buf>,
    Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
) -> Result<(T, BufEvent<'buf>), GamlError>);

// todo: avoid code duplication with read_req_elem
pub(super) fn read_req_elem_rc<'buf, T>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    constructor: ElemConstructorRc<'_, 'buf, T>,
) -> Result<(T, BufEvent<'buf>), GamlError> {
    let mut reader = reader_ref.borrow_mut();
    let next = next_non_whitespace(next, &mut reader)?;
    drop(reader);
    match &next.event {
        Event::Start(e) => {
            if e.name().as_ref() == tag_name {
                Ok(constructor(next, Rc::clone(&reader_ref))?)
            } else {
                Err(GamlError::new(&format!(
                    "Unexpected start tag: {:?}",
                    std::str::from_utf8(e.name().as_ref())
                )))
            }
        }
        e => Err(GamlError::new(&format!(
            "Unexpected event instead of start tag: {:?}",
            &e
        ))),
    }
}

// todo: avoid code duplication with read_sequence
pub(super) fn read_sequence_rc<'buf, T>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    constructor: ElemConstructorRc<'_, 'buf, T>,
) -> Result<(Vec<T>, BufEvent<'buf>), GamlError> {
    let mut reader = reader_ref.borrow_mut();
    let mut next = next_non_whitespace(next, &mut reader)?;
    drop(reader);
    let mut ret = vec![];
    loop {
        match &next.event {
            Event::Start(bytes) | Event::Empty(bytes) => {
                if bytes.name().as_ref() == tag_name {
                    let res = constructor(next, Rc::clone(&reader_ref))?;
                    ret.push(res.0);
                    let mut reader = reader_ref.borrow_mut();
                    next = next_non_whitespace(res.1, &mut reader)?;
                } else {
                    return Ok((ret, next));
                }
            }
            _ => return Ok((ret, next)),
        }
    }
}

// todo: avoid code duplication with read_opt_elem
pub(super) fn read_opt_elem_rc<'buf, T>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    constructor: ElemConstructorRc<'_, 'buf, T>,
) -> Result<(Option<T>, BufEvent<'buf>), GamlError> {
    let mut reader = reader_ref.borrow_mut();
    let next = next_non_whitespace(next, &mut reader)?;
    drop(reader);
    match &next.event {
        Event::Start(e) => {
            if e.name().as_ref() == tag_name {
                let (elem, next) = read_req_elem_rc(tag_name, next, reader_ref, constructor)?;
                Ok((Some(elem), next))
            } else {
                Ok((None, next))
            }
        }
        _ => Ok((None, next)),
    }
}

pub(super) fn read_req_elem_value<'buf, R: BufRead>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader: &mut Reader<R>,
) -> Result<(String, BufEvent<'buf>), GamlError> {
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
) -> Result<(f64, BufEvent<'buf>), GamlError> {
    let (value, next) = read_req_elem_value(tag_name, next, reader)?;
    let value_f64 = value.parse::<f64>().map_err(|e| {
        let tag = String::from_utf8_lossy(tag_name);
        GamlError::from_source(e, format!("Illegal value for {}: {}", tag, value))
    })?;

    Ok((value_f64, next))
}

pub(super) fn map_gaml_parameters(
    raw_params: &[super::gaml_parser::Parameter],
) -> Vec<crate::api::Parameter> {
    let mut parameters = Vec::with_capacity(raw_params.len());
    for raw_param in raw_params {
        let key = if [&raw_param.group, &raw_param.label, &raw_param.alias]
            .iter()
            .any(|s| s.is_some())
        {
            let mut attributes = vec![];
            if let Some(group) = &raw_param.group {
                attributes.push(format!("group={group}"));
            }
            if let Some(label) = &raw_param.label {
                attributes.push(format!("label={label}"));
            }
            if let Some(alias) = &raw_param.alias {
                attributes.push(format!("alias={alias}"));
            }
            format!("{} ({})", raw_param.name, attributes.join(", "))
        } else {
            raw_param.name.to_string()
        };
        let param = crate::api::Parameter::from_str_str(
            key,
            raw_param.value.as_deref().unwrap_or_default(),
        );
        parameters.push(param);
    }

    parameters
}

pub(crate) fn map_values_attributes(prefix: &str, values: &Values) -> Vec<Parameter> {
    let mut parameters = vec![];
    // Values attributes
    let format = Parameter::from_str_str(format!("{prefix} format"), values.format.to_string());
    parameters.push(format);
    let byteorder =
        Parameter::from_str_str(format!("{prefix} byteorder"), values.byteorder.to_string());
    parameters.push(byteorder);
    if let Some(numvalues) = values.numvalues {
        let numvalues = Parameter::from_str_u64(format!("{prefix} numvalues"), numvalues);
        parameters.push(numvalues);
    }

    parameters
}

pub(super) trait TypeName {
    fn display_type_name() -> &'static str;
}

pub(super) fn read_elem<T: TypeName>(slice: &[T], index: usize) -> Result<&T, GamlError> {
    slice.get(index).ok_or(GamlError::new(&format!(
        "Illegal {} index: {}",
        T::display_type_name(),
        index
    )))
}

pub(super) fn generate_child_node_names<T>(
    slice: &[T],
    name_generator: &dyn Fn(&T, usize) -> String,
) -> Vec<String> {
    slice
        .iter()
        .enumerate()
        .map(|(i, item)| name_generator(item, i))
        .collect()
}
