use super::jdx_data_parser::DataParser;
use super::jdx_utils::{is_pure_comment, parse_ldr_start, strip_line_comment, BinBufRead};
use super::JdxError;
use crate::api::{Parser, SeekBufRead};
use crate::jdx::jdx_utils::{
    find_ldr, is_bruker_specific_section_start, parse_string_value, skip_pure_comments,
    skip_to_next_ldr,
};
use std::cell::RefCell;
use std::io::SeekFrom;
use std::rc::Rc;
use std::str::FromStr;

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
    // std::vector<Block> m_blocks;
    /// The XYDATA record if available.
    pub xy_data: Option<XyData<T>>,
    // std::optional<RaData> m_raData;
    // std::optional<XyPoints> m_xyPoints;
    // std::optional<PeakTable> m_peakTable;
    // std::optional<PeakAssignments> m_peakAssignments;
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
        let (block, _next_line) = Self::parse_input(&title, reader, &mut buf)?;
        Ok(block)
    }

    pub fn get_ldr(&self, label: &str) -> Option<&StringLdr> {
        find_ldr(label, &self.ldrs)
    }

    fn parse_first_line(line_opt: Option<&str>) -> Result<String, JdxError> {
        if line_opt.is_none() {
            return Err(JdxError::new("Malformed Block start. First line is empty."));
        }
        let line = line_opt.unwrap();
        let (label, value) = parse_ldr_start(line)?;
        if Self::BLOCK_START_LABEL != label {
            Err(JdxError::new(&format!("Malformed Block start: {line}")))
        } else {
            Ok(value)
        }
    }

    fn parse_input(
        title: &str,
        reader: T,
        buf: &mut Vec<u8>,
    ) -> Result<(Self, Option<String>), JdxError> {
        let reader_ref = Rc::new(RefCell::new(reader));
        let mut reader = reader_ref.borrow_mut();

        let mut ldrs = Vec::<StringLdr>::new();
        let mut ldr_comments = Vec::<String>::new();
        let mut xy_data = Option::<XyData<T>>::None;

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
                Self::BLOCK_START_LABEL => todo!(),
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
                "RADATA" => todo!(),
                "XYPOINTS" => todo!(),
                "PEAKTABLE" => todo!(),
                "PEAKASSIGNMENTS" => todo!(),
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
                xy_data,
            },
            next_line,
        ))
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct XyData<T: SeekBufRead> {
    reader_ref: Rc<RefCell<T>>,
    address: u64,

    label: String,
    variable_list: String,
    parameters: XyParameters,
}

