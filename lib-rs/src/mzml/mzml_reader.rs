use super::mzml_parser::MzMl;
use crate::{
    api::{Node, Parameter, Reader},
    common::SfError,
    mzml::mzml_parser::{
        Cv, CvList, CvParam, FileDescription, ParamGroup, ReferenceableParamGroup,
        ReferenceableParamGroupList, ReferenceableParamGroupRef, Sample, SampleList, SourceFile,
        SourceFileList, UserParam,
    },
    utils::convert_path_to_node_indices,
};
use std::path::Path;

#[allow(dead_code)] // TODO: remove when fully implemented
pub struct MzMlReader {
    path: String,
    file: MzMl,
}

impl MzMlReader {
    pub fn new(path: &str, file: MzMl) -> Self {
        Self {
            path: path.to_owned(),
            file,
        }
    }
}

impl Reader for MzMlReader {
    #[allow(unused_variables)] // TODO: remove when implemented
    fn read(&self, path: &str) -> Result<Node, SfError> {
        let path_indices = convert_path_to_node_indices(path)?;
        let file_path = Path::new(self.path.as_str());
        let name = file_path
            .file_name()
            .map_or("", |f| f.to_str().unwrap_or(""));
        self.file.get_node(name, &path_indices, path)
    }
}

trait NodeMapping {
    fn get_node(&self, name: &str, path: &[usize], context: &str) -> Result<Node, SfError>;
}

impl NodeMapping for MzMl {
    fn get_node(&self, name: &str, path: &[usize], context: &str) -> Result<Node, SfError> {
        match path {
            [] => {
                let name = name.to_owned();
                let parameters = self.get_parameters();
                let child_node_names = self.get_child_node_names();
                Ok(Node {
                    name,
                    parameters,
                    data: vec![],
                    metadata: vec![],
                    table: None,
                    child_node_names,
                })
            }
            [n, tail_path @ ..] => {
                let child_node_names = self.get_child_node_names();
                let child_node_name = match child_node_names.get(*n) {
                    None => return Err(SfError::new(&format!("Illegal path: {}", context))),
                    Some(child_node_name) => child_node_name,
                };
                match child_node_name.as_str() {
                    "cvList" => self.cv_list.get_node("cvList", tail_path, context),
                    "fileDescription" => {
                        self.file_description
                            .get_node("fileDescription", tail_path, context)
                    }
                    "referenceableParamGroupList" => match &self.referenceable_param_group_list {
                        None => Err(SfError::new(&format!(
                            "Internal error for path: {}. referenceableParamGroupList not found.",
                            context
                        ))),
                        Some(referenceable_param_group_list) => referenceable_param_group_list
                            .get_node("referenceableParamGroupList", tail_path, context),
                    },
                    "sampleList" => match &self.sample_list {
                        None => Err(SfError::new(&format!(
                            "Internal error for path: {}. sampleList not found.",
                            context
                        ))),
                        Some(sample_list) => sample_list.get_node("sampleList", tail_path, context),
                    },
                    // TODO: continue
                    // #[serde(rename = "softwareList")]
                    // pub software_list: SoftwareList,
                    // #[serde(rename = "scanSettingsList")]
                    // pub scan_settings_list: Option<ScanSettingsList>,
                    // #[serde(rename = "instrumentConfigurationList")]
                    // pub instrument_configuration_list: InstrumentConfigurationList,
                    // #[serde(rename = "dataProcessingList")]
                    // pub data_processing_list: DataProcessingList,
                    // pub run: Run,
                    _ => Err(SfError::new(&format!(
                        "Not yet implemented for path: {}",
                        context
                    ))),
                    // _ => Err(SfError::new(&format!("Illegal path: {}", context))),
                }
            }
        }
    }
}

impl NodeMapping for CvList {
    fn get_node(&self, name: &str, path: &[usize], context: &str) -> Result<Node, SfError> {
        match path {
            [] => Ok(Self::map_cv_list(self, name)),
            _ => Err(SfError::new(&format!("Illegal path: {}", context))),
        }
    }
}

