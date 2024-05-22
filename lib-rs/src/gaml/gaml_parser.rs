use super::gaml_utils::{
    consume_end, consume_end_rc, next_non_whitespace, read_empty, read_opt_elem, read_opt_elem_rc,
    read_req_elem_rc, read_req_elem_value_f64, read_sequence, read_sequence_rc, read_start,
    read_start_or_empty, read_value, read_value_pos, skip_whitespace, skip_xml_decl, BufEvent,
    TypeName, XmlTagStart,
};
use super::{GamlError, SeekBufRead};
use crate::api::Parser;
use base64::prelude::*;
use chrono::{DateTime, FixedOffset};
use quick_xml::reader::Reader;
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom};
use std::rc::Rc;
use std::str::{self, FromStr};
use strum::{Display, EnumString};

pub struct GamlParser {}

impl<T: Seek + Read + 'static> Parser<T> for GamlParser {
    type R = Gaml;
    type E = GamlError;

    fn parse(name: &str, input: T) -> Result<Self::R, Self::E> {
        let buf_reader: Box<dyn SeekBufRead> = Box::new(BufReader::new(input));
        let reader = Reader::from_reader(buf_reader);
        let reader_ref = Rc::new(RefCell::new(reader));
        Self::R::new(name, reader_ref)
    }
}

#[derive(Debug)]
pub struct Gaml {
    // Attributes
    pub version: String,
    pub name: Option<String>,
    // Elements
    pub integrity: Option<Integrity>,
    pub parameters: Vec<Parameter>,
    pub experiments: Vec<Experiment>,
}

impl Gaml {
    const TAG: &'static [u8] = b"GAML";

    fn new(
        _name: &str,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<Self, GamlError> {
        let mut reader = reader_ref.borrow_mut();
        let mut buf = Vec::new();

        // skip <?xml> element if present
        let next = skip_xml_decl(&mut reader, &mut buf)?;

        // attributes
        let start = read_start(Self::TAG, &reader, &next)?;
        let version = start.get_req_attr("version")?;
        let name = start.get_opt_attr("name");

        // nested elements
        let next = skip_whitespace(&mut reader, &mut buf)?;
        let (integrity, next) = read_opt_elem(b"integrity", next, &mut reader, &Integrity::new)?;
        let (parameters, next) = read_sequence(b"parameter", next, &mut reader, &Parameter::new)?;
        drop(reader);
        let (experiments, next) = read_sequence_rc(
            b"experiment",
            next,
            Rc::clone(&reader_ref),
            &Experiment::new,
        )?;
        let mut reader = reader_ref.borrow_mut();

        let _next = consume_end(Self::TAG, &mut reader, next)?;

        Ok(Self {
            version,
            name,
            integrity,
            parameters,
            experiments,
        })
    }
}

#[derive(Debug)]
pub struct Integrity {
    // Attributes
    pub algorithm: String,
    // Content
    pub value: String,
}

impl Integrity {
    const TAG: &'static [u8] = b"integrity";

    fn new<'buf, R: BufRead>(
        next: BufEvent<'buf>,
        reader: &mut Reader<R>,
    ) -> Result<(Self, BufEvent<'buf>), GamlError> {
        let start = read_start(Self::TAG, reader, &next)?;

        // attributes
        let algorithm = start.get_req_attr("algorithm")?;

        // value
        let (value, next) = read_value(reader, next.buf)?;

        let next = consume_end(Self::TAG, reader, next)?;

        Ok((Self { algorithm, value }, next))
    }
}

#[derive(Debug)]
pub struct Parameter {
    // Attributes
    pub group: Option<String>,
    pub name: String,
    pub label: Option<String>,
    pub alias: Option<String>,
    // Content
    pub value: Option<String>,
}

impl Parameter {
    const TAG: &'static [u8] = b"parameter";

    fn new<'buf, R: BufRead>(
        next: BufEvent<'buf>,
        reader: &mut Reader<R>,
    ) -> Result<(Self, BufEvent<'buf>), GamlError> {
        // attributes
        let start = read_start_or_empty(Self::TAG, reader, &next)?;
        let group = start.get_opt_attr("group");
        let name = start.get_req_attr("name")?;
        let label = start.get_opt_attr("label");
        let alias = start.get_opt_attr("alias");

        // value
        let (value, next) = match start {
            XmlTagStart::Empty(_) => {
                let next_raw = reader.read_event_into(next.buf)?.into_owned();
                (None, BufEvent::new(next_raw, next.buf))
            }
            XmlTagStart::Start(_) => {
                let (value, next) = read_value(reader, next.buf)?;
                let next = consume_end(Self::TAG, reader, next)?;
                (Some(value), next)
            }
        };

        Ok((
            Parameter {
                group,
                name,
                label,
                alias,
                value,
            },
            next,
        ))
    }
}

#[derive(Debug)]
pub struct Experiment {
    // Attributes
    pub name: Option<String>,
    // Elements
    pub collectdate: Option<DateTime<FixedOffset>>,
    pub parameters: Vec<Parameter>,
    pub traces: Vec<Trace>,
}

impl Experiment {
    const TAG: &'static [u8] = b"experiment";

    fn new(
        next: BufEvent<'_>,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<(Self, BufEvent<'_>), GamlError> {
        let mut reader = reader_ref.borrow_mut();

        // attributes
        let start = read_start(Self::TAG, &reader, &next)?;
        let name = start.get_opt_attr("name");

        // nested elements
        let next = skip_whitespace(&mut reader, next.buf)?;
        let (datetime, next) = read_opt_elem(b"collectdate", next, &mut reader, &Collectdate::new)?;
        let collectdate = match datetime {
            None => None,
            Some(dt) => Some(DateTime::parse_from_rfc3339(&dt.value)?),
        };
        let (parameters, next) = read_sequence(b"parameter", next, &mut reader, &Parameter::new)?;
        drop(reader);
        let (traces, next) = read_sequence_rc(b"trace", next, Rc::clone(&reader_ref), &Trace::new)?;

        let next = consume_end_rc(Self::TAG, Rc::clone(&reader_ref), next)?;

        Ok((
            Self {
                name,
                collectdate,
                parameters,
                traces,
            },
            next,
        ))
    }
}

#[derive(Debug)]
struct Collectdate {
    pub value: String,
}

impl Collectdate {
    const TAG: &'static [u8] = b"collectdate";

    pub fn new<'buf, R: BufRead>(
        next: BufEvent<'buf>,
        reader: &mut Reader<R>,
    ) -> Result<(Self, BufEvent<'buf>), GamlError> {
        read_start(Self::TAG, reader, &next)?;

        // Content
        let (value, next) = read_value(reader, next.buf)?;

        let next = consume_end(Self::TAG, reader, next)?;

        Ok((
            Self {
                value: value.trim().into(),
            },
            next,
        ))
    }
}

#[derive(EnumString, PartialEq, Debug, Display)]
pub enum Technique {
    #[strum(serialize = "ATOMIC")]
    Atomic,
    #[strum(serialize = "CHROM")]
    Chrom,
    #[strum(serialize = "FLUOR")]
    Fluor,
    #[strum(serialize = "IR")]
    Ir,
    #[strum(serialize = "MS")]
    Ms,
    #[strum(serialize = "NIR")]
    Nir,
    #[strum(serialize = "NMR")]
    Nmr,
    #[strum(serialize = "PDA")]
    Pda,
    #[strum(serialize = "PARTICLE")]
    Particle,
    #[strum(serialize = "POLAR")]
    Polar,
    #[strum(serialize = "RAMAN")]
    Raman,
    #[strum(serialize = "THERMAL")]
    Thermal,
    #[strum(serialize = "UNKNOWN")]
    Unknown,
    #[strum(serialize = "UVVIS")]
    Uvvis,
    #[strum(serialize = "XRAY")]
    Xray,
}

