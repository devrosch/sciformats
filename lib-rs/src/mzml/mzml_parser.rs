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

use crate::{api::Parser, common::SfError};
use serde::Deserialize;
use std::io::{BufReader, Read, Seek};

pub struct MzMlParser {}

impl<T: Seek + Read> Parser<T> for MzMlParser {
    type R = MzMl;
    type E = SfError;

    fn parse(_name: &str, input: T) -> Result<Self::R, Self::E> {
        let buf_reader = BufReader::new(input);
        let mzml: Self::R = quick_xml::de::from_reader(buf_reader)
            .map_err(|e| SfError::from_source(e, "Error parsing mzML."))?;
        Ok(mzml)
    }
}

#[derive(Deserialize)]
pub struct MzMl {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "@xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "@schemaLocation")]
    pub xsi_schema_location: String,
    #[serde(rename = "@accession")]
    pub accession: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    // #[serde(rename = "cvList")]
    // pub cv_list: CvList,
    // #[serde(rename = "fileDescription")]
    // pub file_description: FileDescription,
    // #[serde(rename = "referenceableParamGroupList")]
    // pub referenceable_param_group_list: ReferenceableParamGroupList,
    // #[serde(rename = "sampleList")]
    // pub sample_list: SampleList,
    // #[serde(rename = "softwareList")]
    // pub software_list: SoftwareList,
    // #[serde(rename = "scanSettingsList")]
    // pub scan_settings_list: ScanSettingsList,
    // #[serde(rename = "instrumentConfigurationList")]
    // pub instrument_configuration_list: InstrumentConfigurationList,
    // #[serde(rename = "dataProcessingList")]
    // pub data_processing_list: DataProcessingList,
    // pub run: Run,
}

// #[derive(Deserialize)]
// pub struct CvList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     pub cv: Vec,
// }

// #[derive(Deserialize)]
// pub struct Cv {
//     #[serde(rename = "@id")]
//     pub id: String,
//     #[serde(rename = "@fullName")]
//     pub full_name: String,
//     #[serde(rename = "@version")]
//     pub version: String,
//     #[serde(rename = "@URI")]
//     pub uri: String,
// }

// #[derive(Deserialize)]
// pub struct FileDescription {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "fileContent")]
//     pub file_content: FileContent,
//     #[serde(rename = "sourceFileList")]
//     pub source_file_list: SourceFileList,
//     pub contact: Contact,
// }

// #[derive(Deserialize)]
// pub struct FileContent {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
// }

// #[derive(Deserialize)]
// pub struct MzMlFileDescriptionFileContentCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct SourceFileList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "sourceFile")]
//     pub source_file: Vec,
// }

// #[derive(Deserialize)]
// pub struct SourceFile {
//     #[serde(rename = "@id")]
//     pub id: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@location")]
//     pub location: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
// }

// #[derive(Deserialize)]
// pub struct FileDescriptionSourceFileListSourceFileCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct Contact {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
// }

// #[derive(Deserialize)]
// pub struct MzMlFileDescriptionContactCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct ReferenceableParamGroupList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "referenceableParamGroup")]
//     pub referenceable_param_group: Vec,
// }

// #[derive(Deserialize)]
// pub struct ReferenceableParamGroup {
//     #[serde(rename = "@id")]
//     pub id: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
// }

// #[derive(Deserialize)]
// pub struct MzMlReferenceableParamGroupListReferenceableParamGroupCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct SampleList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     pub sample: Sample,
// }

// #[derive(Deserialize)]
// pub struct Sample {
//     #[serde(rename = "@id")]
//     pub id: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
// }

// #[derive(Deserialize)]
// pub struct SoftwareList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     pub software: Vec,
// }

// #[derive(Deserialize)]
// pub struct Software {
//     #[serde(rename = "@id")]
//     pub id: String,
//     #[serde(rename = "@version")]
//     pub version: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: MzMlSoftwareListSoftwareCvParam,
// }

// #[derive(Deserialize)]
// pub struct MzMlSoftwareListSoftwareCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct ScanSettingsList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "scanSettings")]
//     pub scan_settings: ScanSettings,
// }

// #[derive(Deserialize)]
// pub struct ScanSettings {
//     #[serde(rename = "@id")]
//     pub id: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "sourceFileRefList")]
//     pub source_file_ref_list: SourceFileRefList,
//     #[serde(rename = "targetList")]
//     pub target_list: TargetList,
// }

// #[derive(Deserialize)]
// pub struct SourceFileRefList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "sourceFileRef")]
//     pub source_file_ref: SourceFileRef,
// }

// #[derive(Deserialize)]
// pub struct SourceFileRef {
//     #[serde(rename = "@ref")]
//     pub source_file_ref_ref: String,
// }