impl CvList {
    fn map_cv(cv: &Cv) -> Parameter {
        let key = format!("{} ({}, {})", cv.full_name, cv.id, cv.version);
        Parameter::from_str_str(key, &cv.uri)
    }

    fn map_cv_list(cv_list: &CvList, name: &str) -> Node {
        let mut parameters = vec![];
        parameters.push(Parameter::from_str_u64("count", cv_list.count));
        for cv in cv_list.cv.iter() {
            parameters.push(Self::map_cv(cv));
        }
        Node {
            name: name.to_owned(),
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: vec![],
        }
    }
}

impl NodeMapping for FileDescription {
    fn get_node(&self, name: &str, path: &[usize], context: &str) -> Result<Node, SfError> {
        match path {
            [] => Self::map_file_description(self, name),
            [n, tail_path @ ..] => {
                let child_node_names = Self::get_file_description_children(self);
                let child_node_name = match child_node_names.get(*n) {
                    None => return Err(SfError::new(&format!("Illegal path: {}", context))),
                    Some(child_node_name) => child_node_name,
                };
                match child_node_name.as_str() {
                    "fileContent" => self.file_content.get_node(
                        "fileContent",
                        tail_path,
                        &format!("{} > fileContent", context),
                    ),
                    "sourceFileList" => match &self.source_file_list {
                        None => Err(SfError::new(&format!(
                            "Internal error for path: {}. sourceFileList not found.",
                            context
                        ))),
                        Some(source_file_list) => source_file_list.get_node(
                            "sourceFileList",
                            tail_path,
                            &format!("{} > sourceFileList", context),
                        ),
                    },
                    "contact" => map_param_group_list("contact", &self.contact, context),
                    _ => Err(SfError::new(&format!("Illegal path: {}", context))),
                }
            }
        }
    }
}

impl FileDescription {
    fn get_file_description_children(file_description: &FileDescription) -> Vec<String> {
        let mut child_node_names = vec!["fileContent".to_owned()];
        if file_description.source_file_list.is_some() {
            child_node_names.push("sourceFileList".to_owned());
        }
        if file_description.contact.len() > 0 {
            child_node_names.push("contact".to_owned());
        }
        child_node_names
    }

    fn map_file_description(
        file_description: &FileDescription,
        name: &str,
    ) -> Result<Node, SfError> {
        let child_node_names = Self::get_file_description_children(file_description);
        Ok(Node {
            name: name.to_owned(),
            parameters: vec![],
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
        })
    }
}

impl MzMl {
    fn get_parameters(&self) -> Vec<Parameter> {
        let mut parameters = vec![];
        if let Some(xmlns) = &self.xmlns {
            parameters.push(Parameter::from_str_str("xmlns", xmlns));
        }
        if let Some(xmlns_xsi) = &self.xmlns_xsi {
            parameters.push(Parameter::from_str_str("xmlns:xsi", xmlns_xsi));
        }
        if let Some(xsi_schema_location) = &self.xsi_schema_location {
            parameters.push(Parameter::from_str_str(
                "schemaLocation",
                xsi_schema_location,
            ));
        }
        if let Some(accession) = &self.accession {
            parameters.push(Parameter::from_str_str("accession", accession));
        }
        parameters.push(Parameter::from_str_str("version", &self.version));
        if let Some(id) = &self.id {
            parameters.push(Parameter::from_str_str("id", id));
        }
        parameters
    }

    fn get_child_node_names(&self) -> Vec<String> {
        let mut child_node_names = vec![];
        child_node_names.push("cvList".to_owned());
        child_node_names.push("fileDescription".to_owned());
        if let Some(_) = self.referenceable_param_group_list {
            child_node_names.push("referenceableParamGroupList".to_owned());
        }
        if let Some(_) = self.sample_list {
            child_node_names.push("sampleList".to_owned());
        }
        child_node_names.push("softwareList".to_owned());
        if let Some(_) = self.scan_settings_list {
            child_node_names.push("scanSettingsList".to_owned());
        }
        child_node_names.push("instrumentConfigurationList".to_owned());
        child_node_names.push("dataProcessingList".to_owned());
        child_node_names.push("run".to_owned());
        child_node_names
    }
}