#[derive(Debug)]
pub struct Trace {
    // Attributes
    pub name: Option<String>,
    pub technique: Technique,
    // Elements
    pub parameters: Vec<Parameter>,
    pub coordinates: Vec<Coordinates>,
    pub x_data: Vec<Xdata>,
}

impl Trace {
    const TAG: &'static [u8] = b"trace";

    fn new(
        next: BufEvent<'_>,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<(Self, BufEvent<'_>), GamlError> {
        let mut reader = reader_ref.borrow_mut();

        // attributes
        let start = read_start(Self::TAG, &reader, &next)?;
        let name = start.get_opt_attr("name");
        let technique =
            start.parse_req_attr("technique", &Technique::from_str, Trace::display_name())?;

        // nested elements
        let next = skip_whitespace(&mut reader, next.buf)?;
        let (parameters, next) = read_sequence(b"parameter", next, &mut reader, &Parameter::new)?;
        drop(reader);
        let (coordinates, next) = read_sequence_rc(
            b"coordinates",
            next,
            Rc::clone(&reader_ref),
            &Coordinates::new,
        )?;
        let (x_data, next) = read_sequence_rc(b"Xdata", next, Rc::clone(&reader_ref), &Xdata::new)?;

        let next = consume_end_rc(Self::TAG, Rc::clone(&reader_ref), next)?;

        Ok((
            Self {
                name,
                technique,
                parameters,
                coordinates,
                x_data,
            },
            next,
        ))
    }
}

#[derive(EnumString, PartialEq, Debug, Display)]
pub enum Units {
    #[strum(serialize = "ABSORBANCE")]
    Absorbance,
    #[strum(serialize = "AMPERES")]
    Amperes,
    #[strum(serialize = "ANGSTROMS")]
    Angstroms,
    #[strum(serialize = "ATOMICMASSUNITS")]
    Atomicmassunits,
    #[strum(serialize = "CALORIES")]
    Calories,
    #[strum(serialize = "CELSIUS")]
    Celsius,
    #[strum(serialize = "CENTIMETERS")]
    Centimeters,
    #[strum(serialize = "DAYS")]
    Days,
    #[strum(serialize = "DECIBELS")]
    Decibels,
    #[strum(serialize = "DEGREES")]
    Degrees,
    #[strum(serialize = "ELECTRONVOLTS")]
    Electronvolts,
    #[strum(serialize = "EMISSION")]
    Emission,
    #[strum(serialize = "FAHRENHEIT")]
    Fahrenheit,
    #[strum(serialize = "GHERTZ")]
    Ghertz,
    #[strum(serialize = "GRAMS")]
    Grams,
    #[strum(serialize = "HERTZ")]
    Hertz,
    #[strum(serialize = "HOURS")]
    Hours,
    #[strum(serialize = "JOULES")]
    Joules,
    #[strum(serialize = "KELVIN")]
    Kelvin,
    #[strum(serialize = "KILOCALORIES")]
    Kilocalories,
    #[strum(serialize = "KILOGRAMS")]
    Kilograms,
    #[strum(serialize = "KILOHERTZ")]
    Kilohertz,
    #[strum(serialize = "KILOMETERS")]
    Kilometers,
    #[strum(serialize = "KILOWATTS")]
    Kilowatts,
    #[strum(serialize = "KUBELKAMUNK")]
    Kubelkamunk,
    #[strum(serialize = "LITERS")]
    Liters,
    #[strum(serialize = "LOGREFLECTANCE")]
    Logreflectance,
    #[strum(serialize = "MASSCHARGERATIO")]
    Masschargeratio,
    #[strum(serialize = "MEGAHERTZ")]
    Megahertz,
    #[strum(serialize = "MEGAWATTS")]
    Megawatts,
    #[strum(serialize = "METERS")]
    Meters,
    #[strum(serialize = "MICROGRAMS")]
    Micrograms,
    #[strum(serialize = "MICRONS")]
    Microns,
    #[strum(serialize = "MICROSECONDS")]
    Microseconds,
    #[strum(serialize = "MILLIABSORBANCE")]
    Milliabsorbance,
    #[strum(serialize = "MILLIAMPS")]
    Milliamps,
    #[strum(serialize = "MILLIGRAMS")]
    Milligrams,
    #[strum(serialize = "MILLILITERS")]
    Milliliters,
    #[strum(serialize = "MILLIMETERS")]
    Millimeters,
    #[strum(serialize = "MILLIMOLAR")]
    Millimolar,
    #[strum(serialize = "MILLISECONDS")]
    Milliseconds,
    #[strum(serialize = "MILLIVOLTS")]
    Millivolts,
    #[strum(serialize = "MILLIWATTS")]
    Milliwatts,
    #[strum(serialize = "MINUTES")]
    Minutes,
    #[strum(serialize = "MOLAR")]
    Molar,
    #[strum(serialize = "MOLES")]
    Moles,
    #[strum(serialize = "NANOGRAMS")]
    Nanograms,
    #[strum(serialize = "NANOMETERS")]
    Nanometers,
    #[strum(serialize = "NANOSECONDS")]
    Nanoseconds,
    #[strum(serialize = "PPB")]
    Ppb,
    #[strum(serialize = "PPM")]
    Ppm,
    #[strum(serialize = "PPT")]
    Ppt,
    #[strum(serialize = "RADIANS")]
    Radians,
    #[strum(serialize = "RAMANSHIFT")]
    Ramanshift,
    #[strum(serialize = "REFLECTANCE")]
    Reflectance,
    #[strum(serialize = "SECONDS")]
    Seconds,
    #[strum(serialize = "TRANSMISSIONPERCENT")]
    Transmissionpercent,
    #[strum(serialize = "TRANSMITTANCE")]
    Transmittance,
    #[strum(serialize = "UNKNOWN")]
    Unknown,
    #[strum(serialize = "VOLTS")]
    Volts,
    #[strum(serialize = "WATTS")]
    Watts,
    #[strum(serialize = "WAVENUMBER")]
    Wavenumber,
    #[strum(serialize = "YEARS")]
    Years,
    #[strum(serialize = "INCHES")]
    Inches,
    #[strum(serialize = "MICROABSORBANCE")]
    Microabsorbance,
    #[strum(serialize = "MICROVOLTS")]
    Microvolts,
    #[strum(serialize = "PERCENT")]
    Percent,
    #[strum(serialize = "PSI")]
    Psi,
    #[strum(serialize = "TESLA")]
    Tesla,
}

#[derive(EnumString, PartialEq, Debug, Display)]
pub enum Valueorder {
    #[strum(serialize = "EVEN")]
    Even,
    #[strum(serialize = "ORDERED")]
    Ordered,
    #[strum(serialize = "UNSPECIFIED")]
    Unspecified,
}

#[derive(Debug)]
pub struct Coordinates {
    // Attributes
    pub units: Units,
    pub label: Option<String>,
    pub linkid: Option<String>,
    pub valueorder: Option<Valueorder>,
    // Elements
    pub links: Vec<Link>,
    pub parameters: Vec<Parameter>,
    pub values: Values,
}

