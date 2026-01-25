// Copyright (c) 2026 Robert Schiwon
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::{
    api::{Parser, Reader, Scanner},
    common::SfError,
    mzml::{mzml_parser::MzMlParser, mzml_reader::MzMlReader},
    utils::is_recognized_extension,
};
use std::{
    cmp,
    io::{Read, Seek, SeekFrom},
};

#[derive(Default)]
pub struct MzmlScanner {}

impl MzmlScanner {
    const ACCEPTED_EXTENSIONS: [&'static str; 1] = ["mzml"];
    const MAGIC_BYTES: &'static [u8; 4] = b"mzML";
    const NUM_START_BYTES: u64 = 128;
}

impl MzmlScanner {
    pub fn new() -> Self {
        Self::default()
    }

    fn read_start<T: Seek + Read>(&self, input: &mut T) -> Result<Vec<u8>, SfError> {
        let len = input.seek(SeekFrom::End(0))?;
        input.seek(SeekFrom::Start(0))?;
        let len = cmp::min(len, Self::NUM_START_BYTES);
        let mut buf = vec![0; len as usize];
        input.read_exact(&mut buf)?;

        Ok(buf)
    }
}

impl<T: Seek + Read> Scanner<T> for MzmlScanner {
    fn is_recognized(&self, path: &str, input: &mut T) -> bool {
        if !is_recognized_extension(path, &Self::ACCEPTED_EXTENSIONS) {
            return false;
        };

        // start of file contains magic bytes "mzML"?
        match self.read_start(input) {
            Err(_) => false,
            Ok(bytes) => {
                let pos = bytes
                    .windows(Self::MAGIC_BYTES.len())
                    .position(|window| window == Self::MAGIC_BYTES);
                pos.is_some()
            }
        }
    }