impl NodeMapping for ParamGroup {
    fn get_node(&self, name: &str, path: &[usize], context: &str) -> Result<Node, SfError> {
        match path {
            [] => Ok(map_param_group(name, self)),
            _ => Err(SfError::new(&format!("Illegal path: {}", context))),
        }
    }
}

fn map_cv_param(cv_param: &CvParam) -> Parameter {
    let key = format!(
        "{} ({}, {})",
        cv_param.name, cv_param.cv_ref, cv_param.accession
    );
    let unit_description = match (&cv_param.unit_accession, &cv_param.unit_cv_ref) {
        (None, None) => None,
        (Some(unit_accession), None) => Some(format!("unit_accession={}", unit_accession)),
        (None, Some(unit_cv_ref)) => Some(format!("unit_cv_ref={}", unit_cv_ref)),
        (Some(unit_accession), Some(unit_cv_ref)) => Some(format!(
            "unit_accession={}, unit_cv_ref={}",
            unit_accession, unit_cv_ref
        )),
    };
    let value = match (&cv_param.value, &cv_param.unit_name, &unit_description) {
        (None, None, None) => None,
        (Some(value), None, None) => Some(value.to_owned()),
        (None, Some(unit_name), None) => Some(format!("{}", unit_name)),
        (None, None, Some(unit_desc)) => Some(format!("({})", unit_desc)),
        (Some(value), Some(unit_name), None) => Some(format!("{} {}", value, unit_name)),
        (Some(value), None, Some(unit_desc)) => Some(format!("{} ({})", value, unit_desc)),
        (None, Some(unit_name), Some(unit_desc)) => Some(format!("{} ({})", unit_name, unit_desc)),
        (Some(value), Some(unit_name), Some(unit_desc)) => {
            Some(format!("{} {} ({})", value, unit_name, unit_desc))
        }
    };
    match value {
        None => Parameter::from_str(key),
        Some(v) => Parameter::from_str_str(key, &v),
    }
}

fn map_user_param(user_param: &UserParam) -> Parameter {
    let key = match &user_param.r#type {
        None => user_param.name.to_owned(),
        Some(r#type) => format!("{} ({})", &user_param.name, r#type),
    };
    let unit_description = match (&user_param.unit_accession, &user_param.unit_cv_ref) {
        (None, None) => None,
        (Some(unit_accession), None) => Some(format!("unit_accession={}", unit_accession)),
        (None, Some(unit_cv_ref)) => Some(format!("unit_cv_ref={}", unit_cv_ref)),
        (Some(unit_accession), Some(unit_cv_ref)) => Some(format!(
            "unit_accession={}, unit_cv_ref={}",
            unit_accession, unit_cv_ref
        )),
    };
    let value = match (&user_param.value, &user_param.unit_name, &unit_description) {
        (None, None, None) => None,
        (Some(value), None, None) => Some(value.to_owned()),
        (None, Some(unit_name), None) => Some(format!("{}", unit_name)),
        (None, None, Some(unit_desc)) => Some(format!("({})", unit_desc)),
        (Some(value), Some(unit_name), None) => Some(format!("{} {}", value, unit_name)),
        (Some(value), None, Some(unit_desc)) => Some(format!("{} ({})", value, unit_desc)),
        (None, Some(unit_name), Some(unit_desc)) => Some(format!("{} ({})", unit_name, unit_desc)),
        (Some(value), Some(unit_name), Some(unit_desc)) => {
            Some(format!("{} {} ({})", value, unit_name, unit_desc))
        }
    };
    match value {
        None => Parameter::from_str(key),
        Some(v) => Parameter::from_str_str(key, &v),
    }
}