impl Coordinates {
    const TAG: &'static [u8] = b"coordinates";

    fn new(
        next: BufEvent<'_>,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<(Self, BufEvent<'_>), GamlError> {
        // attributes
        let (
            DataAttributes {
                units,
                label,
                linkid,
                valueorder,
            },
            next,
        ) = read_data_attributes(
            Self::TAG,
            Self::display_name(),
            Rc::clone(&reader_ref),
            next,
        )?;

        // nested elements
        let (
            DataElements {
                links,
                parameters,
                values,
            },
            next,
        ) = read_data_elements(Rc::clone(&reader_ref), next)?;

        let next = consume_end_rc(Self::TAG, Rc::clone(&reader_ref), next)?;

        Ok((
            Self {
                units,
                label,
                linkid,
                valueorder,
                links,
                parameters,
                values,
            },
            next,
        ))
    }
}

#[derive(Debug)]
pub struct Link {
    // Attributes
    pub linkref: String,
}

impl Link {
    const TAG: &'static [u8] = b"link";

    fn new<'buf, R: BufRead>(
        next: BufEvent<'buf>,
        reader: &mut Reader<R>,
    ) -> Result<(Self, BufEvent<'buf>), GamlError> {
        // attributes
        let start = read_empty(Self::TAG, reader, &next)?;
        let linkref = start.get_req_attr("linkref")?;

        let next_raw = reader.read_event_into(next.buf)?.into_owned();

        Ok((Self { linkref }, BufEvent::new(next_raw, next.buf)))
    }
}

#[derive(EnumString, PartialEq, Debug, Display)]
pub enum Format {
    #[strum(serialize = "FLOAT32")]
    Float32,
    #[strum(serialize = "FLOAT64")]
    Float64,
}

#[derive(EnumString, PartialEq, Debug, Display)]
pub enum Byteorder {
    #[strum(serialize = "INTEL")]
    Intel,
}

pub struct Values {
    // Attributes
    pub format: Format,
    pub byteorder: Byteorder,
    pub numvalues: Option<u64>,

    // Value is lazily read
    value_start_pos: u64,
    value_end_pos: u64,
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
}

impl std::fmt::Debug for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Values")
            .field("format", &self.format)
            .field("byteorder", &self.byteorder)
            .field("numvalues", &self.numvalues)
            .field("value_start_pos", &self.value_start_pos)
            .field("value_end_pos", &self.value_end_pos)
            // skip reader_ref as quickxml::Reader does not implement Debug
            .finish()
    }
}

impl Values {
    const TAG: &'static [u8] = b"values";

    fn new(
        next: BufEvent<'_>,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<(Self, BufEvent<'_>), GamlError> {
        let mut reader = reader_ref.borrow_mut();

        // attributes
        let start = read_start(Self::TAG, &reader, &next)?;
        let format = start.parse_req_attr("format", &Format::from_str, Values::display_name())?;
        let byteorder =
            start.parse_req_attr("byteorder", &Byteorder::from_str, Values::display_name())?;
        let numvalues = start.parse_opt_attr(
            "numvalues",
            &|v: &str| v.parse::<u64>(),
            Values::display_name(),
        )?;

        // skip content
        let (value_start_pos, value_end_pos, next) = read_value_pos(&mut reader, next.buf)?;

        let next = consume_end(Self::TAG, &mut reader, next)?;
        drop(reader);

        Ok((
            Self {
                format,
                byteorder,
                numvalues,
                // value,
                value_start_pos,
                value_end_pos,
                reader_ref,
            },
            next,
        ))
    }

    pub fn get_data(&self) -> Result<Vec<f64>, GamlError> {
        let mut reader = self.reader_ref.borrow_mut();

        let start = self.value_start_pos;
        let end = self.value_end_pos;
        let input = reader.get_mut();
        input.seek(SeekFrom::Start(start))?;
        // Read value bytes into owned buffer to allow quickxml deserialization. When using reader directly,
        // quickxml returns an error when it encounters a closing element after the value text.
        // Is it possible to make this more efficient and still remove possibly interspresed comments?
        let mut input_buffer = vec![0u8; (end - start) as usize];
        input.read_exact(&mut input_buffer)?;
        let mut reader = Reader::from_reader(Cursor::new(input_buffer));
        let mut buf = Vec::<u8>::new();
        let (mut value, _next) = read_value(&mut reader, &mut buf)?;
        value.retain(|c| !c.is_whitespace());

        let raw_data = BASE64_STANDARD
            .decode(value.as_bytes())
            .map_err(|e| GamlError::from_source(e, "Error decoding base64 data."))?;

        let multiple = match &self.format {
            Format::Float32 => 4u64,
            Format::Float64 => 8u64,
        };
        if raw_data.len() as u64 % multiple != 0 {
            return Err(GamlError::new(&format!(
                "Illegal number of data bytes: {}",
                raw_data.len()
            )));
        }
        if let Some(n) = self.numvalues {
            if n != raw_data.len() as u64 / multiple {
                return Err(GamlError::new(&format!(
                    "Number of data bytes does not correspond to numvalues and format attributes: {}",
                    raw_data.len()
                )));
            }
        }

        let data = if Format::Float32 == self.format {
            // see https://stackoverflow.com/a/77388975 for a more elegant solution in the future
            // f32
            raw_data
                .chunks_exact(4)
                .map(TryInto::try_into)
                .map(Result::unwrap)
                .map(f32::from_le_bytes)
                .map(|v| v as f64)
                .collect()
        } else {
            // f64
            raw_data
                .chunks_exact(8)
                .map(TryInto::try_into)
                .map(Result::unwrap)
                .map(f64::from_le_bytes)
                .collect()
        };

        Ok(data)
    }

    /// #[cfg(test)] and pub(super) to allow creating Values in unit tests
    #[cfg(test)]
    pub(super) fn create_values_with(bytes: &[u8], format: Format, byteorder: Byteorder) -> Values {
        let base64 = BASE64_STANDARD.encode(bytes);
        let base64_len = base64.len();
        let input = Cursor::new(base64);
        let buf_reader: Box<dyn SeekBufRead> = Box::new(BufReader::new(input));
        let reader = quick_xml::Reader::from_reader(buf_reader);
        let reader_ref = Rc::new(RefCell::new(reader));

        let numvalues = match &format {
            Format::Float32 => bytes.len() / 4,
            Format::Float64 => bytes.len() / 8,
        };

        Values {
            format,
            byteorder,
            numvalues: Some(numvalues as u64),
            value_start_pos: 0,
            value_end_pos: base64_len as u64,
            reader_ref,
        }
    }
}

#[derive(Debug)]
pub struct Xdata {
    // Attributes
    pub units: Units,
    pub label: Option<String>,
    pub linkid: Option<String>,
    pub valueorder: Option<Valueorder>,
    // Elements
    pub links: Vec<Link>,
    pub parameters: Vec<Parameter>,
    pub values: Values,
    pub alt_x_data: Vec<AltXdata>,
    pub y_data: Vec<Ydata>,
}

impl Xdata {
    const TAG: &'static [u8] = b"Xdata";