    fn get_reader(&self, path: &str, input: T) -> Result<Box<dyn Reader>, SfError> {
        let mzml = MzMlParser::parse(path, input)?;
        Ok(Box::new(MzMlReader::new(path, mzml)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn accepts_valid_mzml() {
        let path = "valid.mzML";
        let xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?>
            <mzML
                xmlns="http://psi.hupo.org/ms/mzml"
                xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                xsi:schemaLocation="http://psi.hupo.org/ms/mzml http://psidev.info/files/ms/mzML/xsd/mzML1.1.0.xsd"
                id="sciformats:simple.mzML" version="1.1.0">
            </mzML>"#;
        let mut reader = Cursor::new(xml);
        let scanner = MzmlScanner::new();

        assert_eq!(true, scanner.is_recognized(path, &mut reader));
    }

    #[test]
    fn rejects_invalid_extension() {
        let path = "invalid.notMzML";
        let xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?>
            <mzML
                xmlns="http://psi.hupo.org/ms/mzml"
                xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                xsi:schemaLocation="http://psi.hupo.org/ms/mzml http://psidev.info/files/ms/mzML/xsd/mzML1.1.0.xsd"
                id="sciformats:simple.mzML" version="1.1.0">
            </mzML>"#;
        let mut reader = Cursor::new(xml);
        let scanner = MzmlScanner::new();

        assert_eq!(false, scanner.is_recognized(path, &mut reader));
    }

    #[test]
    fn rejects_invalid_content() {
        let path = "invalid.mzML";
        let xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?>
            <notMzML
                xmlns="http://psi.hupo.org/ms/mzml"
                xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                xsi:schemaLocation="http://psi.hupo.org/ms/mzml http://psidev.info/files/ms/mzML/xsd/mzML1.1.0.xsd"
                id="sciformats:simple.mzML" version="1.1.0">
            </notMzML>"#;
        let mut reader = Cursor::new(xml);
        let scanner = MzmlScanner::new();

        assert_eq!(false, scanner.is_recognized(path, &mut reader));
    }

    #[test]
    fn provides_reader_for_valid_mzml() {
        let path = "valid.mzML";
        let xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?>
            <mzML
                xmlns="http://psi.hupo.org/ms/mzml"
                xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                xsi:schemaLocation="http://psi.hupo.org/ms/mzml http://psidev.info/files/ms/mzML/xsd/mzML1.1.0.xsd"
                accession="SF:0123456"
                version="1.1.0"
                id="sciformats:all_optional:valid.mzML">

                <cvList count="1">
                <cv
                    id="MS"
                    fullName="Proteomics Standards Initiative Mass Spectrometry Ontology"
                    version="2.26.0"
                    URI="http://psidev.cvs.sourceforge.net/*checkout*/psidev/psi/psi-ms/mzML/controlledVocabulary/psi-ms.obo"/>
                </cvList>
                <fileDescription>
                    <fileContent>
                        <cvParam
                            cvRef="MS"
                            accession="MS:1234567"
                            name="cvParam name 1234567"
                            value="1234567"
                            unitAccession="unitAccession 1234567"
                            unitName="unitName 1234567"
                            unitCvRef="unitCvRef 1234567"/>
                    </fileContent>
                </fileDescription>
                <softwareList count="1">
                    <software id="software_id0" version="1.2.3"></software>
                </softwareList>
                <instrumentConfigurationList count="1">
                    <instrumentConfiguration id="instrumentConfiguration_id0"></instrumentConfiguration>
                </instrumentConfigurationList>
                <dataProcessingList count="1">
                    <dataProcessing id="dataProcessing_id0">
                        <processingMethod order="1" softwareRef="softwareRef1"></processingMethod>
                    </dataProcessing>
                </dataProcessingList>
                <run
                    id="run_id0"
                    defaultInstrumentConfigurationRef="defaultInstrumentConfigurationRef0"
                    sampleRef="sampleRef0"
                    startTimeStamp="2026-01-12T07:25:35.12345"
                    defaultSourceFileRef="defaultSourceFileRef0">
                    <spectrumList count="1" defaultDataProcessingRef="defaultDataProcessingRef0">
                        <spectrum
                            id="spectrum_id0"
                            spotID="spotID0"
                            index="0"
                            defaultArrayLength="2"
                            dataProcessingRef="dataProcessingRef0"
                            sourceFileRef="sourceFileRef0">
                            <scanList count="1">
                                <scan
                                    spectrumRef="spectrumRef0"
                                    sourceFileRef="sourceFileRef0"
                                    externalSpectrumID="externalSpectrumID0"
                                    instrumentConfigurationRef="instrumentConfigurationRef0">
                                </scan>
                            </scanList>
                            <binaryDataArrayList count="2">
                                <binaryDataArray arrayLength="3" dataProcessingRef="dataProcessingRef1" encodedLength="32">
                                    <!-- [1.0, 2.0, 3.0] little endian doubles base 64 encoded -->
                                    <binary>P/AAAAAAAABAAAAAAAAAAEAIAAAAAAAA</binary>
                                </binaryDataArray>
                                <binaryDataArray arrayLength="3" dataProcessingRef="dataProcessingRef2" encodedLength="32">
                                    <!-- [4.0, 5.0, 6.0] little endian doubles base 64 encoded -->
                                    <binary>QBAAAAAAAABAFAAAAAAAAEAYAAAAAAAA</binary>
                                </binaryDataArray>
                            </binaryDataArrayList>
                        </spectrum>
                    </spectrumList>
                    <chromatogramList count="1" defaultDataProcessingRef="defaultDataProcessingRef1">
                        <chromatogram id="chromatogram_id0" index="0" defaultArrayLength="3" dataProcessingRef="dataProcessingRef3">
                            <binaryDataArrayList count="2">
                                <binaryDataArray arrayLength="3" dataProcessingRef="dataProcessingRef4" encodedLength="32">
                                    <!-- [1.0, 2.0, 3.0] little endian doubles base 64 encoded -->
                                    <binary>P/AAAAAAAABAAAAAAAAAAEAIAAAAAAAA</binary>
                                </binaryDataArray>
                                <binaryDataArray arrayLength="3" dataProcessingRef="dataProcessingRef5" encodedLength="32">
                                    <!-- [4.0, 5.0, 6.0] little endian doubles base 64 encoded -->
                                    <binary>QBAAAAAAAABAFAAAAAAAAEAYAAAAAAAA</binary>
                                </binaryDataArray>
                            </binaryDataArrayList>
                        </chromatogram>
                    </chromatogramList>
                </run>
            </mzML>"#;

        let reader = Cursor::new(xml);
        let scanner = MzmlScanner::new();

        assert!(scanner.get_reader(path, reader).is_ok());
    }
}
