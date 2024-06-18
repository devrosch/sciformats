use super::jdx_utils::{parse_ldr_start, BinBufRead};
use super::JdxError;
use crate::api::{Parser, SeekBufRead};
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
    ldrs: Vec<StringLdr>,
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
        todo!()
    }

    fn parse_first_line(line: &str) {
        let start = parse_ldr_start(line);
    }
}

pub struct StringLdr {
    /// The label of the LDR, e.g., "TITLE" for "##TITLE= abc".
    pub label: String,
    /// The value (without initial blank character if any) of the LDR, e.g.,
    /// "abc" for "##TITLE= abc".
    pub value: String,
}

impl StringLdr {
    pub fn is_user_defined(&self) -> bool {
        self.label.chars().nth(0) == Some('$')
    }

    pub fn is_technique_specific(&self) -> bool {
        self.label.chars().nth(0) == Some('.')
    }
}