    fn new(
        next: BufEvent<'_>,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<(Self, BufEvent<'_>), GamlError> {
        // attributes
        let (
            DataAttributes {
                units,
                label,
                linkid,
                valueorder,
            },
            next,
        ) = read_data_attributes(
            Self::TAG,
            Self::display_name(),
            Rc::clone(&reader_ref),
            next,
        )?;

        // nested elements
        let (
            DataElements {
                links,
                parameters,
                values,
            },
            next,
        ) = read_data_elements(Rc::clone(&reader_ref), next)?;
        let (alt_x_data, next) =
            read_sequence_rc(b"altXdata", next, Rc::clone(&reader_ref), &AltXdata::new)?;
        let (y_data, next) = read_sequence_rc(b"Ydata", next, Rc::clone(&reader_ref), &Ydata::new)?;
        if y_data.is_empty() {
            return Err(GamlError::new("No Ydata found for Xdata."));
        }

        let next = consume_end_rc(Self::TAG, Rc::clone(&reader_ref), next)?;

        Ok((
            Self {
                units,
                label,
                linkid,
                valueorder,
                links,
                parameters,
                values,
                alt_x_data,
                y_data,
            },
            next,
        ))
    }
}

#[derive(Debug)]
pub struct AltXdata {
    // Attributes
    pub units: Units,
    pub label: Option<String>,
    pub linkid: Option<String>,
    pub valueorder: Option<Valueorder>,
    // Elements
    pub links: Vec<Link>,
    pub parameters: Vec<Parameter>,
    pub values: Values,
}

impl AltXdata {
    const TAG: &'static [u8] = b"altXdata";

    fn new(
        next: BufEvent<'_>,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<(Self, BufEvent<'_>), GamlError> {
        // attributes
        let (
            DataAttributes {
                units,
                label,
                linkid,
                valueorder,
            },
            next,
        ) = read_data_attributes(
            Self::TAG,
            Self::display_name(),
            Rc::clone(&reader_ref),
            next,
        )?;

        // nested elements
        let (
            DataElements {
                links,
                parameters,
                values,
            },
            next,
        ) = read_data_elements(Rc::clone(&reader_ref), next)?;

        let next = consume_end_rc(Self::TAG, Rc::clone(&reader_ref), next)?;

        Ok((
            Self {
                units,
                label,
                linkid,
                valueorder,
                links,
                parameters,
                values,
            },
            next,
        ))
    }
}

#[derive(Debug)]
pub struct Ydata {
    // Attributes
    pub units: Units,
    pub label: Option<String>,
    // Elements
    pub parameters: Vec<Parameter>,
    pub values: Values,
    pub peaktables: Vec<Peaktable>,
}

impl Ydata {
    const TAG: &'static [u8] = b"Ydata";

    fn new(
        next: BufEvent<'_>,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<(Self, BufEvent<'_>), GamlError> {
        // attributes
        let (DataAttributes { units, label, .. }, next) = read_data_attributes(
            Self::TAG,
            Self::display_name(),
            Rc::clone(&reader_ref),
            next,
        )?;

        // nested elements
        let (
            DataElements {
                links: _,
                parameters,
                values,
            },
            next,
        ) = read_data_elements(Rc::clone(&reader_ref), next)?;
        let (peaktables, next) =
            read_sequence_rc(b"peaktable", next, Rc::clone(&reader_ref), &Peaktable::new)?;

        let next = consume_end_rc(Self::TAG, Rc::clone(&reader_ref), next)?;

        Ok((
            Self {
                units,
                label,
                parameters,
                values,
                peaktables,
            },
            next,
        ))
    }
}

#[derive(Debug)]
pub struct Peaktable {
    // Attributes
    pub name: Option<String>,
    // Elements
    pub parameters: Vec<Parameter>,
    pub peaks: Vec<Peak>,
}

impl Peaktable {
    const TAG: &'static [u8] = b"peaktable";

    fn new(
        next: BufEvent<'_>,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<(Self, BufEvent<'_>), GamlError> {
        let mut reader = reader_ref.borrow_mut();

        // attributes
        let start = read_start(Self::TAG, &reader, &next)?;
        let name = start.get_opt_attr("name");

        // nested elements
        let next = skip_whitespace(&mut reader, next.buf)?;
        let (parameters, next) = read_sequence(b"parameter", next, &mut reader, &Parameter::new)?;
        drop(reader);
        let (peaks, next) = read_sequence_rc(b"peak", next, Rc::clone(&reader_ref), &Peak::new)?;

        let next = consume_end_rc(Self::TAG, Rc::clone(&reader_ref), next)?;

        Ok((
            Self {
                name,
                parameters,
                peaks,
            },
            next,
        ))
    }
}

#[derive(Debug)]
pub struct Peak {
    // Attributes
    pub number: u64,
    pub group: Option<String>,
    pub name: Option<String>,
    // Elements
    pub parameters: Vec<Parameter>,
    pub peak_x_value: f64,
    pub peak_y_value: f64,
    pub baseline: Option<Baseline>,
}

impl Peak {
    const TAG: &'static [u8] = b"peak";

    fn new(
        next: BufEvent<'_>,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<(Self, BufEvent<'_>), GamlError> {
        let mut reader = reader_ref.borrow_mut();

        // attributes
        let start = read_start(Self::TAG, &reader, &next)?;
        let number_str = start.get_req_attr("number")?;
        let number = number_str.parse::<u64>().map_err(|e| {
            GamlError::from_source(e, format!("Illegal peak number attribute: {}", number_str))
        })?;
        if number == 0 {
            // only strictly positive peak numbers allowed by schema
            return Err(GamlError::new("Illegal peak number: 0"));
        }
        let group = start.get_opt_attr("group");
        let name = start.get_opt_attr("name");

        // nested elements
        let next = skip_whitespace(&mut reader, next.buf)?;
        let (parameters, next) = read_sequence(b"parameter", next, &mut reader, &Parameter::new)?;
        let (peak_x_value, next) = read_req_elem_value_f64(b"peakXvalue", next, &mut reader)?;
        let (peak_y_value, next) = read_req_elem_value_f64(b"peakYvalue", next, &mut reader)?;
        drop(reader);
        let (baseline, next) =
            read_opt_elem_rc(b"baseline", next, Rc::clone(&reader_ref), &Baseline::new)?;

        let next = consume_end_rc(Self::TAG, Rc::clone(&reader_ref), next)?;

        Ok((
            Self {
                number,
                group,
                name,
                parameters,
                peak_x_value,
                peak_y_value,
                baseline,
            },
            next,
        ))
    }
}

#[derive(Debug)]
pub struct Baseline {
    // Elements
    pub parameters: Vec<Parameter>,
    pub start_x_value: f64,
    pub start_y_value: f64,
    pub end_x_value: f64,
    pub end_y_value: f64,
    pub basecurve: Option<Basecurve>,
}

impl Baseline {
    const TAG: &'static [u8] = b"baseline";

    fn new(
        next: BufEvent<'_>,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<(Self, BufEvent<'_>), GamlError> {
        let mut reader = reader_ref.borrow_mut();

        // attributes
        let _start = read_start(Self::TAG, &reader, &next)?;

        // nested elements
        let next = skip_whitespace(&mut reader, next.buf)?;
        let (parameters, next) = read_sequence(b"parameter", next, &mut reader, &Parameter::new)?;
        let (start_x_value, next) = read_req_elem_value_f64(b"startXvalue", next, &mut reader)?;
        let (start_y_value, next) = read_req_elem_value_f64(b"startYvalue", next, &mut reader)?;
        let (end_x_value, next) = read_req_elem_value_f64(b"endXvalue", next, &mut reader)?;
        let (end_y_value, next) = read_req_elem_value_f64(b"endYvalue", next, &mut reader)?;
        drop(reader);
        let (basecurve, next) =
            read_opt_elem_rc(b"basecurve", next, Rc::clone(&reader_ref), &Basecurve::new)?;

        let next = consume_end_rc(Self::TAG, Rc::clone(&reader_ref), next)?;

        Ok((
            Self {
                parameters,
                start_x_value,
                start_y_value,
                end_x_value,
                end_y_value,
                basecurve,
            },
            next,
        ))
    }
}