fn map_referenceable_param_group_ref_list(
    referenceable_param_group_ref: &[ReferenceableParamGroupRef],
) -> Vec<Parameter> {
    let mut parameters = vec![];
    for referenceable_param_group_ref in referenceable_param_group_ref.iter() {
        parameters.push(Parameter::from_str_str(
            "referenceableParamGroupRef",
            referenceable_param_group_ref.r#ref.to_owned(),
        ));
    }
    parameters
}

fn map_cv_param_list(cv_params: &[CvParam]) -> Vec<Parameter> {
    let mut parameters = vec![];
    for cv_param in cv_params.iter() {
        parameters.push(map_cv_param(cv_param));
    }
    parameters
}

fn map_user_param_list(user_params: &[UserParam]) -> Vec<Parameter> {
    let mut parameters = vec![];
    for user_param in user_params.iter() {
        parameters.push(map_user_param(user_param));
    }
    parameters
}

fn map_param_group(name: &str, param_group: &ParamGroup) -> Node {
    let mut parameters = vec![];
    parameters.extend(map_referenceable_param_group_ref_list(
        &param_group.referenceable_param_group_ref,
    ));
    parameters.extend(map_cv_param_list(&param_group.cv_param));
    parameters.extend(map_user_param_list(&param_group.user_param));

    Node {
        name: name.to_owned(),
        parameters,
        data: vec![],
        metadata: vec![],
        table: None,
        child_node_names: vec![],
    }
}

impl NodeMapping for SourceFileList {
    fn get_node(&self, name: &str, path: &[usize], context: &str) -> Result<Node, SfError> {
        match path {
            [] => Ok(Self::map_source_file_list(self, name)),
            [n] => {
                let child_node_names = Self::get_source_file_list_children(self);
                let child_node_name = match child_node_names.get(*n) {
                    None => return Err(SfError::new(&format!("Illegal path: {}", context))),
                    Some(child_node_name) => child_node_name,
                };
                match self.source_file.get(*n) {
                    None => {
                        return Err(SfError::new(&format!(
                            "Internal error for path: {}",
                            context
                        )));
                    }
                    Some(source_file) => source_file.get_node(child_node_name, &[], context),
                }
            }
            _ => Err(SfError::new(&format!("Illegal path: {}", context))),
        }
    }
}

impl SourceFileList {
    fn get_source_file_list_children(source_file_list: &SourceFileList) -> Vec<String> {
        let mut child_node_names = vec![];
        for source_file in source_file_list.source_file.iter() {
            child_node_names.push(source_file.get_name());
        }
        child_node_names
    }

    fn map_source_file_list(source_file_list: &SourceFileList, name: &str) -> Node {
        Node {
            name: name.to_owned(),
            parameters: vec![Parameter::from_str_u64("count", source_file_list.count)],
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: Self::get_source_file_list_children(source_file_list),
        }
    }
}

impl NodeMapping for SourceFile {
    fn get_node(&self, name: &str, path: &[usize], context: &str) -> Result<Node, SfError> {
        match path {
            [] => Ok(self.map_source_file(name)),
            _ => Err(SfError::new(&format!("Illegal path: {}", context))),
        }
    }
}

impl SourceFile {
    fn get_name(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    fn map_source_file(&self, name: &str) -> Node {
        let mut parameters = vec![];
        parameters.push(Parameter::from_str_str("id", &self.id));
        parameters.push(Parameter::from_str_str("name", &self.name));
        parameters.push(Parameter::from_str_str("location", &self.location));
        parameters.extend(map_referenceable_param_group_ref_list(
            &self.referenceable_param_group_ref,
        ));
        parameters.extend(map_cv_param_list(&self.cv_param));
        parameters.extend(map_user_param_list(&self.user_param));

        Node {
            name: name.to_owned(),
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: vec![],
        }
    }
}

impl NodeMapping for ReferenceableParamGroupList {
    fn get_node(&self, name: &str, path: &[usize], context: &str) -> Result<Node, SfError> {
        match path {
            [] => Ok(self.map_to_node(name)),
            [n] => {
                let child_node_names = self.get_child_node_names();
                let child_node_name = match child_node_names.get(*n) {
                    None => return Err(SfError::new(&format!("Illegal path: {}", context))),
                    Some(child_node_name) => child_node_name,
                };
                match self.referenceable_param_group.get(*n) {
                    None => {
                        return Err(SfError::new(&format!(
                            "Internal error for path: {}",
                            context
                        )));
                    }
                    Some(child) => child.get_node(child_node_name, &[], context),
                }
            }
            _ => Err(SfError::new(&format!("Illegal path: {}", context))),
        }
    }
}

impl ReferenceableParamGroupList {
    fn get_child_node_names(&self) -> Vec<String> {
        let mut child_node_names = vec![];
        for child in self.referenceable_param_group.iter() {
            child_node_names.push(child.id.clone());
        }
        child_node_names
    }

