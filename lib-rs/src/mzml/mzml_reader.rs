use super::mzml_parser::MzMl;
use crate::{
    api::{Node, Parameter, Reader},
    common::SfError,
    utils::convert_path_to_node_indices,
};
use std::path::Path;

#[allow(dead_code)] // TODO: remove when fully implemented
pub struct MzMlReader {
    path: String,
    file: MzMl,
}

impl Reader for MzMlReader {
    #[allow(unused_variables)] // TODO: remove when implemented
    fn read(&self, path: &str) -> Result<Node, SfError> {
        let path_indices = convert_path_to_node_indices(path)?;
        match &path_indices[..] {
            [] => Ok(Self::map_root(&self.path, &self.file)?), // "", "/"
            // TODO: implement other paths
            _ => Err(SfError::new(&format!(
                "MzMlReader::read not yet implemented for path: {}",
                &path
            ))),
        }
    }
}

impl MzMlReader {
    pub fn new(path: &str, file: MzMl) -> Self {
        Self {
            path: path.to_owned(),
            file,
        }
    }

    fn map_root(path: &str, mzml: &MzMl) -> Result<Node, SfError> {
        let path = Path::new(path);
        let name = path
            .file_name()
            .map_or("", |f| f.to_str().unwrap_or(""))
            .to_owned();

        let mut parameters = vec![];
        if let Some(xmlns) = &mzml.xmlns {
            parameters.push(Parameter::from_str_str("xmlns", xmlns));
        }
        if let Some(xmlns_xsi) = &mzml.xmlns_xsi {
            parameters.push(Parameter::from_str_str("xmlns:xsi", xmlns_xsi));
        }
        if let Some(xsi_schema_location) = &mzml.xsi_schema_location {
            parameters.push(Parameter::from_str_str(
                "schemaLocation",
                xsi_schema_location,
            ));
        }
        if let Some(accession) = &mzml.accession {
            parameters.push(Parameter::from_str_str("accession", accession));
        }
        parameters.push(Parameter::from_str_str("version", &mzml.version));
        if let Some(id) = &mzml.id {
            parameters.push(Parameter::from_str_str("id", id));
        }

        let mut child_node_names = vec![];
        child_node_names.push("cvList".to_owned());
        child_node_names.push("fileDescription".to_owned());
        if let Some(_) = &mzml.referenceable_param_group_list {
            child_node_names.push("referenceableParamGroupList".to_owned());
        }
        if let Some(_) = &mzml.sample_list {
            child_node_names.push("sampleList".to_owned());
        }
        child_node_names.push("softwareList".to_owned());
        if let Some(_) = &mzml.scan_settings_list {
            child_node_names.push("scanSettingsList".to_owned());
        }
        child_node_names.push("instrumentConfigurationList".to_owned());
        child_node_names.push("dataProcessingList".to_owned());

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names,
        })
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

        assert_eq!(root_node.name, "valid.mzml");
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
        assert_eq!(5, root_node_child_noode_names.len());
        assert_eq!("cvList", &root_node_child_noode_names[0]);
        assert_eq!("fileDescription", &root_node_child_noode_names[1]);
        assert_eq!("softwareList", &root_node_child_noode_names[2]);
        assert_eq!(
            "instrumentConfigurationList",
            &root_node_child_noode_names[3]
        );
        assert_eq!("dataProcessingList", &root_node_child_noode_names[4]);
    }
}