// #[derive(Deserialize)]
// pub struct TargetList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     pub target: Vec,
// }

// #[derive(Deserialize)]
// pub struct Target {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: ScanSettingsTargetListTargetCvParam,
// }

// #[derive(Deserialize)]
// pub struct ScanSettingsTargetListTargetCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
//     #[serde(rename = "@unitCvRef")]
//     pub unit_cv_ref: String,
//     #[serde(rename = "@unitAccession")]
//     pub unit_accession: String,
//     #[serde(rename = "@unitName")]
//     pub unit_name: String,
// }

// #[derive(Deserialize)]
// pub struct InstrumentConfigurationList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "instrumentConfiguration")]
//     pub instrument_configuration: InstrumentConfiguration,
// }

// #[derive(Deserialize)]
// pub struct InstrumentConfiguration {
//     #[serde(rename = "@id")]
//     pub id: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
//     #[serde(rename = "componentList")]
//     pub component_list: ComponentList,
//     #[serde(rename = "softwareRef")]
//     pub software_ref: SoftwareRef,
// }

// #[derive(Deserialize)]
// pub struct MzMlInstrumentConfigurationListInstrumentConfigurationCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct ComponentList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     pub source: Source,
//     pub analyzer: Analyzer,
//     pub detector: Detector,
// }

// #[derive(Deserialize)]
// pub struct Source {
//     #[serde(rename = "@order")]
//     pub order: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: InstrumentConfigurationComponentListSourceCvParam,
// }

// #[derive(Deserialize)]
// pub struct InstrumentConfigurationComponentListSourceCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct Analyzer {
//     #[serde(rename = "@order")]
//     pub order: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: InstrumentConfigurationComponentListAnalyzerCvParam,
// }

// #[derive(Deserialize)]
// pub struct InstrumentConfigurationComponentListAnalyzerCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct Detector {
//     #[serde(rename = "@order")]
//     pub order: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: InstrumentConfigurationComponentListDetectorCvParam,
// }

// #[derive(Deserialize)]
// pub struct InstrumentConfigurationComponentListDetectorCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct SoftwareRef {
//     #[serde(rename = "@ref")]
//     pub software_ref_ref: String,
// }

// #[derive(Deserialize)]
// pub struct DataProcessingList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "dataProcessing")]
//     pub data_processing: Vec,
// }

// #[derive(Deserialize)]
// pub struct DataProcessing {
//     #[serde(rename = "@id")]
//     pub id: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "processingMethod")]
//     pub processing_method: ProcessingMethod,
// }

// #[derive(Deserialize)]
// pub struct ProcessingMethod {
//     #[serde(rename = "@order")]
//     pub order: String,
//     #[serde(rename = "@softwareRef")]
//     pub software_ref: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
// }

// #[derive(Deserialize)]
// pub struct DataProcessingListDataProcessingProcessingMethodCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct Run {
//     #[serde(rename = "@id")]
//     pub id: String,
//     #[serde(rename = "@defaultInstrumentConfigurationRef")]
//     pub default_instrument_configuration_ref: String,
//     #[serde(rename = "@sampleRef")]
//     pub sample_ref: String,
//     #[serde(rename = "@startTimeStamp")]
//     pub start_time_stamp: String,
//     #[serde(rename = "@defaultSourceFileRef")]
//     pub default_source_file_ref: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "spectrumList")]
//     pub spectrum_list: SpectrumList,
//     #[serde(rename = "chromatogramList")]
//     pub chromatogram_list: ChromatogramList,
// }

// #[derive(Deserialize)]
// pub struct SpectrumList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "@defaultDataProcessingRef")]
//     pub default_data_processing_ref: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     pub spectrum: Vec,
// }

// #[derive(Deserialize)]
// pub struct Spectrum {
//     #[serde(rename = "@index")]
//     pub index: String,
//     #[serde(rename = "@id")]
//     pub id: String,
//     #[serde(rename = "@defaultArrayLength")]
//     pub default_array_length: String,
//     #[serde(rename = "@sourceFileRef")]
//     pub source_file_ref: Option,
//     #[serde(rename = "@spotID")]
//     pub spot_id: Option,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "precursorList")]
//     pub precursor_list: Option,
//     #[serde(rename = "referenceableParamGroupRef")]
//     pub referenceable_param_group_ref: ReferenceableParamGroupRef,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
//     #[serde(rename = "scanList")]
//     pub scan_list: ScanList,
//     #[serde(rename = "binaryDataArrayList")]
//     pub binary_data_array_list: SpectrumBinaryDataArrayList,
//     #[serde(rename = "userParam")]
//     pub user_param: Option,
// }

// #[derive(Deserialize)]
// pub struct PrecursorList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     pub precursor: PrecursorListPrecursor,
// }