    fn map_to_node(&self, name: &str) -> Node {
        Node {
            name: name.to_owned(),
            parameters: vec![Parameter::from_str_u64("count", self.count)],
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: self.get_child_node_names(),
        }
    }
}

impl NodeMapping for ReferenceableParamGroup {
    fn get_node(&self, name: &str, path: &[usize], context: &str) -> Result<Node, SfError> {
        match path {
            [] => Ok(self.map_to_node(name)),
            _ => Err(SfError::new(&format!("Illegal path: {}", context))),
        }
    }
}

impl ReferenceableParamGroup {
    fn map_to_node(&self, name: &str) -> Node {
        let mut parameters = vec![];
        parameters.push(Parameter::from_str_str("id", &self.id));
        parameters.extend(map_cv_param_list(&self.cv_param));
        parameters.extend(map_user_param_list(&self.user_param));

        Node {
            name: name.to_owned(),
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: vec![],
        }
    }
}

fn map_param_group_list(
    name: &str,
    param_groups: &[ParamGroup],
    context: &str,
) -> Result<Node, SfError> {
    let mut parameters = vec![];
    for param_group in param_groups {
        parameters.extend(param_group.get_node("", &[], context)?.parameters);
    }
    Ok(Node {
        name: name.to_owned(),
        parameters,
        data: vec![],
        metadata: vec![],
        table: None,
        child_node_names: vec![],
    })
}

impl NodeMapping for SampleList {
    fn get_node(&self, name: &str, path: &[usize], context: &str) -> Result<Node, SfError> {
        match path {
            [] => Ok(self.map_to_node(name)),
            [n] => {
                let child_node_names = self.get_child_node_names();
                let child_node_name = match child_node_names.get(*n) {
                    None => return Err(SfError::new(&format!("Illegal path: {}", context))),
                    Some(child_node_name) => child_node_name,
                };
                match self.sample.get(*n) {
                    None => {
                        return Err(SfError::new(&format!(
                            "Internal error for path: {}",
                            context
                        )));
                    }
                    Some(child) => child.get_node(child_node_name, &[], context),
                }
            }
            _ => Err(SfError::new(&format!("Illegal path: {}", context))),
        }
    }
}

impl SampleList {
    fn get_child_node_names(&self) -> Vec<String> {
        let mut child_node_names = vec![];
        for child in self.sample.iter() {
            let name = match child.name.as_deref() {
                None => child.id.clone(),
                Some(name) => format!("{} ({})", name, child.id),
            };
            child_node_names.push(name);
        }
        child_node_names
    }

