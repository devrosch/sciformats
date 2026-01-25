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

#[derive(Deserialize, PartialEq, Debug)]
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
    #[serde(rename = "cvList")]
    pub cv_list: CvList,
    #[serde(rename = "fileDescription")]
    pub file_description: FileDescription,
    #[serde(rename = "referenceableParamGroupList")]
    pub referenceable_param_group_list: Option<ReferenceableParamGroupList>,
    #[serde(rename = "sampleList")]
    pub sample_list: Option<SampleList>,
    #[serde(rename = "softwareList")]
    pub software_list: SoftwareList,
    #[serde(rename = "scanSettingsList")]
    pub scan_settings_list: Option<ScanSettingsList>,
    #[serde(rename = "instrumentConfigurationList")]
    pub instrument_configuration_list: InstrumentConfigurationList,
    #[serde(rename = "dataProcessingList")]
    pub data_processing_list: DataProcessingList,
    pub run: Run,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct CvList {
    #[serde(rename = "@count")]
    pub count: u64,
    pub cv: Vec<Cv>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Cv {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@fullName")]
    pub full_name: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@URI")]
    pub uri: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct FileDescription {
    #[serde(rename = "fileContent")]
    pub file_content: ParamGroup,
    #[serde(rename = "sourceFileList")]
    pub source_file_list: Option<SourceFileList>,
    #[serde(default)]
    pub contact: Vec<ParamGroup>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ParamGroup {
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct CvParam {
    #[serde(rename = "@cvRef")]
    pub cv_ref: String,
    #[serde(rename = "@accession")]
    pub accession: String,
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@unitAccession")]
    pub unit_accession: Option<String>,
    #[serde(rename = "@unitName")]
    pub unit_name: Option<String>,
    #[serde(rename = "@unitCvRef")]
    pub unit_cv_ref: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct UserParam {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@unitAccession")]
    pub unit_accession: Option<String>,
    #[serde(rename = "@unitName")]
    pub unit_name: Option<String>,
    #[serde(rename = "@unitCvRef")]
    pub unit_cv_ref: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct SourceFileList {
    #[serde(rename = "@count")]
    pub count: u64,
    #[serde(rename = "sourceFile")] // minOccurs="1"
    pub source_file: Vec<SourceFile>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct SourceFile {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@location")]
    pub location: String,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ReferenceableParamGroupList {
    #[serde(rename = "@count")]
    pub count: u64,
    #[serde(rename = "referenceableParamGroup")] // minOccurs="1"
    pub referenceable_param_group: Vec<ReferenceableParamGroup>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ReferenceableParamGroup {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct SampleList {
    #[serde(rename = "@count")]
    pub count: u64,
    pub sample: Vec<Sample>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Sample {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: Option<String>,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct SoftwareList {
    #[serde(rename = "@count")]
    pub count: u64,
    pub software: Vec<Software>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Software {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@version")]
    pub version: String,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ScanSettingsList {
    #[serde(rename = "@count")]
    pub count: u64,
    #[serde(rename = "scanSettings")]
    pub scan_settings: Vec<ScanSettings>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ScanSettings {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "sourceFileRefList")]
    pub source_file_ref_list: Option<SourceFileRefList>,
    #[serde(rename = "targetList")]
    pub target_list: Option<TargetList>,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct SourceFileRefList {
    #[serde(rename = "@count")]
    pub count: u64,
    #[serde(rename = "sourceFileRef", default)]
    pub source_file_ref: Vec<SourceFileRef>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct SourceFileRef {
    #[serde(rename = "@ref")]
    pub r#ref: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct TargetList {
    #[serde(rename = "@count")]
    pub count: u64,
    // minOccurs="1"
    pub target: Vec<ParamGroup>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct InstrumentConfigurationList {
    #[serde(rename = "@count")]
    pub count: u64,
    #[serde(rename = "instrumentConfiguration")] // minOccurs="1"
    pub instrument_configuration: Vec<InstrumentConfiguration>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct InstrumentConfiguration {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@scanSettingsRef")]
    pub scan_setting_ref: Option<String>,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,

    // InstrumentConfiguration specific elements
    #[serde(rename = "componentList")]
    pub component_list: Option<ComponentList>,
    #[serde(rename = "softwareRef")]
    pub software_ref: Option<SoftwareRef>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ComponentList {
    #[serde(rename = "@count")]
    pub count: u64,
    // SourceComponentType is extends ComponentType without additions, hence use Component directly
    pub source: Vec<Component>,
    // AnalyzerComponentType is extends ComponentType without additions, hence use Component directly
    pub analyzer: Vec<Component>,
    // DetectorComponentType is extends ComponentType without additions, hence use Component directly
    pub detector: Vec<Component>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Component {
    #[serde(rename = "@order")]
    pub order: i64, // xs:int, so i32 really but there's no harm using i64.

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct SoftwareRef {
    #[serde(rename = "@ref")]
    pub r#ref: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct DataProcessingList {
    #[serde(rename = "@count")]
    pub count: u64,
    #[serde(rename = "dataProcessing")]
    pub data_processing: Vec<DataProcessing>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct DataProcessing {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "processingMethod")]
    pub processing_method: Vec<ProcessingMethod>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ProcessingMethod {
    #[serde(rename = "@order")]
    pub order: u64,
    #[serde(rename = "@softwareRef")]
    pub software_ref: String,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Run {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@defaultInstrumentConfigurationRef")]
    pub default_instrument_configuration_ref: String,
    #[serde(rename = "@defaultSourceFileRef")]
    pub default_source_file_ref: Option<String>,
    #[serde(rename = "@sampleRef")]
    pub sample_ref: Option<String>,
    #[serde(rename = "@startTimeStamp")]
    pub start_time_stamp: Option<String>,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,

    #[serde(rename = "spectrumList")]
    pub spectrum_list: Option<SpectrumList>,
    #[serde(rename = "chromatogramList")]
    pub chromatogram_list: Option<ChromatogramList>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct SpectrumList {
    #[serde(rename = "@count")]
    pub count: u64,
    #[serde(rename = "@defaultDataProcessingRef")]
    pub default_data_processing_ref: String,
    pub spectrum: Vec<Spectrum>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Spectrum {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@spotID")]
    pub spot_id: Option<String>,
    #[serde(rename = "@index")]
    pub index: u64,
    #[serde(rename = "@defaultArrayLength")]
    pub default_array_length: i64, // xs:int, so i32 really but there's no harm using i64.
    #[serde(rename = "@dataProcessingRef")]
    pub data_processing_ref: Option<String>,
    #[serde(rename = "@sourceFileRef")]
    pub source_file_ref: Option<String>,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,

    #[serde(rename = "scanList")]
    pub scan_list: Option<ScanList>,
    #[serde(rename = "precursorList")]
    pub precursor_list: Option<PrecursorList>,
    #[serde(rename = "productList")]
    pub product_list: Option<ProductList>,
    #[serde(rename = "binaryDataArrayList")]
    pub binary_data_array_list: Option<BinaryDataArrayList>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct PrecursorList {
    #[serde(rename = "@count")]
    pub count: u64,
    pub precursor: Vec<Precursor>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Precursor {
    #[serde(rename = "@spectrumRef")]
    pub spectrum_ref: Option<String>,
    #[serde(rename = "@sourceFileRef")]
    pub source_file_ref: Option<String>,
    #[serde(rename = "@externalSpectrumID")]
    pub external_spectrum_id: Option<String>,

    #[serde(rename = "isolationWindow")]
    pub isolation_window: Option<ParamGroup>,
    #[serde(rename = "selectedIonList")]
    pub selected_ion_list: Option<SelectedIonList>,
    pub activation: ParamGroup,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ProductList {
    #[serde(rename = "@count")]
    pub count: u64,
    pub product: Vec<Product>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Product {
    #[serde(rename = "isolationWindow")]
    pub isolation_window: Option<ParamGroup>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct SelectedIonList {
    #[serde(rename = "@count")]
    pub count: u64,
    #[serde(rename = "selectedIon")] // minOccurs="1"
    pub selected_ion: Vec<ParamGroup>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ReferenceableParamGroupRef {
    #[serde(rename = "@ref")]
    pub r#ref: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ScanList {
    #[serde(rename = "@count")]
    pub count: u64,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,

    pub scan: Vec<Scan>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Scan {
    #[serde(rename = "@spectrumRef")]
    pub spectrum_ref: Option<String>,
    #[serde(rename = "@sourceFileRef")]
    pub source_file_ref: Option<String>,
    #[serde(rename = "@externalSpectrumID")]
    pub external_spectrum_id: Option<String>,
    #[serde(rename = "@instrumentConfigurationRef")]
    pub instrument_configuration_ref: Option<String>,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,

    #[serde(rename = "scanWindowList")]
    pub scan_window_list: Option<ScanWindowList>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ScanWindowList {
    #[serde(rename = "@count")]
    pub count: i64, // xs:int, so i32 really but there's no harm using i64.
    #[serde(rename = "scanWindow")]
    pub scan_window: Vec<ParamGroup>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct BinaryDataArrayList {
    #[serde(rename = "@count")]
    pub count: u64,
    #[serde(rename = "binaryDataArray")]
    pub binary_data_array: Vec<BinaryDataArray>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct BinaryDataArray {
    #[serde(rename = "@arrayLength")]
    pub array_length: Option<u64>,
    #[serde(rename = "@dataProcessingRef")]
    pub data_processing_ref: Option<String>,
    #[serde(rename = "@encodedLength")]
    pub encoded_length: u64,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,

    pub binary: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ChromatogramList {
    #[serde(rename = "@count")]
    pub count: u64,
    #[serde(rename = "@defaultDataProcessingRef")]
    pub default_data_processing_ref: String,
    pub chromatogram: Vec<Chromatogram>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Chromatogram {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@index")]
    pub index: u64,
    #[serde(rename = "@defaultArrayLength")]
    pub default_array_length: i64, // xs:int, so i32 really but there's no harm using i64.
    #[serde(rename = "@dataProcessingRef")]
    pub data_processing_ref: Option<String>,

    // ParamGroup elements
    #[serde(rename = "referenceableParamGroupRef", default)]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(rename = "cvParam", default)]
    pub cv_param: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_param: Vec<UserParam>,

    pub precursor: Option<Precursor>,
    pub product: Option<Product>,
    #[serde(rename = "binaryDataArrayList")]
    pub binary_data_array_list: BinaryDataArrayList,
}

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

                <cvList count="2">
                <cv
                    id="MS"
                    fullName="Proteomics Standards Initiative Mass Spectrometry Ontology"
                    version="2.26.0"
                    URI="http://psidev.cvs.sourceforge.net/*checkout*/psidev/psi/psi-ms/mzML/controlledVocabulary/psi-ms.obo"/>
                <cv
                    id="UO"
                    fullName="Unit Ontology"
                    version="14:07:2009"
                    URI="http://obo.cvs.sourceforge.net/*checkout*/obo/obo/ontology/phenotype/unit.obo"/>
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
                        <cvParam
                            cvRef="MS"
                            accession="MS:1234568"
                            name="cvParam name 1234568"
                            value="1234568"
                            unitAccession="unitAccession 1234568"
                            unitName="unitName 1234568"
                            unitCvRef="unitCvRef 1234568"/>
                    </fileContent>
                    <sourceFileList count="1">
                        <sourceFile id="sourceFile_id0" name="source_file.raw" location="file:///path/to/location/">
                            <referenceableParamGroupRef ref="ref0"/>
                            <cvParam
                                cvRef="MS"
                                accession="MS:1234570"
                                name="cvParam name 1234570"
                                value="cvParam value 1234570"
                                unitAccession="cvParam unitAccession 1234570"
                                unitName="cvParam unitName 1234570"
                                unitCvRef="cvParam unitCvRef 1234570"/>
                            <userParam
                                name="userParam name 1234571"
                                type="userParam type 1234571"
                                value="userParam value 1234571"
                                unitAccession="userParam unitAccession 1234571"
                                unitName="userParam unitName 1234571"
                                unitCvRef="userParam unitCvRef 1234571"/>
                        </sourceFile>
                    </sourceFileList>
                    <contact>
                        <referenceableParamGroupRef ref="ref1"/>
                        <cvParam
                            cvRef="MS"
                            accession="MS:1234572"
                            name="cvParam name 1234572"
                            value="cvParam value 1234572"
                            unitAccession="cvParam unitAccession 1234572"
                            unitName="cvParam unitName 1234572"
                            unitCvRef="cvParam unitCvRef 1234572"/>
                        <userParam
                            name="userParam name 1234573"
                            type="userParam type 1234573"
                            value="userParam value 1234573"
                            unitAccession="userParam unitAccession 1234573"
                            unitName="userParam unitName 1234573"
                            unitCvRef="userParam unitCvRef 1234573"/>
                    </contact>
                </fileDescription>
                <referenceableParamGroupList count="2">
                    <referenceableParamGroup id="referenceableParamGroup0">
                        <cvParam
                            cvRef="MS"
                            accession="MS:1234574"
                            name="cvParam name 1234574"
                            value="cvParam value 1234574"
                            unitAccession="cvParam unitAccession 1234574"
                            unitName="cvParam unitName 1234574"
                            unitCvRef="cvParam unitCvRef 1234574"/>
                        <userParam
                            name="userParam name 1234575"
                            type="userParam type 1234575"
                            value="userParam value 1234575"
                            unitAccession="userParam unitAccession 1234575"
                            unitName="userParam unitName 1234575"
                            unitCvRef="userParam unitCvRef 1234575"/>
                    </referenceableParamGroup>
                    <referenceableParamGroup id="referenceableParamGroup1">
                        <cvParam
                            cvRef="MS"
                            accession="MS:1234576"
                            name="cvParam name 1234576"
                            value="cvParam value 1234576"
                            unitAccession="cvParam unitAccession 1234576"
                            unitName="cvParam unitName 1234576"
                            unitCvRef="cvParam unitCvRef 1234576"/>
                        <userParam
                            name="userParam name 1234577"
                            type="userParam type 1234577"
                            value="userParam value 1234577"
                            unitAccession="userParam unitAccession 1234577"
                            unitName="userParam unitName 1234577"
                            unitCvRef="userParam unitCvRef 1234577"/>
                    </referenceableParamGroup>
                </referenceableParamGroupList>
                <sampleList count="1">
                    <sample id="sample0" name="Sample 0">
                        <referenceableParamGroupRef ref="ref2"/>
                        <cvParam
                            cvRef="MS"
                            accession="MS:1234578"
                            name="cvParam name 1234578"
                            value="cvParam value 1234578"
                            unitAccession="cvParam unitAccession 1234578"
                            unitName="cvParam unitName 1234578"
                            unitCvRef="cvParam unitCvRef 1234578"/>
                        <userParam
                            name="userParam name 1234579"
                            type="userParam type 1234579"
                            value="userParam value 1234579"
                            unitAccession="userParam unitAccession 1234579"
                            unitName="userParam unitName 1234579"
                            unitCvRef="userParam unitCvRef 1234579"/>
                    </sample>
                </sampleList>
                <softwareList count="2">
                    <software id="software_id0" version="1.2.3">
                        <referenceableParamGroupRef ref="ref3"/>
                        <cvParam
                            cvRef="MS"
                            accession="MS:1234580"
                            name="cvParam name 1234580"
                            value="cvParam value 1234580"
                            unitAccession="cvParam unitAccession 1234580"
                            unitName="cvParam unitName 1234580"
                            unitCvRef="cvParam unitCvRef 1234580"/>
                        <userParam
                            name="userParam name 1234581"
                            type="userParam type 1234581"
                            value="userParam value 1234581"
                            unitAccession="userParam unitAccession 1234581"
                            unitName="userParam unitName 1234581"
                            unitCvRef="userParam unitCvRef 1234581"/>
                    </software>
                        <software id="software_id1" version="0.1.2">
                    </software>
                </softwareList>
                <scanSettingsList count="1">
                    <scanSettings id="scanSettings_id0">
                        <referenceableParamGroupRef ref="ref6"/>
                        <cvParam
                            cvRef="MS"
                            accession="MS:1234582"
                            name="cvParam name 1234582"
                            value="cvParam value 1234582"
                            unitAccession="cvParam unitAccession 1234582"
                            unitName="cvParam unitName 1234582"
                            unitCvRef="cvParam unitCvRef 1234582"/>
                        <userParam
                            name="userParam name 1234583"
                            type="userParam type 1234583"
                            value="userParam value 1234583"
                            unitAccession="userParam unitAccession 1234583"
                            unitName="userParam unitName 1234583"
                            unitCvRef="userParam unitCvRef 1234583"/>
                        <sourceFileRefList count="1">
                            <sourceFileRef ref="sourceFileRef0"/>
                        </sourceFileRefList>
                        <targetList count="2">
                            <target>
                                <referenceableParamGroupRef ref="ref4"/>
                                <cvParam
                                    cvRef="MS"
                                    accession="MS:1234584"
                                    name="cvParam name 1234584"
                                    value="cvParam value 1234584"
                                    unitAccession="cvParam unitAccession 1234584"
                                    unitName="cvParam unitName 1234584"
                                    unitCvRef="cvParam unitCvRef 1234584"/>
                                <userParam
                                    name="userParam name 1234585"
                                    type="userParam type 1234585"
                                    value="userParam value 1234585"
                                    unitAccession="userParam unitAccession 1234585"
                                    unitName="userParam unitName 1234585"
                                    unitCvRef="userParam unitCvRef 1234585"/>
                            </target>
                            <target>
                                <referenceableParamGroupRef ref="ref5"/>
                                <cvParam
                                    cvRef="MS"
                                    accession="MS:1234586"
                                    name="cvParam name 1234586"
                                    value="cvParam value 1234586"
                                    unitAccession="cvParam unitAccession 1234586"
                                    unitName="cvParam unitName 1234586"
                                    unitCvRef="cvParam unitCvRef 1234586"/>
                                <userParam
                                    name="userParam name 1234587"
                                    type="userParam type 1234587"
                                    value="userParam value 1234587"
                                    unitAccession="userParam unitAccession 1234587"
                                    unitName="userParam unitName 1234587"
                                    unitCvRef="userParam unitCvRef 1234587"/>
                            </target>
                        </targetList>
                    </scanSettings>
                </scanSettingsList>
                <instrumentConfigurationList count="1">
                    <instrumentConfiguration id="instrumentConfiguration_id0" scanSettingsRef="scanSettingsRef0">
                        <referenceableParamGroupRef ref="ref6"/>
                        <cvParam
                            cvRef="MS"
                            accession="MS:1234588"
                            name="cvParam name 1234588"
                            value="cvParam value 1234588"
                            unitAccession="cvParam unitAccession 1234588"
                            unitName="cvParam unitName 1234588"
                            unitCvRef="cvParam unitCvRef 1234588"/>
                        <userParam
                            name="userParam name 1234589"
                            type="userParam type 1234589"
                            value="userParam value 1234589"
                            unitAccession="userParam unitAccession 1234589"
                            unitName="userParam unitName 1234589"
                            unitCvRef="userParam unitCvRef 1234589"/>
                        <componentList count="3">
                            <source order="1">
                                <referenceableParamGroupRef ref="ref7"/>
                                <cvParam
                                    cvRef="MS"
                                    accession="MS:1234590"
                                    name="cvParam name 1234590"
                                    value="cvParam value 1234590"
                                    unitAccession="cvParam unitAccession 1234590"
                                    unitName="cvParam unitName 1234590"
                                    unitCvRef="cvParam unitCvRef 1234590"/>
                                <userParam
                                    name="userParam name 1234591"
                                    type="userParam type 1234591"
                                    value="userParam value 1234591"
                                    unitAccession="userParam unitAccession 1234591"
                                    unitName="userParam unitName 1234591"
                                    unitCvRef="userParam unitCvRef 1234591"/>
                            </source>
                            <analyzer order="2">
                                <referenceableParamGroupRef ref="ref8"/>
                                <cvParam
                                    cvRef="MS"
                                    accession="MS:1234592"
                                    name="cvParam name 1234592"
                                    value="cvParam value 1234592"
                                    unitAccession="cvParam unitAccession 1234592"
                                    unitName="cvParam unitName 1234592"
                                    unitCvRef="cvParam unitCvRef 1234592"/>
                                <userParam
                                    name="userParam name 1234593"
                                    type="userParam type 1234593"
                                    value="userParam value 1234593"
                                    unitAccession="userParam unitAccession 1234593"
                                    unitName="userParam unitName 1234593"
                                    unitCvRef="userParam unitCvRef 1234593"/>
                            </analyzer>
                            <detector order="3">
                                <referenceableParamGroupRef ref="ref9"/>
                                <cvParam
                                    cvRef="MS"
                                    accession="MS:1234594"
                                    name="cvParam name 1234594"
                                    value="cvParam value 1234594"
                                    unitAccession="cvParam unitAccession 1234594"
                                    unitName="cvParam unitName 1234594"
                                    unitCvRef="cvParam unitCvRef 1234594"/>
                                <userParam
                                    name="userParam name 1234595"
                                    type="userParam type 1234595"
                                    value="userParam value 1234595"
                                    unitAccession="userParam unitAccession 1234595"
                                    unitName="userParam unitName 1234595"
                                    unitCvRef="userParam unitCvRef 1234595"/>
                            </detector>
                        </componentList>
                        <softwareRef ref="softwareRef0"/>
                    </instrumentConfiguration>
                </instrumentConfigurationList>
                <dataProcessingList count="1">
                    <dataProcessing id="dataProcessing_id0">
                        <processingMethod order="1" softwareRef="softwareRef1">
                            <referenceableParamGroupRef ref="ref10"/>
                            <cvParam
                                cvRef="MS"
                                accession="MS:1234595"
                                name="cvParam name 1234595"
                                value="cvParam value 1234595"
                                unitAccession="cvParam unitAccession 1234595"
                                unitName="cvParam unitName 1234595"
                                unitCvRef="cvParam unitCvRef 1234595"/>
                            <userParam
                                name="userParam name 1234596"
                                type="userParam type 1234596"
                                value="userParam value 1234596"
                                unitAccession="userParam unitAccession 1234596"
                                unitName="userParam unitName 1234596"
                                unitCvRef="userParam unitCvRef 1234596"/>
                        </processingMethod>
                    </dataProcessing>
                </dataProcessingList>
                <run
                    id="run_id0"
                    defaultInstrumentConfigurationRef="defaultInstrumentConfigurationRef0"
                    sampleRef="sampleRef0"
                    startTimeStamp="2026-01-12T07:25:35.12345"
                    defaultSourceFileRef="defaultSourceFileRef0">
                    <referenceableParamGroupRef ref="ref11"/>
                    <cvParam
                        cvRef="MS"
                        accession="MS:1234597"
                        name="cvParam name 1234597"
                        value="cvParam value 1234597"
                        unitAccession="cvParam unitAccession 1234597"
                        unitName="cvParam unitName 1234597"
                        unitCvRef="cvParam unitCvRef 1234597"/>
                    <userParam
                        name="userParam name 1234598"
                        type="userParam type 1234598"
                        value="userParam value 1234598"
                        unitAccession="userParam unitAccession 1234598"
                        unitName="userParam unitName 1234598"
                        unitCvRef="userParam unitCvRef 1234598"/>
                    <spectrumList count="2" defaultDataProcessingRef="defaultDataProcessingRef0">
                        <spectrum
                            id="spectrum_id0"
                            spotID="spotID0"
                            index="0"
                            defaultArrayLength="2"
                            dataProcessingRef="dataProcessingRef0"
                            sourceFileRef="sourceFileRef0">
                            <referenceableParamGroupRef ref="ref12"/>
                            <cvParam
                                cvRef="MS"
                                accession="MS:1234599"
                                name="cvParam name 1234599"
                                value="cvParam value 1234599"
                                unitAccession="cvParam unitAccession 1234599"
                                unitName="cvParam unitName 1234599"
                                unitCvRef="cvParam unitCvRef 1234599"/>
                            <userParam
                                name="userParam name 1234600"
                                type="userParam type 1234600"
                                value="userParam value 1234600"
                                unitAccession="userParam unitAccession 1234600"
                                unitName="userParam unitName 1234600"
                                unitCvRef="userParam unitCvRef 1234600"/>
                            <scanList count="1">
                                <referenceableParamGroupRef ref="ref13"/>
                                <cvParam
                                    cvRef="MS"
                                    accession="MS:1234601"
                                    name="cvParam name 1234601"
                                    value="cvParam value 1234601"
                                    unitAccession="cvParam unitAccession 1234601"
                                    unitName="cvParam unitName 1234601"
                                    unitCvRef="cvParam unitCvRef 1234601"/>
                                <userParam
                                    name="userParam name 1234602"
                                    type="userParam type 1234602"
                                    value="userParam value 1234602"
                                    unitAccession="userParam unitAccession 1234602"
                                    unitName="userParam unitName 1234602"
                                    unitCvRef="userParam unitCvRef 1234602"/>
                                <scan
                                    spectrumRef="spectrumRef0"
                                    sourceFileRef="sourceFileRef0"
                                    externalSpectrumID="externalSpectrumID0"
                                    instrumentConfigurationRef="instrumentConfigurationRef0">
                                    <referenceableParamGroupRef ref="ref14"/>
                                    <cvParam
                                        cvRef="MS"
                                        accession="MS:1234603"
                                        name="cvParam name 1234603"
                                        value="cvParam value 1234603"
                                        unitAccession="cvParam unitAccession 1234603"
                                        unitName="cvParam unitName 1234603"
                                        unitCvRef="cvParam unitCvRef 1234603"/>
                                    <userParam
                                        name="userParam name 1234604"
                                        type="userParam type 1234604"
                                        value="userParam value 1234604"
                                        unitAccession="userParam unitAccession 1234604"
                                        unitName="userParam unitName 1234604"
                                        unitCvRef="userParam unitCvRef 1234604"/>
                                    <scanWindowList count="1">
                                        <scanWindow>
                                            <referenceableParamGroupRef ref="ref15"/>
                                            <cvParam
                                                cvRef="MS"
                                                accession="MS:1234605"
                                                name="cvParam name 1234605"
                                                value="cvParam value 1234605"
                                                unitAccession="cvParam unitAccession 1234605"
                                                unitName="cvParam unitName 1234605"
                                                unitCvRef="cvParam unitCvRef 1234605"/>
                                            <userParam
                                                name="userParam name 1234606"
                                                type="userParam type 1234606"
                                                value="userParam value 1234606"
                                                unitAccession="userParam unitAccession 1234606"
                                                unitName="userParam unitName 1234606"
                                                unitCvRef="userParam unitCvRef 1234606"/>
                                        </scanWindow>
                                    </scanWindowList>
                                </scan>
                            </scanList>
                            <precursorList count="1">
                                <precursor
                                    spectrumRef="spectrumRef1"
                                    sourceFileRef="sourceFileRef4"
                                    externalSpectrumID="externalSpectrumID1">
                                    <isolationWindow>
                                        <referenceableParamGroupRef ref="ref16"/>
                                        <cvParam
                                            cvRef="MS"
                                            accession="MS:1234607"
                                            name="cvParam name 1234607"
                                            value="cvParam value 1234607"
                                            unitAccession="cvParam unitAccession 1234607"
                                            unitName="cvParam unitName 1234607"
                                            unitCvRef="cvParam unitCvRef 1234607"/>
                                        <userParam
                                            name="userParam name 1234608"
                                            type="userParam type 1234608"
                                            value="userParam value 1234608"
                                            unitAccession="userParam unitAccession 1234608"
                                            unitName="userParam unitName 1234608"
                                            unitCvRef="userParam unitCvRef 1234608"/>
                                    </isolationWindow>
                                    <selectedIonList count="1">
                                        <selectedIon>
                                            <referenceableParamGroupRef ref="ref17"/>
                                            <cvParam
                                                cvRef="MS"
                                                accession="MS:1234609"
                                                name="cvParam name 1234609"
                                                value="cvParam value 1234609"
                                                unitAccession="cvParam unitAccession 1234609"
                                                unitName="cvParam unitName 1234609"
                                                unitCvRef="cvParam unitCvRef 1234609"/>
                                            <userParam
                                                name="userParam name 1234610"
                                                type="userParam type 1234610"
                                                value="userParam value 1234610"
                                                unitAccession="userParam unitAccession 1234610"
                                                unitName="userParam unitName 1234610"
                                                unitCvRef="userParam unitCvRef 1234610"/>
                                        </selectedIon>
                                    </selectedIonList>
                                    <activation>
                                        <referenceableParamGroupRef ref="ref18"/>
                                        <cvParam
                                            cvRef="MS"
                                            accession="MS:1234611"
                                            name="cvParam name 1234611"
                                            value="cvParam value 1234611"
                                            unitAccession="cvParam unitAccession 1234611"
                                            unitName="cvParam unitName 1234611"
                                            unitCvRef="cvParam unitCvRef 1234611"/>
                                        <userParam
                                            name="userParam name 1234612"
                                            type="userParam type 1234612"
                                            value="userParam value 1234612"
                                            unitAccession="userParam unitAccession 1234612"
                                            unitName="userParam unitName 1234612"
                                            unitCvRef="userParam unitCvRef 1234612"/>
                                    </activation>
                                </precursor>
                            </precursorList>
                            <productList count="1">
                                <product>
                                    <isolationWindow>
                                        <referenceableParamGroupRef ref="ref19"/>
                                        <cvParam
                                            cvRef="MS"
                                            accession="MS:1234613"
                                            name="cvParam name 1234613"
                                            value="cvParam value 1234613"
                                            unitAccession="cvParam unitAccession 1234613"
                                            unitName="cvParam unitName 1234613"
                                            unitCvRef="cvParam unitCvRef 1234613"/>
                                        <userParam
                                            name="userParam name 1234614"
                                            type="userParam type 1234614"
                                            value="userParam value 1234614"
                                            unitAccession="userParam unitAccession 1234614"
                                            unitName="userParam unitName 1234614"
                                            unitCvRef="userParam unitCvRef 1234614"/>
                                    </isolationWindow>
                                </product>
                            </productList>                            
                            <binaryDataArrayList count="2">
                                <binaryDataArray arrayLength="3" dataProcessingRef="dataProcessingRef1" encodedLength="32">
                                    <referenceableParamGroupRef ref="ref20"/>
                                    <cvParam
                                        cvRef="MS"
                                        accession="MS:1234615"
                                        name="cvParam name 1234615"
                                        value="cvParam value 1234615"
                                        unitAccession="cvParam unitAccession 1234615"
                                        unitName="cvParam unitName 1234615"
                                        unitCvRef="cvParam unitCvRef 1234615"/>
                                    <userParam
                                        name="userParam name 1234616"
                                        type="userParam type 1234616"
                                        value="userParam value 1234616"
                                        unitAccession="userParam unitAccession 1234616"
                                        unitName="userParam unitName 1234616"
                                        unitCvRef="userParam unitCvRef 1234616"/>
                                    <!-- [1.0, 2.0, 3.0] little endian doubles base 64 encoded -->
                                    <binary>P/AAAAAAAABAAAAAAAAAAEAIAAAAAAAA</binary>
                                </binaryDataArray>
                                <binaryDataArray arrayLength="3" dataProcessingRef="dataProcessingRef2" encodedLength="32">
                                    <referenceableParamGroupRef ref="ref21"/>
                                    <cvParam
                                        cvRef="MS"
                                        accession="MS:1234617"
                                        name="cvParam name 1234617"
                                        value="cvParam value 1234617"
                                        unitAccession="cvParam unitAccession 1234617"
                                        unitName="cvParam unitName 1234617"
                                        unitCvRef="cvParam unitCvRef 1234617"/>
                                    <userParam
                                        name="userParam name 1234618"
                                        type="userParam type 1234618"
                                        value="userParam value 1234618"
                                        unitAccession="userParam unitAccession 1234618"
                                        unitName="userParam unitName 1234618"
                                        unitCvRef="userParam unitCvRef 1234618"/>
                                    <!-- [4.0, 5.0, 6.0] little endian doubles base 64 encoded -->
                                    <binary>QBAAAAAAAABAFAAAAAAAAEAYAAAAAAAA</binary>
                                </binaryDataArray>
                            </binaryDataArrayList>
                        </spectrum>
                        <spectrum
                            id="spectrum_id1"
                            index="1"
                            defaultArrayLength="2">
                            <!-- TODO: add more elements -->
                            </spectrum>
                    </spectrumList>
                    <chromatogramList count="1" defaultDataProcessingRef="defaultDataProcessingRef1">
                        <chromatogram id="chromatogram_id0" index="0" defaultArrayLength="3" dataProcessingRef="dataProcessingRef3">
                            <referenceableParamGroupRef ref="ref22"/>
                            <cvParam
                                cvRef="MS"
                                accession="MS:1234619"
                                name="cvParam name 1234619"
                                value="cvParam value 1234619"
                                unitAccession="cvParam unitAccession 1234619"
                                unitName="cvParam unitName 1234619"
                                unitCvRef="cvParam unitCvRef 1234619"/>
                            <userParam
                                name="userParam name 1234620"
                                type="userParam type 1234620"
                                value="userParam value 1234620"
                                unitAccession="userParam unitAccession 1234620"
                                unitName="userParam unitName 1234620"
                                unitCvRef="userParam unitCvRef 1234620"/>
                            <precursor
                                spectrumRef="spectrumRef2"
                                sourceFileRef="sourceFileRef5"
                                externalSpectrumID="externalSpectrumID2">
                                <isolationWindow>
                                    <referenceableParamGroupRef ref="ref23"/>
                                    <cvParam
                                        cvRef="MS"
                                        accession="MS:1234621"
                                        name="cvParam name 1234621"
                                        value="cvParam value 1234621"
                                        unitAccession="cvParam unitAccession 1234621"
                                        unitName="cvParam unitName 1234621"
                                        unitCvRef="cvParam unitCvRef 1234621"/>
                                    <userParam
                                        name="userParam name 1234622"
                                        type="userParam type 1234622"
                                        value="userParam value 1234622"
                                        unitAccession="userParam unitAccession 1234622"
                                        unitName="userParam unitName 1234622"
                                        unitCvRef="userParam unitCvRef 1234622"/>
                                </isolationWindow>
                                <selectedIonList count="1">
                                    <selectedIon>
                                        <referenceableParamGroupRef ref="ref24"/>
                                        <cvParam
                                            cvRef="MS"
                                            accession="MS:1234623"
                                            name="cvParam name 1234623"
                                            value="cvParam value 1234623"
                                            unitAccession="cvParam unitAccession 1234623"
                                            unitName="cvParam unitName 1234623"
                                            unitCvRef="cvParam unitCvRef 1234623"/>
                                        <userParam
                                            name="userParam name 1234624"
                                            type="userParam type 1234624"
                                            value="userParam value 1234624"
                                            unitAccession="userParam unitAccession 1234624"
                                            unitName="userParam unitName 1234624"
                                            unitCvRef="userParam unitCvRef 1234624"/>
                                    </selectedIon>
                                </selectedIonList>
                                <activation>
                                    <referenceableParamGroupRef ref="ref25"/>
                                    <cvParam
                                        cvRef="MS"
                                        accession="MS:1234625"
                                        name="cvParam name 1234625"
                                        value="cvParam value 1234625"
                                        unitAccession="cvParam unitAccession 1234625"
                                        unitName="cvParam unitName 1234625"
                                        unitCvRef="cvParam unitCvRef 1234625"/>
                                    <userParam
                                        name="userParam name 1234626"
                                        type="userParam type 1234626"
                                        value="userParam value 1234626"
                                        unitAccession="userParam unitAccession 1234626"
                                        unitName="userParam unitName 1234626"
                                        unitCvRef="userParam unitCvRef 1234626"/>
                                </activation>
                            </precursor>
                            <product>
                                <isolationWindow>
                                    <referenceableParamGroupRef ref="ref26"/>
                                    <cvParam
                                        cvRef="MS"
                                        accession="MS:1234627"
                                        name="cvParam name 1234627"
                                        value="cvParam value 1234627"
                                        unitAccession="cvParam unitAccession 1234627"
                                        unitName="cvParam unitName 1234627"
                                        unitCvRef="cvParam unitCvRef 1234627"/>
                                    <userParam
                                        name="userParam name 1234628"
                                        type="userParam type 1234628"
                                        value="userParam value 1234628"
                                        unitAccession="userParam unitAccession 1234628"
                                        unitName="userParam unitName 1234628"
                                        unitCvRef="userParam unitCvRef 1234628"/>
                                </isolationWindow>
                            </product>
                            <binaryDataArrayList count="2">
                                <binaryDataArray arrayLength="3" dataProcessingRef="dataProcessingRef4" encodedLength="32">
                                    <referenceableParamGroupRef ref="ref27"/>
                                    <cvParam
                                        cvRef="MS"
                                        accession="MS:1234629"
                                        name="cvParam name 1234629"
                                        value="cvParam value 1234629"
                                        unitAccession="cvParam unitAccession 1234629"
                                        unitName="cvParam unitName 1234629"
                                        unitCvRef="cvParam unitCvRef 1234629"/>
                                    <userParam
                                        name="userParam name 1234630"
                                        type="userParam type 1234630"
                                        value="userParam value 1234630"
                                        unitAccession="userParam unitAccession 1234630"
                                        unitName="userParam unitName 1234630"
                                        unitCvRef="userParam unitCvRef 1234630"/>
                                    <!-- [1.0, 2.0, 3.0] little endian doubles base 64 encoded -->
                                    <binary>P/AAAAAAAABAAAAAAAAAAEAIAAAAAAAA</binary>
                                </binaryDataArray>
                                <binaryDataArray arrayLength="3" dataProcessingRef="dataProcessingRef5" encodedLength="32">
                                    <referenceableParamGroupRef ref="ref28"/>
                                    <cvParam
                                        cvRef="MS"
                                        accession="MS:1234631"
                                        name="cvParam name 1234631"
                                        value="cvParam value 1234631"
                                        unitAccession="cvParam unitAccession 1234631"
                                        unitName="cvParam unitName 1234631"
                                        unitCvRef="cvParam unitCvRef 1234631"/>
                                    <userParam
                                        name="userParam name 1234632"
                                        type="userParam type 1234632"
                                        value="userParam value 1234632"
                                        unitAccession="userParam unitAccession 1234632"
                                        unitName="userParam unitName 1234632"
                                        unitCvRef="userParam unitCvRef 1234632"/>
                                    <!-- [4.0, 5.0, 6.0] little endian doubles base 64 encoded -->
                                    <binary>QBAAAAAAAABAFAAAAAAAAEAYAAAAAAAA</binary>
                                </binaryDataArray>
                            </binaryDataArrayList>
                        </chromatogram>
                    </chromatogramList>
                </run>
            </mzML>"#;
        let reader = Cursor::new(xml);
        let mzml = MzMlParser::parse(path, reader).unwrap();

        assert_eq!(Some("SF:0123456".to_owned()), mzml.accession);
        assert_eq!("1.1.0", mzml.version);
        assert_eq!(
            Some("sciformats:all_optional:valid.mzML".to_owned()),
            mzml.id
        );

        let cv_list = &mzml.cv_list;
        assert_eq!(2, cv_list.count);
        assert_eq!(2, cv_list.cv.len());
        assert_eq!(Cv {
            id: "MS".to_owned(),
            full_name: "Proteomics Standards Initiative Mass Spectrometry Ontology".to_owned(),
            version: "2.26.0".to_owned(),
            uri: "http://psidev.cvs.sourceforge.net/*checkout*/psidev/psi/psi-ms/mzML/controlledVocabulary/psi-ms.obo".to_owned(),
        }, cv_list.cv[0]);
        assert_eq!(
            Cv {
                id: "UO".to_owned(),
                full_name: "Unit Ontology".to_owned(),
                version: "14:07:2009".to_owned(),
                uri:
                    "http://obo.cvs.sourceforge.net/*checkout*/obo/obo/ontology/phenotype/unit.obo"
                        .to_owned(),
            },
            cv_list.cv[1]
        );

        let file_description = &mzml.file_description;
        let file_content = &file_description.file_content;
        assert_eq!(2, file_content.cv_param.len());
        assert_eq!(
            file_content.cv_param[0],
            CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234567".to_owned(),
                name: "cvParam name 1234567".to_owned(),
                value: Some("1234567".to_owned()),
                unit_accession: Some("unitAccession 1234567".to_owned()),
                unit_name: Some("unitName 1234567".to_owned()),
                unit_cv_ref: Some("unitCvRef 1234567".to_owned()),
            }
        );
        assert_eq!(
            file_content.cv_param[1],
            CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234568".to_owned(),
                name: "cvParam name 1234568".to_owned(),
                value: Some("1234568".to_owned()),
                unit_accession: Some("unitAccession 1234568".to_owned()),
                unit_name: Some("unitName 1234568".to_owned()),
                unit_cv_ref: Some("unitCvRef 1234568".to_owned()),
            }
        );

        let source_file_list = file_description.source_file_list.as_ref().unwrap();
        assert_eq!(1, source_file_list.count);
        assert_eq!(1, source_file_list.source_file.len());
        assert_eq!(
            source_file_list.source_file[0],
            SourceFile {
                id: "sourceFile_id0".to_owned(),
                name: "source_file.raw".to_owned(),
                location: "file:///path/to/location/".to_owned(),
                referenceable_param_group_ref: vec![ReferenceableParamGroupRef {
                    r#ref: "ref0".to_owned(),
                }],
                cv_param: vec![CvParam {
                    cv_ref: "MS".to_owned(),
                    accession: "MS:1234570".to_owned(),
                    name: "cvParam name 1234570".to_owned(),
                    value: Some("cvParam value 1234570".to_owned()),
                    unit_accession: Some("cvParam unitAccession 1234570".to_owned()),
                    unit_name: Some("cvParam unitName 1234570".to_owned()),
                    unit_cv_ref: Some("cvParam unitCvRef 1234570".to_owned()),
                }],
                user_param: vec![UserParam {
                    name: "userParam name 1234571".to_owned(),
                    r#type: Some("userParam type 1234571".to_owned()),
                    value: Some("userParam value 1234571".to_owned()),
                    unit_accession: Some("userParam unitAccession 1234571".to_owned()),
                    unit_name: Some("userParam unitName 1234571".to_owned()),
                    unit_cv_ref: Some("userParam unitCvRef 1234571".to_owned()),
                }],
            }
        );

        let contacts = &file_description.contact;
        assert_eq!(1, contacts.len());
        assert_eq!(
            ParamGroup {
                referenceable_param_group_ref: vec![ReferenceableParamGroupRef {
                    r#ref: "ref1".to_owned(),
                }],
                cv_param: vec![CvParam {
                    cv_ref: "MS".to_owned(),
                    accession: "MS:1234572".to_owned(),
                    name: "cvParam name 1234572".to_owned(),
                    value: Some("cvParam value 1234572".to_owned()),
                    unit_accession: Some("cvParam unitAccession 1234572".to_owned()),
                    unit_name: Some("cvParam unitName 1234572".to_owned()),
                    unit_cv_ref: Some("cvParam unitCvRef 1234572".to_owned()),
                }],
                user_param: vec![UserParam {
                    name: "userParam name 1234573".to_owned(),
                    r#type: Some("userParam type 1234573".to_owned()),
                    value: Some("userParam value 1234573".to_owned()),
                    unit_accession: Some("userParam unitAccession 1234573".to_owned()),
                    unit_name: Some("userParam unitName 1234573".to_owned()),
                    unit_cv_ref: Some("userParam unitCvRef 1234573".to_owned()),
                }],
            },
            contacts[0]
        );

        let referenceable_param_group_list = &mzml.referenceable_param_group_list.unwrap();
        assert_eq!(2, referenceable_param_group_list.count);
        assert_eq!(
            2,
            referenceable_param_group_list
                .referenceable_param_group
                .len()
        );
        assert_eq!(
            ReferenceableParamGroup {
                id: "referenceableParamGroup0".to_owned(),
                cv_param: vec![CvParam {
                    cv_ref: "MS".to_owned(),
                    accession: "MS:1234574".to_owned(),
                    name: "cvParam name 1234574".to_owned(),
                    value: Some("cvParam value 1234574".to_owned()),
                    unit_accession: Some("cvParam unitAccession 1234574".to_owned()),
                    unit_name: Some("cvParam unitName 1234574".to_owned()),
                    unit_cv_ref: Some("cvParam unitCvRef 1234574".to_owned()),
                }],
                user_param: vec![UserParam {
                    name: "userParam name 1234575".to_owned(),
                    r#type: Some("userParam type 1234575".to_owned()),
                    value: Some("userParam value 1234575".to_owned()),
                    unit_accession: Some("userParam unitAccession 1234575".to_owned()),
                    unit_name: Some("userParam unitName 1234575".to_owned()),
                    unit_cv_ref: Some("userParam unitCvRef 1234575".to_owned()),
                }],
            },
            referenceable_param_group_list.referenceable_param_group[0]
        );
        assert_eq!(
            ReferenceableParamGroup {
                id: "referenceableParamGroup1".to_owned(),
                cv_param: vec![CvParam {
                    cv_ref: "MS".to_owned(),
                    accession: "MS:1234576".to_owned(),
                    name: "cvParam name 1234576".to_owned(),
                    value: Some("cvParam value 1234576".to_owned()),
                    unit_accession: Some("cvParam unitAccession 1234576".to_owned()),
                    unit_name: Some("cvParam unitName 1234576".to_owned()),
                    unit_cv_ref: Some("cvParam unitCvRef 1234576".to_owned()),
                }],
                user_param: vec![UserParam {
                    name: "userParam name 1234577".to_owned(),
                    r#type: Some("userParam type 1234577".to_owned()),
                    value: Some("userParam value 1234577".to_owned()),
                    unit_accession: Some("userParam unitAccession 1234577".to_owned()),
                    unit_name: Some("userParam unitName 1234577".to_owned()),
                    unit_cv_ref: Some("userParam unitCvRef 1234577".to_owned()),
                }],
            },
            referenceable_param_group_list.referenceable_param_group[1]
        );

        let sample_list = &mzml.sample_list.unwrap();
        assert_eq!(1, sample_list.count);
        assert_eq!(1, sample_list.sample.len());
        assert_eq!(
            Sample {
                id: "sample0".to_owned(),
                name: Some("Sample 0".to_owned()),
                referenceable_param_group_ref: vec![ReferenceableParamGroupRef {
                    r#ref: "ref2".to_owned(),
                }],
                cv_param: vec![CvParam {
                    cv_ref: "MS".to_owned(),
                    accession: "MS:1234578".to_owned(),
                    name: "cvParam name 1234578".to_owned(),
                    value: Some("cvParam value 1234578".to_owned()),
                    unit_accession: Some("cvParam unitAccession 1234578".to_owned()),
                    unit_name: Some("cvParam unitName 1234578".to_owned()),
                    unit_cv_ref: Some("cvParam unitCvRef 1234578".to_owned()),
                }],
                user_param: vec![UserParam {
                    name: "userParam name 1234579".to_owned(),
                    r#type: Some("userParam type 1234579".to_owned()),
                    value: Some("userParam value 1234579".to_owned()),
                    unit_accession: Some("userParam unitAccession 1234579".to_owned()),
                    unit_name: Some("userParam unitName 1234579".to_owned()),
                    unit_cv_ref: Some("userParam unitCvRef 1234579".to_owned()),
                }],
            },
            sample_list.sample[0]
        );

        let software_list = &mzml.software_list;
        assert_eq!(2, software_list.count);
        assert_eq!(2, software_list.software.len());
        assert_eq!(
            Software {
                id: "software_id0".to_owned(),
                version: "1.2.3".to_owned(),
                referenceable_param_group_ref: vec![ReferenceableParamGroupRef {
                    r#ref: "ref3".to_owned(),
                }],
                cv_param: vec![CvParam {
                    cv_ref: "MS".to_owned(),
                    accession: "MS:1234580".to_owned(),
                    name: "cvParam name 1234580".to_owned(),
                    value: Some("cvParam value 1234580".to_owned()),
                    unit_accession: Some("cvParam unitAccession 1234580".to_owned()),
                    unit_name: Some("cvParam unitName 1234580".to_owned()),
                    unit_cv_ref: Some("cvParam unitCvRef 1234580".to_owned()),
                }],
                user_param: vec![UserParam {
                    name: "userParam name 1234581".to_owned(),
                    r#type: Some("userParam type 1234581".to_owned()),
                    value: Some("userParam value 1234581".to_owned()),
                    unit_accession: Some("userParam unitAccession 1234581".to_owned()),
                    unit_name: Some("userParam unitName 1234581".to_owned()),
                    unit_cv_ref: Some("userParam unitCvRef 1234581".to_owned()),
                }],
            },
            software_list.software[0]
        );
        assert_eq!(
            Software {
                id: "software_id1".to_owned(),
                version: "0.1.2".to_owned(),
                referenceable_param_group_ref: vec![],
                cv_param: vec![],
                user_param: vec![],
            },
            software_list.software[1]
        );

        let scan_settings_list = &mzml.scan_settings_list.unwrap();
        assert_eq!(1, scan_settings_list.count);
        assert_eq!(1, scan_settings_list.scan_settings.len());
        let scan_settings = &scan_settings_list.scan_settings[0];
        assert_eq!("scanSettings_id0".to_owned(), scan_settings.id);
        assert_eq!(
            scan_settings.referenceable_param_group_ref,
            vec![ReferenceableParamGroupRef {
                r#ref: "ref6".to_owned(),
            }]
        );
        assert_eq!(1, scan_settings.cv_param.len());
        assert_eq!(
            CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234582".to_owned(),
                name: "cvParam name 1234582".to_owned(),
                value: Some("cvParam value 1234582".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234582".to_owned()),
                unit_name: Some("cvParam unitName 1234582".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234582".to_owned()),
            },
            scan_settings.cv_param[0]
        );
        assert_eq!(1, scan_settings.user_param.len());
        assert_eq!(
            UserParam {
                name: "userParam name 1234583".to_owned(),
                r#type: Some("userParam type 1234583".to_owned()),
                value: Some("userParam value 1234583".to_owned()),
                unit_accession: Some("userParam unitAccession 1234583".to_owned()),
                unit_name: Some("userParam unitName 1234583".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234583".to_owned()),
            },
            scan_settings.user_param[0]
        );
        let source_file_ref_list = scan_settings.source_file_ref_list.as_ref().unwrap();
        assert_eq!(1, source_file_ref_list.count);
        assert_eq!(1, source_file_ref_list.source_file_ref.len());
        assert_eq!(
            source_file_ref_list.source_file_ref[0],
            SourceFileRef {
                r#ref: "sourceFileRef0".to_owned(),
            }
        );
        let target_list = scan_settings.target_list.as_ref().unwrap();
        assert_eq!(2, target_list.count);
        assert_eq!(2, target_list.target.len());
        assert_eq!(
            target_list.target[0],
            ParamGroup {
                referenceable_param_group_ref: vec![ReferenceableParamGroupRef {
                    r#ref: "ref4".to_owned(),
                }],
                cv_param: vec![CvParam {
                    cv_ref: "MS".to_owned(),
                    accession: "MS:1234584".to_owned(),
                    name: "cvParam name 1234584".to_owned(),
                    value: Some("cvParam value 1234584".to_owned()),
                    unit_accession: Some("cvParam unitAccession 1234584".to_owned()),
                    unit_name: Some("cvParam unitName 1234584".to_owned()),
                    unit_cv_ref: Some("cvParam unitCvRef 1234584".to_owned()),
                }],
                user_param: vec![UserParam {
                    name: "userParam name 1234585".to_owned(),
                    r#type: Some("userParam type 1234585".to_owned()),
                    value: Some("userParam value 1234585".to_owned()),
                    unit_accession: Some("userParam unitAccession 1234585".to_owned()),
                    unit_name: Some("userParam unitName 1234585".to_owned()),
                    unit_cv_ref: Some("userParam unitCvRef 1234585".to_owned()),
                }],
            }
        );
        assert_eq!(
            target_list.target[1],
            ParamGroup {
                referenceable_param_group_ref: vec![ReferenceableParamGroupRef {
                    r#ref: "ref5".to_owned(),
                }],
                cv_param: vec![CvParam {
                    cv_ref: "MS".to_owned(),
                    accession: "MS:1234586".to_owned(),
                    name: "cvParam name 1234586".to_owned(),
                    value: Some("cvParam value 1234586".to_owned()),
                    unit_accession: Some("cvParam unitAccession 1234586".to_owned()),
                    unit_name: Some("cvParam unitName 1234586".to_owned()),
                    unit_cv_ref: Some("cvParam unitCvRef 1234586".to_owned()),
                }],
                user_param: vec![UserParam {
                    name: "userParam name 1234587".to_owned(),
                    r#type: Some("userParam type 1234587".to_owned()),
                    value: Some("userParam value 1234587".to_owned()),
                    unit_accession: Some("userParam unitAccession 1234587".to_owned()),
                    unit_name: Some("userParam unitName 1234587".to_owned()),
                    unit_cv_ref: Some("userParam unitCvRef 1234587".to_owned()),
                }],
            }
        );

        let instrument_configuration_list = &mzml.instrument_configuration_list;
        assert_eq!(1, instrument_configuration_list.count);
        assert_eq!(
            1,
            instrument_configuration_list.instrument_configuration.len()
        );
        let instrument_configuration = &instrument_configuration_list.instrument_configuration[0];
        assert_eq!(
            "instrumentConfiguration_id0".to_owned(),
            instrument_configuration.id
        );
        assert_eq!(
            Some("scanSettingsRef0".to_owned()),
            instrument_configuration.scan_setting_ref
        );
        assert_eq!(
            instrument_configuration.referenceable_param_group_ref,
            vec![ReferenceableParamGroupRef {
                r#ref: "ref6".to_owned(),
            }]
        );
        assert_eq!(1, instrument_configuration.cv_param.len());
        assert_eq!(
            CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234588".to_owned(),
                name: "cvParam name 1234588".to_owned(),
                value: Some("cvParam value 1234588".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234588".to_owned()),
                unit_name: Some("cvParam unitName 1234588".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234588".to_owned()),
            },
            instrument_configuration.cv_param[0]
        );
        assert_eq!(1, instrument_configuration.user_param.len());
        assert_eq!(
            UserParam {
                name: "userParam name 1234589".to_owned(),
                r#type: Some("userParam type 1234589".to_owned()),
                value: Some("userParam value 1234589".to_owned()),
                unit_accession: Some("userParam unitAccession 1234589".to_owned()),
                unit_name: Some("userParam unitName 1234589".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234589".to_owned()),
            },
            instrument_configuration.user_param[0]
        );
        let component_list = instrument_configuration.component_list.as_ref().unwrap();
        assert_eq!(3, component_list.count);
        assert_eq!(1, component_list.source.len());
        assert_eq!(
            component_list.source[0],
            Component {
                order: 1,
                referenceable_param_group_ref: vec![ReferenceableParamGroupRef {
                    r#ref: "ref7".to_owned(),
                }],
                cv_param: vec![CvParam {
                    cv_ref: "MS".to_owned(),
                    accession: "MS:1234590".to_owned(),
                    name: "cvParam name 1234590".to_owned(),
                    value: Some("cvParam value 1234590".to_owned()),
                    unit_accession: Some("cvParam unitAccession 1234590".to_owned()),
                    unit_name: Some("cvParam unitName 1234590".to_owned()),
                    unit_cv_ref: Some("cvParam unitCvRef 1234590".to_owned()),
                }],
                user_param: vec![UserParam {
                    name: "userParam name 1234591".to_owned(),
                    r#type: Some("userParam type 1234591".to_owned()),
                    value: Some("userParam value 1234591".to_owned()),
                    unit_accession: Some("userParam unitAccession 1234591".to_owned()),
                    unit_name: Some("userParam unitName 1234591".to_owned()),
                    unit_cv_ref: Some("userParam unitCvRef 1234591".to_owned()),
                }],
            }
        );
        assert_eq!(1, component_list.analyzer.len());
        assert_eq!(
            component_list.analyzer[0],
            Component {
                order: 2,
                referenceable_param_group_ref: vec![ReferenceableParamGroupRef {
                    r#ref: "ref8".to_owned(),
                }],
                cv_param: vec![CvParam {
                    cv_ref: "MS".to_owned(),
                    accession: "MS:1234592".to_owned(),
                    name: "cvParam name 1234592".to_owned(),
                    value: Some("cvParam value 1234592".to_owned()),
                    unit_accession: Some("cvParam unitAccession 1234592".to_owned()),
                    unit_name: Some("cvParam unitName 1234592".to_owned()),
                    unit_cv_ref: Some("cvParam unitCvRef 1234592".to_owned()),
                }],
                user_param: vec![UserParam {
                    name: "userParam name 1234593".to_owned(),
                    r#type: Some("userParam type 1234593".to_owned()),
                    value: Some("userParam value 1234593".to_owned()),
                    unit_accession: Some("userParam unitAccession 1234593".to_owned()),
                    unit_name: Some("userParam unitName 1234593".to_owned()),
                    unit_cv_ref: Some("userParam unitCvRef 1234593".to_owned()),
                }],
            }
        );
        assert_eq!(1, component_list.detector.len());
        assert_eq!(
            component_list.detector[0],
            Component {
                order: 3,
                referenceable_param_group_ref: vec![ReferenceableParamGroupRef {
                    r#ref: "ref9".to_owned(),
                }],
                cv_param: vec![CvParam {
                    cv_ref: "MS".to_owned(),
                    accession: "MS:1234594".to_owned(),
                    name: "cvParam name 1234594".to_owned(),
                    value: Some("cvParam value 1234594".to_owned()),
                    unit_accession: Some("cvParam unitAccession 1234594".to_owned()),
                    unit_name: Some("cvParam unitName 1234594".to_owned()),
                    unit_cv_ref: Some("cvParam unitCvRef 1234594".to_owned()),
                }],
                user_param: vec![UserParam {
                    name: "userParam name 1234595".to_owned(),
                    r#type: Some("userParam type 1234595".to_owned()),
                    value: Some("userParam value 1234595".to_owned()),
                    unit_accession: Some("userParam unitAccession 1234595".to_owned()),
                    unit_name: Some("userParam unitName 1234595".to_owned()),
                    unit_cv_ref: Some("userParam unitCvRef 1234595".to_owned()),
                }],
            }
        );
        assert_eq!(
            instrument_configuration.software_ref,
            Some(SoftwareRef {
                r#ref: "softwareRef0".to_owned(),
            })
        );

        let data_processing_list = &mzml.data_processing_list;
        assert_eq!(1, data_processing_list.count);
        assert_eq!(1, data_processing_list.data_processing.len());
        let data_processing = &data_processing_list.data_processing[0];
        assert_eq!("dataProcessing_id0".to_owned(), data_processing.id);
        assert_eq!(1, data_processing.processing_method.len());
        let processing_method = &data_processing.processing_method[0];
        assert_eq!(1, processing_method.order);
        assert_eq!("softwareRef1".to_owned(), processing_method.software_ref);
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref10".to_owned(),
            }],
            processing_method.referenceable_param_group_ref
        );
        assert_eq!(1, processing_method.cv_param.len());
        assert_eq!(
            processing_method.cv_param[0],
            CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234595".to_owned(),
                name: "cvParam name 1234595".to_owned(),
                value: Some("cvParam value 1234595".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234595".to_owned()),
                unit_name: Some("cvParam unitName 1234595".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234595".to_owned()),
            }
        );
        assert_eq!(1, processing_method.user_param.len());
        assert_eq!(
            processing_method.user_param[0],
            UserParam {
                name: "userParam name 1234596".to_owned(),
                r#type: Some("userParam type 1234596".to_owned()),
                value: Some("userParam value 1234596".to_owned()),
                unit_accession: Some("userParam unitAccession 1234596".to_owned()),
                unit_name: Some("userParam unitName 1234596".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234596".to_owned()),
            }
        );

        let run = &mzml.run;
        assert_eq!(run.id, "run_id0");
        assert_eq!(
            "defaultInstrumentConfigurationRef0",
            run.default_instrument_configuration_ref
        );
        assert_eq!(
            Some("defaultSourceFileRef0".to_owned()),
            run.default_source_file_ref
        );
        assert_eq!(Some("sampleRef0".to_owned()), run.sample_ref);
        assert_eq!(
            Some("2026-01-12T07:25:35.12345".to_owned()),
            run.start_time_stamp
        );
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref11".to_owned(),
            }],
            run.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234597".to_owned(),
                name: "cvParam name 1234597".to_owned(),
                value: Some("cvParam value 1234597".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234597".to_owned()),
                unit_name: Some("cvParam unitName 1234597".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234597".to_owned()),
            }],
            run.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234598".to_owned(),
                r#type: Some("userParam type 1234598".to_owned()),
                value: Some("userParam value 1234598".to_owned()),
                unit_accession: Some("userParam unitAccession 1234598".to_owned()),
                unit_name: Some("userParam unitName 1234598".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234598".to_owned()),
            }],
            run.user_param
        );

        let spectrum_list = run.spectrum_list.as_ref().unwrap();
        assert_eq!(2, spectrum_list.count);
        assert_eq!(
            "defaultDataProcessingRef0",
            spectrum_list.default_data_processing_ref,
        );
        assert_eq!(2, spectrum_list.spectrum.len());

        let spectrum0 = &spectrum_list.spectrum[0];
        assert_eq!(spectrum0.id, "spectrum_id0");
        assert_eq!(spectrum0.spot_id.as_deref(), Some("spotID0"));
        assert_eq!(spectrum0.index, 0);
        assert_eq!(spectrum0.default_array_length, 2);
        assert_eq!(
            Some("dataProcessingRef0".to_owned()),
            spectrum0.data_processing_ref
        );
        assert_eq!(Some("sourceFileRef0".to_owned()), spectrum0.source_file_ref);
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref12".to_owned(),
            }],
            spectrum0.referenceable_param_group_ref
        );
        assert_eq!(
            spectrum0.cv_param,
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234599".to_owned(),
                name: "cvParam name 1234599".to_owned(),
                value: Some("cvParam value 1234599".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234599".to_owned()),
                unit_name: Some("cvParam unitName 1234599".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234599".to_owned()),
            }]
        );
        assert_eq!(
            spectrum0.user_param,
            vec![UserParam {
                name: "userParam name 1234600".to_owned(),
                r#type: Some("userParam type 1234600".to_owned()),
                value: Some("userParam value 1234600".to_owned()),
                unit_accession: Some("userParam unitAccession 1234600".to_owned()),
                unit_name: Some("userParam unitName 1234600".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234600".to_owned()),
            }]
        );

        // Test scanList in spectrum0
        let scan_list = spectrum0.scan_list.as_ref().expect("scanList should exist");
        assert_eq!(scan_list.count, 1);
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref13".to_owned(),
            }],
            scan_list.referenceable_param_group_ref
        );
        assert_eq!(
            scan_list.cv_param,
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234601".to_owned(),
                name: "cvParam name 1234601".to_owned(),
                value: Some("cvParam value 1234601".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234601".to_owned()),
                unit_name: Some("cvParam unitName 1234601".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234601".to_owned()),
            }]
        );
        assert_eq!(
            scan_list.user_param,
            vec![UserParam {
                name: "userParam name 1234602".to_owned(),
                r#type: Some("userParam type 1234602".to_owned()),
                value: Some("userParam value 1234602".to_owned()),
                unit_accession: Some("userParam unitAccession 1234602".to_owned()),
                unit_name: Some("userParam unitName 1234602".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234602".to_owned()),
            }]
        );

        // Test scan in scanList
        let scan = &scan_list.scan[0];
        assert_eq!(scan.spectrum_ref, Some("spectrumRef0".to_owned()));
        assert_eq!(scan.source_file_ref, Some("sourceFileRef0".to_owned()));
        assert_eq!(
            scan.external_spectrum_id,
            Some("externalSpectrumID0".to_owned())
        );
        assert_eq!(
            scan.instrument_configuration_ref,
            Some("instrumentConfigurationRef0".to_owned())
        );
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref14".to_owned(),
            }],
            scan.referenceable_param_group_ref
        );
        assert_eq!(
            scan.cv_param,
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234603".to_owned(),
                name: "cvParam name 1234603".to_owned(),
                value: Some("cvParam value 1234603".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234603".to_owned()),
                unit_name: Some("cvParam unitName 1234603".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234603".to_owned()),
            }]
        );
        assert_eq!(
            scan.user_param,
            vec![UserParam {
                name: "userParam name 1234604".to_owned(),
                r#type: Some("userParam type 1234604".to_owned()),
                value: Some("userParam value 1234604".to_owned()),
                unit_accession: Some("userParam unitAccession 1234604".to_owned()),
                unit_name: Some("userParam unitName 1234604".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234604".to_owned()),
            }]
        );

        let scan_window_list = scan
            .scan_window_list
            .as_ref()
            .expect("scanWindowList should exist");
        assert_eq!(scan_window_list.count, 1);
        let scan_window = &scan_window_list.scan_window[0];
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref15".to_owned(),
            }],
            scan_window.referenceable_param_group_ref
        );
        assert_eq!(
            scan_window.cv_param,
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234605".to_owned(),
                name: "cvParam name 1234605".to_owned(),
                value: Some("cvParam value 1234605".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234605".to_owned()),
                unit_name: Some("cvParam unitName 1234605".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234605".to_owned()),
            }]
        );
        assert_eq!(
            scan_window.user_param,
            vec![UserParam {
                name: "userParam name 1234606".to_owned(),
                r#type: Some("userParam type 1234606".to_owned()),
                value: Some("userParam value 1234606".to_owned()),
                unit_accession: Some("userParam unitAccession 1234606".to_owned()),
                unit_name: Some("userParam unitName 1234606".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234606".to_owned()),
            }]
        );

        let precursor_list = spectrum0.precursor_list.as_ref().unwrap();
        assert_eq!(1, precursor_list.count);
        assert_eq!(1, precursor_list.precursor.len());
        let precursor0 = &precursor_list.precursor[0];
        assert_eq!(Some("spectrumRef1".to_owned()), precursor0.spectrum_ref);
        assert_eq!(
            Some("sourceFileRef4".to_owned()),
            precursor0.source_file_ref
        );
        assert_eq!(
            Some("externalSpectrumID1".to_owned()),
            precursor0.external_spectrum_id
        );
        let isolation_window0 = precursor0.isolation_window.as_ref().unwrap();
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref16".to_owned(),
            }],
            isolation_window0.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234607".to_owned(),
                name: "cvParam name 1234607".to_owned(),
                value: Some("cvParam value 1234607".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234607".to_owned()),
                unit_name: Some("cvParam unitName 1234607".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234607".to_owned()),
            }],
            isolation_window0.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234608".to_owned(),
                r#type: Some("userParam type 1234608".to_owned()),
                value: Some("userParam value 1234608".to_owned()),
                unit_accession: Some("userParam unitAccession 1234608".to_owned()),
                unit_name: Some("userParam unitName 1234608".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234608".to_owned()),
            }],
            isolation_window0.user_param
        );
        let selected_ion_list = precursor0.selected_ion_list.as_ref().unwrap();
        assert_eq!(1, selected_ion_list.count);
        assert_eq!(1, selected_ion_list.selected_ion.len());
        let selected_ion0 = &selected_ion_list.selected_ion[0];
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref17".to_owned(),
            }],
            selected_ion0.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234609".to_owned(),
                name: "cvParam name 1234609".to_owned(),
                value: Some("cvParam value 1234609".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234609".to_owned()),
                unit_name: Some("cvParam unitName 1234609".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234609".to_owned()),
            }],
            selected_ion0.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234610".to_owned(),
                r#type: Some("userParam type 1234610".to_owned()),
                value: Some("userParam value 1234610".to_owned()),
                unit_accession: Some("userParam unitAccession 1234610".to_owned()),
                unit_name: Some("userParam unitName 1234610".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234610".to_owned()),
            }],
            selected_ion0.user_param
        );
        let activation0 = &precursor0.activation;
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref18".to_owned(),
            }],
            activation0.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234611".to_owned(),
                name: "cvParam name 1234611".to_owned(),
                value: Some("cvParam value 1234611".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234611".to_owned()),
                unit_name: Some("cvParam unitName 1234611".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234611".to_owned()),
            }],
            activation0.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234612".to_owned(),
                r#type: Some("userParam type 1234612".to_owned()),
                value: Some("userParam value 1234612".to_owned()),
                unit_accession: Some("userParam unitAccession 1234612".to_owned()),
                unit_name: Some("userParam unitName 1234612".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234612".to_owned()),
            }],
            activation0.user_param
        );

        let product_list0 = spectrum0.product_list.as_ref().unwrap();
        assert_eq!(1, product_list0.count);
        assert_eq!(1, product_list0.product.len());
        let product0 = &product_list0.product[0];
        let product_isolation_window0 = product0.isolation_window.as_ref().unwrap();
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref19".to_owned(),
            }],
            product_isolation_window0.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234613".to_owned(),
                name: "cvParam name 1234613".to_owned(),
                value: Some("cvParam value 1234613".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234613".to_owned()),
                unit_name: Some("cvParam unitName 1234613".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234613".to_owned()),
            }],
            product_isolation_window0.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234614".to_owned(),
                r#type: Some("userParam type 1234614".to_owned()),
                value: Some("userParam value 1234614".to_owned()),
                unit_accession: Some("userParam unitAccession 1234614".to_owned()),
                unit_name: Some("userParam unitName 1234614".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234614".to_owned()),
            }],
            product_isolation_window0.user_param
        );

        let binary_data_array_list0 = spectrum0.binary_data_array_list.as_ref().unwrap();
        assert_eq!(2, binary_data_array_list0.count);
        assert_eq!(2, binary_data_array_list0.binary_data_array.len());
        let binary_data_array0 = &binary_data_array_list0.binary_data_array[0];
        assert_eq!(3, binary_data_array0.array_length.unwrap());
        assert_eq!(
            Some("dataProcessingRef1".to_owned()),
            binary_data_array0.data_processing_ref
        );
        assert_eq!(32, binary_data_array0.encoded_length);
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref20".to_owned(),
            }],
            binary_data_array0.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234615".to_owned(),
                name: "cvParam name 1234615".to_owned(),
                value: Some("cvParam value 1234615".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234615".to_owned()),
                unit_name: Some("cvParam unitName 1234615".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234615".to_owned()),
            }],
            binary_data_array0.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234616".to_owned(),
                r#type: Some("userParam type 1234616".to_owned()),
                value: Some("userParam value 1234616".to_owned()),
                unit_accession: Some("userParam unitAccession 1234616".to_owned()),
                unit_name: Some("userParam unitName 1234616".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234616".to_owned()),
            }],
            binary_data_array0.user_param
        );
        assert_eq!(
            "P/AAAAAAAABAAAAAAAAAAEAIAAAAAAAA",
            binary_data_array0.binary
        );

        let binary_data_array1 = &binary_data_array_list0.binary_data_array[1];
        assert_eq!(3, binary_data_array1.array_length.unwrap());
        assert_eq!(
            Some("dataProcessingRef2".to_owned()),
            binary_data_array1.data_processing_ref
        );
        assert_eq!(32, binary_data_array1.encoded_length);
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref21".to_owned(),
            }],
            binary_data_array1.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234617".to_owned(),
                name: "cvParam name 1234617".to_owned(),
                value: Some("cvParam value 1234617".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234617".to_owned()),
                unit_name: Some("cvParam unitName 1234617".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234617".to_owned()),
            }],
            binary_data_array1.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234618".to_owned(),
                r#type: Some("userParam type 1234618".to_owned()),
                value: Some("userParam value 1234618".to_owned()),
                unit_accession: Some("userParam unitAccession 1234618".to_owned()),
                unit_name: Some("userParam unitName 1234618".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234618".to_owned()),
            }],
            binary_data_array1.user_param
        );
        assert_eq!(
            "QBAAAAAAAABAFAAAAAAAAEAYAAAAAAAA",
            binary_data_array1.binary
        );

        let chromatogram_list = run.chromatogram_list.as_ref().unwrap();
        assert_eq!(1, chromatogram_list.count);
        assert_eq!(1, chromatogram_list.chromatogram.len());
        let chromatogram0 = &chromatogram_list.chromatogram[0];
        assert_eq!("chromatogram_id0", chromatogram0.id);
        assert_eq!(0, chromatogram0.index);
        assert_eq!(3, chromatogram0.default_array_length);
        assert_eq!(
            Some("dataProcessingRef3".to_owned()),
            chromatogram0.data_processing_ref
        );
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref22".to_owned(),
            }],
            chromatogram0.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234619".to_owned(),
                name: "cvParam name 1234619".to_owned(),
                value: Some("cvParam value 1234619".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234619".to_owned()),
                unit_name: Some("cvParam unitName 1234619".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234619".to_owned()),
            }],
            chromatogram0.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234620".to_owned(),
                r#type: Some("userParam type 1234620".to_owned()),
                value: Some("userParam value 1234620".to_owned()),
                unit_accession: Some("userParam unitAccession 1234620".to_owned()),
                unit_name: Some("userParam unitName 1234620".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234620".to_owned()),
            }],
            chromatogram0.user_param
        );

        let precursor = chromatogram0.precursor.as_ref().unwrap();
        assert_eq!(Some("spectrumRef2".to_owned()), precursor.spectrum_ref);
        assert_eq!(Some("sourceFileRef5".to_owned()), precursor.source_file_ref);
        assert_eq!(
            Some("externalSpectrumID2".to_owned()),
            precursor.external_spectrum_id
        );
        let isolation_window = precursor.isolation_window.as_ref().unwrap();
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref23".to_owned(),
            }],
            isolation_window.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234621".to_owned(),
                name: "cvParam name 1234621".to_owned(),
                value: Some("cvParam value 1234621".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234621".to_owned()),
                unit_name: Some("cvParam unitName 1234621".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234621".to_owned()),
            }],
            isolation_window.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234622".to_owned(),
                r#type: Some("userParam type 1234622".to_owned()),
                value: Some("userParam value 1234622".to_owned()),
                unit_accession: Some("userParam unitAccession 1234622".to_owned()),
                unit_name: Some("userParam unitName 1234622".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234622".to_owned()),
            }],
            isolation_window.user_param
        );
        let selected_ion_list = precursor.selected_ion_list.as_ref().unwrap();
        assert_eq!(1, selected_ion_list.count);
        assert_eq!(1, selected_ion_list.selected_ion.len());
        let selected_ion0 = &selected_ion_list.selected_ion[0];
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref24".to_owned(),
            }],
            selected_ion0.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234623".to_owned(),
                name: "cvParam name 1234623".to_owned(),
                value: Some("cvParam value 1234623".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234623".to_owned()),
                unit_name: Some("cvParam unitName 1234623".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234623".to_owned()),
            }],
            selected_ion0.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234624".to_owned(),
                r#type: Some("userParam type 1234624".to_owned()),
                value: Some("userParam value 1234624".to_owned()),
                unit_accession: Some("userParam unitAccession 1234624".to_owned()),
                unit_name: Some("userParam unitName 1234624".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234624".to_owned()),
            }],
            selected_ion0.user_param
        );
        let activation0 = &precursor.activation;
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref25".to_owned(),
            }],
            activation0.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234625".to_owned(),
                name: "cvParam name 1234625".to_owned(),
                value: Some("cvParam value 1234625".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234625".to_owned()),
                unit_name: Some("cvParam unitName 1234625".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234625".to_owned()),
            }],
            activation0.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234626".to_owned(),
                r#type: Some("userParam type 1234626".to_owned()),
                value: Some("userParam value 1234626".to_owned()),
                unit_accession: Some("userParam unitAccession 1234626".to_owned()),
                unit_name: Some("userParam unitName 1234626".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234626".to_owned()),
            }],
            activation0.user_param
        );

        let product = chromatogram0.product.as_ref().unwrap();
        let product_isolation_window = product.isolation_window.as_ref().unwrap();
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref26".to_owned(),
            }],
            product_isolation_window.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234627".to_owned(),
                name: "cvParam name 1234627".to_owned(),
                value: Some("cvParam value 1234627".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234627".to_owned()),
                unit_name: Some("cvParam unitName 1234627".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234627".to_owned()),
            }],
            product_isolation_window.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234628".to_owned(),
                r#type: Some("userParam type 1234628".to_owned()),
                value: Some("userParam value 1234628".to_owned()),
                unit_accession: Some("userParam unitAccession 1234628".to_owned()),
                unit_name: Some("userParam unitName 1234628".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234628".to_owned()),
            }],
            product_isolation_window.user_param
        );

        let binary_data_array_list = &chromatogram0.binary_data_array_list;
        assert_eq!(2, binary_data_array_list.count);
        assert_eq!(2, binary_data_array_list.binary_data_array.len());
        let binary_data_array0 = &binary_data_array_list.binary_data_array[0];
        assert_eq!(3, binary_data_array0.array_length.unwrap());
        assert_eq!(
            Some("dataProcessingRef4".to_owned()),
            binary_data_array0.data_processing_ref
        );
        assert_eq!(32, binary_data_array0.encoded_length);
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref27".to_owned(),
            }],
            binary_data_array0.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234629".to_owned(),
                name: "cvParam name 1234629".to_owned(),
                value: Some("cvParam value 1234629".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234629".to_owned()),
                unit_name: Some("cvParam unitName 1234629".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234629".to_owned()),
            }],
            binary_data_array0.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234630".to_owned(),
                r#type: Some("userParam type 1234630".to_owned()),
                value: Some("userParam value 1234630".to_owned()),
                unit_accession: Some("userParam unitAccession 1234630".to_owned()),
                unit_name: Some("userParam unitName 1234630".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234630".to_owned()),
            }],
            binary_data_array0.user_param
        );
        assert_eq!(
            "P/AAAAAAAABAAAAAAAAAAEAIAAAAAAAA",
            binary_data_array0.binary
        );

        let binary_data_array1 = &binary_data_array_list.binary_data_array[1];
        assert_eq!(3, binary_data_array1.array_length.unwrap());
        assert_eq!(
            Some("dataProcessingRef5".to_owned()),
            binary_data_array1.data_processing_ref
        );
        assert_eq!(32, binary_data_array1.encoded_length);
        assert_eq!(
            vec![ReferenceableParamGroupRef {
                r#ref: "ref28".to_owned(),
            }],
            binary_data_array1.referenceable_param_group_ref
        );
        assert_eq!(
            vec![CvParam {
                cv_ref: "MS".to_owned(),
                accession: "MS:1234631".to_owned(),
                name: "cvParam name 1234631".to_owned(),
                value: Some("cvParam value 1234631".to_owned()),
                unit_accession: Some("cvParam unitAccession 1234631".to_owned()),
                unit_name: Some("cvParam unitName 1234631".to_owned()),
                unit_cv_ref: Some("cvParam unitCvRef 1234631".to_owned()),
            }],
            binary_data_array1.cv_param
        );
        assert_eq!(
            vec![UserParam {
                name: "userParam name 1234632".to_owned(),
                r#type: Some("userParam type 1234632".to_owned()),
                value: Some("userParam value 1234632".to_owned()),
                unit_accession: Some("userParam unitAccession 1234632".to_owned()),
                unit_name: Some("userParam unitName 1234632".to_owned()),
                unit_cv_ref: Some("userParam unitCvRef 1234632".to_owned()),
            }],
            binary_data_array1.user_param
        );
        assert_eq!(
            "QBAAAAAAAABAFAAAAAAAAEAYAAAAAAAA",
            binary_data_array1.binary
        );

        // Test second spectrum (minimal).
        let spectrum1 = &spectrum_list.spectrum[1];
        assert_eq!("spectrum_id1", spectrum1.id);
        assert_eq!(1, spectrum1.index);
        assert_eq!(2, spectrum1.default_array_length);
    }
}
