use super::jdx_data_parser::{parse_xppyy_data, parse_xyxy_data};
use super::jdx_peak_assignments_parser::PeakAssignmentsParser;
use super::jdx_peak_table_parser::PeakTableParser;
use super::jdx_utils::{
    is_ldr_start, is_pure_comment, parse_ldr_start, parse_parameter, parse_single_parameter,
    strip_line_comment, validate_input, BinBufRead,
};
use super::JdxError;
use crate::api::{Parser, SeekBufRead};
use crate::jdx::jdx_utils::{
    find_ldr, is_bruker_specific_section_start, parse_string_value, skip_pure_comments,
    skip_to_next_ldr,
};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::SeekFrom;
use std::rc::Rc;
use std::str::FromStr;
use std::vec;

pub struct JdxParser {}

impl<T: SeekBufRead + 'static> Parser<T> for JdxParser {
    type R = JdxBlock<T>;
    type E = JdxError;

    fn parse(name: &str, input: T) -> Result<Self::R, Self::E> {
        Self::R::new(name, input)
    }
}

#[derive(Debug)]
pub struct JdxBlock<T: SeekBufRead> {
    /// The labeled data records (LDRs) of the Block.
    ///
    /// This does not include the following LDRs:
    /// - comments ("##=")
    /// - data (XYDATA, XYPOINTS, PEAK TABLE, PEAK ASSIGNMENTS, RADATA,
    ///   NTUPLES)
    /// These are available as dedicated peroperties.
    ///
    /// Also does not include "##END=" LDR.
    ///
    /// The key is the normalized label without "##" and "=" and the value is
    /// the content (without initial blank character if any).E.g. the LDR
    /// "##TITLE= abc" has label "TITLE" and content "abc" and the LDR
    /// "##DATA_POINTS=   5" has label "DATAPOINTS" and content "  5".
    pub ldrs: Vec<StringLdr>,

    /// The labeled data records (LDRs) of the Block that are comments (i.e.
    /// "##= <comment>"). The value holds the comment contents. The content of
    /// a comment is the text following the "=" without initial blank character
    /// if any. E.g. the comment "##= abc" has content "abc".
    pub ldr_comments: Vec<String>,

    /// BLOCKs that are nested in this (LINK) block.
    pub blocks: Vec<JdxBlock<T>>,

    /// The XYDATA record if available.
    pub xy_data: Option<XyData<T>>,

    /// The RADATA record if available.
    pub ra_data: Option<RaData<T>>,

    /// The XYPOINTS record if available.
    pub xy_points: Option<XyPoints<T>>,

    /// The PEAK TABLE record if available.
    pub peak_table: Option<PeakTable<T>>,

    /// The PEAK ASSIGNMENTS record if available.
    pub peak_assignments: Option<PeakAssignments<T>>,
    // std::optional<NTuples> m_nTuples;
    // std::optional<AuditTrail> m_auditTrail;
    // std::vector<BrukerSpecificParameters> m_brukerSpecificParameters;
    // std::vector<BrukerRelaxSection> m_brukerRelaxSections;
}

impl<T: SeekBufRead> JdxBlock<T> {
    const BLOCK_START_LABEL: &'static str = "TITLE";
    const BLOCK_END_LABEL: &'static str = "END";

    pub fn new(_name: &str, mut reader: T) -> Result<Self, JdxError> {
        let mut buf = Vec::<u8>::with_capacity(1024);
        let line = reader.read_line_iso_8859_1(&mut buf)?;
        let title = Self::parse_first_line(line.as_deref())?;
        let reader_ref = Rc::new(RefCell::new(reader));
        let (block, _next_line) = Self::parse_input(&title, reader_ref, &mut buf)?;
        Ok(block)
    }

    pub fn new_nested(
        title: &str,
        reader_ref: Rc<RefCell<T>>,
        buf: &mut Vec<u8>,
    ) -> Result<(Self, Option<String>), JdxError> {
        let (block, next_line) = Self::parse_input(title, reader_ref, buf)?;
        Ok((block, next_line))
    }

    pub fn get_ldr(&self, label: &str) -> Option<&StringLdr> {
        find_ldr(label, &self.ldrs)
    }

    fn parse_first_line(line_opt: Option<&str>) -> Result<String, JdxError> {
        if line_opt.is_none() {
            return Err(JdxError::new("Malformed block start. First line is empty."));
        }
        let line = line_opt.unwrap();
        let (label, value) = parse_ldr_start(line)?;
        if Self::BLOCK_START_LABEL != label {
            Err(JdxError::new(&format!("Malformed block start: {line}")))
        } else {
            Ok(value)
        }
    }