// #[derive(Deserialize)]
// pub struct PrecursorListPrecursor {
//     #[serde(rename = "@spectrumRef")]
//     pub spectrum_ref: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "isolationWindow")]
//     pub isolation_window: PrecursorListPrecursorIsolationWindow,
//     #[serde(rename = "selectedIonList")]
//     pub selected_ion_list: SelectedIonList,
//     pub activation: PrecursorListPrecursorActivation,
// }

// #[derive(Deserialize)]
// pub struct PrecursorListPrecursorIsolationWindow {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
// }

// #[derive(Deserialize)]
// pub struct PrecursorListPrecursorIsolationWindowCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
//     #[serde(rename = "@unitCvRef")]
//     pub unit_cv_ref: String,
//     #[serde(rename = "@unitAccession")]
//     pub unit_accession: String,
//     #[serde(rename = "@unitName")]
//     pub unit_name: String,
// }

// #[derive(Deserialize)]
// pub struct SelectedIonList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "selectedIon")]
//     pub selected_ion: SelectedIon,
// }

// #[derive(Deserialize)]
// pub struct SelectedIon {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
// }

// #[derive(Deserialize)]
// pub struct PrecursorSelectedIonListSelectedIonCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
//     #[serde(rename = "@unitCvRef")]
//     pub unit_cv_ref: Option,
//     #[serde(rename = "@unitAccession")]
//     pub unit_accession: Option,
//     #[serde(rename = "@unitName")]
//     pub unit_name: Option,
// }

// #[derive(Deserialize)]
// pub struct PrecursorListPrecursorActivation {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
// }

// #[derive(Deserialize)]
// pub struct PrecursorListPrecursorActivationCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
//     #[serde(rename = "@unitName")]
//     pub unit_name: Option,
//     #[serde(rename = "@unitAccession")]
//     pub unit_accession: Option,
//     #[serde(rename = "@unitCvRef")]
//     pub unit_cv_ref: Option,
// }

// #[derive(Deserialize)]
// pub struct ReferenceableParamGroupRef {
//     #[serde(rename = "@ref")]
//     pub referenceable_param_group_ref_ref: String,
// }

// #[derive(Deserialize)]
// pub struct RunSpectrumListSpectrumCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
//     #[serde(rename = "@unitName")]
//     pub unit_name: Option,
//     #[serde(rename = "@unitAccession")]
//     pub unit_accession: Option,
//     #[serde(rename = "@unitCvRef")]
//     pub unit_cv_ref: Option,
// }

// #[derive(Deserialize)]
// pub struct ScanList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: SpectrumListSpectrumScanListCvParam,
//     pub scan: Scan,
// }

// #[derive(Deserialize)]
// pub struct SpectrumListSpectrumScanListCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct Scan {
//     #[serde(rename = "@instrumentConfigurationRef")]
//     pub instrument_configuration_ref: Option,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "scanWindowList")]
//     pub scan_window_list: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
// }

// #[derive(Deserialize)]
// pub struct ScanWindowList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "scanWindow")]
//     pub scan_window: ScanWindow,
// }

// #[derive(Deserialize)]
// pub struct ScanWindow {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
// }

// #[derive(Deserialize)]
// pub struct ScanScanWindowListScanWindowCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
//     #[serde(rename = "@unitCvRef")]
//     pub unit_cv_ref: String,
//     #[serde(rename = "@unitAccession")]
//     pub unit_accession: String,
//     #[serde(rename = "@unitName")]
//     pub unit_name: String,
// }

// #[derive(Deserialize)]
// pub struct SpectrumScanListScanCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
//     #[serde(rename = "@unitCvRef")]
//     pub unit_cv_ref: Option,
//     #[serde(rename = "@unitAccession")]
//     pub unit_accession: Option,
//     #[serde(rename = "@unitName")]
//     pub unit_name: Option,
// }

// #[derive(Deserialize)]
// pub struct SpectrumBinaryDataArrayList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "binaryDataArray")]
//     pub binary_data_array: Vec,
// }

// #[derive(Deserialize)]
// pub struct SpectrumBinaryDataArrayListBinaryDataArray {
//     #[serde(rename = "@encodedLength")]
//     pub encoded_length: String,
//     #[serde(rename = "@dataProcessingRef")]
//     pub data_processing_ref: Option,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
//     pub binary: String,
// }

// #[derive(Deserialize)]
// pub struct SpectrumBinaryDataArrayListBinaryDataArrayCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
//     #[serde(rename = "@unitName")]
//     pub unit_name: Option,
//     #[serde(rename = "@unitAccession")]
//     pub unit_accession: Option,
//     #[serde(rename = "@unitCvRef")]
//     pub unit_cv_ref: Option,
// }

