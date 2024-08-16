use super::{
    jdx_parser::{
        AuditTrail, BrukerRelaxSection, BrukerSpecificParameters, JdxBlock, NTuples,
        PeakAssignments, PeakTable,
    },
    JdxError,
};
use crate::{
    api::{Node, Parameter, PointXy, Reader, SeekBufRead, Table},
    utils::convert_path_to_node_indices,
};
use std::error::Error;

pub struct JdxReader {
    _path: String,
    file: JdxBlock<Box<dyn SeekBufRead>>,
}

impl Reader for JdxReader {
    fn read(&self, path: &str) -> Result<Node, Box<dyn Error>> {
        let node_indices = convert_path_to_node_indices(path)?;
        Ok(self.retrieve_node(&node_indices)?)
    }
}

impl JdxReader {
    pub fn new(path: &str, file: JdxBlock<Box<dyn SeekBufRead>>) -> Self {
        Self {
            _path: path.to_owned(),
            file,
        }
    }

    fn get_block_name(block: &JdxBlock<Box<dyn SeekBufRead>>) -> &str {
        block
            .get_ldr("TITLE")
            .map(|ldr| ldr.value.as_str())
            .unwrap_or_default()
    }

    fn retrieve_node(&self, node_indices: &[usize]) -> Result<Node, JdxError> {
        let generate_illegal_path_error =
            |node_index: usize, block: &JdxBlock<Box<dyn SeekBufRead>>| -> JdxError {
                let block_title = Self::get_block_name(block);
                JdxError::new(&format!(
                    "Illegal path for reading node. Block: \"{}\", child index: {}",
                    block_title, node_index
                ))
            };

        let mut block = &self.file;
        let mut iteration_index: usize = 0;

        for node_index in node_indices.to_owned() {
            let bruker_relax_start_index = 0usize;
            let bruker_relax_end_index =
                bruker_relax_start_index + block.bruker_relax_sections.len();
            let bruker_params_start_index = bruker_relax_end_index;
            let bruker_params_end_index =
                bruker_params_start_index + block.bruker_specific_parameters.len();
            let n_tuples_index = bruker_params_end_index;
            let audit_trail_index = if block.n_tuples.is_some() {
                n_tuples_index + 1
            } else {
                n_tuples_index
            };
            let child_blocks_start_index = if block.audit_trail.is_some() {
                audit_trail_index + 1
            } else {
                audit_trail_index
            };

            // node_index >= bruker_relax_start_index, always true
            if node_index < bruker_relax_end_index && !block.bruker_relax_sections.is_empty() {
                // Bruker ##$RELAX section
                if iteration_index < node_indices.len() - 1 {
                    // not a leaf node
                    return Err(generate_illegal_path_error(node_index, &block));
                }
                return Self::map_bruker_relax_section(
                    &block.bruker_relax_sections[node_index - bruker_params_start_index],
                );
            }
            if node_index >= bruker_params_start_index
                && node_index < bruker_params_end_index
                && !block.bruker_specific_parameters.is_empty()
            {
                // $$ Bruker specific parameters section
                if iteration_index < node_indices.len() - 1 {
                    // not a leaf node
                    return Err(generate_illegal_path_error(node_index, &block));
                }
                return Self::map_bruker_specific_parameters(
                    &block.bruker_specific_parameters[node_index - bruker_params_start_index],
                );
            }
            if node_index == n_tuples_index && block.n_tuples.is_some() {
                // consider NTUPLES LDR as child node
                let n_tuples_indices = &node_indices[node_index..];
                return Self::map_n_tuples(
                    block.n_tuples.as_ref().unwrap(),
                    n_tuples_indices,
                    Self::is_peak_data(&block),
                );
            }
            if node_index == audit_trail_index && block.audit_trail.is_some() {
                // consider AUDIT TRAIL LDR as child node
                if iteration_index < node_indices.len() - 1 {
                    // not a leaf node
                    return Err(generate_illegal_path_error(node_index, &block));
                }
                return Self::map_audit_trail(block.audit_trail.as_ref().unwrap());
            }
            let child_block = block.blocks.get(node_index - child_blocks_start_index);
            match child_block {
                None => return Err(generate_illegal_path_error(node_index, &block)),
                Some(b) => {
                    block = b;
                    iteration_index += 1;
                }
            }
        }
        // block is leaf node
        Self::map_block(block, Self::is_peak_data(block))
    }