#[derive(Debug)]
pub struct Basecurve {
    // Elements
    pub base_x_data: Vec<Values>,
    pub base_y_data: Vec<Values>,
}

impl Basecurve {
    const TAG: &'static [u8] = b"basecurve";

    fn new(
        next: BufEvent<'_>,
        reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    ) -> Result<(Self, BufEvent<'_>), GamlError> {
        let mut reader = reader_ref.borrow_mut();

        // attributes
        let _start = read_start(Self::TAG, &reader, &next)?;

        // nested elements
        let next = skip_whitespace(&mut reader, next.buf)?;
        drop(reader);
        let (base_x_data, next) = read_base_values(b"baseXdata", next, Rc::clone(&reader_ref))?;
        let (base_y_data, next) = read_base_values(b"baseYdata", next, Rc::clone(&reader_ref))?;

        let next = consume_end_rc(Self::TAG, Rc::clone(&reader_ref), next)?;

        Ok((
            Self {
                base_x_data,
                base_y_data,
            },
            next,
        ))
    }

    fn get_data(&self, values: &[Values]) -> Result<Vec<f64>, GamlError> {
        let mut ret = Vec::<f64>::new();
        for values_elem in values {
            let mut arr = values_elem.get_data()?;
            ret.append(&mut arr);
        }

        Ok(ret)
    }

    pub fn get_x_data(&self) -> Result<Vec<f64>, GamlError> {
        self.get_data(&self.base_x_data)
    }

    pub fn get_y_data(&self) -> Result<Vec<f64>, GamlError> {
        self.get_data(&self.base_y_data)
    }
}

// -------------------------------------------------------------
// private
// -------------------------------------------------------------

fn read_base_values<'buf>(
    tag_name: &[u8],
    next: BufEvent<'buf>,
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
) -> Result<(Vec<Values>, BufEvent<'buf>), GamlError> {
    let mut reader = reader_ref.borrow_mut();
    let next = next_non_whitespace(next, &mut reader)?;

    let _start = read_start(tag_name, &reader, &next)?;
    let next = skip_whitespace(&mut reader, next.buf)?;
    drop(reader);
    let (values, next) = read_sequence_rc(b"values", next, Rc::clone(&reader_ref), &Values::new)?;

    let next = consume_end_rc(tag_name, Rc::clone(&reader_ref), next)?;

    Ok((values, next))
}

#[derive(Debug)]
struct DataAttributes {
    pub units: Units,
    pub label: Option<String>,
    pub linkid: Option<String>,
    pub valueorder: Option<Valueorder>,
}

fn read_data_attributes<'buf>(
    tag_name: &[u8],
    display_name: &str,
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    next: BufEvent<'buf>,
) -> Result<(DataAttributes, BufEvent<'buf>), GamlError> {
    let mut reader = reader_ref.borrow_mut();
    // attributes
    let start = read_start(tag_name, &reader, &next)?;
    let units = start.parse_req_attr("units", &Units::from_str, display_name)?;
    let label = start.get_opt_attr("label");
    let linkid = start.get_opt_attr("linkid");
    let valueorder = start.parse_opt_attr("valueorder", &Valueorder::from_str, display_name)?;

    let next = skip_whitespace(&mut reader, next.buf)?;

    Ok((
        DataAttributes {
            units,
            label,
            linkid,
            valueorder,
        },
        next,
    ))
}

#[derive(Debug)]
struct DataElements {
    pub links: Vec<Link>,
    pub parameters: Vec<Parameter>,
    pub values: Values,
}