    fn parse_input(
        title: &str,
        reader_ref: Rc<RefCell<T>>,
        buf: &mut Vec<u8>,
    ) -> Result<(Self, Option<String>), JdxError> {
        let mut reader = reader_ref.borrow_mut();

        let mut ldrs = Vec::<StringLdr>::new();
        let mut ldr_comments = Vec::<String>::new();
        let mut blocks = Vec::<JdxBlock<T>>::new();
        let mut xy_data = Option::<XyData<T>>::None;
        let mut ra_data = Option::<RaData<T>>::None;
        let mut xy_points = Option::<XyPoints<T>>::None;
        let mut peak_table = Option::<PeakTable<T>>::None;
        let mut peak_assignments = Option::<PeakAssignments<T>>::None;

        let (title, mut next_line) = parse_string_value(title, &mut *reader, buf)?;
        ldrs.push(StringLdr {
            label: Self::BLOCK_START_LABEL.into(),
            value: title.clone(),
        });

        while let Some(ref line) = next_line {
            if is_pure_comment(line) {
                if is_bruker_specific_section_start(line) {
                    todo!();
                }
                next_line = skip_pure_comments(next_line, true, &mut *reader, buf)?;
                continue;
            }

            let (label, mut value) = parse_ldr_start(line)?;
            match label.as_str() {
                "" => {
                    // LDR start is an LDR comment "##="
                    (value, next_line) = parse_string_value(&value, &mut *reader, buf)?;
                    ldr_comments.push(value);
                }
                Self::BLOCK_END_LABEL => break,
                Self::BLOCK_START_LABEL => {
                    drop(reader);
                    let (block, next) = JdxBlock::new_nested(&value, Rc::clone(&reader_ref), buf)?;
                    reader = reader_ref.borrow_mut();
                    blocks.push(block);
                    next_line = next;
                }
                "XYDATA" => {
                    if xy_data.is_some() {
                        return Err(JdxError::new(&format!(
                            "Multiple \"{}\" LDRs found in block: {}",
                            label, title
                        )));
                    }
                    drop(reader);
                    let (data, next) =
                        XyData::new(&label, &value, &ldrs, next_line, Rc::clone(&reader_ref))?;
                    reader = reader_ref.borrow_mut();
                    xy_data = Some(data);
                    next_line = next;
                }
                "RADATA" => {
                    // todo: refactor to reduce code duplication
                    if ra_data.is_some() {
                        return Err(JdxError::new(&format!(
                            "Multiple \"{}\" LDRs found in block: {}",
                            label, title
                        )));
                    }
                    drop(reader);
                    let (data, next) =
                        RaData::new(&label, &value, &ldrs, next_line, Rc::clone(&reader_ref))?;
                    reader = reader_ref.borrow_mut();
                    ra_data = Some(data);
                    next_line = next;
                }
                "XYPOINTS" => {
                    // todo: refactor to reduce code duplication
                    if xy_points.is_some() {
                        return Err(JdxError::new(&format!(
                            "Multiple \"{}\" LDRs found in block: {}",
                            label, title
                        )));
                    }
                    drop(reader);
                    let (data, next) =
                        XyPoints::new(&label, &value, &ldrs, next_line, Rc::clone(&reader_ref))?;
                    reader = reader_ref.borrow_mut();
                    xy_points = Some(data);
                    next_line = next;
                }
                "PEAKTABLE" => {
                    // todo: refactor to reduce code duplication
                    if peak_table.is_some() {
                        return Err(JdxError::new(&format!(
                            "Multiple \"{}\" LDRs found in block: {}",
                            label, title
                        )));
                    }
                    drop(reader);
                    let (data, next) =
                        PeakTable::new(&label, &value, next_line, Rc::clone(&reader_ref))?;
                    reader = reader_ref.borrow_mut();
                    peak_table = Some(data);
                    next_line = next;
                }
                "PEAKASSIGNMENTS" => {
                    // todo: refactor to reduce code duplication
                    if peak_assignments.is_some() {
                        return Err(JdxError::new(&format!(
                            "Multiple \"{}\" LDRs found in block: {}",
                            label, title
                        )));
                    }
                    drop(reader);
                    let (data, next) =
                        PeakAssignments::new(&label, &value, next_line, Rc::clone(&reader_ref))?;
                    reader = reader_ref.borrow_mut();
                    peak_assignments = Some(data);
                    next_line = next;
                }
                "NTUPLES" => todo!(),
                "AUDITTRAIL" => todo!(),
                "$RELAX" => todo!(),
                _ => {
                    // LDR is a regular LDR
                    (value, next_line) = parse_string_value(&value, &mut *reader, buf)?;

                    let existing_ldr = find_ldr(&label, &ldrs);
                    if let Some(ldr) = existing_ldr {
                        // reference implementation seems to overwrite LDR with
                        // duplicate, but spec (JCAMP-DX IR 3.2) says
                        // a duplicate LDR is illegal in a block
                        // => accept if content is identical
                        if ldr.value != value {
                            return Err(JdxError::new(&format!(
                                "Multiple non-identical values found for \"{}\" in block: {}",
                                label, title
                            )));
                        }
                    }

                    ldrs.push(StringLdr::new(label, value));
                }
            }
        }

        if next_line.is_none() || "END" != parse_ldr_start(&next_line.unwrap())?.0 {
            return Err(JdxError::new(&format!(
                "No END LDR encountered for block: {}",
                title
            )));
        }
        next_line = reader.read_line_iso_8859_1(buf)?;

        Ok((
            JdxBlock {
                ldrs,
                ldr_comments,
                blocks,
                xy_data,
                ra_data,
                xy_points,
                peak_table,
                peak_assignments,
            },
            next_line,
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLdr {
    /// The label of the LDR, e.g., "TITLE" for "##TITLE= abc".
    pub label: String,
    /// The value (without initial blank character if any) of the LDR, e.g.,
    /// "abc" for "##TITLE= abc".
    pub value: String,
}

impl StringLdr {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }

    pub fn is_user_defined(&self) -> bool {
        self.label.chars().nth(0) == Some('$')
    }

    pub fn is_technique_specific(&self) -> bool {
        self.label.chars().nth(0) == Some('.')
    }
}

/// A JCAMP-DX XYDATA record.
#[derive(Debug, PartialEq)]
pub struct XyData<T: SeekBufRead> {
    reader_ref: Rc<RefCell<T>>,
    address: u64,

    label: String,
    variable_list: String,
    parameters: XyParameters,
}

impl<T: SeekBufRead> XyData<T> {
    const LABEL: &'static str = "XYDATA";
    // quirk variable list found in some sample data
    // that violates the spec but is unambiguous and thus accepted
    const QUIRK_OO_VARIABLE_LIST: &'static str = "(XY..XY)";
    const VARIABLE_LISTS: [&'static str; 4] = [
        "(X++(Y..Y))",
        "(X++(R..R))",
        "(X++(I..I))",
        Self::QUIRK_OO_VARIABLE_LIST,
    ];

    fn new(
        label: &str,
        variable_list: &str,
        ldrs: &[StringLdr],
        next_line: Option<String>,
        reader_ref: Rc<RefCell<T>>,
    ) -> Result<(XyData<T>, Option<String>), JdxError> {
        validate_input(
            label,
            Some(variable_list),
            Self::LABEL,
            Some(&Self::VARIABLE_LISTS),
        )?;
        let mut reader = reader_ref.borrow_mut();
        let address = reader.stream_position()?;
        let parameters = parse_xydata_parameters(ldrs)?;
        let next_line = skip_to_next_ldr(next_line, true, &mut *reader, &mut vec![])?;
        drop(reader);

        Ok((
            XyData {
                reader_ref,
                address,
                label: label.to_owned(),
                variable_list: variable_list.to_owned(),
                parameters,
            },
            next_line,
        ))
    }

    /// Provides the parsed xy data.
    ///
    /// Returns pairs of xy data. Invalid values ("?") will be represented by NaN.
    pub fn get_data(&self) -> Result<Vec<(f64, f64)>, JdxError> {
        // todo: move check to constructor
        // todo: Required? Should have been caught in new().
        if !Self::VARIABLE_LISTS.contains(&self.variable_list.as_str()) {
            return Err(JdxError::new(&format!(
                "Unsupported variable list for XYDATA: {}",
                &self.variable_list,
            )));
        }
        let data = if self.variable_list == Self::QUIRK_OO_VARIABLE_LIST {
            // Ocean Optics quirk
            parse_xyxy_data(
                &self.label,
                self.parameters.x_factor,
                self.parameters.y_factor,
                Some(self.parameters.n_points),
                self.address,
                &mut *self.reader_ref.borrow_mut(),
            )?
        } else {
            parse_xppyy_data(
                &self.label,
                self.parameters.first_x,
                self.parameters.last_x,
                self.parameters.y_factor,
                self.parameters.n_points,
                self.address,
                &mut *self.reader_ref.borrow_mut(),
            )?
        };

        Ok(data)
    }
}

fn parse_xydata_parameters(ldrs: &[StringLdr]) -> Result<XyParameters, JdxError> {
    // required
    // string
    let x_units = parse_parameter::<String>("XUNITS", ldrs)?;
    let y_units = parse_parameter::<String>("YUNITS", ldrs)?;
    // double
    let first_x = parse_parameter::<f64>("FIRSTX", ldrs)?;
    let last_x = parse_parameter::<f64>("LASTX", ldrs)?;
    let x_factor = parse_parameter::<f64>("XFACTOR", ldrs)?;
    let y_factor = parse_parameter::<f64>("YFACTOR", ldrs)?;
    // u64
    let n_points = parse_parameter::<u64>("NPOINTS", ldrs)?;
    // optional
    // double
    let first_y = parse_parameter::<f64>("FIRSTY", ldrs)?;
    let max_x = parse_parameter::<f64>("MAXX", ldrs)?;
    let min_x = parse_parameter::<f64>("MINX", ldrs)?;
    let max_y = parse_parameter::<f64>("MAXY", ldrs)?;
    let min_y = parse_parameter::<f64>("MINY", ldrs)?;
    let resolution = parse_parameter::<f64>("RESOLUTION", ldrs)?;
    let delta_x = parse_parameter::<f64>("DELTAX", ldrs)?;

    let mut missing = vec![];
    if x_units.is_none() {
        missing.push("XUNITS");
    }
    if y_units.is_none() {
        missing.push("YUNITS");
    }
    if first_x.is_none() {
        missing.push("FIRSTX");
    }
    if last_x.is_none() {
        missing.push("LASTX");
    }
    if x_factor.is_none() {
        missing.push("XFACTOR");
    }
    if y_factor.is_none() {
        missing.push("YFACTOR");
    }
    if n_points.is_none() {
        missing.push("NPOINTS");
    }
    if !missing.is_empty() {
        return Err(JdxError::new(&format!(
            // todo: also for XYPOINTS?
            "Required LDR(s) missing for XYDATA: {}",
            missing.join(", ")
        )));
    }

    Ok(XyParameters {
        x_units: x_units.unwrap(),
        y_units: y_units.unwrap(),
        x_factor: x_factor.unwrap(),
        y_factor: y_factor.unwrap(),
        n_points: n_points.unwrap(),
        first_x: first_x.unwrap(),
        last_x: last_x.unwrap(),
        first_y,
        max_x,
        min_x,
        max_y,
        min_y,
        resolution,
        delta_x,
    })
}

/// JCAMP-DX spectral parameters describing an XYDATA record.
#[derive(Debug, PartialEq)]
pub struct XyParameters {
    /// Abscissa units.
    ///
    /// Not required for parsing but for displaying.
    x_units: String,
    /// Ordinate units.
    ///
    /// Not required for parsing but for displaying.
    y_units: String,
    /// The factor by which to multiply raw x values to arrive at the actual value.
    x_factor: f64,
    /// The factor by which to multiply raw y values to arrive at the actual value.
    y_factor: f64,
    /// The number of xy pairs in this record.
    n_points: u64,
    /// The first x value.
    first_x: f64,
    /// The last x value.
    last_x: f64,
    /// The first actual Y value (after scaling).
    first_y: Option<f64>,
    /// Maximum X.
    max_x: Option<f64>,
    /// Minimum X.
    min_x: Option<f64>,
    /// Maximum Y.
    max_y: Option<f64>,
    /// Minimum Y.
    min_y: Option<f64>,
    /// The resolution of the data.
    resolution: Option<f64>,
    /// The x distance between adjacent data points (if constant).
    delta_x: Option<f64>,
}

/// A JCAMP-DX RADATA record.
#[derive(Debug, PartialEq)]
pub struct RaData<T: SeekBufRead> {
    reader_ref: Rc<RefCell<T>>,
    address: u64,

    label: String,
    variable_list: String,
    parameters: RaParameters,
}

impl<T: SeekBufRead> RaData<T> {
    const LABEL: &'static str = "RADATA";
    const VARIABLE_LISTS: [&'static str; 1] = ["(R++(A..A))"];

    fn new(
        label: &str,
        variable_list: &str,
        ldrs: &[StringLdr],
        next_line: Option<String>,
        reader_ref: Rc<RefCell<T>>,
    ) -> Result<(RaData<T>, Option<String>), JdxError> {
        validate_input(
            label,
            Some(variable_list),
            Self::LABEL,
            Some(&Self::VARIABLE_LISTS),
        )?;
        let mut reader = reader_ref.borrow_mut();
        let address = reader.stream_position()?;
        let parameters = Self::parse_parameters(ldrs)?;
        let next_line = skip_to_next_ldr(next_line, true, &mut *reader, &mut vec![])?;
        drop(reader);

        Ok((
            RaData {
                reader_ref,
                address,
                label: label.to_owned(),
                variable_list: variable_list.to_owned(),
                parameters,
            },
            next_line,
        ))
    }

    fn parse_parameters(ldrs: &[StringLdr]) -> Result<RaParameters, JdxError> {
        // required
        // string
        let r_units = parse_parameter::<String>("RUNITS", ldrs)?;
        let a_units = parse_parameter::<String>("AUNITS", ldrs)?;
        // double
        let first_r = parse_parameter::<f64>("FIRSTR", ldrs)?;
        let last_r = parse_parameter::<f64>("LASTR", ldrs)?;
        let r_factor = parse_parameter::<f64>("RFACTOR", ldrs)?;
        let a_factor = parse_parameter::<f64>("AFACTOR", ldrs)?;
        // u64
        let n_points = parse_parameter::<u64>("NPOINTS", ldrs)?;
        // optional
        // double
        let first_a = parse_parameter::<f64>("FIRSTA", ldrs)?;
        // required, according to standard
        let max_a = parse_parameter::<f64>("MAXA", ldrs)?;
        // required, according to standard
        let min_a = parse_parameter::<f64>("MINA", ldrs)?;
        let resolution = parse_parameter::<f64>("RESOLUTION", ldrs)?;
        let delta_r = parse_parameter::<f64>("DELTAR", ldrs)?;
        let zdp = parse_parameter::<f64>("ZDP", ldrs)?;
        // string
        let alias = parse_parameter::<String>("ALIAS", ldrs)?;

        let mut missing = vec![];
        if r_units.is_none() {
            missing.push("RUNITS");
        }
        if a_units.is_none() {
            missing.push("AUNITS");
        }
        if first_r.is_none() {
            missing.push("FIRSTR");
        }
        if last_r.is_none() {
            missing.push("LASTR");
        }
        if r_factor.is_none() {
            missing.push("RFACTOR");
        }
        if a_factor.is_none() {
            missing.push("AFACTOR");
        }
        if n_points.is_none() {
            missing.push("NPOINTS");
        }
        if !missing.is_empty() {
            return Err(JdxError::new(&format!(
                "Required LDR(s) missing for RADATA: {}",
                missing.join(", ")
            )));
        }

        Ok(RaParameters {
            r_units: r_units.unwrap(),
            a_units: a_units.unwrap(),
            first_r: first_r.unwrap(),
            last_r: last_r.unwrap(),
            r_factor: r_factor.unwrap(),
            a_factor: a_factor.unwrap(),
            n_points: n_points.unwrap(),
            first_a,
            max_a,
            min_a,
            resolution,
            delta_r,
            zdp,
            alias,
        })
    }

    /// Provides the parsed xy data.
    ///
    /// Returns pairs of xy data. Invalid values ("?") will be represented by NaN.
    pub fn get_data(&self) -> Result<Vec<(f64, f64)>, JdxError> {
        // todo: Required? Should have been caught in new().
        if !Self::VARIABLE_LISTS.contains(&self.variable_list.as_str()) {
            return Err(JdxError::new(&format!(
                "Unsupported variable list for RADATA: {}",
                &self.variable_list,
            )));
        }
        let data = parse_xppyy_data(
            &self.label,
            self.parameters.first_r,
            self.parameters.last_r,
            self.parameters.a_factor,
            self.parameters.n_points,
            self.address,
            &mut *self.reader_ref.borrow_mut(),
        )?;

        Ok(data)
    }
}

/// JCAMP-DX spectral parameters describing an RADATA record.
#[derive(Debug, PartialEq)]
pub struct RaParameters {
    /// Abscissa units.
    ///
    /// Not required for parsing but for displaying.
    r_units: String,
    /// Ordinate units.
    ///
    /// Not required for parsing but for displaying.
    a_units: String,
    /// The factor by which to multiply raw R values to arrive at the actual value.
    r_factor: f64,
    /// The factor by which to multiply raw A values to arrive at the actual value.
    a_factor: f64,
    /// The number of ra pairs in this record.
    n_points: u64,
    /// The first R value.
    first_r: f64,
    /// The last R value.
    last_r: f64,
    /// The first actual A value (after scaling).
    first_a: Option<f64>,
    // no MAXR, MINR according to standard
    /// Maximum A. Required, according to standard.
    max_a: Option<f64>,
    /// Minimum A. Required, according to standard.
    min_a: Option<f64>,
    /// The resolution of the data.
    resolution: Option<f64>,
    /// The R distance between adjacent data points (if constant).
    delta_r: Option<f64>,
    /// The number of data points before zero path difference.
    zdp: Option<f64>,
    /// Alias. Standard says type is AFFN, but gives "1/1" and "1/2" as examples.
    alias: Option<String>,
    // In addition, XUNITS, YUNITS, FIRSTX, LASTX, DELTAX are given in examples
    // in the standard with not quite clear meaning.
}

/// A JCAMP-DX XYPOINTS record.
#[derive(Debug, PartialEq)]
pub struct XyPoints<T: SeekBufRead> {
    reader_ref: Rc<RefCell<T>>,
    address: u64,

    label: String,
    variable_list: String,
    // todo: really all XYDATA parameters required?
    parameters: XyParameters,
}

impl<T: SeekBufRead> XyPoints<T> {
    const LABEL: &'static str = "XYPOINTS";
    const VARIABLE_LISTS: [&'static str; 3] = ["(XY..XY)", "(XR..XR)", "(XI..XI)"];

    fn new(
        label: &str,
        variable_list: &str,
        ldrs: &[StringLdr],
        next_line: Option<String>,
        reader_ref: Rc<RefCell<T>>,
    ) -> Result<(XyPoints<T>, Option<String>), JdxError> {
        validate_input(
            label,
            Some(variable_list),
            Self::LABEL,
            Some(&Self::VARIABLE_LISTS),
        )?;
        let mut reader = reader_ref.borrow_mut();
        let address = reader.stream_position()?;
        // todo: really all XYDATA parameters required?
        let parameters = parse_xydata_parameters(ldrs)?;
        let next_line = skip_to_next_ldr(next_line, true, &mut *reader, &mut vec![])?;
        drop(reader);

        Ok((
            XyPoints {
                reader_ref,
                address,
                label: label.to_owned(),
                variable_list: variable_list.to_owned(),
                parameters,
            },
            next_line,
        ))
    }

    /// Provides the parsed xy data.
    ///
    /// Returns pairs of xy data. Invalid values ("?") will be represented by NaN.
    pub fn get_data(&self) -> Result<Vec<(f64, f64)>, JdxError> {
        // todo: move check to constructor
        if !Self::VARIABLE_LISTS.contains(&self.variable_list.as_str()) {
            return Err(JdxError::new(&format!(
                "Unsupported variable list for XYPOINTS: {}",
                &self.variable_list,
            )));
        }
        let data = parse_xyxy_data(
            &self.label,
            self.parameters.x_factor,
            self.parameters.y_factor,
            Some(self.parameters.n_points),
            self.address,
            &mut *self.reader_ref.borrow_mut(),
        )?;

        Ok(data)
    }
}

/// A JCAMP-DX DATA TABLE record.
#[derive(Debug, PartialEq)]
pub struct PeakTable<T: SeekBufRead> {
    reader_ref: Rc<RefCell<T>>,
    address: u64,

    label: String,
    variable_list: String,
}

impl<T: SeekBufRead> PeakTable<T> {
    const LABEL: &'static str = "PEAKTABLE";
    const VARIABLE_LISTS: [&'static str; 3] = ["(XY..XY)", "(XYW..XYW)", "(XYM..XYM)"];

    fn new(
        label: &str,
        variable_list: &str,
        next_line: Option<String>,
        reader_ref: Rc<RefCell<T>>,
    ) -> Result<(PeakTable<T>, Option<String>), JdxError> {
        validate_input(
            label,
            Some(variable_list),
            Self::LABEL,
            Some(&Self::VARIABLE_LISTS),
        )?;
        let mut reader = reader_ref.borrow_mut();
        let address = reader.stream_position()?;
        let next_line = skip_to_next_ldr(next_line, true, &mut *reader, &mut vec![])?;
        drop(reader);

        Ok((
            PeakTable {
                reader_ref,
                address,
                label: label.to_owned(),
                variable_list: variable_list.to_owned(),
            },
            next_line,
        ))
    }

    pub fn get_width_function(&self) -> Result<Option<String>, JdxError> {
        // remember stream position
        let reader = &mut *self.reader_ref.borrow_mut();
        let initial_pos = reader.stream_position()?;
        reader.seek(SeekFrom::Start(self.address))?;
        let mut buf = Vec::<u8>::with_capacity(128);

        // read possible initial comment lines
        let mut kernel_lines = Vec::<String>::new();
        while let Some(line) = reader.read_line_iso_8859_1(&mut buf)? {
            if is_ldr_start(&line) {
                break;
            }
            if let (_content, Some(comment)) = strip_line_comment(&line, false, true) {
                kernel_lines.push(comment.to_owned());
            } else {
                break;
            }
        }

        // reset stream position
        reader.seek(SeekFrom::Start(initial_pos))?;

        if kernel_lines.is_empty() {
            Ok(None)
        } else {
            Ok(Some(kernel_lines.join("\n")))
        }
    }

    /// Provides the parsed peak data.
    pub fn get_data(&self) -> Result<Vec<Peak>, JdxError> {
        // todo: Required? Should have been caught in new().
        if !Self::VARIABLE_LISTS.contains(&self.variable_list.as_str()) {
            return Err(JdxError::new(&format!(
                "Unsupported variable list for PEAK TABLE: {}",
                &self.variable_list,
            )));
        }

        // remember stream position
        let reader = &mut *self.reader_ref.borrow_mut();
        let initial_pos = reader.stream_position()?;
        reader.seek(SeekFrom::Start(self.address))?;

        // skip possible initial comment lines
        let mut pos = reader.stream_position()?;
        let mut buf = Vec::<u8>::with_capacity(128);
        while let Some(line) = reader.read_line_iso_8859_1(&mut buf)? {
            if !is_pure_comment(&line) {
                break;
            }
            pos = reader.stream_position()?;
        }
        // move stream to start of first non pure comment line
        reader.seek(SeekFrom::Start(pos))?;

        // parse peaks
        let mut parser = PeakTableParser::new(&self.variable_list, reader)?;
        let mut peaks = Vec::<Peak>::new();
        while let Some(peak) = parser.next()? {
            peaks.push(peak);
        }

        // reset stream position
        reader.seek(SeekFrom::Start(initial_pos))?;

        Ok(peaks)
    }
}

/// A JCAMP-DX peak, i.e., one item in a PEAK TABLE.
#[derive(Debug, PartialEq)]
pub struct Peak {
    /// Peak position.
    pub x: f64,
    /// Intensity.
    pub y: f64,
    /// Multiplicity.
    ///
    /// S, D, Т, Q for singlets, douЬlets, triplets, or quadruplets,
    /// М for multiple, and U for unassigned. Used only for NMR.
    pub m: Option<String>,
    /// Width.
    pub w: Option<f64>,
}

/// A JCAMP-DX PEAK ASSIGNMENTS record.
#[derive(Debug, PartialEq)]
pub struct PeakAssignments<T: SeekBufRead> {
    reader_ref: Rc<RefCell<T>>,
    address: u64,

    label: String,
    variable_list: String,
}

// todo: reduce code duplication
impl<T: SeekBufRead> PeakAssignments<T> {
    const LABEL: &'static str = "PEAKASSIGNMENTS";
    const VARIABLE_LISTS: [&'static str; 4] = ["(XYA)", "(XYWA)", "(XYMA)", "(XYMWA)"];

    fn new(
        label: &str,
        variable_list: &str,
        next_line: Option<String>,
        reader_ref: Rc<RefCell<T>>,
    ) -> Result<(PeakAssignments<T>, Option<String>), JdxError> {
        validate_input(
            label,
            Some(variable_list),
            Self::LABEL,
            Some(&Self::VARIABLE_LISTS),
        )?;
        let mut reader = reader_ref.borrow_mut();
        let address = reader.stream_position()?;
        let next_line = skip_to_next_ldr(next_line, true, &mut *reader, &mut vec![])?;
        drop(reader);

        Ok((
            PeakAssignments {
                reader_ref,
                address,
                label: label.to_owned(),
                variable_list: variable_list.to_owned(),
            },
            next_line,
        ))
    }

    pub fn get_width_function(&self) -> Result<Option<String>, JdxError> {
        // remember stream position
        let reader = &mut *self.reader_ref.borrow_mut();
        let initial_pos = reader.stream_position()?;
        reader.seek(SeekFrom::Start(self.address))?;
        let mut buf = Vec::<u8>::with_capacity(128);

        // read possible initial comment lines
        let mut kernel_lines = Vec::<String>::new();
        while let Some(line) = reader.read_line_iso_8859_1(&mut buf)? {
            if is_ldr_start(&line) {
                break;
            }
            if let (_content, Some(comment)) = strip_line_comment(&line, false, true) {
                kernel_lines.push(comment.to_owned());
            } else {
                break;
            }
        }

        // reset stream position
        reader.seek(SeekFrom::Start(initial_pos))?;

        if kernel_lines.is_empty() {
            Ok(None)
        } else {
            Ok(Some(kernel_lines.join("\n")))
        }
    }

    /// Provides the parsed peak data.
    pub fn get_data(&self) -> Result<Vec<PeakAssignment>, JdxError> {
        // todo: Required? Should have been caught in new().
        if !Self::VARIABLE_LISTS.contains(&self.variable_list.as_str()) {
            return Err(JdxError::new(&format!(
                "Unsupported variable list for PEAK ASSIGNMENTS: {}",
                &self.variable_list,
            )));
        }

        // remember stream position
        let reader = &mut *self.reader_ref.borrow_mut();
        let initial_pos = reader.stream_position()?;
        reader.seek(SeekFrom::Start(self.address))?;

        // skip possible initial comment lines
        let mut pos = reader.stream_position()?;
        let mut buf = Vec::<u8>::with_capacity(128);
        while let Some(line) = reader.read_line_iso_8859_1(&mut buf)? {
            if !is_pure_comment(&line) {
                break;
            }
            pos = reader.stream_position()?;
        }
        // move stream to start of first non pure comment line
        reader.seek(SeekFrom::Start(pos))?;

        // parse peaks
        let mut parser = PeakAssignmentsParser::new(&self.variable_list, reader)?;
        let mut peaks = Vec::<PeakAssignment>::new();
        while let Some(peak) = parser.next()? {
            peaks.push(peak);
        }

        // reset stream position
        reader.seek(SeekFrom::Start(initial_pos))?;

        Ok(peaks)
    }
}

/// A JCAMP-DX peak assignment, i.e., one item in PEAK ASSIGNMENTS.
#[derive(Debug, PartialEq)]
pub struct PeakAssignment {
    /// Peak position.
    pub x: f64,
    // standard is ambiguous whether this is optional
    /// Intensity.
    pub y: Option<f64>,
    /// Multiplicity.
    ///
    /// S, D, Т, Q for singlets, douЬlets, triplets, or quadruplets,
    /// М for multiple, and U for unassigned. Used only for NMR.
    pub m: Option<String>,
    /// Width.
    pub w: Option<f64>,
    /// The peak assignment string.
    pub a: String,
}

/// A JCAMP-DX NTUPLES record.
#[derive(Debug, PartialEq)]
pub struct NTuples<T: SeekBufRead> {
    /// The data form of the NTUPLES record (value of the
    /// first line of the LDR), e.g., "NMR FID" or "MASS SPECTRUM".
    pub data_form: String,
    /// The LDRs in this record excluding PAGEs.
    pub ldrs: Vec<StringLdr>,
    /// The page attributes parsed from the LDRs.
    pub attributes: Vec<NTuplesAttributes>,
    /// The NTUPLES PAGE LDRs in this record.
    pub pages: Vec<Page<T>>,
}

impl<T: SeekBufRead> NTuples<T> {
    const LABEL: &'static str = "NTUPLES";
    const STANDARD_ATTR_NAMES: [&'static str; 11] = [
        "VARNAME", "SYMBOL", "VARTYPE", "VARFORM", "VARDIM", "UNITS", "FIRST", "LAST", "MIN",
        "MAX", "FACTOR",
    ];

    fn new(
        label: &str,
        data_form: &str,
        block_ldrs: &[StringLdr],
        next_line: Option<String>,
        reader_ref: Rc<RefCell<T>>,
    ) -> Result<(Self, Option<String>), JdxError> {
        validate_input(label, None, Self::LABEL, None)?;
        let (ldrs, attributes, pages, next_line) =
            Self::parse(block_ldrs, data_form, next_line, reader_ref)?;
        Ok((
            Self {
                data_form: data_form.trim().to_owned(),
                ldrs,
                attributes,
                pages,
            },
            next_line,
        ))
    }

    fn parse(
        block_ldrs: &[StringLdr],
        data_form: &str,
        next_line: Option<String>,
        reader_ref: Rc<RefCell<T>>,
    ) -> Result<
        (
            Vec<StringLdr>,
            Vec<NTuplesAttributes>,
            Vec<Page<T>>,
            Option<String>,
        ),
        JdxError,
    > {
        let mut buf = vec![];
        let mut reader = reader_ref.borrow_mut();
        // skip potential comment lines
        let next_line = reader.read_line_iso_8859_1(&mut buf)?;
        let next_line = skip_pure_comments(next_line, true, &mut *reader, &mut buf)?;

        let mut pages = Vec::<Page<T>>::new();
        // parse PAGE parameters
        let (ldrs, attributes, mut next_line) =
            Self::parse_attributes(data_form, next_line, &mut reader, &mut buf)?;

        while let Some(line) = next_line.as_ref() {
            if !is_ldr_start(line) {
                break;
            }
            let (label, page_var) = parse_ldr_start(line)?;
            let (page_var, _comment) = strip_line_comment(&page_var, true, false);

            if label == "ENDNTUPLES" {
                // ##END NTUPLES is described as optional in JCAMP6_2b Draft
                // but is required for indicating the NTUPLES end

                // skip ##END NTUPLES
                next_line = reader.read_line_iso_8859_1(&mut buf)?;
                return Ok((ldrs, attributes, pages, next_line));
            }
            if label != "PAGE" {
                return Err(JdxError::new(&format!(
                    "Unexpected content found in NTUPLES record: {}",
                    line
                )));
            }
            next_line = reader.read_line_iso_8859_1(&mut buf)?;

            drop(reader);
            let (page, next) = Page::new(
                &label,
                page_var,
                &attributes,
                block_ldrs,
                next_line,
                Rc::clone(&reader_ref),
            )?;
            pages.push(page);
            next_line = next;
            reader = reader_ref.borrow_mut();
        }
        if next_line.is_none() {
            return Err(JdxError::new(&format!(
                "Unexpected end of NTUPLES record: {}",
                data_form
            )));
        }

        Ok((ldrs, attributes, pages, next_line))
    }

    fn parse_attributes(
        data_form: &str,
        next_line: Option<String>,
        reader: &mut T,
        buf: &mut Vec<u8>,
    ) -> Result<(Vec<StringLdr>, Vec<NTuplesAttributes>, Option<String>), JdxError> {
        let (ldrs, next_line) = Self::read_ldrs(next_line, reader, buf)?;
        let mut attr_map = Self::split_values(&ldrs)?;
        let mut standard_attr_map = Self::extract_standard_attributes(&mut attr_map);

        let attr_names_opt = standard_attr_map.get_mut("VARNAME");
        if attr_names_opt.is_none() {
            // VARNAMEs are required by the spec
            return Err(JdxError::new(&format!(
                "No \"VAR_NAME\" LDR found in NTUPLES: {}",
                data_form
            )));
        }
        let attr_names = attr_names_opt.unwrap();
        if let Some(last_var_name) = attr_names.last() {
            // check if last VAR_NAME is blank, i.e., there is a trailing comma
            // if so, remove, thus ignore column in subsequent processing
            // required to sucessfully process test data set
            if last_var_name.trim().is_empty() {
                attr_names.pop();
            }
        }

        let mut output = vec![];
        for i in 0..attr_names.len() {
            let ntv = Self::map(&standard_attr_map, &attr_map, i)?;
            output.push(ntv);
        }
        return Ok((ldrs, output, next_line));
    }

    fn read_ldrs(
        mut next_line: Option<String>,
        reader: &mut T,
        buf: &mut Vec<u8>,
    ) -> Result<(Vec<StringLdr>, Option<String>), JdxError> {
        let mut output = vec![];
        while let Some(line) = &next_line {
            let (title, value) = parse_ldr_start(line)?;
            if title == "PAGE" || title == "ENDNTUPLES" || title == "END" {
                // all NTUPLES LDRs read
                break;
            }
            let (value, next) = parse_string_value(&value, reader, buf)?;
            output.push(StringLdr::new(title, value));
            next_line = next;
        }

        Ok((output, next_line))
    }

    fn split_values(ldrs: &[StringLdr]) -> Result<HashMap<String, Vec<String>>, JdxError> {
        let mut output = HashMap::new();
        for ldr in ldrs {
            let (value_string, _comment) = strip_line_comment(&ldr.value, true, false);
            let values: Vec<String> = value_string
                .split(",")
                .map(|v| v.trim().to_owned())
                .collect();
            let old = output.insert(ldr.label.clone(), values);
            if old.is_some() {
                return Err(JdxError::new(&format!(
                    "Duplicate LDR found in NTUPLE: {}",
                    &ldr.label
                )));
            }
        }

        Ok(output)
    }

    fn extract_standard_attributes(
        attributes: &mut HashMap<String, Vec<String>>,
    ) -> HashMap<String, Vec<String>> {
        let mut standard_attrs = HashMap::new();

        let keys: Vec<String> = attributes.keys().map(|k| k.to_owned()).collect();
        // remove standard attributes
        for key in keys {
            let is_standard_attr = Self::STANDARD_ATTR_NAMES.contains(&key.as_str());
            if is_standard_attr {
                let value_opt = attributes.remove(&key);
                if let Some(value) = value_opt {
                    standard_attrs.insert(key, value);
                }
            }
        }

        standard_attrs
    }

    fn map(
        standard_attributes: &HashMap<String, Vec<String>>,
        additional_attributes: &HashMap<String, Vec<String>>,
        value_column_index: usize,
    ) -> Result<NTuplesAttributes, JdxError> {
        let var_name =
            Self::parse_attribute::<String>("VARNAME", value_column_index, standard_attributes)?;
        if var_name.is_none() {
            // VARNAMEs are required by the spec
            return Err(JdxError::new(&format!(
                "VAR_NAME missing in NTUPLES column: {}",
                value_column_index
            )));
        }
        let var_name = var_name.unwrap();
        let symbol =
            Self::parse_attribute::<String>("SYMBOL", value_column_index, standard_attributes)?;
        if symbol.is_none() {
            return Err(JdxError::new(&format!(
                "SYMBOL missing in NTUPLES column: {}",
                value_column_index
            )));
        }
        let symbol = symbol.unwrap();
        let var_type =
            Self::parse_attribute::<String>("VARTYPE", value_column_index, standard_attributes)?;
        let var_form =
            Self::parse_attribute::<String>("VARFORM", value_column_index, standard_attributes)?;
        let var_dim =
            Self::parse_attribute::<u64>("VARDIM", value_column_index, standard_attributes)?;
        let units =
            Self::parse_attribute::<String>("UNITS", value_column_index, standard_attributes)?;
        let first = Self::parse_attribute::<f64>("FIRST", value_column_index, standard_attributes)?;
        let last = Self::parse_attribute::<f64>("LAST", value_column_index, standard_attributes)?;
        let min = Self::parse_attribute::<f64>("MIN", value_column_index, standard_attributes)?;
        let max = Self::parse_attribute::<f64>("MAX", value_column_index, standard_attributes)?;
        let factor =
            Self::parse_attribute::<f64>("FACTOR", value_column_index, standard_attributes)?;

        let mut application_attributes = Vec::<StringLdr>::new();
        for (key, values) in additional_attributes {
            let value_opt = values.get(value_column_index);
            if let Some(value) = value_opt {
                application_attributes.push(StringLdr::new(key, value));
            }
        }

        Ok(NTuplesAttributes {
            var_name,
            symbol,
            var_type,
            var_form,
            var_dim,
            units,
            first,
            last,
            min,
            max,
            factor,
            application_attributes,
        })
    }

    pub fn parse_attribute<P: FromStr>(
        key: &str,
        index: usize,
        attributes: &HashMap<String, Vec<String>>,
    ) -> Result<Option<P>, JdxError> {
        let value_opt = attributes
            .get(key)
            .and_then(|vec| vec.get(index))
            .map(|v| v.trim())
            .filter(|v| !v.is_empty());
        if let Some(value) = value_opt {
            let parsed_value = value.parse::<P>().map_err(|_e| {
                JdxError::new(&format!(
                    "Error parsing NTUPLES. Illegal value for \"{}\": {}",
                    key, value
                ))
            })?;
            return Ok(Some(parsed_value));
        }
        Ok(None)
    }
}

/// A JCAMP-DX NTUPLES PAGE record.
#[derive(Debug, PartialEq)]
pub struct Page<T: SeekBufRead> {
    /// The page variables of the PAGE record (value of
    /// the first line of the LDR), e.g., "N=1" or "X=2.2, Y=3.3".
    pub page_variables: String,
    /// The LDRs contained by the PAGE, e.g.
    /// "NPOINTS", not including "DATA TABLE".
    pub page_ldrs: Vec<StringLdr>,
    /// The DATA TABLE.
    pub data_table: Option<DataTable<T>>,
}

const PAGE_VARS_REGEX_PATTERN: &str = r"(\(.*\))(?:\s*,\s*)?(.*)";
lazy_static! {
    static ref PAGE_VARS_REGEX: regex::Regex = regex::Regex::new(PAGE_VARS_REGEX_PATTERN).unwrap();
}

impl<T: SeekBufRead> Page<T> {
    const LABEL: &'static str = "PAGE";

    fn new(
        label: &str,
        page_var: &str,
        attributes: &[NTuplesAttributes],
        block_ldrs: &[StringLdr],
        next_line: Option<String>,
        reader_ref: Rc<RefCell<T>>,
    ) -> Result<(Self, Option<String>), JdxError> {
        validate_input(label, None, Self::LABEL, None)?;
        Self::parse(page_var, attributes, block_ldrs, next_line, reader_ref)
    }

    fn parse(
        page_var: &str,
        attributes: &[NTuplesAttributes],
        block_ldrs: &[StringLdr],
        next_line: Option<String>,
        reader_ref: Rc<RefCell<T>>,
    ) -> Result<(Self, Option<String>), JdxError> {
        let mut buf = vec![];
        let mut reader = reader_ref.borrow_mut();

        // skip potential comment lines
        let next_line = skip_pure_comments(next_line, false, &mut *reader, &mut buf)?;
        let (page_ldrs, next_line) = Self::parse_page_ldrs(next_line, &mut reader, &mut buf)?;
        if next_line.is_none() || !is_ldr_start(next_line.as_ref().unwrap()) {
            return Err(JdxError::new(&format!(
                "Unexpected content found while parsing NTUPLES PAGE: {}",
                next_line.unwrap_or("<end of file>".to_owned())
            )));
        }
        let (label, value) = parse_ldr_start(next_line.as_ref().unwrap())?;
        if ["PAGE", "ENDNTUPLES", "END"].contains(&label.as_str()) {
            // end of page, page is empty
            // todo: read next_line?
            drop(reader);
            return Ok((
                Page {
                    page_variables: page_var.to_owned(),
                    page_ldrs,
                    data_table: None,
                },
                next_line,
            ));
        }
        if label != "DATATABLE" {
            return Err(JdxError::new(&format!(
                "Unexpected content found while parsing NTUPLES PAGE: {}",
                next_line.unwrap()
            )));
        }
        let (data_table_var_list, plot_desc) = Self::parse_data_table_vars(&value)?;
        drop(reader);
        let (data_table, next_line) = DataTable::new(
            &label,
            &data_table_var_list,
            plot_desc.as_deref(),
            attributes,
            block_ldrs,
            &page_ldrs,
            next_line,
            reader_ref,
        )?;

        return Ok((
            Page {
                page_variables: page_var.to_owned(),
                page_ldrs,
                data_table: Some(data_table),
            },
            next_line,
        ));
    }

    fn parse_page_ldrs(
        mut next_line: Option<String>,
        reader: &mut T,
        buf: &mut Vec<u8>,
    ) -> Result<(Vec<StringLdr>, Option<String>), JdxError> {
        let mut page_ldrs = Vec::<StringLdr>::new();
        while let Some(line) = &next_line {
            let (label, mut value) = parse_ldr_start(line)?;
            if ["PAGE", "ENDNTUPLES", "END", "DATATABLE"].contains(&label.as_str()) {
                // end of page or start of DATA TABLE
                break;
            }
            // LDR is a regular LDR
            (value, next_line) = parse_string_value(&value, reader, buf)?;
            page_ldrs.push(StringLdr::new(label, value));
        }

        Ok((page_ldrs, next_line))
    }

    fn parse_data_table_vars(raw_page_vars: &str) -> Result<(String, Option<String>), JdxError> {
        let raw_page_vars_trimmed = strip_line_comment(raw_page_vars, true, false).0;
        if raw_page_vars_trimmed.is_empty() {
            return Err(JdxError::new(&format!(
                "Missing variable list in DATA TABLE: {}",
                raw_page_vars
            )));
        }

        let caps_opt = PAGE_VARS_REGEX.captures(raw_page_vars);
        let caps = caps_opt.ok_or(JdxError::new(&format!(
            "Unexpected content found at DATA TABLE start: {}",
            raw_page_vars
        )))?;

        let var_list_opt = caps.get(1);
        let plot_desc_opt = caps.get(2);

        if var_list_opt.is_none() {
            return Err(JdxError::new(&format!(
                "Missing variable list in DATA TABLE: {}",
                raw_page_vars
            )));
        }

        let var_list = var_list_opt.unwrap().as_str().trim().to_owned();
        let plot_desc =
            plot_desc_opt.and_then(|m| match strip_line_comment(m.as_str(), true, false).0 {
                se if se.is_empty() => None,
                sne => Some(sne.to_owned()),
            });

        Ok((var_list, plot_desc))
    }
}

/// A JCAMP-DX NTUPLES DATA TABLE record.
#[derive(Debug, PartialEq)]
pub struct DataTable<T: SeekBufRead> {
    /// The plot descriptor of the data table, e.g., "XYDATA" for
    /// "(X++(R..R)), XYDATA".
    pub plot_descriptor: Option<String>,
    /// The attributes for the DATA TABLE.
    /// The relevant parameters merged from LDRs of BLOCK,
    /// NTUPLES, and PAGE for the DATA TABLE.
    /// The fisrt and second tuple items hold attributes for x and y respectively.
    pub attributes: (NTuplesAttributes, NTuplesAttributes),
    /// The record's variable list.
    pub variable_list: String,

    reader_ref: Rc<RefCell<T>>,
    address: u64,
}

impl<T: SeekBufRead> DataTable<T> {
    const LABEL: &'static str = "DATATABLE";
    const VARIABLE_LISTS: [&'static str; 9] = [
        "(X++(Y..Y))",
        "(X++(R..R))",
        "(X++(I..I))",
        "(T2++(R..R))",
        "(T2++(I..I))",
        "(F2++(Y..Y))",
        "(XY..XY)",
        "(XR..XR)",
        "(XI..XI)",
    ];
    const PLOT_DESCRIPTORS: [&'static str; 4] = ["PROFILE", "XYDATA", "PEAKS", "CONTOUR"];
    const X_SYMBOLS: [&'static str; 3] = ["X", "T2", "F2"];
    const Y_SYMBOLS: [&'static str; 3] = ["Y", "R", "I"];

    fn new(
        label: &str,
        var_list: &str,
        plot_desc: Option<&str>,
        attributes: &[NTuplesAttributes],
        block_ldrs: &[StringLdr],
        page_ldrs: &[StringLdr],
        next_line: Option<String>,
        reader_ref: Rc<RefCell<T>>,
    ) -> Result<(Self, Option<String>), JdxError> {
        // validate label and variable list
        validate_input(
            label,
            Some(var_list),
            Self::LABEL,
            Some(&Self::VARIABLE_LISTS),
        )?;
        // validate plot descriptor if present
        if plot_desc.is_some() && !Self::PLOT_DESCRIPTORS.contains(&plot_desc.unwrap()) {
            return Err(JdxError::new(&format!(
                "Illegal plot descriptor in NTUPLES PAGE: {}",
                plot_desc.unwrap()
            )));
        }

        Self::parse(
            var_list, plot_desc, attributes, block_ldrs, page_ldrs, next_line, reader_ref,
        )
    }

    fn get_data(&self) -> Result<Vec<(f64, f64)>, JdxError> {
        let mut reader = self.reader_ref.borrow_mut();

        if ["(XY..XY)", "(XR..XR)", "(XI..XI)"].contains(&self.variable_list.as_str()) {
            let x_factor = self.attributes.0.factor.unwrap_or(1.0);
            let y_factor = self.attributes.1.factor.unwrap_or(1.0);
            let n_points = self.attributes.1.var_dim;

            return parse_xyxy_data(
                Self::LABEL,
                x_factor,
                y_factor,
                n_points,
                self.address,
                &mut *reader,
            );
        }

        let first_x = self.attributes.0.first.ok_or(JdxError::new(
            "Required attribute missing for NTUPLES DATA TABLE: FIRSTX",
        ))?;
        let last_x = self.attributes.0.last.ok_or(JdxError::new(
            "Required attribute missing for NTUPLES DATA TABLE: LASTX",
        ))?;
        let n_points = self.attributes.1.var_dim.ok_or(JdxError::new(
            "Required attribute missing for NTUPLES DATA TABLE: VAR_DIM",
        ))?;
        let y_factor = self.attributes.1.factor.unwrap_or(1.0);

        parse_xppyy_data(
            Self::LABEL,
            first_x,
            last_x,
            y_factor,
            n_points,
            self.address,
            &mut *reader,
        )
    }

    fn parse(
        var_list: &str,
        plot_desc: Option<&str>,
        attributes: &[NTuplesAttributes],
        block_ldrs: &[StringLdr],
        page_ldrs: &[StringLdr],
        next_line: Option<String>,
        reader_ref: Rc<RefCell<T>>,
    ) -> Result<(Self, Option<String>), JdxError> {
        // todo: turn var_list into enum and match OR
        // use regexes for determining the var names
        // let s = r"^\((.+)\+\+\((.+)\.\.(.+)\)\)$";
        // let s = r"^\((.+)(.)\.\.(.+)(.)\)$"
        let (x_col_index, y_col_index) = match var_list {
            "(X++(Y..Y))" | "(XY..XY)" => (
                Self::find_ntuples_index("X", attributes)?,
                Self::find_ntuples_index("Y", attributes)?,
            ),
            "(X++(R..R))" | "(XR..XR)" => (
                Self::find_ntuples_index("X", attributes)?,
                Self::find_ntuples_index("R", attributes)?,
            ),
            "(X++(I..I))" | "(XI..XI)" => (
                Self::find_ntuples_index("X", attributes)?,
                Self::find_ntuples_index("I", attributes)?,
            ),
            "(T2++(R..R))" => (
                Self::find_ntuples_index("T2", attributes)?,
                Self::find_ntuples_index("R", attributes)?,
            ),
            "(T2++(I..I))" => (
                Self::find_ntuples_index("T2", attributes)?,
                Self::find_ntuples_index("I", attributes)?,
            ),
            "(F2++(Y..Y))" => (
                Self::find_ntuples_index("F2", attributes)?,
                Self::find_ntuples_index("Y", attributes)?,
            ),
            _ => {
                // should never happen
                return Err(JdxError::new(&format!(
                    "Unsupported variabe list in DATA TABLE: {}",
                    var_list
                )));
            }
        };

        let x_ntuples_attrs = &attributes[x_col_index];
        let y_ntuples_attrs = &attributes[y_col_index];

        let mut merged_x_vars = Self::merge_vars(x_ntuples_attrs, block_ldrs, page_ldrs)?;
        let mut merged_y_vars = Self::merge_vars(y_ntuples_attrs, block_ldrs, page_ldrs)?;

        // special treatment for "FIRST" page LDR if present
        // this is described in the README for the JCAMP-DX nD-NMR test file round
        // robin
        // todo: can this be combined with merge_vars?
        Self::merge_page_first_ldr(&mut merged_x_vars, page_ldrs, x_col_index)?;
        Self::merge_page_first_ldr(&mut merged_y_vars, page_ldrs, y_col_index)?;

        let mut reader = reader_ref.borrow_mut();
        let address = reader.stream_position()?;
        let next_line = skip_to_next_ldr(next_line, true, &mut *reader, &mut vec![])?;
        drop(reader);

        Ok((
            Self {
                plot_descriptor: plot_desc.map(|s| s.to_owned()),
                attributes: (merged_x_vars, merged_y_vars),
                variable_list: var_list.to_owned(),
                reader_ref,
                address,
            },
            next_line,
        ))
    }

    fn find_ntuples_index<'a>(
        symbol: &str,
        attributes: &'a [NTuplesAttributes],
    ) -> Result<usize, JdxError> {
        let index_opt = attributes.iter().position(|attr| attr.symbol == symbol);
        match index_opt {
            Some(idx) => Ok(idx),
            None => Err(JdxError::new(&format!(
                "Could not find NTUPLES parameters for SYMBOL: {}",
                symbol
            ))),
        }
    }

    fn merge_vars(
        ntuples_vars: &NTuplesAttributes,
        block_ldrs: &[StringLdr],
        page_ldrs: &[StringLdr],
    ) -> Result<NTuplesAttributes, JdxError> {
        let mut output_vars = ntuples_vars.clone();
        // todo: why clear?
        output_vars.application_attributes.clear();

        if Self::X_SYMBOLS.contains(&ntuples_vars.symbol.as_str()) {
            // fill in block params for missing NTUPLE attributes
            Self::merge_x_ldrs(&mut output_vars, block_ldrs, false)?;
            // replace with page LDR values if available
            Self::merge_x_ldrs(&mut output_vars, page_ldrs, true)?;
        } else if Self::Y_SYMBOLS.contains(&ntuples_vars.symbol.as_str()) {
            // Also check for other symbols but Y? Does not seem relevant for NMR
            // and MS.
            // fill in block params for missing NTUPLE attributes
            Self::merge_y_ldrs(&mut output_vars, block_ldrs, false)?;
            // replace with page LDR values if available
            Self::merge_y_ldrs(&mut output_vars, page_ldrs, true)?;
        } else {
            return Err(JdxError::new(&format!(
                "Unexpected symbol found during parsing of PAGE: {}",
                &ntuples_vars.symbol
            )));
        }

        Ok(output_vars)
    }

    fn merge_x_ldrs(
        vars: &mut NTuplesAttributes,
        ldrs: &[StringLdr],
        replace: bool,
    ) -> Result<(), JdxError> {
        for ldr in ldrs {
            match ldr.label.as_str() {
                "XUNITS" if replace || vars.units.is_none() => {
                    vars.units = parse_single_parameter::<String>(ldr)?
                }
                "FIRSTX" if replace || vars.first.is_none() => {
                    vars.first = parse_single_parameter::<f64>(ldr)?
                }
                "LASTX" if replace || vars.last.is_none() => {
                    vars.last = parse_single_parameter::<f64>(ldr)?
                }
                "MINX" if replace || vars.min.is_none() => {
                    vars.min = parse_single_parameter::<f64>(ldr)?
                }
                "MAXX" if replace || vars.max.is_none() => {
                    vars.max = parse_single_parameter::<f64>(ldr)?
                }
                "XFACTOR" if replace || vars.factor.is_none() => {
                    vars.factor = parse_single_parameter::<f64>(ldr)?
                }
                "NPOINTS" if replace || vars.var_dim.is_none() => {
                    vars.var_dim = parse_single_parameter::<u64>(ldr)?
                }
                _ => { /* noop */ }
            }
        }

        Ok(())
    }

    fn merge_y_ldrs(
        vars: &mut NTuplesAttributes,
        ldrs: &[StringLdr],
        replace: bool,
    ) -> Result<(), JdxError> {
        for ldr in ldrs {
            match ldr.label.as_str() {
                "YUNITS" if replace || vars.units.is_none() => {
                    vars.units = parse_single_parameter::<String>(ldr)?
                }
                "FIRSTY" if replace || vars.first.is_none() => {
                    vars.first = parse_single_parameter::<f64>(ldr)?
                }
                "LASTY" if replace || vars.last.is_none() => {
                    vars.last = parse_single_parameter::<f64>(ldr)?
                }
                "MINY" if replace || vars.min.is_none() => {
                    vars.min = parse_single_parameter::<f64>(ldr)?
                }
                "MAXY" if replace || vars.max.is_none() => {
                    vars.max = parse_single_parameter::<f64>(ldr)?
                }
                "YFACTOR" if replace || vars.factor.is_none() => {
                    vars.factor = parse_single_parameter::<f64>(ldr)?
                }
                "NPOINTS" if replace || vars.var_dim.is_none() => {
                    vars.var_dim = parse_single_parameter::<u64>(ldr)?
                }
                _ => { /* noop */ }
            }
        }

        Ok(())
    }

    fn merge_page_first_ldr(
        merged_vars: &mut NTuplesAttributes,
        page_ldrs: &[StringLdr],
        col_index: usize,
    ) -> Result<(), JdxError> {
        for ldr in page_ldrs {
            if "FIRST" == &ldr.label {
                let segments: Vec<&str> = ldr.value.split(",").map(|v| v.trim()).collect();
                if let Some(segment) = segments.get(col_index) {
                    let value = segment.parse::<f64>().map_err(|_e| {
                        JdxError::new(&format!(
                            "Illegal value for \"{}\": {}",
                            &ldr.label, &ldr.value
                        ))
                    })?;
                    merged_vars.first = Some(value);
                }
            }
        }
        Ok(())
    }
}

/// A collection of attributes describing NTUPLES data.
#[derive(Debug, PartialEq, Clone)]
pub struct NTuplesAttributes {
    /// VAR_NAME.
    pub var_name: String,
    /// SYMBOL.
    symbol: String,
    /// VAR_TYPE.
    var_type: Option<String>,
    /// VAR_FORM.
    var_form: Option<String>,
    /// VAR_DIM.
    ///
    /// Option, as it may be blank for mass spectra.
    var_dim: Option<u64>,
    /// UNITS.
    units: Option<String>,
    /// FIRST.
    first: Option<f64>,
    /// LAST.
    last: Option<f64>,
    /// MIN.
    min: Option<f64>,
    /// MAX.
    max: Option<f64>,
    /// FACTOR.
    factor: Option<f64>,
    /// Additional application specific LDRs.
    application_attributes: Vec<StringLdr>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn block_parses_all_string_ldrs() {
        let input = b"##TITLE= Test\r\n\
                           ##JCAMP-DX= 4.24 $$ or later\r\n\
                           ##DATA TYPE= INFRARED SPECTRUM\r\n\
                           $$ random comment #1\r\n\
                           ##ORIGIN=devrosch\r\n\
                           ##OWNER= PUBLIC DOMAIN\n\
                           ##SPECTROMETER/DATA SYSTEM= Some=\r\n\
                           thing\r\n\
                           $$ random comment #2\r\n\
                           ##END=\
                           $$ random comment #3\r\n";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();

        assert_eq!(6, block.ldrs.len());
        assert_eq!(StringLdr::new("TITLE", "Test"), block.ldrs[0]);
        assert_eq!(StringLdr::new("JCAMPDX", "4.24 $$ or later"), block.ldrs[1]);
        assert_eq!(
            StringLdr::new("DATATYPE", "INFRARED SPECTRUM\n$$ random comment #1"),
            block.ldrs[2]
        );
        assert_eq!(StringLdr::new("ORIGIN", "devrosch"), block.ldrs[3]);
        assert_eq!(StringLdr::new("OWNER", "PUBLIC DOMAIN"), block.ldrs[4]);
        assert_eq!(
            StringLdr::new("SPECTROMETERDATASYSTEM", "Something\n$$ random comment #2"),
            block.ldrs[5]
        );
    }

    #[test]
    fn block_get_ldr_finds_ldrs_for_non_normalized_search_strings() {
        let input = b"##TITLE= Test\r\n\
                           ##JCAMP-DX= 4.24\r\n\
                           ##END=\r\n";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();

        assert_eq!(2, block.ldrs.len());
        assert_eq!(StringLdr::new("TITLE", "Test"), block.ldrs[0]);
        assert_eq!(StringLdr::new("JCAMPDX", "4.24"), block.ldrs[1]);

        assert_eq!(
            Some(&StringLdr::new("JCAMPDX", "4.24")),
            block.get_ldr("JCAMPDX")
        );
        assert_eq!(
            Some(&StringLdr::new("JCAMPDX", "4.24")),
            block.get_ldr("JCAMP-DX")
        );
        assert_eq!(
            Some(&StringLdr::new("JCAMPDX", "4.24")),
            block.get_ldr(" J_/CAMP DX-")
        );
    }

    #[test]
    fn block_get_ldr_parses_ldr_comments() {
        let input = b"##TITLE= Test\r\n\
                           ##= a comment\r\n\
                           ##=another comment\r\n\
                           ##END=\r\n";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();

        assert_eq!(2, block.ldr_comments.len());
        assert_eq!("a comment", block.ldr_comments[0]);
        assert_eq!("another comment", block.ldr_comments[1]);
    }

    #[test]
    fn block_parsing_reports_block_comments_separately() {
        let input = b"##TITLE= Test Block\r\n\
                                ##= comment 1\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##= comment 2 line 1\r\n\
                                comment 2 line 2\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();

        assert_eq!(2, block.ldrs.len());
        let ldr_comments = &block.ldr_comments;
        assert_eq!(2, ldr_comments.len());
        assert_eq!(
            &vec![
                "comment 1".to_owned(),
                "comment 2 line 1\ncomment 2 line 2".to_owned()
            ],
            ldr_comments
        );
    }

    #[test]
    fn block_parsing_fails_for_illegal_block_start() {
        let input = b"##ILLEGAL_BLOCK_START= Test Block\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let error = JdxBlock::new("test.jdx", &mut reader).unwrap_err();

        assert!(error.to_string().contains("Malformed block start"));
    }

    #[test]
    fn block_parsing_fails_for_missing_end_ldr() {
        let input = b"##TITLE= Test Block\r\n\
                                ##JCAMP-DX= 5.00\r\n";
        let mut reader = Cursor::new(input);

        let error = JdxBlock::new("test.jdx", &mut reader).unwrap_err();

        assert!(error.to_string().contains("END"));
    }

    #[test]
    fn block_parsing_fails_for_duplicate_generic_ldrs_with_different_content() {
        let input = b"##TITLE= Test Block\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##JCAMP-DX= 5.00\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let error = JdxBlock::new("test.jdx", &mut reader).unwrap_err();

        assert!(error.to_string().contains("Multiple non-identical"));
    }

    #[test]
    fn block_parsing_succeeds_for_duplicate_generic_ldrs_with_same_content() {
        let input = b"##TITLE= Test Block\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();
        assert_eq!(
            Some(&StringLdr::new("JCAMPDX", "4.24")),
            block.get_ldr("JCAMP-DX")
        );
    }

    #[test]
    fn block_parses_xydata() {
        let input = b"##TITLE= Test\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##DATA TYPE= INFRARED SPECTRUM\r\n\
                                $$ random comment #1\r\n\
                                ##ORIGIN= devrosch\r\n\
                                ##OWNER= PUBLIC DOMAIN\r\n\
                                ##SPECTROMETER/DATA SYSTEM= Some=\r\n\
                                thing\r\n\
                                ##XUNITS= 1/CM\r\n\
                                ##YUNITS= ABSORBANCE\r\n\
                                ##XFACTOR= 1.0\r\n\
                                ##YFACTOR= 1.0\r\n\
                                ##FIRSTX= 450\r\n\
                                ##LASTX= 451\r\n\
                                ##NPOINTS= 2\r\n\
                                ##FIRSTY= 10\r\n\
                                ##XYDATA= (X++(Y..Y))\r\n\
                                450.0, 10.0\r\n\
                                451.0, 11.0\r\n\
                                $$ random comment #2\r\n\
                                ##END=\
                                $$ random comment #3\r\n";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();

        // does NOT contain "##END=" even though technically an LDR
        // does NOT contain "##XYDATA=" as it's available through specialized member
        assert_eq!(14, block.ldrs.len());
        let xy_data = &block.xy_data.unwrap();
        assert_eq!(
            vec![(450.0, 10.0), (451.0, 11.0)],
            xy_data.get_data().unwrap()
        );
    }

    #[test]
    fn block_fails_parsing_duplicate_xydata() {
        let input = b"##TITLE= Test\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##DATA TYPE= INFRARED SPECTRUM\r\n\
                                $$ random comment #1\r\n\
                                ##ORIGIN= devrosch\r\n\
                                ##OWNER= PUBLIC DOMAIN\r\n\
                                ##SPECTROMETER/DATA SYSTEM= Some=\r\n\
                                thing\r\n\
                                ##XUNITS= 1/CM\r\n\
                                ##YUNITS= ABSORBANCE\r\n\
                                ##XFACTOR= 1.0\r\n\
                                ##YFACTOR= 1.0\r\n\
                                ##FIRSTX= 450\r\n\
                                ##LASTX= 451\r\n\
                                ##NPOINTS= 2\r\n\
                                ##FIRSTY= 10\r\n\
                                ##XYDATA= (X++(Y..Y))\r\n\
                                450.0, 10.0\r\n\
                                451.0, 11.0\r\n\
                                ##XYDATA= (X++(Y..Y))\r\n\
                                450.0, 10.0\r\n\
                                451.0, 11.0\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let error = JdxBlock::new("test.jdx", &mut reader).unwrap_err();
        assert!(error.to_string().contains("Multiple \"XYDATA\" LDRs"));
    }

    #[test]
    fn block_fails_parsing_xydata_with_missing_required_ldrs() {
        // "##FIRSTX= 450\r\n" // required for XYDATA
        // "##NPOINTS= 2\r\n" // required for XYDATA
        let input = b"##TITLE= Test\r\n\
                                  ##JCAMP-DX= 4.24\r\n\
                                  ##DATA TYPE= INFRARED SPECTRUM\r\n\
                                  ##ORIGIN= devrosch\r\n\
                                  ##OWNER= PUBLIC DOMAIN\r\n\
                                  ##XUNITS= 1/CM\r\n\
                                  ##YUNITS= ABSORBANCE\r\n\
                                  ##XFACTOR= 1.0\r\n\
                                  ##YFACTOR= 1.0\r\n\
                                  ##LASTX= 451\r\n\
                                  ##FIRSTY= 10\r\n\
                                  ##XYDATA= (X++(Y..Y))\r\n\
                                  450.0, 10.0\r\n\
                                  451.0, 11.0\r\n\
                                  ##END=";
        let mut reader = Cursor::new(input);

        let error = JdxBlock::new("test.jdx", &mut reader).unwrap_err();
        assert!(error.to_string().contains("NPOINTS") && error.to_string().contains("FIRSTX"));
    }

    #[test]
    fn block_parses_radata() {
        let input = b"##TITLE= Test\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##DATA TYPE= INFRARED INTERFEROGRAM\r\n\
                                ##ORIGIN= devrosch\r\n\
                                ##OWNER= PUBLIC DOMAIN\r\n\
                                ##RUNITS= MICROMETERS\r\n\
                                ##AUNITS= ARBITRARY UNITS\r\n\
                                ##RFACTOR= 1.0\r\n\
                                ##AFACTOR= 1.0\r\n\
                                ##FIRSTR= 0\r\n\
                                ##LASTR= 1\r\n\
                                ##NPOINTS= 2\r\n\
                                ##FIRSTA= 10\r\n\
                                ##RADATA= (R++(A..A))\r\n\
                                0, 10.0\r\n\
                                1, 11.0\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();

        // does NOT contain "##END=" even though technically an LDR
        // does NOT contain "##XYDATA=" as it's available through specialized member
        assert_eq!(13, block.ldrs.len());
        assert_eq!(
            Some(&StringLdr::new("TITLE", "Test")),
            block.get_ldr("TITLE")
        );

        let ra_data = &block.ra_data.unwrap();
        assert_eq!(vec![(0.0, 10.0), (1.0, 11.0)], ra_data.get_data().unwrap());
    }

    #[test]
    fn block_fails_parsing_duplicate_radata() {
        let input = b"##TITLE= Test\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##DATA TYPE= INFRARED INTERFEROGRAM\r\n\
                                ##ORIGIN= devrosch\r\n\
                                ##OWNER= PUBLIC DOMAIN\r\n\
                                ##RUNITS= MICROMETERS\r\n\
                                ##AUNITS= ARBITRARY UNITS\r\n\
                                ##RFACTOR= 1.0\r\n\
                                ##AFACTOR= 1.0\r\n\
                                ##FIRSTR= 0\r\n\
                                ##LASTR= 1\r\n\
                                ##NPOINTS= 2\r\n\
                                ##FIRSTA= 10\r\n\
                                ##RADATA= (R++(A..A))\r\n\
                                0, 10.0\r\n\
                                1, 11.0\r\n\
                                ##RADATA= (R++(A..A))\r\n\
                                0, 10.0\r\n\
                                1, 11.0\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let error = JdxBlock::new("test.jdx", &mut reader).unwrap_err();
        assert!(error.to_string().contains("Multiple \"RADATA\" LDRs"));
    }

    #[test]
    fn block_parses_xypoints() {
        let input = b"##TITLE= Test\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##DATA TYPE= INFRARED SPECTRUM\r\n\
                                ##XUNITS= 1/CM\r\n\
                                ##YUNITS= ABSORBANCE\r\n\
                                ##XFACTOR= 1.0\r\n\
                                ##YFACTOR= 1.0\r\n\
                                ##FIRSTX= 450\r\n\
                                ##LASTX= 461\r\n\
                                ##NPOINTS= 4\r\n\
                                ##FIRSTY= 10\r\n\
                                ##XYPOINTS= (XY..XY)\r\n\
                                450.0, 10.0; 451.0, 11.0\r\n\
                                460.0, ?; 461.0, 21.0\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();
        assert!(block.xy_points.is_some());
        let xy_points = &block.xy_points.unwrap();
        assert_eq!(4, xy_points.get_data().unwrap().len());
    }

    #[test]
    fn block_fails_parsing_duplicate_xypoints() {
        let input = b"##TITLE= Test\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##DATA TYPE= INFRARED SPECTRUM\r\n\
                                ##XUNITS= 1/CM\r\n\
                                ##YUNITS= ABSORBANCE\r\n\
                                ##XFACTOR= 1.0\r\n\
                                ##YFACTOR= 1.0\r\n\
                                ##FIRSTX= 450\r\n\
                                ##LASTX= 461\r\n\
                                ##NPOINTS= 4\r\n\
                                ##FIRSTY= 10\r\n\
                                ##XYPOINTS= (XY..XY)\r\n\
                                450.0, 10.0; 451.0, 11.0\r\n\
                                460.0, ?; 461.0, 21.0\r\n\
                                ##XYPOINTS= (XY..XY)\r\n\
                                450.0, 10.0; 451.0, 11.0\r\n\
                                460.0, ?; 461.0, 21.0\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let error = JdxBlock::new("test.jdx", &mut reader).unwrap_err();
        assert!(error.to_string().contains("Multiple \"XYPOINTS\" LDRs"));
    }

    #[test]
    fn block_parses_peak_table() {
        let input = b"##TITLE= Test\r\n\
                                 ##JCAMP-DX= 4.24\r\n\
                                 ##PEAK TABLE= (XY..XY)\r\n\
                                 0, 10.0\r\n\
                                 1, 11.0\r\n\
                                 ##END=";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();
        assert!(block.peak_table.is_some());
        let table = &block.peak_table.unwrap();
        assert_eq!(2, table.get_data().unwrap().len());
    }

    #[test]
    fn block_fails_parsing_duplicate_peak_tables() {
        let input = b"##TITLE= Test\r\n\
                                 ##JCAMP-DX= 4.24\r\n\
                                 ##PEAK TABLE= (XY..XY)\r\n\
                                 0, 10.0\r\n\
                                 1, 11.0\r\n\
                                 ##PEAK TABLE= (XY..XY)\r\n\
                                 0, 10.0\r\n\
                                 1, 11.0\r\n\
                                 ##END=";
        let mut reader = Cursor::new(input);

        let error = JdxBlock::new("test.jdx", &mut reader).unwrap_err();
        assert!(error.to_string().contains("Multiple \"PEAKTABLE\" LDRs"));
    }

    #[test]
    fn block_parses_peak_assignments() {
        let input = b"##TITLE= Test\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##PEAK ASSIGNMENTS= (XYA)\r\n\
                                $$ peak width function\r\n\
                                (1.0, 10.0, <peak assignment 1>)\r\n\
                                (2.0, 20.0, <peak assignment 2> )\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();
        assert!(block.peak_assignments.is_some());
        let assignments = &block.peak_assignments.unwrap();
        assert_eq!(2, assignments.get_data().unwrap().len());
    }

    #[test]
    fn block_fails_parsing_duplicate_peak_assignments() {
        let input = b"##TITLE= Test\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##PEAK ASSIGNMENTS= (XYA)\r\n\
                                $$ peak width function\r\n\
                                (1.0, 10.0, <peak assignment 1>)\r\n\
                                (2.0, 20.0, <peak assignment 2> )\r\n\
                                ##PEAK ASSIGNMENTS= (XYA)\r\n\
                                $$ peak width function\r\n\
                                (1.0, 10.0, <peak assignment 1>)\r\n\
                                (2.0, 20.0, <peak assignment 2> )\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let error = JdxBlock::new("test.jdx", &mut reader).unwrap_err();
        assert!(error
            .to_string()
            .contains("Multiple \"PEAKASSIGNMENTS\" LDRs"));
    }

    #[test]
    fn block_parses_link_block() {
        let input = b"##TITLE= Root LINK BLOCK\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##DATA TYPE= LINK\r\n\
                                ##BLOCKS= 3\r\n\
                                ##TITLE= Data XYDATA (PAC) Block\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##DATA TYPE= INFRARED SPECTRUM\r\n\
                                ##XUNITS= 1/CM\r\n\
                                ##YUNITS= ABSORBANCE\r\n\
                                ##XFACTOR= 1.0\r\n\
                                ##YFACTOR= 1.0\r\n\
                                ##FIRSTX= 450\r\n\
                                ##LASTX= 451\r\n\
                                ##NPOINTS= 2\r\n\
                                ##FIRSTY= 10\r\n\
                                ##XYDATA= (X++(Y..Y))\r\n\
                                +450+10\r\n\
                                +451+11\r\n\
                                ##END=\r\n\
                                ##TITLE= Data RADATA (PAC) Block\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##DATA TYPE= INFRARED INTERFEROGRAM\r\n\
                                ##RUNITS= MICROMETERS\r\n\
                                ##AUNITS= ARBITRARY UNITS\r\n\
                                ##FIRSTR= 0\r\n\
                                ##LASTR= 2\r\n\
                                ##RFACTOR= 1.0\r\n\
                                ##AFACTOR= 1.0\r\n\
                                ##NPOINTS= 3\r\n\
                                ##RADATA= (R++(A..A))\r\n\
                                +0+10\r\n\
                                +1+11\r\n\
                                +2+12\r\n\
                                ##END=\r\n\
                                $$ potentially problematic comment\r\n\
                                ##END=\r\n";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();

        assert_eq!(4, block.ldrs.len());
        assert_eq!(
            Some(&StringLdr::new("TITLE", "Root LINK BLOCK")),
            block.get_ldr("TITLE")
        );
        assert!(block.xy_data.is_none());
        assert!(block.ra_data.is_none());
        assert!(block.xy_points.is_none());
        assert!(block.peak_table.is_none());
        assert!(block.peak_assignments.is_none());
        assert_eq!(2, block.blocks.len())
    }

    #[test]
    fn block_parses_nested_blocks() {
        let input = b"##TITLE= Test Link Block\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##DATA TYPE= LINK\r\n\
                                ##BLOCKS= 1\r\n\
                                ##TITLE= Test Nested Block\r\n\
                                ##JCAMP-DX= 4.24\r\n\
                                ##DATA TYPE= INFRARED SPECTRUM\r\n\
                                ##ORIGIN= devrosch\r\n\
                                ##OWNER= PUBLIC DOMAIN\r\n\
                                ##XUNITS= 1/CM\r\n\
                                ##YUNITS= ABSORBANCE\r\n\
                                ##XFACTOR= 1.0\r\n\
                                ##YFACTOR= 1.0\r\n\
                                ##FIRSTX= 450\r\n\
                                ##LASTX= 451\r\n\
                                ##NPOINTS= 2\r\n\
                                ##FIRSTY= 10\r\n\
                                ##XYPOINTS= (XY..XY)\r\n\
                                450.0, 10.0\r\n\
                                451.0, 11.0\r\n\
                                ##END=\r\n\
                                ##END=";
        let mut reader = Cursor::new(input);

        let block = JdxBlock::new("test.jdx", &mut reader).unwrap();

        assert_eq!(4, block.ldrs.len());
        assert_eq!(
            Some(&StringLdr::new("TITLE", "Test Link Block")),
            block.get_ldr("TITLE")
        );
        assert_eq!(
            Some(&StringLdr::new("DATATYPE", "LINK")),
            block.get_ldr("DATATYPE")
        );

        assert_eq!(1, block.blocks.len());
        let inner_block = &block.blocks[0];
        assert_eq!(
            Some(&StringLdr::new("TITLE", "Test Nested Block")),
            inner_block.get_ldr("TITLE")
        );
    }

    // todo: remaining block tests:
    // - parses block with NTUPLES
    // - parses block with AUDIT TRAIL

    #[test]
    fn xydata_parses_affn_xppyy_data_with_required_parameters_only() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "450.0"),
            StringLdr::new("LASTX", "452.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "3"),
        ];
        let label = "XYDATA";
        let variables = "(X++(Y..Y))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0\r\n\
                                 451.0, 11.0\r\n\
                                 452.0, 12.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_data, next) = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("XYDATA", &xy_data.label);
        assert_eq!("(X++(Y..Y))", &xy_data.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let xy_vec = xy_data.get_data().unwrap();
        assert_eq!(3, xy_vec.len());
        assert_eq!(vec![(450.0, 10.0), (451.0, 11.0), (452.0, 12.0)], xy_vec);

        let params = &xy_data.parameters;
        assert_eq!("1/CM", &params.x_units);
        assert_eq!("ABSORBANCE", &params.y_units);
        assert_eq!(450.0, params.first_x);
        assert_eq!(452.0, params.last_x);
        assert_eq!(1.0, params.x_factor);
        assert_eq!(1.0, params.y_factor);
        assert_eq!(3, params.n_points);
        assert!(params.max_x.is_none());
        assert!(params.min_x.is_none());
        assert!(params.max_y.is_none());
        assert!(params.min_y.is_none());
        assert!(params.delta_x.is_none());
        assert!(params.resolution.is_none());
    }

    #[test]
    fn xydata_parses_affn_xppyy_data_with_all_optional_parameters() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "450.0"),
            StringLdr::new("LASTX", "452.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "3"),
            StringLdr::new("MAXX", "452.0"),
            StringLdr::new("MINX", "450.0"),
            StringLdr::new("MAXY", "12.0"),
            StringLdr::new("MINY", "10.0"),
            StringLdr::new("DELTAX", "1.0"),
            StringLdr::new("RESOLUTION", "2.0"),
        ];
        let label = "XYDATA";
        let variables = "(X++(Y..Y))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0\r\n\
                                 451.0, 11.0\r\n\
                                 452.0, 12.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_data, next) = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("XYDATA", &xy_data.label);
        assert_eq!("(X++(Y..Y))", &xy_data.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let xy_vec = xy_data.get_data().unwrap();
        assert_eq!(3, xy_vec.len());
        assert_eq!(vec![(450.0, 10.0), (451.0, 11.0), (452.0, 12.0)], xy_vec);

        let params = &xy_data.parameters;
        assert_eq!("1/CM", &params.x_units);
        assert_eq!("ABSORBANCE", &params.y_units);
        assert_eq!(450.0, params.first_x);
        assert_eq!(452.0, params.last_x);
        assert_eq!(1.0, params.x_factor);
        assert_eq!(1.0, params.y_factor);
        assert_eq!(3, params.n_points);
        assert_eq!(Some(452.0), params.max_x);
        assert_eq!(Some(450.0), params.min_x);
        assert_eq!(Some(12.0), params.max_y);
        assert_eq!(Some(10.0), params.min_y);
        assert_eq!(Some(1.0), params.delta_x);
        assert_eq!(Some(2.0), params.resolution);
    }

    #[test]
    fn xydata_parses_accepts_blank_values_for_optional_parameters() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "450.0"),
            StringLdr::new("LASTX", "452.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "3"),
            StringLdr::new("MAXX", ""),
            StringLdr::new("MINX", ""),
            StringLdr::new("MAXY", ""),
            StringLdr::new("MINY", ""),
            StringLdr::new("DELTAX", ""),
            StringLdr::new("RESOLUTION", ""),
        ];
        let label = "XYDATA";
        let variables = "(X++(Y..Y))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0\r\n\
                                 451.0, 11.0\r\n\
                                 452.0, 12.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_data, next) = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("XYDATA", &xy_data.label);
        assert_eq!("(X++(Y..Y))", &xy_data.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let xy_vec = xy_data.get_data().unwrap();
        assert_eq!(3, xy_vec.len());
        assert_eq!(vec![(450.0, 10.0), (451.0, 11.0), (452.0, 12.0)], xy_vec);

        let params = &xy_data.parameters;
        assert_eq!("1/CM", &params.x_units);
        assert_eq!("ABSORBANCE", &params.y_units);
        assert_eq!(450.0, params.first_x);
        assert_eq!(452.0, params.last_x);
        assert_eq!(1.0, params.x_factor);
        assert_eq!(1.0, params.y_factor);
        assert_eq!(3, params.n_points);
        assert!(params.max_x.is_none());
        assert!(params.min_x.is_none());
        assert!(params.max_y.is_none());
        assert!(params.min_y.is_none());
        assert!(params.delta_x.is_none());
        assert!(params.resolution.is_none());
    }

    #[test]
    fn xydata_parses_single_data_point_record() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "450.0"),
            StringLdr::new("LASTX", "450.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "1"),
        ];
        let label = "XYDATA";
        let variables = "(X++(Y..Y))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_data, next) = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("XYDATA", &xy_data.label);
        assert_eq!("(X++(Y..Y))", &xy_data.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let xy_vec = xy_data.get_data().unwrap();
        assert_eq!(1, xy_vec.len());
        assert_eq!(vec![(450.0, 10.0)], xy_vec);
    }

    #[test]
    fn xydata_parses_xpprr_data() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "450.0"),
            StringLdr::new("LASTX", "450.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "5.0"),
            StringLdr::new("NPOINTS", "1"),
        ];
        let label = "XYDATA";
        let variables = "(X++(R..R))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_data, next) = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("XYDATA", &xy_data.label);
        assert_eq!("(X++(R..R))", &xy_data.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let xy_vec = xy_data.get_data().unwrap();
        assert_eq!(1, xy_vec.len());
        assert_eq!(vec![(450.0, 50.0)], xy_vec);

        let params = &xy_data.parameters;
        assert_eq!("1/CM", &params.x_units);
        assert_eq!("ABSORBANCE", &params.y_units);
        assert_eq!(450.0, params.first_x);
        assert_eq!(450.0, params.last_x);
        assert_eq!(1.0, params.x_factor);
        assert_eq!(5.0, params.y_factor);
        assert_eq!(1, params.n_points);
        assert!(params.max_x.is_none());
        assert!(params.min_x.is_none());
        assert!(params.max_y.is_none());
        assert!(params.min_y.is_none());
        assert!(params.delta_x.is_none());
        assert!(params.resolution.is_none());
    }

    #[test]
    fn xydata_parses_xppii_data() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "450.0"),
            StringLdr::new("LASTX", "450.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "5.0"),
            StringLdr::new("NPOINTS", "1"),
        ];
        let label = "XYDATA";
        let variables = "(X++(I..I))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_data, next) = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("XYDATA", &xy_data.label);
        assert_eq!("(X++(I..I))", &xy_data.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let xy_vec = xy_data.get_data().unwrap();
        assert_eq!(1, xy_vec.len());
        assert_eq!(vec![(450.0, 50.0)], xy_vec);

        let params = &xy_data.parameters;
        assert_eq!("1/CM", &params.x_units);
        assert_eq!("ABSORBANCE", &params.y_units);
        assert_eq!(450.0, params.first_x);
        assert_eq!(450.0, params.last_x);
        assert_eq!(1.0, params.x_factor);
        assert_eq!(5.0, params.y_factor);
        assert_eq!(1, params.n_points);
        assert!(params.max_x.is_none());
        assert!(params.min_x.is_none());
        assert!(params.max_y.is_none());
        assert!(params.min_y.is_none());
        assert!(params.delta_x.is_none());
        assert!(params.resolution.is_none());
    }

    #[test]
    fn xydata_detects_mismatching_npoints() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "450.0"),
            StringLdr::new("LASTX", "452.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "1"),
        ];
        let label = "XYDATA";
        let variables = "(X++(Y..Y))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0\r\n\
                                 451.0, 11.0\r\n\
                                 452.0, 12.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_data, _next) = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        let error = xy_data.get_data().unwrap_err();
        assert!(error.to_string().contains("Mismatch") && error.to_string().contains("NPOINTS"));
    }

    #[test]
    fn xydata_detects_illegal_variable_list() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "450.0"),
            StringLdr::new("LASTX", "450.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "1"),
        ];
        let label = "XYDATA";
        let variables = "(R++(A..A))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let error = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap_err();
        assert!(error.to_string().contains("Illegal variable list"));
    }

    #[test]
    fn xydata_detects_illegal_stream_position() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "450.0"),
            StringLdr::new("LASTX", "450.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "1"),
        ];
        let label = "NPOINTS";
        let variables = "1";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##XYDATA= (X++(Y..Y))\r\n\
                                 450.0, 10.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let error = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap_err();
        assert!(error.to_string().contains("Illegal label"));
    }

    #[test]
    fn xydata_omits_y_value_check_if_last_digit_in_previous_line_is_not_dif_encoded() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "1.0"),
            StringLdr::new("LASTX", "8.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "8"),
        ];
        let label = "XYDATA";
        let variables = "(X++(Y..Y))";
        let next_line = Some(format!("##{label}= {variables}"));
        // y values: 10 11 12 13  20 21 22 23
        let input = b"1 A0JJA3\r\n\
                                 5 B0JJB3\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_data, _next) = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        let xy_vec = xy_data.get_data().unwrap();
        assert_eq!(
            vec![
                (1.0, 10.0),
                (2.0, 11.0),
                (3.0, 12.0),
                (4.0, 13.0),
                (5.0, 20.0),
                (6.0, 21.0),
                (7.0, 22.0),
                (8.0, 23.0)
            ],
            xy_vec
        );
    }

    #[test]
    fn xydata_parses_zero_data_point_record() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "450.0"),
            StringLdr::new("LASTX", "450.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "0"),
        ];
        let label = "XYDATA";
        let variables = "(X++(Y..Y))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_data, _next) = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        let xy_vec = xy_data.get_data().unwrap();
        assert!(xy_vec.is_empty());
    }

    #[test]
    fn xydata_accepts_xyxy_variable_list_quirk() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "900.0"),
            StringLdr::new("LASTX", "922.0"),
            StringLdr::new("XFACTOR", "2.0"),
            StringLdr::new("YFACTOR", "10.0"),
            StringLdr::new("NPOINTS", "4"),
        ];
        let label = "XYDATA";
        let variables = "(XY..XY)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0; 451.0, 11.0\r\n\
                                 460.0, 20.0; 461.0, 21.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_data, _next) = XyData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("(XY..XY)", &xy_data.variable_list);

        let xy_vec = xy_data.get_data().unwrap();
        assert_eq!(
            vec![
                (900.0, 100.0),
                (902.0, 110.0),
                (920.0, 200.0),
                (922.0, 210.0),
            ],
            xy_vec
        );

        let params = &xy_data.parameters;
        assert_eq!("1/CM", &params.x_units);
        assert_eq!("ABSORBANCE", &params.y_units);
        assert_eq!(900.0, params.first_x);
        assert_eq!(922.0, params.last_x);
        assert_eq!(2.0, params.x_factor);
        assert_eq!(10.0, params.y_factor);
        assert_eq!(4, params.n_points);
        assert!(params.max_x.is_none());
        assert!(params.min_x.is_none());
        assert!(params.max_y.is_none());
        assert!(params.min_y.is_none());
        assert!(params.delta_x.is_none());
        assert!(params.resolution.is_none());
    }

    #[test]
    fn radata_parses_affn_ra_data_with_required_parameters_only() {
        let ldrs = &[
            StringLdr::new("RUNITS", "MICROMETERS"),
            StringLdr::new("AUNITS", "ARBITRARY UNITS"),
            StringLdr::new("FIRSTR", "0"),
            StringLdr::new("LASTR", "2"),
            StringLdr::new("RFACTOR", "1.0"),
            StringLdr::new("AFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "3"),
        ];
        let label = "RADATA";
        let variables = "(R++(A..A))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"0, 10.0\r\n\
                                 1, 11.0\r\n\
                                 2, 12.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (ra_data, next) = RaData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("RADATA", &ra_data.label);
        assert_eq!("(R++(A..A))", &ra_data.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let ra_vec = ra_data.get_data().unwrap();
        assert_eq!(3, ra_vec.len());
        assert_eq!(vec![(0.0, 10.0), (1.0, 11.0), (2.0, 12.0)], ra_vec);

        let params = &ra_data.parameters;
        assert_eq!("MICROMETERS", &params.r_units);
        assert_eq!("ARBITRARY UNITS", &params.a_units);
        assert_eq!(0.0, params.first_r);
        assert_eq!(2.0, params.last_r);
        assert_eq!(1.0, params.r_factor);
        assert_eq!(1.0, params.a_factor);
        assert_eq!(3, params.n_points);
        assert!(params.first_a.is_none());
        assert!(params.max_a.is_none());
        assert!(params.min_a.is_none());
        assert!(params.resolution.is_none());
        assert!(params.delta_r.is_none());
        assert!(params.zdp.is_none());
        assert!(params.alias.is_none());
    }

    #[test]
    fn radata_parses_affn_ra_data_with_all_parameters() {
        let ldrs = &[
            StringLdr::new("RUNITS", "MICROMETERS"),
            StringLdr::new("AUNITS", "ARBITRARY UNITS"),
            StringLdr::new("FIRSTR", "0"),
            StringLdr::new("LASTR", "2"),
            StringLdr::new("RFACTOR", "1.0"),
            StringLdr::new("AFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "3"),
            StringLdr::new("FIRSTA", "10.0"),
            StringLdr::new("MAXA", "12.0"),
            StringLdr::new("MINA", "10.0"),
            StringLdr::new("RESOLUTION", "2.0"),
            StringLdr::new("DELTAR", "1.0"),
            StringLdr::new("ZDP", "1"),
            StringLdr::new("ALIAS", "1/2"),
        ];
        let label = "RADATA";
        let variables = "(R++(A..A))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"0, 10.0\r\n\
                                 1, 11.0\r\n\
                                 2, 12.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (ra_data, next) = RaData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("RADATA", &ra_data.label);
        assert_eq!("(R++(A..A))", &ra_data.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let ra_vec = ra_data.get_data().unwrap();
        assert_eq!(3, ra_vec.len());
        assert_eq!(vec![(0.0, 10.0), (1.0, 11.0), (2.0, 12.0)], ra_vec);

        let params = &ra_data.parameters;
        assert_eq!("MICROMETERS", &params.r_units);
        assert_eq!("ARBITRARY UNITS", &params.a_units);
        assert_eq!(0.0, params.first_r);
        assert_eq!(2.0, params.last_r);
        assert_eq!(1.0, params.r_factor);
        assert_eq!(1.0, params.a_factor);
        assert_eq!(3, params.n_points);
        assert_eq!(Some(10.0), params.first_a);
        assert_eq!(Some(12.0), params.max_a);
        assert_eq!(Some(10.0), params.min_a);
        assert_eq!(Some(2.0), params.resolution);
        assert_eq!(Some(1.0), params.delta_r);
        assert_eq!(Some(1.0), params.zdp);
        assert_eq!(Some("1/2".to_owned()), params.alias);
    }

    #[test]
    fn radata_accepts_blank_values_for_optional_ra_parameters() {
        let ldrs = &[
            StringLdr::new("RUNITS", "MICROMETERS"),
            StringLdr::new("AUNITS", "ARBITRARY UNITS"),
            StringLdr::new("FIRSTR", "0"),
            StringLdr::new("LASTR", "2"),
            StringLdr::new("RFACTOR", "1.0"),
            StringLdr::new("AFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "3"),
            StringLdr::new("FIRSTA", ""),
            StringLdr::new("MAXA", ""),
            StringLdr::new("MINA", ""),
            StringLdr::new("RESOLUTION", ""),
            StringLdr::new("DELTAR", ""),
            StringLdr::new("ZDP", ""),
            StringLdr::new("ALIAS", ""),
        ];
        let label = "RADATA";
        let variables = "(R++(A..A))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"0, 10.0\r\n\
                                 1, 11.0\r\n\
                                 2, 12.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (ra_data, next) = RaData::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("RADATA", &ra_data.label);
        assert_eq!("(R++(A..A))", &ra_data.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let ra_vec = ra_data.get_data().unwrap();
        assert_eq!(3, ra_vec.len());
        assert_eq!(vec![(0.0, 10.0), (1.0, 11.0), (2.0, 12.0)], ra_vec);

        let params = &ra_data.parameters;
        assert_eq!("MICROMETERS", &params.r_units);
        assert_eq!("ARBITRARY UNITS", &params.a_units);
        assert_eq!(0.0, params.first_r);
        assert_eq!(2.0, params.last_r);
        assert_eq!(1.0, params.r_factor);
        assert_eq!(1.0, params.a_factor);
        assert_eq!(3, params.n_points);
        assert!(params.first_a.is_none());
        assert!(params.max_a.is_none());
        assert!(params.min_a.is_none());
        assert!(params.resolution.is_none());
        assert!(params.delta_r.is_none());
        assert!(params.zdp.is_none());
        assert!(params.alias.is_none());
    }

    #[test]
    fn radata_detects_mismatching_variable_list() {
        let ldrs = &[
            StringLdr::new("RUNITS", "MICROMETERS"),
            StringLdr::new("AUNITS", "ARBITRARY UNITS"),
            StringLdr::new("FIRSTR", "0"),
            StringLdr::new("LASTR", "2"),
            StringLdr::new("RFACTOR", "1.0"),
            StringLdr::new("AFACTOR", "1.0"),
            // NPOINTS missing
        ];
        let label = "RADATA";
        let variables = "(R++(A..A))";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"0, 10.0\r\n\
                                 1, 11.0\r\n\
                                 2, 12.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let error = RaData::new(label, variables, ldrs, next_line, reader_ref).unwrap_err();
        assert!(error.to_string().contains("missing") && error.to_string().contains("NPOINTS"));
    }

    #[test]
    fn xypoints_parses_unevenly_spaced_xyxy_data() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "900.0"),
            StringLdr::new("LASTX", "922.0"),
            StringLdr::new("XFACTOR", "2.0"),
            StringLdr::new("YFACTOR", "10.0"),
            StringLdr::new("NPOINTS", "4"),
        ];
        let label = "XYPOINTS";
        let variables = "(XY..XY)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0; 451.0, 11.0\r\n\
                                 460.0, ?; 461.0, 21.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_points, next) =
            XyPoints::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("XYPOINTS", &xy_points.label);
        assert_eq!("(XY..XY)", &xy_points.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let xy_vec = xy_points.get_data().unwrap();
        assert_eq!(4, xy_vec.len());
        assert_eq!((900.0, 100.0), xy_vec[0]);
        assert_eq!((902.0, 110.0), xy_vec[1]);
        assert_eq!(920.0, xy_vec[2].0);
        assert!(xy_vec[2].1.is_nan());
        assert_eq!((922.0, 210.0), xy_vec[3]);

        let params = &xy_points.parameters;
        assert_eq!("1/CM", &params.x_units);
        assert_eq!("ABSORBANCE", &params.y_units);
        assert_eq!(900.0, params.first_x);
        assert_eq!(922.0, params.last_x);
        assert_eq!(2.0, params.x_factor);
        assert_eq!(10.0, params.y_factor);
        assert_eq!(4, params.n_points);
        assert!(params.max_x.is_none());
        assert!(params.min_x.is_none());
        assert!(params.max_y.is_none());
        assert!(params.min_y.is_none());
        assert!(params.delta_x.is_none());
        assert!(params.resolution.is_none());
    }

    #[test]
    fn xypoints_parses_xrxr_data() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "900.0"),
            StringLdr::new("LASTX", "922.0"),
            StringLdr::new("XFACTOR", "2.0"),
            StringLdr::new("YFACTOR", "10.0"),
            StringLdr::new("NPOINTS", "2"),
        ];
        let label = "XYPOINTS";
        let variables = "(XR..XR)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0; 451.0, 11.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_points, next) =
            XyPoints::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("XYPOINTS", &xy_points.label);
        assert_eq!("(XR..XR)", &xy_points.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let xy_vec = xy_points.get_data().unwrap();
        assert_eq!(2, xy_vec.len());
        assert_eq!((900.0, 100.0), xy_vec[0]);
        assert_eq!((902.0, 110.0), xy_vec[1]);
    }

    #[test]
    fn xypoints_parses_xixi_data() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "900.0"),
            StringLdr::new("LASTX", "922.0"),
            StringLdr::new("XFACTOR", "2.0"),
            StringLdr::new("YFACTOR", "10.0"),
            StringLdr::new("NPOINTS", "2"),
        ];
        let label = "XYPOINTS";
        let variables = "(XI..XI)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0; 451.0, 11.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_points, next) =
            XyPoints::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        assert_eq!("XYPOINTS", &xy_points.label);
        assert_eq!("(XI..XI)", &xy_points.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let xy_vec = xy_points.get_data().unwrap();
        assert_eq!(2, xy_vec.len());
        assert_eq!((900.0, 100.0), xy_vec[0]);
        assert_eq!((902.0, 110.0), xy_vec[1]);
    }

    #[test]
    fn xypoints_fails_parsing_question_mark_as_x_value() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "450.0"),
            StringLdr::new("LASTX", "461.0"),
            StringLdr::new("XFACTOR", "1.0"),
            StringLdr::new("YFACTOR", "1.0"),
            StringLdr::new("NPOINTS", "4"),
        ];
        let label = "XYPOINTS";
        let variables = "(XY..XY)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0; ?, 11.0\r\n\
                                 460.0, 20.0; 461.0, 21.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_points, _next) =
            XyPoints::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        let error = xy_points.get_data().unwrap_err();
        assert!(error.to_string().contains("NaN") && error.to_string().contains("x value"));
    }

    #[test]
    fn xypoints_fails_parsing_npoints_mismatch() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "900.0"),
            StringLdr::new("LASTX", "922.0"),
            StringLdr::new("XFACTOR", "2.0"),
            StringLdr::new("YFACTOR", "10.0"),
            StringLdr::new("NPOINTS", "3"),
        ];
        let label = "XYPOINTS";
        let variables = "(XY..XY)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0; 451.0, 11.0\r\n\
                                 460.0, 20.0; 461.0, 21.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_points, _next) =
            XyPoints::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        let error = xy_points.get_data().unwrap_err();
        assert!(error.to_string().contains("NPOINTS") && error.to_string().contains("Mismatch"));
    }

    #[test]
    fn xypoints_fails_parsing_incomplete_xy_pair() {
        let ldrs = &[
            StringLdr::new("XUNITS", "1/CM"),
            StringLdr::new("YUNITS", "ABSORBANCE"),
            StringLdr::new("FIRSTX", "900.0"),
            StringLdr::new("LASTX", "922.0"),
            StringLdr::new("XFACTOR", "2.0"),
            StringLdr::new("YFACTOR", "10.0"),
            StringLdr::new("NPOINTS", "4"),
        ];
        let label = "XYPOINTS";
        let variables = "(XY..XY)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0; 451.0, 11.0\r\n\
                                 460.0, 20.0; 461.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (xy_points, _next) =
            XyPoints::new(label, variables, ldrs, next_line, reader_ref).unwrap();
        let error = xy_points.get_data().unwrap_err();
        assert!(error.to_string().contains("Uneven"));
    }

    #[test]
    fn peak_table_parses_xyxy_peaks() {
        let label = "PEAKTABLE";
        let variables = "(XY..XY)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"$$ peak width kernel line 1\r\n\
                                $$ peak width kernel line 2\r\n\
                                450.0,  10.0\r\n\
                                460.0, 11.0 $$ test comment\r\n\
                                \x20470.0, 12.0E2 480.0, 13.0\r\n\
                                490.0, 14.0;  500.0, 15.0\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (table, next) = PeakTable::new(label, &variables, next_line, reader_ref).unwrap();

        assert_eq!(label, table.label);
        assert_eq!(variables, table.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        let width_function = table.get_width_function().unwrap();
        assert!(width_function.is_some());
        assert_eq!(
            Some(
                "peak width kernel line 1\n\
                 peak width kernel line 2"
                    .to_owned()
            ),
            width_function
        );

        let peaks = table.get_data().unwrap();
        assert_eq!(
            vec![
                Peak {
                    x: 450.0,
                    y: 10.0,
                    m: None,
                    w: None
                },
                Peak {
                    x: 460.0,
                    y: 11.0,
                    m: None,
                    w: None
                },
                Peak {
                    x: 470.0,
                    y: 1200.0,
                    m: None,
                    w: None
                },
                Peak {
                    x: 480.0,
                    y: 13.0,
                    m: None,
                    w: None
                },
                Peak {
                    x: 490.0,
                    y: 14.0,
                    m: None,
                    w: None
                },
                Peak {
                    x: 500.0,
                    y: 15.0,
                    m: None,
                    w: None
                }
            ],
            peaks
        )
    }

    #[test]
    fn peak_table_parses_xywxyw_peaks() {
        let label = "PEAKTABLE";
        let variables = "(XYW..XYW)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0, 1.0\r\n\
                                460.0,\t11.0,\t2.0\r\n\
                                470.0, 12.0, 3.0 480.0, 13.0, 4.0\r\n\
                                490.0, 14.0, 5.0; 500.0, 15.0, 6.0\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (table, next) = PeakTable::new(label, &variables, next_line, reader_ref).unwrap();

        assert_eq!(label, table.label);
        assert_eq!(variables, table.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        assert!(table.get_width_function().unwrap().is_none());

        let peaks = table.get_data().unwrap();
        assert_eq!(
            vec![
                Peak {
                    x: 450.0,
                    y: 10.0,
                    m: None,
                    w: Some(1.0)
                },
                Peak {
                    x: 460.0,
                    y: 11.0,
                    m: None,
                    w: Some(2.0)
                },
                Peak {
                    x: 470.0,
                    y: 12.0,
                    m: None,
                    w: Some(3.0)
                },
                Peak {
                    x: 480.0,
                    y: 13.0,
                    m: None,
                    w: Some(4.0)
                },
                Peak {
                    x: 490.0,
                    y: 14.0,
                    m: None,
                    w: Some(5.0)
                },
                Peak {
                    x: 500.0,
                    y: 15.0,
                    m: None,
                    w: Some(6.0)
                }
            ],
            peaks
        )
    }

    #[test]
    fn peak_table_parses_xymxym_peaks() {
        let label = "PEAKTABLE";
        let variables = "(XYM..XYM)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0, T\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (table, next) = PeakTable::new(label, &variables, next_line, reader_ref).unwrap();

        assert_eq!(label, table.label);
        assert_eq!(variables, table.variable_list);
        assert_eq!(Some("##END=".to_owned()), next);

        assert!(table.get_width_function().unwrap().is_none());

        let peaks = table.get_data().unwrap();
        assert_eq!(
            vec![Peak {
                x: 450.0,
                y: 10.0,
                m: Some("T".to_owned()),
                w: None
            },],
            peaks
        )
    }

    #[test]
    fn peak_table_fails_parsing_xyxy_peaks_with_excess_column() {
        let label = "PEAKTABLE";
        let variables = "(XY..XY)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0, 1.0\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (table, _next) = PeakTable::new(label, &variables, next_line, reader_ref).unwrap();
        let error = table.get_data().unwrap_err();
        assert!(error.to_string().contains("Illegal"));
    }

    #[test]
    fn peak_table_fails_parsing_xywxyw_peaks_with_excess_column() {
        let label = "PEAKTABLE";
        let variables = "(XYW..XYW)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0, 1.0, -1.0\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (table, _next) = PeakTable::new(label, &variables, next_line, reader_ref).unwrap();
        let error = table.get_data().unwrap_err();
        assert!(error.to_string().contains("Illegal"));
    }

    #[test]
    fn peak_table_fails_parsing_xyxy_peaks_with_incomplete_pair() {
        let label = "PEAKTABLE";
        let variables = "(XY..XY)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0\r\n\
                                 460.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (table, _next) = PeakTable::new(label, &variables, next_line, reader_ref).unwrap();
        let error = table.get_data().unwrap_err();
        assert!(error.to_string().contains("Illegal"));
    }

    #[test]
    fn peak_table_parsing_reports_blanks_as_nan() {
        let label = "PEAKTABLE";
        let variables = "(XYW..XYW)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0,, 10.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (table, _next) = PeakTable::new(label, &variables, next_line, reader_ref).unwrap();

        let data = table.get_data().unwrap();
        assert_eq!(1, data.len());
        assert_eq!(450.0, data[0].x);
        assert!(data[0].y.is_nan());
        assert_eq!(None, data[0].m);
        assert_eq!(Some(10.0), data[0].w);
    }

    #[test]
    fn peak_table_fails_parsing_for_illegal_variable_list() {
        let label = "PEAKTABLE";
        let variables = "(XYWABC..XYWABC)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 3.0, 10.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let error = PeakTable::new(label, &variables, next_line, reader_ref).unwrap_err();
        assert!(
            error.to_string().contains("Illegal") && error.to_string().contains("variable list")
        );
    }

    #[test]
    fn peak_table_fails_parsing_xywxyw_peaks_with_incomplete_tuple() {
        let label = "PEAKTABLE";
        let variables = "(XYW..XYW)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"450.0, 10.0\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (table, _next) = PeakTable::new(label, &variables, next_line, reader_ref).unwrap();
        let error = table.get_data().unwrap_err();
        assert!(error.to_string().contains("Illegal"));
    }

    #[test]
    fn peak_table_parses_peak_width_function_and_zero_peaks() {
        let label = "PEAKTABLE";
        let variables = "(XY..XY)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"$$ peak width kernel line 1\r\n\
                                 $$ peak width kernel line 2\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (table, _next) = PeakTable::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = table.get_width_function().unwrap();
        assert_eq!(
            Some(
                "peak width kernel line 1\n\
                 peak width kernel line 2"
                    .to_owned()
            ),
            width_function
        );
    }

    #[test]
    fn peak_assignments_parses_xya() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"$$ peak width function\r\n\
                                (1.0, 10.0, <peak assignment 1>)\r\n\
                                ( 2.0,20.0,<peak assignment 2> )\r\n\
                                (3.0, <peak assignment 3>)\r\n\
                                (4.0, , <peak assignment 4>)\r\n\
                                (5.0,\r\n\
                                50.0\r\n\
                                , <peak\r\n\
                                assignment 5>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(Some("peak width function".to_owned()), width_function);

        let data = assignments.get_data().unwrap();
        assert_eq!(5, data.len());
        assert_eq!(
            PeakAssignment {
                x: 1.0,
                y: Some(10.0),
                m: None,
                w: None,
                a: "peak assignment 1".to_owned(),
            },
            data[0]
        );
        assert_eq!(
            PeakAssignment {
                x: 2.0,
                y: Some(20.0),
                m: None,
                w: None,
                a: "peak assignment 2".to_owned(),
            },
            data[1]
        );
        assert_eq!(
            PeakAssignment {
                x: 3.0,
                y: None,
                m: None,
                w: None,
                a: "peak assignment 3".to_owned(),
            },
            data[2]
        );
        let assignment3 = &data[3];
        assert_eq!(4.0, assignment3.x);
        assert!(assignment3.y.unwrap().is_nan());
        assert_eq!(None, assignment3.m);
        assert_eq!(None, assignment3.w);
        assert_eq!("peak assignment 4".to_owned(), assignment3.a);
        assert_eq!(
            PeakAssignment {
                x: 5.0,
                y: Some(50.0),
                m: None,
                w: None,
                a: "peak assignment 5".to_owned(),
            },
            data[4]
        );
    }

    #[test]
    fn peak_assignments_parses_xywa() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYWA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"$$ peak width function\r\n\
                                (1.0, 10.0, 100.0, <peak assignment 1>)\r\n\
                                ( 2.0,20.0,200.0,<peak assignment 2> )\r\n\
                                (3.0, <peak assignment 3>)\r\n\
                                (4.0, ,, <peak assignment 4>)\r\n\
                                (5.0,\r\n\
                                ,\r\n\
                                500.0,\r\n\
                                <peak\r\n\
                                assignment 5>)\r\n\
                                (6.0, 60.0, , <peak assignment 6>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(Some("peak width function".to_owned()), width_function);

        let data = assignments.get_data().unwrap();
        assert_eq!(6, data.len());
        assert_eq!(
            PeakAssignment {
                x: 1.0,
                y: Some(10.0),
                m: None,
                w: Some(100.0),
                a: "peak assignment 1".to_owned(),
            },
            data[0]
        );
        assert_eq!(
            PeakAssignment {
                x: 2.0,
                y: Some(20.0),
                m: None,
                w: Some(200.0),
                a: "peak assignment 2".to_owned(),
            },
            data[1]
        );
        assert_eq!(
            PeakAssignment {
                x: 3.0,
                y: None,
                m: None,
                w: None,
                a: "peak assignment 3".to_owned(),
            },
            data[2]
        );
        let assignment3 = &data[3];
        assert_eq!(4.0, assignment3.x);
        assert!(assignment3.y.unwrap().is_nan());
        assert_eq!(None, assignment3.m);
        assert!(assignment3.w.unwrap().is_nan());
        assert_eq!("peak assignment 4".to_owned(), assignment3.a);
        let assignment4 = &data[4];
        assert_eq!(5.0, assignment4.x);
        assert!(assignment4.y.unwrap().is_nan());
        assert_eq!(None, assignment4.m);
        assert_eq!(Some(500.0), assignment4.w);
        assert_eq!("peak assignment 5".to_owned(), assignment4.a);
        let assignment5 = &data[5];
        assert_eq!(6.0, assignment5.x);
        assert_eq!(Some(60.0), assignment5.y);
        assert_eq!(None, assignment5.m);
        assert!(assignment5.w.unwrap().is_nan());
        assert_eq!("peak assignment 6".to_owned(), assignment5.a);
    }

    #[test]
    fn peak_assignments_parses_xyma() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYMA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"(1.0, 10.0, D, <peak assignment 1>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let data = assignments.get_data().unwrap();
        assert_eq!(
            vec![PeakAssignment {
                x: 1.0,
                y: Some(10.0),
                m: Some("D".to_owned()),
                w: None,
                a: "peak assignment 1".to_owned(),
            }],
            data
        );
    }

    #[test]
    fn peak_assignments_parses_xymwa() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYMWA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"(1.0, 10.0, D, 100.0, <peak assignment 1>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let data = assignments.get_data().unwrap();
        assert_eq!(
            vec![PeakAssignment {
                x: 1.0,
                y: Some(10.0),
                m: Some("D".to_owned()),
                w: Some(100.0),
                a: "peak assignment 1".to_owned(),
            }],
            data
        );
    }

    #[test]
    fn peak_assignments_fails_parsing_xya_with_excess_column() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"(1.0, 10.0, 100.0, <peak assignment 1>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let error = assignments.get_data().unwrap_err();
        assert!(error.to_string().contains("Illegal"));
    }

    #[test]
    fn peak_assignments_fails_parsing_xywa_with_excess_column() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYWA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"(1.0, 10.0, 100.0, 1000.0, <peak assignment 1>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let error = assignments.get_data().unwrap_err();
        assert!(error.to_string().contains("Illegal"));
    }

    #[test]
    fn peak_assignments_fails_parsing_xywa_with_ambiguous_column() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYWA)";
        let next_line = Some(format!("##{label}= {variables}"));
        // 10.0 could be Y or W
        let input = b"(1.0, 10.0, <peak assignment 1>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let error = assignments.get_data().unwrap_err();
        assert!(error.to_string().contains("Ambiguous"));
    }

    #[test]
    fn peak_assignments_fails_parsing_xymwa_with_ambiguous_column() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYMWA)";
        let next_line = Some(format!("##{label}= {variables}"));
        // 10.0 could be Y or W
        let input = b"(1.0, 10.0, 2.0, <peak assignment 1>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let error = assignments.get_data().unwrap_err();
        assert!(error.to_string().contains("Ambiguous"));
    }

    #[test]
    fn peak_assignments_fails_parsing_xywa_with_missing_opening_parenthesis() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYWA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"1.0, 10.0, 100.0, <peak assignment 1>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let error = assignments.get_data().unwrap_err();
        assert!(error.to_string().contains("Illegal"));
    }

    #[test]
    fn peak_assignments_fails_parsing_xywa_with_missing_closing_parenthesis() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYWA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"(1.0, 10.0, 100.0, <peak assignment 1>\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let error = assignments.get_data().unwrap_err();
        assert!(error.to_string().contains("No closing parenthesis"));
    }

    #[test]
    fn peak_assignments_fails_parsing_xya_with_assignment_missing_opening_angle_bracket() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"(1.0, 10.0, peak assignment 1>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let error = assignments.get_data().unwrap_err();
        assert!(error.to_string().contains("Illegal"));
    }

    #[test]
    fn peak_assignments_fails_parsing_xya_with_assignment_missing_closing_angle_bracket() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"(1.0, 10.0, <peak assignment 1)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let error = assignments.get_data().unwrap_err();
        assert!(error.to_string().contains("Illegal"));
    }

    #[test]
    fn peak_assignments_fails_parsing_xya_with_illegal_separator() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"(1.0 10.0; <peak assignment 1>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let error = assignments.get_data().unwrap_err();
        assert!(error.to_string().contains("Illegal"));
    }

    #[test]
    fn peak_assignments_fails_parsing_illegal_variable_list() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYAUVW)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"1.0, 10.0, <peak assignment 1>)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let error = PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap_err();

        assert!(
            error.to_string().contains("Illegal") && error.to_string().contains("variable list")
        );
    }

    #[test]
    fn peak_assignments_fails_parsing_xya_with_missing_component() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"(1.0)\r\n\
                                ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let error = assignments.get_data().unwrap_err();
        assert!(error.to_string().contains("Illegal"));
    }

    #[test]
    fn peak_assignments_fails_parsing_xya_with_some_missing_closing_parenthesis() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"(1.0, 10.0, <peak assignment 1>)\r\n\
                                 (1.0, 10.0, <peak assignment 1>\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(None, width_function);

        let error = assignments.get_data().unwrap_err();
        assert!(error.to_string().contains("No closing parenthesis"));
    }

    #[test]
    fn peak_assignments_parses_peak_width_function_even_if_zero_peaks() {
        let label = "PEAKASSIGNMENTS";
        let variables = "(XYA)";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"$$ peak width function\r\n\
                                 ##END=";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));

        let (assignments, _next) =
            PeakAssignments::new(label, &variables, next_line, reader_ref).unwrap();

        let width_function = assignments.get_width_function().unwrap();
        assert_eq!(Some("peak width function".to_owned()), width_function);

        let data = assignments.get_data().unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn ntuples_parses_nmr_record() {
        let label = "NTUPLES";
        let variables = "NMR SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,    SPECTRUM/IMAG, PAGE NUMBER\n\
                                   ##SYMBOL=             X,                R,                I,           N\n\
                                   ##VAR_TYPE= INDEPENDENT,        DEPENDENT,        DEPENDENT,        PAGE\n\
                                   ##VAR_FORM=        AFFN,             ASDF,             ASDF,        AFFN\n\
                                   ##VAR_DIM=            4,                4,                4,           2\n\
                                   ##UNITS=             HZ,  ARBITRARY UNITS,  ARBITRARY UNITS,            \n\
                                   ##FIRST=            0.1,             50.0,            300.0,           1\n\
                                   ##LAST=            0.25,            105.0,            410.0,           2\n\
                                   ##MIN=              0.1,             50.0,            300.0,           1\n\
                                   ##MAX=             0.25,            105.0,            410.0,           2\n\
                                   ##FACTOR=           0.1,              5.0,             10.0,           1\n\
                                   ##$CUSTOM_LDR=     VAL1,             VAL2,             VAL3,       VAL4,\n\
                                   ##PAGE= N=1\n\
                                   ##DATA TABLE= (X++(R..R)), XYDATA   $$ Real data points\n\
                                   1.0 +10+11\n\
                                   2.0 +20+21\n\
                                   ##PAGE= N=2\n\
                                   ##DATA TABLE= (X++(I..I)), XYDATA   $$ Imaginary data points\n\
                                   1.0 +30+31\n\
                                   2.0 +40+41\n\
                                   ##END NTUPLES= NMR SPECTRUM\n\
                                   ##END=\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let (ntuples, _next) =
            NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref).unwrap();

        assert_eq!(2, ntuples.pages.len());
        assert_eq!("NMR SPECTRUM", ntuples.data_form);

        assert_eq!(12, ntuples.ldrs.len());
        assert_eq!(
            StringLdr::new(
                "VARNAME",
                "FREQUENCY,    SPECTRUM/REAL,    SPECTRUM/IMAG, PAGE NUMBER"
            ),
            ntuples.ldrs[0]
        );
        assert_eq!(
            StringLdr::new(
                "$CUSTOMLDR",
                "VAL1,             VAL2,             VAL3,       VAL4,"
            ),
            ntuples.ldrs[11]
        );
        assert!(ntuples.ldrs[11].is_user_defined());

        let page_n1 = &ntuples.pages[0];
        assert_eq!("N=1", &page_n1.page_variables);
        assert!(page_n1.page_ldrs.is_empty());
        assert_eq!(4, ntuples.attributes.len());
        let page_attrs0 = &ntuples.attributes[0];
        assert_eq!(1, page_attrs0.application_attributes.len());
        assert_eq!(
            StringLdr::new("$CUSTOMLDR", "VAL1"),
            page_attrs0.application_attributes[0]
        );

        assert!(page_n1.data_table.is_some());
        let page_n1_data_table = page_n1.data_table.as_ref().unwrap();
        assert_eq!("(X++(R..R))", page_n1_data_table.variable_list);
        assert_eq!(
            Some("XYDATA".to_owned()),
            page_n1_data_table.plot_descriptor
        );

        let page_n1_x_attributes = &page_n1_data_table.attributes.0;
        assert_eq!("FREQUENCY", page_n1_x_attributes.var_name);
        assert_eq!("X", page_n1_x_attributes.symbol);
        assert_eq!(
            Some("INDEPENDENT".to_owned()),
            page_n1_x_attributes.var_type
        );
        assert_eq!(Some("AFFN".to_owned()), page_n1_x_attributes.var_form);
        assert_eq!(Some(4), page_n1_x_attributes.var_dim);
        assert_eq!(Some("HZ".to_owned()), page_n1_x_attributes.units);
        assert_eq!(Some(0.1), page_n1_x_attributes.first);
        assert_eq!(Some(0.25), page_n1_x_attributes.last);
        assert_eq!(Some(0.1), page_n1_x_attributes.min);
        assert_eq!(Some(0.25), page_n1_x_attributes.max);
        assert_eq!(Some(0.1), page_n1_x_attributes.factor);

        let page_n1_y_attributes = &page_n1_data_table.attributes.1;
        assert_eq!("SPECTRUM/REAL", page_n1_y_attributes.var_name);
        assert_eq!("R", page_n1_y_attributes.symbol);
        assert_eq!(Some("DEPENDENT".to_owned()), page_n1_y_attributes.var_type);
        assert_eq!(Some("ASDF".to_owned()), page_n1_y_attributes.var_form);
        assert_eq!(Some(4), page_n1_y_attributes.var_dim);
        assert_eq!(
            Some("ARBITRARY UNITS".to_owned()),
            page_n1_y_attributes.units
        );
        assert_eq!(Some(50.0), page_n1_y_attributes.first);
        assert_eq!(Some(105.0), page_n1_y_attributes.last);
        assert_eq!(Some(50.0), page_n1_y_attributes.min);
        assert_eq!(Some(105.0), page_n1_y_attributes.max);
        assert_eq!(Some(5.0), page_n1_y_attributes.factor);

        let page_n1_data = page_n1_data_table.get_data().unwrap();
        assert_eq!(4, page_n1_data.len());
        assert_eq!((0.1, 50.0), page_n1_data[0]);
        assert_eq!((0.25, 105.0), page_n1_data[3]);

        let page_n2 = &ntuples.pages[1];
        assert_eq!("N=2", &page_n2.page_variables);
        assert!(page_n2.page_ldrs.is_empty());

        assert!(page_n2.data_table.is_some());
        let page_n2_data_table = page_n2.data_table.as_ref().unwrap();
        assert_eq!("(X++(I..I))", page_n2_data_table.variable_list);
        assert_eq!(
            Some("XYDATA".to_owned()),
            page_n2_data_table.plot_descriptor
        );

        let page_n2_data = page_n2_data_table.get_data().unwrap();
        assert_eq!(4, page_n2_data.len());
        assert_eq!((0.1, 300.0), page_n2_data[0]);
        assert_eq!((0.25, 410.0), page_n2_data[3]);
    }

    #[test]
    fn ntuples_parses_nmr_fid_record_in_round_robin_format() {
        let label = "NTUPLES";
        let variables = "nD NMR FID";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR NAME= TIME1,         TIME2,           FID/REAL,        FID/IMAG\n\
                                ##SYMBOL=   T1,            T2,              R,               I\n\
                                ##.NUCLEUS=     1H, 1H\n\
                                ##VAR TYPE= INDEPENDENT,   INDEPENDENT,     DEPENDENT,       DEPENDENT\n\
                                ##VAR FORM= AFFN,          ASDF,            ASDF,            ASDF\n\
                                ##VAR DIM=  2, 4, 4, 4\n\
                                ##UNITS=    SECONDS,       SECONDS,         ARBITRARY UNITS, ARBITRARY UNITS\n\
                                ##FIRST=    0.0, 1.0, , $$FIRST for R and I are in PAGEs\n\
                                ##LAST=     0.1, 2.5, ,\n\
                                ##FACTOR=   1.0, 1.0, 1.0, 1.0\n\
                                ##PAGE= T1=0.0\n\
                                ##FIRST=    0, 1.0, 10.0, 30.0\n\
                                ##DATA TABLE= (T2++(R..R)), PROFILE   $$ Real data points\n\
                                1.0 +10+11\n\
                                2.0 +20+21\n\
                                ##PAGE= T1=0.1\n\
                                ##FIRST=    0, 1.0, 10.0, 30.0\n\
                                ##DATA TABLE= (T2++(I..I)), PROFILE   $$ Imaginary data points\n\
                                1.0 +30+31\n\
                                2.0 +40+41\n\
                                ##END NTUPLES= nD NMR FID\n\
                                ##END=\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let (ntuples, _next) =
            NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref).unwrap();

        assert_eq!(10, ntuples.ldrs.len());
        assert_eq!(
            StringLdr::new(
                "VARNAME",
                "TIME1,         TIME2,           FID/REAL,        FID/IMAG"
            ),
            ntuples.ldrs[0]
        );

        assert_eq!(4, ntuples.attributes.len());
        let ntuples_attrs_t1 = &ntuples.attributes[0];
        assert_eq!(1, ntuples_attrs_t1.application_attributes.len());
        assert_eq!(
            StringLdr::new(".NUCLEUS", "1H"),
            ntuples_attrs_t1.application_attributes[0]
        );
        let ntuples_attrs_r = &ntuples.attributes[2];
        assert!(ntuples_attrs_r.application_attributes.is_empty());

        assert_eq!(2, ntuples.pages.len());
        assert_eq!("nD NMR FID", ntuples.data_form);

        let page_t0 = &ntuples.pages[0];
        assert_eq!("T1=0.0", &page_t0.page_variables);
        let page_ldrs0 = &page_t0.page_ldrs;
        assert_eq!(1, page_ldrs0.len());
        assert_eq!(StringLdr::new("FIRST", "0, 1.0, 10.0, 30.0"), page_ldrs0[0]);

        assert!(page_t0.data_table.is_some());
        let page_t0_data_table = page_t0.data_table.as_ref().unwrap();
        assert_eq!("(T2++(R..R))", page_t0_data_table.variable_list);
        assert_eq!(
            Some("PROFILE".to_owned()),
            page_t0_data_table.plot_descriptor
        );

        let page_t0_data_r_attributes = &page_t0_data_table.attributes.1;
        assert_eq!("FID/REAL", page_t0_data_r_attributes.var_name);
        assert_eq!("R", page_t0_data_r_attributes.symbol);
        assert_eq!(
            Some("DEPENDENT".to_owned()),
            page_t0_data_r_attributes.var_type
        );
        assert_eq!(Some("ASDF".to_owned()), page_t0_data_r_attributes.var_form);
        assert_eq!(Some(4), page_t0_data_r_attributes.var_dim);
        assert_eq!(
            Some("ARBITRARY UNITS".to_owned()),
            page_t0_data_r_attributes.units
        );
        assert_eq!(Some(10.0), page_t0_data_r_attributes.first);
        assert_eq!(None, page_t0_data_r_attributes.last);
        assert_eq!(None, page_t0_data_r_attributes.min);
        assert_eq!(None, page_t0_data_r_attributes.max);
        assert_eq!(Some(1.0), page_t0_data_r_attributes.factor);

        let page_t0_data = page_t0_data_table.get_data().unwrap();
        assert_eq!(4, page_t0_data.len());
        assert_eq!((1.0, 10.0), page_t0_data[0]);
        assert_eq!((2.5, 21.0), page_t0_data[3]);
    }

    #[test]
    fn ntuples_parses_nmr_spectrum_record_in_round_robin_format() {
        let label = "NTUPLES";
        let variables = "nD NMR SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR NAME= FREQUENCY1,    FREQUENCY2,      SPECTRUM\n\
                                ##SYMBOL=   F1,            F2,              Y\n\
                                ##.NUCLEUS=     1H, 1H\n\
                                ##VAR TYPE= INDEPENDENT,   INDEPENDENT,     DEPENDENT\n\
                                ##VAR FORM= AFFN,          ASDF,            ASDF\n\
                                ##VAR DIM=  2, 4, 4\n\
                                ##UNITS=    SECONDS,       SECONDS,         ARBITRARY UNITS\n\
                                ##FIRST=    0.0, 1.0\n\
                                ##LAST=     0.0, 2.5\n\
                                ##FACTOR=   1.0, 1.0, 1.0\n\
                                ##PAGE= F1=0.0\n\
                                ##FIRST=    0, 1.0, 10.0\n\
                                ##DATA TABLE= (F2++(Y..Y)), PROFILE\n\
                                1.0 +10+11\n\
                                2.0 +20+21\n\
                                ##END NTUPLES= nD NMR SPECTRUM\n\
                                ##END=\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let (ntuples, _next) =
            NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref).unwrap();

        assert_eq!(10, ntuples.ldrs.len());
        assert_eq!(
            StringLdr::new("VARNAME", "FREQUENCY1,    FREQUENCY2,      SPECTRUM"),
            ntuples.ldrs[0]
        );
        assert_eq!(StringLdr::new(".NUCLEUS", "1H, 1H"), ntuples.ldrs[2]);
        assert!(ntuples.ldrs[2].is_technique_specific());
        assert_eq!(StringLdr::new("FACTOR", "1.0, 1.0, 1.0"), ntuples.ldrs[9]);

        assert_eq!(3, ntuples.attributes.len());
        let ntuples_attrs_t1 = &ntuples.attributes[0];
        assert_eq!(1, ntuples_attrs_t1.application_attributes.len());
        assert_eq!(
            StringLdr::new(".NUCLEUS", "1H"),
            ntuples_attrs_t1.application_attributes[0]
        );
        let ntuples_attrs_r = &ntuples.attributes[2];
        assert!(ntuples_attrs_r.application_attributes.is_empty());

        assert_eq!(1, ntuples.pages.len());
        assert_eq!("nD NMR SPECTRUM", ntuples.data_form);

        let page_t0 = &ntuples.pages[0];
        assert_eq!("F1=0.0", &page_t0.page_variables);
        let page_ldrs0 = &page_t0.page_ldrs;
        assert_eq!(1, page_ldrs0.len());
        assert_eq!(StringLdr::new("FIRST", "0, 1.0, 10.0"), page_ldrs0[0]);

        assert!(page_t0.data_table.is_some());
        let page_f0_data_table = page_t0.data_table.as_ref().unwrap();
        assert_eq!("(F2++(Y..Y))", page_f0_data_table.variable_list);
        assert_eq!(
            Some("PROFILE".to_owned()),
            page_f0_data_table.plot_descriptor
        );

        let page_f0_data_r_attributes = &page_f0_data_table.attributes.1;
        assert_eq!("SPECTRUM", page_f0_data_r_attributes.var_name);
        assert_eq!("Y", page_f0_data_r_attributes.symbol);
        assert_eq!(
            Some("DEPENDENT".to_owned()),
            page_f0_data_r_attributes.var_type
        );
        assert_eq!(Some("ASDF".to_owned()), page_f0_data_r_attributes.var_form);
        assert_eq!(Some(4), page_f0_data_r_attributes.var_dim);
        assert_eq!(
            Some("ARBITRARY UNITS".to_owned()),
            page_f0_data_r_attributes.units
        );
        assert_eq!(Some(10.0), page_f0_data_r_attributes.first);
        assert_eq!(None, page_f0_data_r_attributes.last);
        assert_eq!(None, page_f0_data_r_attributes.min);
        assert_eq!(None, page_f0_data_r_attributes.max);
        assert_eq!(Some(1.0), page_f0_data_r_attributes.factor);

        let page_t0_data = page_f0_data_table.get_data().unwrap();
        assert_eq!(4, page_t0_data.len());
        assert_eq!((1.0, 10.0), page_t0_data[0]);
        assert_eq!((2.5, 21.0), page_t0_data[3]);
    }

    #[test]
    fn ntuples_parses_ms_record() {
        let label = "NTUPLES";
        let variables = "         MASS SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR_NAME=        MASS,          INTENSITY,          RETENTION TIME\n\
                                ##SYMBOL=          X,             Y,                  T\n\
                                ##VAR_TYPE=        INDEPENDENT,   DEPENDENT,          INDEPENDENT\n\
                                ##VAR_FORM=        AFFN,          AFFN,               AFFN\n\
                                ##VAR_DIM=         ,              ,                   3\n\
                                ##UNITS=           M/Z,           RELATIVE ABUNDANCE, SECONDS\n\
                                ##FIRST=           ,              ,                   5\n\
                                ##LAST=            ,              ,                   15\n\
                                ##PAGE=            T = 5\n\
                                ##DATA TABLE=      (XY..XY),      PEAKS\n\
                                100,  50.0;  110,  60.0;  120,  70.0   \n\
                                130,  80.0;  140,  90.0                \n\
                                ##PAGE=            T = 10              \n\
                                ##NPOINTS=         4                   \n\
                                ##DATA TABLE= (XY..XY), PEAKS          \n\
                                200,  55.0;  220,  77.0                \n\
                                230,  88.0;  240,  99.0                \n\
                                ##PAGE=            T = 15              \n\
                                ##DATA TABLE= (XY..XY), PEAKS          \n\
                                300,  55.5;  310,  66.6;  320,  77.7   \n\
                                330,  88.8;  340,  99.9                \n\
                                ##END NTUPLES= MASS SPECTRUM\n\
                                ##END=\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let (ntuples, _next) =
            NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref).unwrap();

        assert_eq!(3, ntuples.pages.len());
        assert_eq!("MASS SPECTRUM", ntuples.data_form);

        let page_t5 = &ntuples.pages[0];
        assert_eq!("T = 5", &page_t5.page_variables);
        assert!(&page_t5.page_ldrs.is_empty());

        assert!(page_t5.data_table.is_some());
        let page_t5_data_table = page_t5.data_table.as_ref().unwrap();
        assert_eq!("(XY..XY)", page_t5_data_table.variable_list);
        assert_eq!(Some("PEAKS".to_owned()), page_t5_data_table.plot_descriptor);

        let page_t5_x_attributes = &page_t5_data_table.attributes.0;
        assert_eq!("MASS", page_t5_x_attributes.var_name);
        assert_eq!("X", page_t5_x_attributes.symbol);
        assert_eq!(
            Some("INDEPENDENT".to_owned()),
            page_t5_x_attributes.var_type
        );
        assert_eq!(Some("AFFN".to_owned()), page_t5_x_attributes.var_form);
        assert!(page_t5_x_attributes.var_dim.is_none());
        assert_eq!(Some("M/Z".to_owned()), page_t5_x_attributes.units);
        assert_eq!(None, page_t5_x_attributes.first);
        assert_eq!(None, page_t5_x_attributes.last);
        assert_eq!(None, page_t5_x_attributes.min);
        assert_eq!(None, page_t5_x_attributes.max);
        assert_eq!(None, page_t5_x_attributes.factor);

        let page_t5_y_attributes = &page_t5_data_table.attributes.1;
        assert_eq!("INTENSITY", page_t5_y_attributes.var_name);
        assert_eq!("Y", page_t5_y_attributes.symbol);
        assert_eq!(Some("DEPENDENT".to_owned()), page_t5_y_attributes.var_type);
        assert_eq!(Some("AFFN".to_owned()), page_t5_y_attributes.var_form);
        assert!(page_t5_y_attributes.var_dim.is_none());
        assert_eq!(
            Some("RELATIVE ABUNDANCE".to_owned()),
            page_t5_y_attributes.units
        );
        assert_eq!(None, page_t5_y_attributes.first);
        assert_eq!(None, page_t5_y_attributes.last);
        assert_eq!(None, page_t5_y_attributes.min);
        assert_eq!(None, page_t5_y_attributes.max);
        assert_eq!(None, page_t5_y_attributes.factor);

        let page_t5_data = page_t5_data_table.get_data().unwrap();
        assert_eq!(5, page_t5_data.len());
        assert_eq!((100.0, 50.0), page_t5_data[0]);
        assert_eq!((140.0, 90.0), page_t5_data[4]);

        let page_t10 = &ntuples.pages[1];
        assert_eq!("T = 10", &page_t10.page_variables);
        assert_eq!(1, page_t10.page_ldrs.len());

        let page_t10_data = page_t10.data_table.as_ref().unwrap().get_data().unwrap();
        assert_eq!(4, page_t10_data.len());
        assert_eq!((200.0, 55.0), page_t10_data[0]);
        assert_eq!((240.0, 99.0), page_t10_data[3]);
    }

    #[test]
    fn ntuples_parses_ms_record_with_trailing_blank_var_name() {
        // strictly, the trailing blank VAR_NAME shoud be interpreted as " " name
        // however, as the JCAMP-DX test data set contains one such file and
        // the expectation is to ignore the blank VARN_NAME, have special
        // treatment for this case
        let label = "NTUPLES";
        let variables = "         MASS SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR_NAME=        MASS,          INTENSITY,          RETENTION TIME, \n\
                                ##SYMBOL=          X,             Y,                  T\n\
                                ##VAR_TYPE=        INDEPENDENT,   DEPENDENT,          INDEPENDENT\n\
                                ##VAR_FORM=        AFFN,          AFFN,               AFFN\n\
                                ##VAR_DIM=         ,              ,                   3\n\
                                ##UNITS=           M/Z,           RELATIVE ABUNDANCE, SECONDS\n\
                                ##FIRST=           ,              ,                   5\n\
                                ##LAST=            ,              ,                   15\n\
                                ##PAGE=            T = 5\n\
                                ##DATA TABLE=      (XY..XY),      PEAKS\n\
                                100,  50.0;  110,  60.0;  120,  70.0   \n\
                                130,  80.0;  140,  90.0                \n\
                                ##PAGE=            T = 10              \n\
                                ##NPOINTS=         4                   \n\
                                ##DATA TABLE= (XY..XY), PEAKS          \n\
                                200,  55.0;  220,  77.0                \n\
                                230,  88.0;  240,  99.0                \n\
                                ##PAGE=            T = 15              \n\
                                ##DATA TABLE= (XY..XY), PEAKS          \n\
                                300,  55.5;  310,  66.6;  320,  77.7   \n\
                                330,  88.8;  340,  99.9                \n\
                                ##END NTUPLES= MASS SPECTRUM\n\
                                ##END=\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let ntuples_result = NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref);

        assert!(ntuples_result.is_ok());
        let ntuples = ntuples_result.unwrap().0;
        assert_eq!(3, ntuples.attributes.len());
    }

    #[test]
    fn ntuples_uses_block_ldrs_to_fill_missing_ntuples_attributes() {
        let label = "NTUPLES";
        let variables = "         MASS SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR_NAME=        MASS,          INTENSITY,          RETENTION TIME\n\
                                ##SYMBOL=          X,             Y,                  T\n\
                                ##VAR_TYPE=        INDEPENDENT,   DEPENDENT,          INDEPENDENT\n\
                                ##VAR_FORM=        AFFN,          AFFN,               AFFN\n\
                                ##PAGE=            T = 5\n\
                                ##DATA TABLE=      (XY..XY)            \n\
                                100,  50.0;  110,  60.0;  120,  70.0   \n\
                                130,  80.0;  140,  90.0                \n\
                                ##END NTUPLES= MASS SPECTRUM\n\
                                ##END=\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = vec![
            StringLdr::new("XUNITS", "XUNITS-TEST"),
            StringLdr::new("FIRSTX", "200.0"),
            StringLdr::new("LASTX", "280.0"),
            StringLdr::new("MINX", "200.0"),
            StringLdr::new("MAXX", "280.0"),
            StringLdr::new("XFACTOR", "2.0"),
            StringLdr::new("YUNITS", "YUNITS-TEST"),
            StringLdr::new("FIRSTY", "150.0"),
            StringLdr::new("LASTY", "270.0"),
            StringLdr::new("MINY", "150.0"),
            StringLdr::new("MAXY", "270.0"),
            StringLdr::new("YFACTOR", "3.0"),
            StringLdr::new("NPOINTS", "5"),
        ];

        let (ntuples, _next) =
            NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref).unwrap();

        assert_eq!(1, ntuples.pages.len());
        assert_eq!("MASS SPECTRUM", ntuples.data_form);

        let page_t5 = &ntuples.pages[0];
        assert_eq!("T = 5", &page_t5.page_variables);
        assert!(&page_t5.page_ldrs.is_empty());

        assert!(page_t5.data_table.is_some());
        let page_t5_data_table = page_t5.data_table.as_ref().unwrap();
        assert_eq!("(XY..XY)", page_t5_data_table.variable_list);
        assert_eq!(None, page_t5_data_table.plot_descriptor);

        let page_t5_x_attributes = &page_t5_data_table.attributes.0;
        assert_eq!("MASS", page_t5_x_attributes.var_name);
        assert_eq!("X", page_t5_x_attributes.symbol);
        assert_eq!(
            Some("INDEPENDENT".to_owned()),
            page_t5_x_attributes.var_type
        );
        assert_eq!(Some("AFFN".to_owned()), page_t5_x_attributes.var_form);
        assert_eq!(Some(5), page_t5_x_attributes.var_dim);
        assert_eq!(Some("XUNITS-TEST".to_owned()), page_t5_x_attributes.units);
        assert_eq!(Some(200.0), page_t5_x_attributes.first);
        assert_eq!(Some(280.0), page_t5_x_attributes.last);
        assert_eq!(Some(200.0), page_t5_x_attributes.min);
        assert_eq!(Some(280.0), page_t5_x_attributes.max);
        assert_eq!(Some(2.0), page_t5_x_attributes.factor);

        let page_t5_y_attributes = &page_t5_data_table.attributes.1;
        assert_eq!("INTENSITY", page_t5_y_attributes.var_name);
        assert_eq!("Y", page_t5_y_attributes.symbol);
        assert_eq!(Some("DEPENDENT".to_owned()), page_t5_y_attributes.var_type);
        assert_eq!(Some("AFFN".to_owned()), page_t5_y_attributes.var_form);
        assert_eq!(Some(5), page_t5_y_attributes.var_dim);
        assert_eq!(Some("YUNITS-TEST".to_owned()), page_t5_y_attributes.units);
        assert_eq!(Some(150.0), page_t5_y_attributes.first);
        assert_eq!(Some(270.0), page_t5_y_attributes.last);
        assert_eq!(Some(150.0), page_t5_y_attributes.min);
        assert_eq!(Some(270.0), page_t5_y_attributes.max);
        assert_eq!(Some(3.0), page_t5_y_attributes.factor);
    }

    // todo: harmonize naming attributes / variables
    #[test]
    fn ntuples_uses_page_ldrs_to_fill_missing_or_override_ntuples_variables() {
        let label = "NTUPLES";
        let variables = "         MASS SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR_NAME=        MASS,          INTENSITY,          RETENTION TIME\n\
                                ##SYMBOL=          X,             Y,                  T\n\
                                ##VAR_TYPE=        INDEPENDENT,   DEPENDENT,          INDEPENDENT\n\
                                ##VAR_FORM=        AFFN,          AFFN,               AFFN\n\
                                ##PAGE=            T = 5\n\
                                ##XUNITS=          XUNITS-TEST\n\
                                ##FIRSTX=          200.0\n\
                                ##LASTX=           280.0\n\
                                ##MINX=            200.0\n\
                                ##MAXX=            280.0\n\
                                ##XFACTOR=         2.0\n\
                                ##YUNITS=          YUNITS-TEST\n\
                                ##FIRSTY=          150.0\n\
                                ##LASTY=           270.0\n\
                                ##MINY=            150.0\n\
                                ##MAXY=            270.0\n\
                                ##YFACTOR=         3.0\n\
                                ##NPOINTS=         5\n\
                                ##DATA TABLE=      (XY..XY)            \n\
                                100,  50.0;  110,  60.0;  120,  70.0   \n\
                                130,  80.0;  140,  90.0                \n\
                                ##END NTUPLES= MASS SPECTRUM\n\
                                ##END=\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = vec![
            // to be overridden by PAGE LDR
            StringLdr::new("NPOINTS", "10"),
        ];

        let (ntuples, _next) =
            NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref).unwrap();

        assert_eq!(1, ntuples.pages.len());
        let page_t5 = &ntuples.pages[0];
        assert!(page_t5.data_table.is_some());
        let page_t5_data_table = page_t5.data_table.as_ref().unwrap();
        let page_t5_x_attributes = &page_t5_data_table.attributes.0;
        assert_eq!("MASS", page_t5_x_attributes.var_name);
        assert_eq!("X", page_t5_x_attributes.symbol);
        assert_eq!(
            Some("INDEPENDENT".to_owned()),
            page_t5_x_attributes.var_type
        );
        assert_eq!(Some("AFFN".to_owned()), page_t5_x_attributes.var_form);
        assert_eq!(Some(5), page_t5_x_attributes.var_dim);
        assert_eq!(Some("XUNITS-TEST".to_owned()), page_t5_x_attributes.units);
        assert_eq!(Some(200.0), page_t5_x_attributes.first);
        assert_eq!(Some(280.0), page_t5_x_attributes.last);
        assert_eq!(Some(200.0), page_t5_x_attributes.min);
        assert_eq!(Some(280.0), page_t5_x_attributes.max);
        assert_eq!(Some(2.0), page_t5_x_attributes.factor);

        let page_t5_y_attributes = &page_t5_data_table.attributes.1;
        assert_eq!("INTENSITY", page_t5_y_attributes.var_name);
        assert_eq!("Y", page_t5_y_attributes.symbol);
        assert_eq!(Some("DEPENDENT".to_owned()), page_t5_y_attributes.var_type);
        assert_eq!(Some("AFFN".to_owned()), page_t5_y_attributes.var_form);
        assert_eq!(Some(5), page_t5_y_attributes.var_dim);
        assert_eq!(Some("YUNITS-TEST".to_owned()), page_t5_y_attributes.units);
        assert_eq!(Some(150.0), page_t5_y_attributes.first);
        assert_eq!(Some(270.0), page_t5_y_attributes.last);
        assert_eq!(Some(150.0), page_t5_y_attributes.min);
        assert_eq!(Some(270.0), page_t5_y_attributes.max);
        assert_eq!(Some(3.0), page_t5_y_attributes.factor);
    }

    #[test]
    fn ntuples_fails_when_record_is_missing_var_name_ldr() {
        let label = "NTUPLES";
        let variables = " NMR SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        // missing:
        // "##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,    PAGE NUMBER\n"
        let input = b"##SYMBOL=             X,                Y,             N\n\
                                ##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n\
                                ##VAR_FORM=        AFFN,             ASDF,          AFFN\n\
                                ##VAR_DIM=            4,                4,             1\n\
                                ##UNITS=             HZ,  ARBITRARY UNITS,              \n\
                                ##PAGE= N=1\n\
                                ##DATA TABLE= (X++(Y..Y)), XYDATA   $$ Real data points\n\
                                1.0 +10+11\n\
                                2.0 +20+21\n\
                                ##PAGE= N=2\n\
                                ##END NTUPLES= NMR SPECTRUM\n\
                                ##END=\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let ntuples_res = NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref);

        assert!(ntuples_res.is_err());
        assert!(ntuples_res.unwrap_err().to_string().contains("VAR_NAME"));
    }

    #[test]
    fn ntuples_fails_when_record_contains_duplicate_ldrs() {
        let label = "NTUPLES";
        let variables = " NMR SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n\
                                ##SYMBOL=             X,                Y,             N\n\
                                ##SYMBOL=             X,                Y,             N\n\
                                ##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n\
                                ##VAR_FORM=        AFFN,             ASDF,          AFFN\n\
                                ##VAR_DIM=            4,                4,             1\n\
                                ##UNITS=             HZ,  ARBITRARY UNITS,              \n\
                                ##PAGE= N=1\n\
                                ##DATA TABLE= (X++(Y..Y)), XYDATA   $$ Real data points\n\
                                1.0 +10+11\n\
                                2.0 +20+21\n\
                                ##PAGE= N=2\n\
                                ##END NTUPLES= NMR SPECTRUM\n\
                                ##END=\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let ntuples_res = NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref);

        assert!(ntuples_res.is_err());
        assert!(ntuples_res.unwrap_err().to_string().contains("Duplicate"));
    }

    #[test]
    fn ntuples_handles_standard_variable_ldr_missing_columns() {
        let label = "NTUPLES";
        let variables = " NMR SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        // only one UNITS column
        let input = b"##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n\
                                ##SYMBOL=             X,                Y,             N\n\
                                ##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n\
                                ##VAR_FORM=        AFFN,             ASDF,          AFFN\n\
                                ##VAR_DIM=            4,                4,             1\n\
                                ##UNITS=             HZ\n\
                                ##PAGE= N=1\n\
                                ##DATA TABLE= (X++(Y..Y)), XYDATA   $$ Real data points\n\
                                1.0 +10+11\n\
                                2.0 +20+21\n\
                                ##PAGE= N=2\n\
                                ##END NTUPLES= NMR SPECTRUM\n\
                                ##END=\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let (ntuples, _next) =
            NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref).unwrap();

        assert_eq!(3, ntuples.attributes.len());
        let attributes_x = &ntuples.attributes[0];
        assert_eq!(Some("HZ".to_owned()), attributes_x.units);
        let attributes_y = &ntuples.attributes[1];
        assert_eq!(None, attributes_y.units);
        let attributes_n = &ntuples.attributes[2];
        assert_eq!(None, attributes_n.units);
    }

    #[test]
    fn ntuples_handles_custom_variable_ldr_missing_columns() {
        let label = "NTUPLES";
        let variables = " NMR SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        // only one CUSTOM_LDR column
        let input = b"##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n\
                                ##SYMBOL=             X,                Y,             N\n\
                                ##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n\
                                ##VAR_FORM=        AFFN,             ASDF,          AFFN\n\
                                ##VAR_DIM=            4,                4,             1\n\
                                ##UNITS=             HZ,  ARBITRARY UNITS,              \n\
                                ##$CUSTOM_LDR=     VAL1\n\
                                ##PAGE= N=1\n\
                                ##DATA TABLE= (X++(Y..Y)), XYDATA   $$ Real data points\n\
                                1.0 +10+11\n\
                                2.0 +20+21\n\
                                ##PAGE= N=2\n\
                                ##END NTUPLES= NMR SPECTRUM\n\
                                ##END=\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let (ntuples, _next) =
            NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref).unwrap();

        assert_eq!(3, ntuples.attributes.len());
        let attributes_x = &ntuples.attributes[0];
        assert_eq!(1, attributes_x.application_attributes.len());
        assert_eq!(
            StringLdr::new("$CUSTOMLDR", "VAL1"),
            attributes_x.application_attributes[0]
        );
        let attributes_y = &ntuples.attributes[1];
        assert!(attributes_y.application_attributes.is_empty());
        let attributes_n = &ntuples.attributes[2];
        assert!(attributes_n.application_attributes.is_empty());
    }

    #[test]
    fn ntuples_fails_when_ntuples_record_ends_prematurely() {
        let label = "NTUPLES";
        let variables = " NMR SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n\
                                ##SYMBOL=             X,                Y,             N\n\
                                ##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n\
                                ##VAR_FORM=        AFFN,             ASDF,          AFFN\n\
                                ##VAR_DIM=            4,                4,             1\n\
                                ##UNITS=             HZ,  ARBITRARY UNITS,              \n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let ntuples_res = NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref);

        assert!(ntuples_res.is_err());
        assert!(ntuples_res
            .unwrap_err()
            .to_string()
            .contains("Unexpected end"));
    }

    #[test]
    fn ntuples_fails_when_page_record_ends_prematurely() {
        let label = "NTUPLES";
        let variables = " NMR SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n\
                                ##SYMBOL=             X,                Y,             N\n\
                                ##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n\
                                ##VAR_FORM=        AFFN,             ASDF,          AFFN\n\
                                ##VAR_DIM=            4,                4,             1\n\
                                ##UNITS=             HZ,  ARBITRARY UNITS,              \n\
                                ##PAGE= N=1\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let ntuples_res = NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref);

        assert!(ntuples_res.is_err());
        assert!(ntuples_res.unwrap_err().to_string().contains("Unexpected"));
    }

    #[test]
    fn ntuples_fails_for_missing_data_table_variable_list() {
        let label = "NTUPLES";
        let variables = " NMR SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n\
                                ##SYMBOL=             X,                Y,             N\n\
                                ##PAGE= N=1\n\
                                ##DATA TABLE=                   $$ missing variable list\n\
                                ##END NTUPLES= NMR SPECTRUM\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let ntuples_res = NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref);

        assert!(ntuples_res.is_err());
        assert!(ntuples_res.unwrap_err().to_string().contains("Missing"));
    }

    #[test]
    fn ntuples_fails_for_illegal_data_table_variable_list() {
        let label = "NTUPLES";
        let variables = " NMR SPECTRUM";
        let next_line = Some(format!("##{label}= {variables}"));
        let input = b"##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n\
                                ##SYMBOL=             X,                Y,             N\n\
                                ##PAGE= N=1\n\
                                ##DATA TABLE= a, b, c           $$ illegal variable list\n\
                                ##END NTUPLES= NMR SPECTRUM\n";
        let reader_ref = Rc::new(RefCell::new(Cursor::new(input)));
        let block_ldrs = Vec::<StringLdr>::new();

        let ntuples_res = NTuples::new(label, &variables, &block_ldrs, next_line, reader_ref);

        assert!(ntuples_res.is_err());
        assert!(ntuples_res.unwrap_err().to_string().contains("Unexpected"));
    }
}
