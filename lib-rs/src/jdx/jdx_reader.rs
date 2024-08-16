use super::{
    jdx_parser::{AuditTrail, BrukerRelaxSection, BrukerSpecificParameters, JdxBlock, NTuples},
    JdxError,
};
use crate::{
    api::{Node, Reader, SeekBufRead},
    jdx::jdx_parser::StringLdr,
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

    fn retrieve_node(&self, node_indices: &[usize]) -> Result<Node, JdxError> {
        let generate_illegal_path_error =
            |node_index: usize, block: &JdxBlock<Box<dyn SeekBufRead>>| -> JdxError {
                let block_title = if let Some(title) = block.get_ldr("TITLE") {
                    title.value.as_str()
                } else {
                    ""
                };
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
        todo!()
    }

    fn map_bruker_specific_parameters(
        section: &BrukerSpecificParameters,
    ) -> Result<Node, JdxError> {
        todo!()
    }

    fn map_n_tuples(
        n_tuples: &NTuples<Box<dyn SeekBufRead>>,
        node_indices: &[usize],
        is_peak_data: bool,
    ) -> Result<Node, JdxError> {
        todo!()
    }

    fn is_peak_data(block: &JdxBlock<Box<dyn SeekBufRead>>) -> bool {
        todo!()
    }

    fn map_audit_trail(audit_trail: &AuditTrail<Box<dyn SeekBufRead>>) -> Result<Node, JdxError> {
        todo!()
    }

    fn map_block(
        block: &JdxBlock<Box<dyn SeekBufRead>>,
        is_peak_data: bool,
    ) -> Result<Node, JdxError> {
        todo!()
    }
}