// #[derive(Deserialize)]
// pub struct UserParam {
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct ChromatogramList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "@defaultDataProcessingRef")]
//     pub default_data_processing_ref: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     pub chromatogram: Vec,
// }

// #[derive(Deserialize)]
// pub struct Chromatogram {
//     #[serde(rename = "@index")]
//     pub index: String,
//     #[serde(rename = "@id")]
//     pub id: String,
//     #[serde(rename = "@defaultArrayLength")]
//     pub default_array_length: String,
//     #[serde(rename = "@dataProcessingRef")]
//     pub data_processing_ref: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: RunChromatogramListChromatogramCvParam,
//     #[serde(rename = "binaryDataArrayList")]
//     pub binary_data_array_list: ChromatogramBinaryDataArrayList,
//     pub product: Option,
//     pub precursor: Option,
// }

// #[derive(Deserialize)]
// pub struct RunChromatogramListChromatogramCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

// #[derive(Deserialize)]
// pub struct ChromatogramBinaryDataArrayList {
//     #[serde(rename = "@count")]
//     pub count: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "binaryDataArray")]
//     pub binary_data_array: Vec,
// }

// #[derive(Deserialize)]
// pub struct ChromatogramBinaryDataArrayListBinaryDataArray {
//     #[serde(rename = "@encodedLength")]
//     pub encoded_length: String,
//     #[serde(rename = "@dataProcessingRef")]
//     pub data_processing_ref: String,
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: Vec,
//     pub binary: String,
// }

// #[derive(Deserialize)]
// pub struct ChromatogramBinaryDataArrayListBinaryDataArrayCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
//     #[serde(rename = "@unitName")]
//     pub unit_name: Option,
//     #[serde(rename = "@unitAccession")]
//     pub unit_accession: Option,
//     #[serde(rename = "@unitCvRef")]
//     pub unit_cv_ref: Option,
// }

// #[derive(Deserialize)]
// pub struct Product {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "isolationWindow")]
//     pub isolation_window: ChromatogramProductIsolationWindow,
// }

// #[derive(Deserialize)]
// pub struct ChromatogramProductIsolationWindow {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: ChromatogramProductIsolationWindowCvParam,
// }

// #[derive(Deserialize)]
// pub struct ChromatogramProductIsolationWindowCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
//     #[serde(rename = "@unitCvRef")]
//     pub unit_cv_ref: String,
//     #[serde(rename = "@unitAccession")]
//     pub unit_accession: String,
//     #[serde(rename = "@unitName")]
//     pub unit_name: String,
// }

// #[derive(Deserialize)]
// pub struct ChromatogramPrecursor {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "isolationWindow")]
//     pub isolation_window: ChromatogramPrecursorIsolationWindow,
//     pub activation: ChromatogramPrecursorActivation,
// }

// #[derive(Deserialize)]
// pub struct ChromatogramPrecursorIsolationWindow {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: ChromatogramPrecursorIsolationWindowCvParam,
// }

// #[derive(Deserialize)]
// pub struct ChromatogramPrecursorIsolationWindowCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
//     #[serde(rename = "@unitCvRef")]
//     pub unit_cv_ref: String,
//     #[serde(rename = "@unitAccession")]
//     pub unit_accession: String,
//     #[serde(rename = "@unitName")]
//     pub unit_name: String,
// }

// #[derive(Deserialize)]
// pub struct ChromatogramPrecursorActivation {
//     #[serde(rename = "$text")]
//     pub text: Option,
//     #[serde(rename = "cvParam")]
//     pub cv_param: ChromatogramPrecursorActivationCvParam,
// }

// #[derive(Deserialize)]
// pub struct ChromatogramPrecursorActivationCvParam {
//     #[serde(rename = "@cvRef")]
//     pub cv_ref: String,
//     #[serde(rename = "@accession")]
//     pub accession: String,
//     #[serde(rename = "@name")]
//     pub name: String,
//     #[serde(rename = "@value")]
//     pub value: String,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parses_mzml_with_all_optional_elements() {
        let path = "valid.mzML";
        let xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?>
            <mzML
                xmlns="http://psi.hupo.org/ms/mzml"
                xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                xsi:schemaLocation="http://psi.hupo.org/ms/mzml http://psidev.info/files/ms/mzML/xsd/mzML1.1.0.xsd"
                accession="SF:0123456"
                version="1.1.0"
                id="sciformats:all_optional:valid.mzML">
            </mzML>"#;
        let reader = Cursor::new(xml);
        let mzml = MzMlParser::parse(path, reader).unwrap();

        assert_eq!(Some("SF:0123456".to_owned()), mzml.accession);
        assert_eq!("1.1.0", mzml.version);
        assert_eq!(
            Some("sciformats:all_optional:valid.mzML".to_owned()),
            mzml.id
        );
    }
}