impl<T: SeekBufRead> XyData<T> {
    const XYDATA_START_LABEL: &'static str = "XYDATA";
    // quirk variable list found in some sample data
    // that violates the spec but is unambiguous and thus accepted
    const QUIRK_OO_VARIABLE_LIST: &'static str = "(XY..XY)";
    const XYDATA_VARIABLE_LISTS: [&'static str; 4] = [
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
        Self::validate_input(
            label,
            variable_list,
            Self::XYDATA_START_LABEL,
            &Self::XYDATA_VARIABLE_LISTS,
        )?;
        let mut reader = reader_ref.borrow_mut();
        let address = reader.stream_position()?;
        let parameters = Self::parse_parameters(ldrs)?;
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

    fn validate_input(
        label: &str,
        variable_list: &str,
        expected_label: &str,
        expected_variable_lists: &[&str],
    ) -> Result<(), JdxError> {
        if label != expected_label {
            return Err(JdxError::new(&format!(
                "Illegal label at \"{}\" start encountered: {}",
                expected_label, label
            )));
        }
        if !expected_variable_lists.contains(&variable_list) {
            return Err(JdxError::new(&format!(
                "Illegal variable list for \"{}\" encountered: {}",
                label, variable_list
            )));
        }
        Ok(())
    }

    fn parse_parameters(ldrs: &[StringLdr]) -> Result<XyParameters, JdxError> {
        // required
        // string
        let x_units = find_ldr("XUNITS", ldrs).map(|ldr| &ldr.value);
        let y_units = find_ldr("YUNITS", ldrs).map(|ldr| &ldr.value);
        // double
        let first_x = Self::parse_parameter::<f64>("FIRSTX", ldrs)?;
        let last_x = Self::parse_parameter::<f64>("LASTX", ldrs)?;
        let x_factor = Self::parse_parameter::<f64>("XFACTOR", ldrs)?;
        let y_factor = Self::parse_parameter::<f64>("YFACTOR", ldrs)?;
        // u64
        let n_points = Self::parse_parameter::<u64>("NPOINTS", ldrs)?;
        // optional
        // double
        let first_y = Self::parse_parameter::<f64>("FIRSTY", ldrs)?;
        let max_x = Self::parse_parameter::<f64>("MAXX", ldrs)?;
        let min_x = Self::parse_parameter::<f64>("MINX", ldrs)?;
        let max_y = Self::parse_parameter::<f64>("MAXY", ldrs)?;
        let min_y = Self::parse_parameter::<f64>("MINY", ldrs)?;
        let resolution = Self::parse_parameter::<f64>("RESOLUTION", ldrs)?;
        let delta_x = Self::parse_parameter::<f64>("DELTAX", ldrs)?;

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
                "Required LDR(s) missing for XYDATA: {}",
                missing.join(", ")
            )));
        }

        Ok(XyParameters {
            x_units: x_units.unwrap().to_owned(),
            y_units: y_units.unwrap().to_owned(),
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

    fn parse_parameter<P: FromStr>(key: &str, ldrs: &[StringLdr]) -> Result<Option<P>, JdxError> {
        if let Some(ldr) = find_ldr(key, ldrs) {
            let value = strip_line_comment(&ldr.value, true, false).0;
            if value.is_empty() {
                return Ok(None);
            }
            let parsed_value = value.parse::<P>().map_err(|_e| {
                JdxError::new(&format!("Illegal value for \"{}\": {}", key, ldr.value))
            })?;
            return Ok(Some(parsed_value));
        }
        Ok(None)
    }

    fn parse_xppyy_data(
        label: &str,
        first_x: f64,
        last_x: f64,
        y_factor: f64,
        n_points: u64,
        data_address: u64,
        reader: &mut T,
    ) -> Result<Vec<(f64, f64)>, JdxError> {
        // read raw data from stream
        // remember stream position
        let pos = reader.stream_position()?;
        reader.seek(SeekFrom::Start(data_address))?;
        let y_data = DataParser::read_xppyy_data(reader)?;
        // reset stream position
        reader.seek(SeekFrom::Start(pos))?;
        if y_data.len() as u64 != n_points {
            return Err(JdxError::new(&format!(
                "Mismatch between NPOINTS and actual number of points \
                in \"{}\". NPOINTS: {}, actual: {}",
                label,
                n_points,
                y_data.len()
            )));
        }

        // prepare processing
        let mut xy_data = Vec::<(f64, f64)>::with_capacity(y_data.len());
        // cover special cases nPoints == 0 and nPoints == 1
        if n_points == 0 {
            return Ok(xy_data);
        }
        let (nominator, denominator) = if n_points == 1 {
            (first_x, 1f64)
        } else {
            (last_x - first_x, (n_points - 1) as f64)
        };

        // generate and return xy data
        for (i, y_raw) in y_data.iter().enumerate() {
            let x = first_x + nominator / denominator * i as f64;
            let y = y_factor * y_raw;
            xy_data.push((x, y));
        }

        Ok(xy_data)
    }

    fn parse_xyxy_data(
        label: &str,
        x_factor: f64,
        y_factor: f64,
        n_points: Option<u64>,
        data_address: u64,
        reader: &mut T,
    ) -> Result<Vec<(f64, f64)>, JdxError> {
        // read raw data from stream
        // remember stream position
        let pos = reader.stream_position()?;
        reader.seek(SeekFrom::Start(data_address))?;
        let mut xy_data = DataParser::read_xyxy_data(reader)?;
        // reset stream position
        reader.seek(SeekFrom::Start(pos))?;
        if let Some(np) = n_points {
            if xy_data.len() as u64 != np {
                return Err(JdxError::new(&format!(
                    "Mismatch between NPOINTS and actual number of points \
                    in \"{}\". NPOINTS: {}, actual: {}",
                    label,
                    np,
                    xy_data.len()
                )));
            }
        }

        // generate and return xy data
        for (x, y) in xy_data.iter_mut() {
            *x *= x_factor;
            *y *= y_factor;
        }

        Ok(xy_data)
    }

    /// Provides the parsed xy data.
    ///
    /// Returns pairs of xy data. Invalid values ("?") will be represented by NaN.
    pub fn get_data(&self) -> Result<Vec<(f64, f64)>, JdxError> {
        if !Self::XYDATA_VARIABLE_LISTS.contains(&self.variable_list.as_str()) {
            return Err(JdxError::new(&format!(
                "Unsupported variable list for XYDATA: {}",
                &self.variable_list,
            )));
        }
        let data = if self.variable_list == Self::QUIRK_OO_VARIABLE_LIST {
            // Ocean Optics quirk
            Self::parse_xyxy_data(
                &self.label,
                self.parameters.x_factor,
                self.parameters.y_factor,
                Some(self.parameters.n_points),
                self.address,
                &mut *self.reader_ref.borrow_mut(),
            )?
        } else {
            Self::parse_xppyy_data(
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
    /// The factor by which to multiply raw x values to arrive at the
    /// actual value.
    x_factor: f64,
    /// The factor by which to multiply raw y values to arrive at the
    /// actual value.
    y_factor: f64,
    /// The number of xy pairs in this record.
    n_points: u64,
    /// The first x value.
    first_x: f64,
    /// The last x value.
    last_x: f64,
    /// The first actual Y value (after scaling).
    first_y: Option<f64>,
    max_x: Option<f64>,
    min_x: Option<f64>,
    max_y: Option<f64>,
    min_y: Option<f64>,
    /// The resolution of the data.
    resolution: Option<f64>,
    /// The x distance between adjacent data points (if constant).
    delta_x: Option<f64>,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

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
}