fn read_data_elements(
    reader_ref: Rc<RefCell<Reader<Box<dyn SeekBufRead>>>>,
    next: BufEvent<'_>,
) -> Result<(DataElements, BufEvent<'_>), GamlError> {
    let mut reader = reader_ref.borrow_mut();

    let (links, next) = read_sequence(b"link", next, &mut reader, &Link::new)?;
    let (parameters, next) = read_sequence(b"parameter", next, &mut reader, &Parameter::new)?;
    drop(reader);
    let (values, next) = read_req_elem_rc(b"values", next, Rc::clone(&reader_ref), &Values::new)?;

    Ok((
        DataElements {
            links,
            parameters,
            values,
        },
        next,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use std::io::Cursor;

    #[test]
    fn parses_utf8_gaml_xml_1_0() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml test file\">\n
                            <integrity algorithm=\"SHA1\">03cfd743661f07975fa2f1220c5194cbaff48451</integrity>\n
                            <parameter name=\"parameter0\" label=\"Parameter label 0\" group=\"Parameter group 0\">Parameter value 0</parameter>\n
                            <!-- A comment -->
                            <parameter name=\"parameter1\" label=\"Parameter label 1\" group=\"Parameter group 1\">\
                            <!-- A comment -->Parameter <!-- A comment -->value 1<!-- A comment --></parameter>\
                            <parameter name=\"parameter2\" label=\"Parameter label 2\" group=\"Parameter group 2\"/>\n
                            <experiment name=\"Experiment name\">
                                <collectdate>2024-03-27T06:46:00Z</collectdate>
                                <parameter name=\"exp-parameter0\" label=\"Experiment parameter label 0\">Experiment parameter value 0</parameter>
                                <trace name=\"Trace 0\" technique=\"UNKNOWN\">
                                    <parameter name=\"trace-parameter0\" label=\"Trace parameter label 0\">Trace parameter value 0</parameter>
                                    <coordinates label=\"Coordinate label\" units=\"MICRONS\" linkid=\"coordinates-linkid\" valueorder=\"UNSPECIFIED\">
                                        <link linkref=\"co-linkref\"/>
                                        <parameter name=\"co-parameter0\" label=\"Coordinates parameter label 0\">Coordinates parameter value 0</parameter>
                                        <values byteorder=\"INTEL\" format=\"FLOAT32\" numvalues=\"2\">
                                            <!-- A values comment -->
                                            AACAPw\nAAAEA=
                                        </values>
                                    </coordinates>
                                    <Xdata label=\"Xdata label\" units=\"MICRONS\" linkid=\"xdata-linkid\" valueorder=\"UNSPECIFIED\">
                                        <link linkref=\"xdata-linkref\"/>
                                        <parameter name=\"xdata-parameter0\" label=\"Xdata parameter label 0\">Xdata parameter value 0</parameter>
                                        <values byteorder=\"INTEL\" format=\"FLOAT32\" numvalues=\"2\">
                                            <!-- A values comment -->
                                            AACAPw\nAAAEA=
                                        </values>
                                        <altXdata label=\"altXdata label\" units=\"MICRONS\" linkid=\"altxdata-linkid\" valueorder=\"UNSPECIFIED\">
                                            <link linkref=\"altxdata-linkref\"/>
                                            <parameter name=\"altxdata-parameter0\" label=\"altXdata parameter label 0\">altXdata parameter value 0</parameter>
                                            <values byteorder=\"INTEL\" format=\"FLOAT32\" numvalues=\"2\">
                                                AACAPw\nAAAEA=
                                                <!-- A values comment -->
                                            </values>
                                        </altXdata>
                                        <Ydata label=\"Ydata label\" units=\"MICRONS\" linkid=\"altxdata-linkid\" valueorder=\"UNSPECIFIED\">
                                            <parameter name=\"ydata-parameter0\" label=\"Ydata parameter label 0\">Ydata parameter value 0</parameter>
                                            <values byteorder=\"INTEL\" format=\"FLOAT32\" numvalues=\"2\">
                                                <!-- A values comment -->
                                                AACAPw\nAAAEA=
                                            </values>
                                            <peaktable name=\"pt-name\">
                                                <parameter name=\"pt-parameter0\" label=\"Peaktable parameter label 0\">Peaktable parameter value 0</parameter>
                                                <peak number=\"1\" group=\"p0-group\" name=\"p0-name\">
                                                    <parameter name=\"p0-parameter0\" label=\"Peak 0 parameter label 0\">Peak 0 parameter value 0</parameter>
                                                    <peakXvalue>0.1</peakXvalue>
                                                    <peakYvalue>100.0</peakYvalue>
                                                    <baseline>
                                                        <startXvalue>1.1</startXvalue>
                                                        <startYvalue>11.1</startYvalue>
                                                        <endXvalue>2.2</endXvalue>
                                                        <endYvalue>22.2</endYvalue>
                                                        <basecurve>
                                                            <baseXdata>
                                                                <values byteorder=\"INTEL\" format=\"FLOAT32\" numvalues=\"2\">
                                                                    AACAPw\nAAAEA=
                                                                </values>
                                                                <values byteorder=\"INTEL\" format=\"FLOAT32\" numvalues=\"2\">
                                                                    AACAPw\nAAAEA=
                                                                </values>
                                                            </baseXdata>
                                                            <baseYdata>
                                                                <values byteorder=\"INTEL\" format=\"FLOAT32\" numvalues=\"2\">
                                                                    AACAPw\nAAAEA=
                                                                </values>
                                                                <values byteorder=\"INTEL\" format=\"FLOAT32\" numvalues=\"2\">
                                                                    AACAPw\nAAAEA=
                                                                </values>
                                                            </baseYdata>
                                                        </basecurve>
                                                    </baseline>
                                                </peak>
                                            </peaktable>
                                        </Ydata>
                                    </Xdata>
                                </trace>
                            </experiment>
                        </GAML>";
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
        assert_eq!(Some("Parameter value 0".into()), parameters[0].value);
        assert_eq!("parameter1", &parameters[1].name);
        assert_eq!(Some("Parameter label 1".into()), parameters[1].label);
        assert_eq!(Some("Parameter group 1".into()), parameters[1].group);
        assert_eq!(Some("Parameter value 1".into()), parameters[1].value);
        assert_eq!("parameter2", &parameters[2].name);
        assert_eq!(Some("Parameter label 2".into()), parameters[2].label);
        assert_eq!(Some("Parameter group 2".into()), parameters[2].group);
        assert_eq!(None, parameters[2].value);

        let experiments = &gaml.experiments;
        assert_eq!(1, experiments.len());
        let date = NaiveDate::from_ymd_opt(2024, 03, 27).unwrap();
        let time = NaiveTime::from_hms_opt(06, 46, 0).unwrap();
        assert_eq!(
            DateTime::<FixedOffset>::from_naive_utc_and_offset(
                NaiveDateTime::new(date, time),
                FixedOffset::east_opt(0).unwrap()
            ),
            experiments[0].collectdate.unwrap()
        );
        let experiment_parameters = &experiments[0].parameters;
        assert_eq!(1, experiment_parameters.len());
        assert_eq!("exp-parameter0", &experiment_parameters[0].name);
        assert_eq!(
            Some("Experiment parameter label 0".into()),
            experiment_parameters[0].label
        );
        assert_eq!(None, experiment_parameters[0].group);
        assert_eq!(
            Some("Experiment parameter value 0".into()),
            experiment_parameters[0].value
        );

        let traces = &experiments[0].traces;
        assert_eq!(1, traces.len());
        let trace = &traces[0];
        assert_eq!(Some("Trace 0".into()), trace.name);
        assert_eq!(Technique::Unknown, trace.technique);
        let trace_parameters = &trace.parameters;
        assert_eq!(1, trace_parameters.len());
        assert_eq!("trace-parameter0", &trace_parameters[0].name);
        assert_eq!(
            Some("Trace parameter label 0".into()),
            trace_parameters[0].label
        );
        assert_eq!(None, trace_parameters[0].group);
        assert_eq!(
            Some("Trace parameter value 0".into()),
            trace_parameters[0].value
        );

        let coordinates = &trace.coordinates;
        assert_eq!(1, coordinates.len());
        assert_eq!(Some("Coordinate label".into()), coordinates[0].label);
        assert_eq!(Units::Microns, coordinates[0].units);
        assert_eq!(Some("coordinates-linkid".into()), coordinates[0].linkid);
        assert_eq!(
            &Valueorder::Unspecified,
            coordinates[0].valueorder.as_ref().unwrap()
        );

        let links = &coordinates[0].links;
        assert_eq!(1, links.len());
        assert_eq!("co-linkref", links[0].linkref);

        let co_parameters = &coordinates[0].parameters;
        assert_eq!("co-parameter0", &co_parameters[0].name);
        assert_eq!(
            Some("Coordinates parameter label 0".into()),
            co_parameters[0].label
        );
        assert_eq!(None, co_parameters[0].group);
        assert_eq!(
            Some("Coordinates parameter value 0".into()),
            co_parameters[0].value
        );

        let co_values = &coordinates[0].values;
        assert_eq!(Format::Float32, co_values.format);
        assert_eq!(Byteorder::Intel, co_values.byteorder);
        assert_eq!(Some(2), co_values.numvalues);
        // value is lazily read
        // converted data
        let decoded_values = co_values.get_data().unwrap();
        assert_eq!(2, decoded_values.len());
        assert_eq!(1.0f32 as f64, decoded_values[0]);
        assert_eq!(2.0f32 as f64, decoded_values[1]);

        let x_data = &trace.x_data;
        assert_eq!(1, x_data.len());
        assert_eq!(Some("Xdata label".into()), x_data[0].label);
        assert_eq!(Units::Microns, x_data[0].units);
        assert_eq!(Some("xdata-linkid".into()), x_data[0].linkid);
        assert_eq!(
            &Valueorder::Unspecified,
            x_data[0].valueorder.as_ref().unwrap()
        );

        let xdata_links = &x_data[0].links;
        assert_eq!(1, xdata_links.len());
        assert_eq!("xdata-linkref", xdata_links[0].linkref);

        let xdata_parameters = &x_data[0].parameters;
        assert_eq!("xdata-parameter0", &xdata_parameters[0].name);
        assert_eq!(
            Some("Xdata parameter label 0".into()),
            xdata_parameters[0].label
        );
        assert_eq!(None, xdata_parameters[0].group);
        assert_eq!(
            Some("Xdata parameter value 0".into()),
            xdata_parameters[0].value
        );

        let xdata_values = &x_data[0].values;
        assert_eq!(Format::Float32, xdata_values.format);
        assert_eq!(Byteorder::Intel, xdata_values.byteorder);
        assert_eq!(Some(2), xdata_values.numvalues);
        // value is lazily read
        // converted data
        let decoded_values = xdata_values.get_data().unwrap();
        assert_eq!(2, decoded_values.len());
        assert_eq!(1.0f32 as f64, decoded_values[0]);
        assert_eq!(2.0f32 as f64, decoded_values[1]);

        let alt_x_data = &x_data[0].alt_x_data;
        assert_eq!(1, alt_x_data.len());
        assert_eq!(Some("altXdata label".into()), alt_x_data[0].label);
        assert_eq!(Units::Microns, alt_x_data[0].units);
        assert_eq!(Some("altxdata-linkid".into()), alt_x_data[0].linkid);
        assert_eq!(
            &Valueorder::Unspecified,
            alt_x_data[0].valueorder.as_ref().unwrap()
        );

        let alt_x_data_links = &alt_x_data[0].links;
        assert_eq!(1, alt_x_data_links.len());
        assert_eq!("altxdata-linkref", alt_x_data_links[0].linkref);

        let alt_x_data_parameters = &alt_x_data[0].parameters;
        assert_eq!("altxdata-parameter0", &alt_x_data_parameters[0].name);
        assert_eq!(
            Some("altXdata parameter label 0".into()),
            alt_x_data_parameters[0].label
        );
        assert_eq!(None, alt_x_data_parameters[0].group);
        assert_eq!(
            Some("altXdata parameter value 0".into()),
            alt_x_data_parameters[0].value
        );

        let alt_x_data_values = &alt_x_data[0].values;
        assert_eq!(Format::Float32, alt_x_data_values.format);
        assert_eq!(Byteorder::Intel, alt_x_data_values.byteorder);
        assert_eq!(Some(2), alt_x_data_values.numvalues);
        // value is lazily read
        // converted data
        let decoded_values = alt_x_data_values.get_data().unwrap();
        assert_eq!(2, decoded_values.len());
        assert_eq!(1.0f32 as f64, decoded_values[0]);
        assert_eq!(2.0f32 as f64, decoded_values[1]);

        let y_data = &x_data[0].y_data;
        assert_eq!(1, y_data.len());
        assert_eq!(Some("Ydata label".into()), y_data[0].label);
        assert_eq!(Units::Microns, y_data[0].units);

        let y_data_parameters = &y_data[0].parameters;
        assert_eq!("ydata-parameter0", &y_data_parameters[0].name);
        assert_eq!(
            Some("Ydata parameter label 0".into()),
            y_data_parameters[0].label
        );
        assert_eq!(None, y_data_parameters[0].group);
        assert_eq!(
            Some("Ydata parameter value 0".into()),
            y_data_parameters[0].value
        );

        let y_data_values = &y_data[0].values;
        assert_eq!(Format::Float32, y_data_values.format);
        assert_eq!(Byteorder::Intel, y_data_values.byteorder);
        assert_eq!(Some(2), y_data_values.numvalues);
        // value is lazily read
        // converted data
        let decoded_values = y_data_values.get_data().unwrap();
        assert_eq!(2, decoded_values.len());
        assert_eq!(1.0f32 as f64, decoded_values[0]);
        assert_eq!(2.0f32 as f64, decoded_values[1]);

        let peaktables = &y_data[0].peaktables;
        assert_eq!(1, peaktables.len());
        let peaktable = &peaktables[0];
        let peaktable_parameters = &peaktable.parameters;
        assert_eq!("pt-parameter0", &peaktable_parameters[0].name);
        assert_eq!(
            Some("Peaktable parameter label 0".into()),
            peaktable_parameters[0].label
        );
        assert_eq!(None, xdata_parameters[0].group);
        assert_eq!(
            Some("Peaktable parameter value 0".into()),
            peaktable_parameters[0].value
        );

        let peaks = &peaktable.peaks;
        assert_eq!(1, peaks.len());
        let peak = &peaks[0];
        assert_eq!(1, peak.number);
        assert_eq!(Some("p0-group".into()), peak.group);
        assert_eq!(Some("p0-name".into()), peak.name);
        let peak_parameters = &peak.parameters;
        assert_eq!("p0-parameter0", &peak_parameters[0].name);
        assert_eq!(
            Some("Peak 0 parameter label 0".into()),
            peak_parameters[0].label
        );
        assert_eq!(None, xdata_parameters[0].group);
        assert_eq!(
            Some("Peak 0 parameter value 0".into()),
            peak_parameters[0].value
        );
        assert_eq!(0.1, peak.peak_x_value);
        assert_eq!(100.0, peak.peak_y_value);

        let baseline = peak.baseline.as_ref().unwrap();
        assert!(baseline.parameters.is_empty());
        assert_eq!(1.1, baseline.start_x_value);
        assert_eq!(11.1, baseline.start_y_value);
        assert_eq!(2.2, baseline.end_x_value);
        assert_eq!(22.2, baseline.end_y_value);

        let basecurve = baseline.basecurve.as_ref().unwrap();
        assert_eq!(vec![1.0, 2.0, 1.0, 2.0], basecurve.get_x_data().unwrap());
        assert_eq!(vec![1.0, 2.0, 1.0, 2.0], basecurve.get_y_data().unwrap());
    }

    #[test]
    fn parses_iso_8859_1_gaml_xml_1_0() {
        // GAML name="Gaml umlaut ÄÖÜäöü test file"
        let xml = b"<?xml version=\"1.0\" encoding=\"iso-8859-1\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml umlaut \xc4\xd6\xdc\xe4\xf6\xfc test file\">\n
                        </GAML>";
        let cursor = Cursor::new(xml);

        let gaml = GamlParser::parse("iso_8859_1.gaml", cursor).unwrap();
        assert_eq!("1.20", gaml.version);
        assert_eq!(Some("Gaml umlaut ÄÖÜäöü test file".into()), gaml.name);
    }

    #[test]
    fn parses_iso_8859_1_gaml_xml_1_1() {
        // GAML name="Gaml umlaut ÄÖÜäöü test file"
        let xml = b"<?xml version=\"1.1\" encoding=\"iso-8859-1\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml umlaut \xc4\xd6\xdc\xe4\xf6\xfc test file\">\n
                        </GAML>";
        let cursor = Cursor::new(xml);

        let gaml = GamlParser::parse("iso_8859_1.gaml", cursor).unwrap();
        assert_eq!("1.20", gaml.version);
        assert_eq!(Some("Gaml umlaut ÄÖÜäöü test file".into()), gaml.name);
    }

    #[test]
    fn parses_utf8_gaml_no_xml_declaration() {
        let xml = "<GAML version=\"1.20\" name=\"Gaml umlaut ÄÖÜäöü test file\"></GAML>";
        let cursor = Cursor::new(xml);

        let gaml = GamlParser::parse("iso_8859_1.gaml", cursor).unwrap();
        assert_eq!("1.20", gaml.version);
        assert_eq!(Some("Gaml umlaut ÄÖÜäöü test file".into()), gaml.name);
    }

    #[test]
    fn fails_to_parse_illegal_gaml_missing_root() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <NOGAML version=\"1.20\" name=\"NoGaml test file\"></NOGAML>";
        let cursor = Cursor::new(xml);

        let gaml_err = GamlParser::parse("test.gaml", cursor).unwrap_err();
        assert!(gaml_err.message.contains("Unexpected tag"));
    }

    #[test]
    fn fails_to_parse_illegal_gaml_wrong_closing_tag() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"NoGaml test file\"></NOGAML>";
        let cursor = Cursor::new(xml);

        // error does not contain "Unexpected end tag" as quickxml already errors
        // when encountering a close tag without corresponding open tag
        let gaml_res = GamlParser::parse("test.gaml", cursor);
        assert!(gaml_res.is_err());
    }

    #[test]
    fn fails_to_parse_illegal_trace_attribute() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml test file\">\n
                            <experiment>
                                <trace name=\"Trace 0\" technique=\"ILLEGAL_TECHNIQUE\">
                                </trace>
                            </experiment>
                        </GAML>";
        let cursor = Cursor::new(xml);

        let gaml_err = GamlParser::parse("test.gaml", cursor).unwrap_err();
        assert!(gaml_err.message.contains("ILLEGAL_TECHNIQUE"));
        assert!(gaml_err.message.contains("technique"));
        assert!(gaml_err.message.contains("trace"));
    }

    #[test]
    fn fails_to_parse_illegal_coordinates_unit_attribute() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml test file\">\n
                            <experiment>
                                <trace name=\"Trace 0\" technique=\"UNKNOWN\">
                                    <coordinates units=\"ILLEGAL_UNITS\" valueorder=\"UNSPECIFIED\">
                                        <values byteorder=\"INTEL\" format=\"FLOAT32\" numvalues=\"2\">
                                            AACAPw\nAAAEA=
                                        </values>
                                    </coordinates>
                                </trace>
                            </experiment>
                        </GAML>";
        let cursor = Cursor::new(xml);

        let gaml_err = GamlParser::parse("test.gaml", cursor).unwrap_err();
        assert!(gaml_err.message.contains("ILLEGAL_UNITS"));
        assert!(gaml_err.message.contains("units"));
        assert!(gaml_err.message.contains("coordinates"));
    }

    #[test]
    fn fails_to_parse_illegal_coordinates_valueorder_attribute() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml test file\">\n
                            <experiment>
                                <trace name=\"Trace 0\" technique=\"UNKNOWN\">
                                    <coordinates units=\"MICRONS\" valueorder=\"ILLEGAL_VALUEORDER\">
                                        <values byteorder=\"INTEL\" format=\"FLOAT32\" numvalues=\"2\">
                                            AACAPw\nAAAEA=
                                        </values>
                                    </coordinates>
                                </trace>
                            </experiment>
                        </GAML>";
        let cursor = Cursor::new(xml);

        let gaml_err = GamlParser::parse("test.gaml", cursor).unwrap_err();
        assert!(gaml_err.message.contains("ILLEGAL_VALUEORDER"));
        assert!(gaml_err.message.contains("valueorder"));
        assert!(gaml_err.message.contains("coordinates"));
    }

    #[test]
    fn fails_to_parse_illegal_xdata_units_attribute() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml test file\">\n
                            <experiment>
                                <trace name=\"Trace 0\" technique=\"UNKNOWN\">
                                    <Xdata label=\"Xdata label\" units=\"ILLEGAL_UNITS\" linkid=\"xdata-linkid\" valueorder=\"UNSPECIFIED\">
                                    </Xdata>
                                </trace>
                            </experiment>
                        </GAML>";
        let cursor = Cursor::new(xml);

        let gaml_err = GamlParser::parse("test.gaml", cursor).unwrap_err();
        assert!(gaml_err.message.contains("ILLEGAL_UNITS"));
        assert!(gaml_err.message.contains("units"));
        assert!(gaml_err.message.contains("Xdata"));
    }

    #[test]
    fn fails_to_parse_illegal_xdata_valueorder_attribute() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml test file\">\n
                            <experiment>
                                <trace name=\"Trace 0\" technique=\"UNKNOWN\">
                                    <Xdata label=\"Xdata label\" units=\"MICRONS\" linkid=\"xdata-linkid\" valueorder=\"ILLEGAL_VALUEORDER\">
                                    </Xdata>
                                </trace>
                            </experiment>
                        </GAML>";
        let cursor = Cursor::new(xml);

        let gaml_err = GamlParser::parse("test.gaml", cursor).unwrap_err();
        assert!(gaml_err.message.contains("ILLEGAL_VALUEORDER"));
        assert!(gaml_err.message.contains("valueorder"));
        assert!(gaml_err.message.contains("Xdata"));
    }

    #[test]
    fn fails_to_parse_illegal_values_byteorder_attribute() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml test file\">\n
                            <experiment>
                                <trace name=\"Trace 0\" technique=\"UNKNOWN\">
                                    <Xdata label=\"Xdata label\" units=\"MICRONS\" linkid=\"xdata-linkid\" valueorder=\"UNSPECIFIED\">
                                        <values byteorder=\"ILLEGAL_BYTEORDER\" format=\"FLOAT32\" numvalues=\"2\">
                                            <!-- A values comment -->
                                            AACAPw\nAAAEA=
                                        </values>
                                    </Xdata>
                                </trace>
                            </experiment>
                        </GAML>";
        let cursor = Cursor::new(xml);

        let gaml_err = GamlParser::parse("test.gaml", cursor).unwrap_err();
        assert!(gaml_err.message.contains("ILLEGAL_BYTEORDER"));
        assert!(gaml_err.message.contains("byteorder"));
        assert!(gaml_err.message.contains("values"));
    }

    #[test]
    fn fails_to_parse_illegal_values_format_attribute() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml test file\">\n
                            <experiment>
                                <trace name=\"Trace 0\" technique=\"UNKNOWN\">
                                    <Xdata label=\"Xdata label\" units=\"MICRONS\" linkid=\"xdata-linkid\" valueorder=\"UNSPECIFIED\">
                                        <values byteorder=\"INTEL\" format=\"ILLEGAL_FORMAT\" numvalues=\"2\">
                                            <!-- A values comment -->
                                            AACAPw\nAAAEA=
                                        </values>
                                    </Xdata>
                                </trace>
                            </experiment>
                        </GAML>";
        let cursor = Cursor::new(xml);

        let gaml_err = GamlParser::parse("test.gaml", cursor).unwrap_err();
        assert!(gaml_err.message.contains("ILLEGAL_FORMAT"));
        assert!(gaml_err.message.contains("format"));
        assert!(gaml_err.message.contains("values"));
    }

    #[test]
    fn fails_to_parse_illegal_values_numvalues_attribute() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml test file\">\n
                            <experiment>
                                <trace name=\"Trace 0\" technique=\"UNKNOWN\">
                                    <Xdata label=\"Xdata label\" units=\"MICRONS\" linkid=\"xdata-linkid\" valueorder=\"UNSPECIFIED\">
                                        <values byteorder=\"INTEL\" format=\"FLOAT32\" numvalues=\"ILLEGAL_NUMVALUES\">
                                            <!-- A values comment -->
                                            AACAPw\nAAAEA=
                                        </values>
                                    </Xdata>
                                </trace>
                            </experiment>
                        </GAML>";
        let cursor = Cursor::new(xml);

        let gaml_err = GamlParser::parse("test.gaml", cursor).unwrap_err();
        assert!(gaml_err.message.contains("ILLEGAL_NUMVALUES"));
        assert!(gaml_err.message.contains("numvalues"));
        assert!(gaml_err.message.contains("values"));
    }
}