    fn map_to_node(&self, name: &str) -> Node {
        Node {
            name: name.to_owned(),
            parameters: vec![Parameter::from_str_u64("count", self.count)],
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: self.get_child_node_names(),
        }
    }
}

impl NodeMapping for Sample {
    fn get_node(&self, name: &str, path: &[usize], context: &str) -> Result<Node, SfError> {
        match path {
            [] => Ok(self.map_to_node(name)),
            _ => Err(SfError::new(&format!("Illegal path: {}", context))),
        }
    }
}

impl Sample {
    fn map_to_node(&self, name: &str) -> Node {
        let mut parameters = vec![];
        parameters.push(Parameter::from_str_str("id", &self.id));
        if let Some(name) = &self.name {
            parameters.push(Parameter::from_str_str("name", name));
        }
        parameters.extend(map_referenceable_param_group_ref_list(
            &self.referenceable_param_group_ref,
        ));
        parameters.extend(map_cv_param_list(&self.cv_param));
        parameters.extend(map_user_param_list(&self.user_param));

        Node {
            name: name.to_owned(),
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mzml::mzml_parser::{
        CvList, DataProcessingList, FileDescription, InstrumentConfigurationList, ParamGroup, Run,
        SoftwareList,
    };

    use super::*;

    fn create_valid_mzml() -> MzMl {
        MzMl {
            xmlns: Some("http://psi.hupo.org/ms/mzml".to_owned()),
            xmlns_xsi: Some("http://www.w3.org/2001/XMLSchema-instance".to_owned()),
            xsi_schema_location: Some(
                "http://psi.hupo.org/ms/mzml http://psi.hupo.org/ms/mzml/schema/mzML1.1.0.xsd"
                    .to_owned(),
            ),
            accession: Some("MS:1000000".to_owned()),
            version: "1.1.0".to_owned(),
            id: Some("ValidID".to_owned()),
            cv_list: CvList {
                count: 0,
                cv: vec![],
            },
            file_description: FileDescription {
                file_content: ParamGroup {
                    referenceable_param_group_ref: vec![],
                    cv_param: vec![],
                    user_param: vec![],
                },
                source_file_list: None,
                contact: vec![],
            },
            referenceable_param_group_list: None,
            sample_list: None,
            software_list: SoftwareList {
                count: 0,
                software: vec![],
            },
            scan_settings_list: None,
            instrument_configuration_list: InstrumentConfigurationList {
                count: 0,
                instrument_configuration: vec![],
            },
            data_processing_list: DataProcessingList {
                count: 0,
                data_processing: vec![],
            },
            run: Run {
                id: "Run1".to_owned(),
                default_instrument_configuration_ref: "".to_owned(),
                spectrum_list: None,
                chromatogram_list: None,
                default_source_file_ref: None,
                sample_ref: None,
                start_time_stamp: None,
                referenceable_param_group_ref: vec![],
                cv_param: vec![],
                user_param: vec![],
            },
        }
    }

    #[test]
    fn maps_valid_mzml() {
        let mzml = create_valid_mzml();
        let reader = MzMlReader::new("valid.mzml", mzml);
        let root_node = reader.read("").unwrap();

        assert_eq!("valid.mzml", root_node.name);
        assert_eq!(
            &Parameter::from_str_str("xmlns", "http://psi.hupo.org/ms/mzml"),
            &root_node.parameters[0]
        );
        assert_eq!(
            &Parameter::from_str_str("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"),
            &root_node.parameters[1]
        );
        assert_eq!(
            &Parameter::from_str_str(
                "schemaLocation",
                "http://psi.hupo.org/ms/mzml http://psi.hupo.org/ms/mzml/schema/mzML1.1.0.xsd"
            ),
            &root_node.parameters[2]
        );
        assert_eq!(
            &Parameter::from_str_str("accession", "MS:1000000"),
            &root_node.parameters[3]
        );
        assert_eq!(
            &Parameter::from_str_str("version", "1.1.0"),
            &root_node.parameters[4]
        );
        assert_eq!(
            &Parameter::from_str_str("id", "ValidID"),
            &root_node.parameters[5]
        );
        assert!(root_node.data.is_empty());
        assert!(root_node.metadata.is_empty());
        assert!(root_node.table.is_none());
        let root_node_child_noode_names = &root_node.child_node_names;
        assert_eq!(6, root_node_child_noode_names.len());
        assert_eq!("cvList", &root_node_child_noode_names[0]);
        assert_eq!("fileDescription", &root_node_child_noode_names[1]);
        assert_eq!("softwareList", &root_node_child_noode_names[2]);
        assert_eq!(
            "instrumentConfigurationList",
            &root_node_child_noode_names[3]
        );
        assert_eq!("dataProcessingList", &root_node_child_noode_names[4]);
        assert_eq!("run", &root_node_child_noode_names[5]);
    }
}