    fn map_bruker_relax_section(section: &BrukerRelaxSection) -> Result<Node, JdxError> {
        let name = section.name.clone();
        // todo: use different param type once available
        let parameters = vec![Parameter::from_str_str("", &section.content)];

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: vec![],
        })
    }

    fn map_bruker_specific_parameters(
        section: &BrukerSpecificParameters,
    ) -> Result<Node, JdxError> {
        let name = section.name.clone();
        let mut parameters = vec![];
        for ldr in &section.content {
            parameters.push(Parameter::from_str_str(&ldr.label, &ldr.value));
        }

        Ok(Node {
            name,
            parameters,
            data: vec![],
            metadata: vec![],
            table: None,
            child_node_names: vec![],
        })
    }

    fn map_n_tuples(
        n_tuples: &NTuples<Box<dyn SeekBufRead>>,
        node_indices: &[usize],
        is_peak_data: bool,
    ) -> Result<Node, JdxError> {
        todo!()
    }

    fn is_peak_data(block: &JdxBlock<Box<dyn SeekBufRead>>) -> bool {
        let data_type = block
            .get_ldr("DATATYPE")
            .map(|ldr| ldr.value.as_str())
            .unwrap_or_default()
            .to_lowercase();
        data_type == "mass spectrum"
    }

    fn map_audit_trail(audit_trail: &AuditTrail<Box<dyn SeekBufRead>>) -> Result<Node, JdxError> {
        todo!()
    }

    fn map_block(
        block: &JdxBlock<Box<dyn SeekBufRead>>,
        is_peak_data: bool,
    ) -> Result<Node, JdxError> {
        let name = Self::get_block_name(block).to_owned();

        let mut parameters = Vec::<Parameter>::new();
        for ldr in &block.ldrs {
            parameters.push(Parameter::from_str_str(
                ldr.label.as_str(),
                ldr.value.as_str(),
            ));
        }

        let mut data = Self::map_data(block)?;

        let table = if let Some(peak_assignments) = &block.peak_assignments {
            Some(Self::map_peak_assignments(peak_assignments)?)
        } else if let Some(peak_table) = &block.peak_table {
            let table = Self::map_peak_table(peak_table)?;
            if is_peak_data && data.is_empty() {
                // for MS, map peak table as data if no other data is present
                data = Self::map_peak_table_as_data(peak_table)?;
            }
            Some(table)
        } else {
            None
        };

        let mut child_node_names = Vec::<String>::new();
        for bruker_relax_section in &block.bruker_relax_sections {
            child_node_names.push(bruker_relax_section.name.clone());
        }
        for bruker_specific_section in &block.bruker_specific_parameters {
            child_node_names.push(bruker_specific_section.name.clone());
        }
        if let Some(n_tuples) = &block.n_tuples {
            // consider NTUPLES LDR as child node
            child_node_names.push(n_tuples.data_form.clone());
        }
        if block.audit_trail.is_some() {
            // consider AUDIT TRAIL LDR as child node
            child_node_names.push("AUDITTRAIL".to_owned());
        }
        for block in &block.blocks {
            child_node_names.push(Self::get_block_name(block).to_owned());
        }

        let metadata = Self::map_block_metadata(block)?;

        Ok(Node {
            name,
            parameters,
            data,
            metadata,
            table,
            child_node_names,
        })
    }

    fn map_data(block: &JdxBlock<Box<dyn SeekBufRead>>) -> Result<Vec<PointXy>, JdxError> {
        todo!()
    }

    fn map_peak_assignments(
        block: &PeakAssignments<Box<dyn SeekBufRead>>,
    ) -> Result<Table, JdxError> {
        todo!()
    }

    fn map_peak_table(block: &PeakTable<Box<dyn SeekBufRead>>) -> Result<Table, JdxError> {
        todo!()
    }

    fn map_peak_table_as_data(
        block: &PeakTable<Box<dyn SeekBufRead>>,
    ) -> Result<Vec<PointXy>, JdxError> {
        todo!()
    }

    fn map_block_metadata(
        block: &JdxBlock<Box<dyn SeekBufRead>>,
    ) -> Result<Vec<(String, String)>, JdxError> {
        todo!()
    }
}
