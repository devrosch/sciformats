use super::gaml_utils::{
    check_end, get_attributes, get_opt_attr, get_req_attr, next_non_whitespace, read_opt_elem,
    read_sequence, read_start, read_value, skip_whitespace, skip_xml_decl,
};
use super::GamlError;
use crate::api::Parser;
use chrono::{DateTime, FixedOffset};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::io::{BufRead, BufReader, Read, Seek};
use std::str::{self, FromStr};
use strum::EnumString;

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
    pub experiments: Vec<Experiment>,
}

impl Gaml {
    const TAG: &'static [u8] = b"GAML";

    pub fn new<R: BufRead>(_name: &str, mut reader: Reader<R>) -> Result<Self, GamlError> {
        let mut buf = Vec::new();

        // skip <?xml> element
        let next = skip_xml_decl(&mut reader, &mut buf)?;

        // attributes
        let start = read_start(Self::TAG, &next)?;
        let attr_map = get_attributes(start, &reader);
        let version = get_req_attr("version", &attr_map)?;
        let name = get_opt_attr("name", &attr_map);
        let next = skip_whitespace(&mut reader, &mut buf)?;

        // nested elements
        let (integrity, next) =
            read_opt_elem(b"integrity", next, &mut reader, &mut buf, &Integrity::new)?;
        let next = next_non_whitespace(next, &mut reader, &mut buf)?;
        let (parameters, next) =
            read_sequence(b"parameter", next, &mut reader, &mut buf, &Parameter::new)?;
        let next = next_non_whitespace(next, &mut reader, &mut buf)?;
        let (experiments, next) =
            read_sequence(b"experiment", next, &mut reader, &mut buf, &Experiment::new)?;
        let next = next_non_whitespace(next, &mut reader, &mut buf)?;

        check_end(Self::TAG, &next)?;

        Ok(Self {
            version,
            name,
            integrity,
            parameters,
            experiments,
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
    const TAG: &'static [u8] = b"integrity";

    pub fn new<R: BufRead>(
        event: &Event<'_>,
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        let start = read_start(Self::TAG, event)?;

        // attributes
        let attr_map = get_attributes(start, reader);
        let algorithm = get_req_attr("algorithm", &attr_map)?;

        // value
        let (value, next) = read_value(reader, buf)?;

        check_end(Self::TAG, &next)?;

        Ok(Self { algorithm, value })
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
    const TAG: &'static [u8] = b"parameter";

    pub fn new<R: BufRead>(
        event: &Event<'_>,
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        let start = read_start(Self::TAG, event)?;

        // attributes
        let attr_map = get_attributes(start, reader);
        let group = get_opt_attr("group", &attr_map);
        let name = get_req_attr("name", &attr_map)?;
        let label = get_opt_attr("label", &attr_map);
        let alias = get_opt_attr("alias", &attr_map);

        // value
        let (value, next) = read_value(reader, buf)?;

        check_end(Self::TAG, &next)?;

        Ok(Parameter {
            group,
            name,
            label,
            alias,
            value,
        })
    }
}

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

    pub fn new<R: BufRead>(
        event: &Event<'_>,
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        let start = read_start(Self::TAG, event)?;

        // attributes
        let attr_map = get_attributes(start, reader);
        let name = get_opt_attr("name", &attr_map);

        // nested elements
        let next = skip_whitespace(reader, buf)?;
        let (datetime, next) = read_opt_elem(b"collectdate", next, reader, buf, &Collectdate::new)?;
        let collectdate = match datetime {
            None => None,
            Some(dt) => Some(DateTime::parse_from_rfc3339(&dt.value)?),
        };
        let next = next_non_whitespace(next, reader, buf)?;
        let (parameters, next) = read_sequence(b"parameter", next, reader, buf, &Parameter::new)?;
        let next = next_non_whitespace(next, reader, buf)?;
        let (traces, next) = read_sequence(b"trace", next, reader, buf, &Trace::new)?;
        let next = next_non_whitespace(next, reader, buf)?;

        check_end(Self::TAG, &next)?;

        Ok(Self {
            name,
            collectdate,
            parameters,
            traces,
        })
    }
}

struct Collectdate {
    pub value: String,
}

impl Collectdate {
    const TAG: &'static [u8] = b"collectdate";

    pub fn new<R: BufRead>(
        event: &Event<'_>,
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        read_start(Self::TAG, event)?;
        // value
        let (value, next) = read_value(reader, buf)?;
        check_end(Self::TAG, &next)?;

        Ok(Self {
            value: value.trim().into(),
        })
    }
}

#[derive(EnumString, PartialEq, Debug)]
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

pub struct Trace {
    // Attributes
    pub name: Option<String>,
    pub technique: Technique,
    // Elements
    pub parameters: Vec<Parameter>,
    pub coordinates: Vec<Coordinates>,
    // todo:
    // coordinates
    // xdata
}

impl Trace {
    const TAG: &'static [u8] = b"trace";

    pub fn new<R: BufRead>(
        event: &Event<'_>,
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        let start = read_start(Self::TAG, event)?;

        // attributes
        let attr_map = get_attributes(start, reader);
        let name = get_opt_attr("name", &attr_map);
        let technique_str = get_req_attr("technique", &attr_map)?;
        let technique = Technique::from_str(&technique_str).map_err(|e| {
            GamlError::from_source(
                e,
                format!(
                    "Error parsing trace. Unexpected technique attribute: {}",
                    &technique_str
                ),
            )
        })?;

        // nested elements
        let next = skip_whitespace(reader, buf)?;
        let (parameters, next) = read_sequence(b"parameter", next, reader, buf, &Parameter::new)?;
        let next = next_non_whitespace(next, reader, buf)?;
        let (coordinates, next) =
            read_sequence(b"coordinates", next, reader, buf, &Coordinates::new)?;
        let next = next_non_whitespace(next, reader, buf)?;

        // todo: read sequences of xdata

        check_end(Self::TAG, &next)?;

        Ok(Self {
            name,
            technique,
            parameters,
            coordinates,
        })
    }
}

#[derive(EnumString, PartialEq, Debug)]
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

#[derive(EnumString, PartialEq, Debug)]
pub enum Valueorder {
    #[strum(serialize = "EVEN")]
    Even,
    #[strum(serialize = "ORDERED")]
    Ordered,
    #[strum(serialize = "UNSPECIFIED")]
    Unspecified,
}

pub struct Coordinates {
    // Attributes
    pub units: Units,
    pub label: Option<String>,
    pub linkid: Option<String>,
    pub valueorder: Valueorder,
    // Elements
    // todo:
    // link
    // pub parameters: Vec<Parameter>,
    // value
}

impl Coordinates {
    const TAG: &'static [u8] = b"coordinates";

    pub fn new<R: BufRead>(
        event: &Event<'_>,
        reader: &mut Reader<R>,
        buf: &mut Vec<u8>,
    ) -> Result<Self, GamlError> {
        let start = read_start(Self::TAG, event)?;

        // attributes
        let attr_map = get_attributes(start, reader);
        let units_str = get_req_attr("units", &attr_map)?;
        let units = Units::from_str(&units_str).map_err(|e| {
            GamlError::from_source(
                e,
                format!(
                    "Error parsing coordinates. Unexpected units attribute: {}",
                    &units_str
                ),
            )
        })?;
        let label = get_opt_attr("label", &attr_map);
        let linkid = get_opt_attr("linkid", &attr_map);
        let valueorder_str = get_req_attr("valueorder", &attr_map)?;
        let valueorder = Valueorder::from_str(&valueorder_str).map_err(|e| {
            GamlError::from_source(
                e,
                format!(
                    "Error parsing coordinates. Unexpected valueorder attribute: {}",
                    &units_str
                ),
            )
        })?;

        // nested elements
        let next = skip_whitespace(reader, buf)?;

        // todo: read sequences of links, parameters, values

        check_end(Self::TAG, &next)?;

        Ok(Self {
            units,
            label,
            linkid,
            valueorder,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use super::*;
    use std::io::Cursor;

    #[test]
    fn parsing_simple_gaml_succeeds() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n
                        <GAML version=\"1.20\" name=\"Gaml test file\">\n
                            <integrity algorithm=\"SHA1\">03cfd743661f07975fa2f1220c5194cbaff48451</integrity>\n
                            <parameter name=\"parameter0\" label=\"Parameter label 0\" group=\"Parameter group 0\">Parameter value 0</parameter>\n
                            <!-- A comment -->
                            <parameter name=\"parameter1\" label=\"Parameter label 1\" group=\"Parameter group 1\">\
                            <!-- A comment -->Parameter <!-- A comment -->value 1<!-- A comment --></parameter>\
                            <parameter name=\"parameter2\" label=\"Parameter label 2\" group=\"Parameter group 2\">Parameter value 2</parameter>\n
                            <experiment name=\"Experiment name\">
                                <collectdate>2024-03-27T06:46:00Z</collectdate>
                                <parameter name=\"exp-parameter0\" label=\"Experiment parameter label 0\">Experiment parameter value 0</parameter>
                                <trace name=\"Trace 0\" technique=\"UNKNOWN\">
                                    <parameter name=\"trace-parameter0\" label=\"Trace parameter label 0\">Trace parameter value 0</parameter>
                                    <coordinates label=\"Coordinate label\" units=\"MICRONS\" linkid=\"coordinates-linkid\" valueorder=\"UNSPECIFIED\">
                                    </coordinates>
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
        assert_eq!("Parameter value 0", &parameters[0].value);
        assert_eq!("parameter1", &parameters[1].name);
        assert_eq!(Some("Parameter label 1".into()), parameters[1].label);
        assert_eq!(Some("Parameter group 1".into()), parameters[1].group);
        assert_eq!("Parameter value 1", &parameters[1].value);
        assert_eq!("parameter2", &parameters[2].name);
        assert_eq!(Some("Parameter label 2".into()), parameters[2].label);
        assert_eq!(Some("Parameter group 2".into()), parameters[2].group);
        assert_eq!("Parameter value 2", &parameters[2].value);

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
            "Experiment parameter value 0",
            &experiment_parameters[0].value
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
        assert_eq!("Trace parameter value 0", trace_parameters[0].value);

        let coordinates = &trace.coordinates;
        assert_eq!(1, coordinates.len());
        assert_eq!(Some("Coordinate label".into()), coordinates[0].label);
        assert_eq!(Units::Microns, coordinates[0].units);
        assert_eq!(Some("coordinates-linkid".into()), coordinates[0].linkid);
        assert_eq!(Valueorder::Unspecified, coordinates[0].valueorder);
    }
}
