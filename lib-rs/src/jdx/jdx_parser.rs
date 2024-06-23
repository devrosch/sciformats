use super::jdx_utils::{is_pure_comment, parse_ldr_start, BinBufRead};
use super::JdxError;
use crate::api::{Parser, SeekBufRead};
use crate::jdx::jdx_utils::{
    find_ldr, is_bruker_specific_section_start, parse_string_value, skip_pure_comments,
};
use std::marker::PhantomData;

pub struct JdxParser {}

impl<T: SeekBufRead + 'static> Parser<T> for JdxParser {
    type R = JdxBlock<T>;
    type E = JdxError;

    fn parse(name: &str, input: T) -> Result<Self::R, Self::E> {
        Self::R::new(name, input)
    }
}

pub struct JdxBlock<T: SeekBufRead> {
    // todo: remove once T is actually used
    phantom: PhantomData<T>,

    /// The labeled data records (LDRs) of the Block.
    ///
    /// This does not include the following LDRs:
    /// - comments ("##=")
    /// - data (XYDATA, XYPOINTS, PEAK TABLE, PEAK ASSIGNMENTS, RADATA,
    ///   NTUPLES)
    /// These are available as dedicated peroperties.
    ///
    /// The key is the normalized label without "##" and "=" and the value is
    /// the content (without initial blank character if any).E.g. the LDR
    /// "##TITLE= abc" has label "TITLE" and content "abc" and the LDR
    /// "##DATA_POINTS=   5" has label "DATAPOINTS" and content "  5".
    pub ldrs: Vec<StringLdr>,
    // std::vector<std::string> m_ldrComments;
    // std::vector<Block> m_blocks;
    // std::optional<XyData> m_xyData;
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
        mut reader: T,
        buf: &mut Vec<u8>,
    ) -> Result<(Self, Option<String>), JdxError> {
        let mut ldrs = Vec::<StringLdr>::new();

        let (title, mut next_line) = parse_string_value(title, &mut reader, buf)?;
        ldrs.push(StringLdr {
            label: Self::BLOCK_START_LABEL.into(),
            value: title.clone(),
        });

        while let Some(ref line) = next_line {
            if is_pure_comment(line) {
                if is_bruker_specific_section_start(line) {
                    todo!();
                }
                next_line = skip_pure_comments(next_line, true, &mut reader, buf)?;
                continue;
            }

            let (label, mut value) = parse_ldr_start(line)?;
            match label.as_str() {
                "END" => break,
                Self::BLOCK_START_LABEL => todo!(),
                "XYDATA" => todo!(),
                "RADATA" => todo!(),
                "XYPOINTS" => todo!(),
                "PEAKTABLE" => todo!(),
                "PEAKASSIGNMENTS" => todo!(),
                "NTUPLES" => todo!(),
                "AUDITTRAIL" => todo!(),
                "$RELAX" => todo!(),
                _ => {
                    // LDR is a regular LDR
                    (value, next_line) = parse_string_value(&value, &mut reader, buf)?;

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
                phantom: PhantomData,
                ldrs,
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
}
