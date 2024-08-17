use super::{
    jdx_parser::{
        AuditTrail, BrukerRelaxSection, BrukerSpecificParameters, JdxBlock, NTuples, Page,
        PeakAssignments, PeakTable,
    },
    JdxError,
};
use crate::{
    api::{Column, Node, Parameter, PointXy, Reader, SeekBufRead, Table, Value},
    utils::convert_path_to_node_indices,
};
use std::{collections::HashMap, error::Error};

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

    fn is_peak_data(block: &JdxBlock<Box<dyn SeekBufRead>>) -> bool {
        let data_type = block
            .get_ldr("DATATYPE")
            .map(|ldr| ldr.value.as_str())
            .unwrap_or_default()
            .to_lowercase();
        data_type == "mass spectrum"
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
                let n_tuples_indices = &node_indices[(node_index + 1)..];
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
        if node_indices.is_empty() {
            // map NTUPLES record
            let name = n_tuples.data_form.to_owned();

            let mut parameters = vec![];
            for ldr in &n_tuples.ldrs {
                parameters.push(Parameter::from_str_str(&ldr.label, &ldr.value));
            }

            let mut child_node_names = Vec::<String>::new();
            for page in &n_tuples.pages {
                child_node_names.push(Self::map_n_tuples_page_name(page));
            }

            return Ok(Node {
                name,
                parameters,
                data: vec![],
                metadata: vec![],
                table: None,
                child_node_names,
            });
        }

        if node_indices.len() > 1 || node_indices[0] > n_tuples.pages.len() {
            let path = node_indices
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            return Err(JdxError::new(&format!(
                "Illegal indices for reading NTUPLES node: {}",
                path
            )));
        }

        Self::map_n_tuples_page(&n_tuples.pages[node_indices[0]], is_peak_data)
    }

    fn map_n_tuples_page_name(page: &Page<Box<dyn SeekBufRead>>) -> String {
        let mut name = page.page_variables.to_owned();
        if let Some(data_table) = &page.data_table {
            name += " - ";
            name += &data_table.attributes.1.var_name;
        }
        name
    }

    fn map_n_tuples_page(
        page: &Page<Box<dyn SeekBufRead>>,
        is_peak_data: bool,
    ) -> Result<Node, JdxError> {
        let name = Self::map_n_tuples_page_name(page);

        let mut parameters = vec![];
        for ldr in &page.page_ldrs {
            parameters.push(Parameter::from_str_str(&ldr.label, &ldr.value));
        }

        let mut data = vec![];
        let table = match &page.data_table {
            None => None,
            Some(data_table) => {
                if let Some(plot_desc) = &data_table.plot_descriptor {
                    parameters.push(Parameter::from_str_str("Plot Descriptor", plot_desc));
                }

                let d = data_table.get_data()?;
                data = Self::map_xy_data(&d);

                if is_peak_data {
                    Some(Self::map_data_as_peak_table(&d))
                } else {
                    None
                }
            }
        };

        let metadata = Self::map_page_metadata(page, is_peak_data);

        Ok(Node {
            name,
            parameters,
            data,
            metadata,
            table,
            child_node_names: vec![],
        })
    }

    fn map_page_metadata(
        page: &Page<Box<dyn SeekBufRead>>,
        is_peak_data: bool,
    ) -> Vec<(String, String)> {
        let mut metadata = vec![];

        if let Some(data_table) = &page.data_table {
            let page_attributes = &data_table.attributes;
            if let Some(x_units) = &page_attributes.0.units {
                metadata.push(("x.unit".to_owned(), x_units.to_owned()));
            }
            if let Some(y_units) = &page_attributes.1.units {
                metadata.push(("y.unit".to_owned(), y_units.to_owned()));
            }
            if is_peak_data {
                metadata.push(("plot.style".to_owned(), "sticks".to_owned()));
            } else {
                // don't use VAR_NAMEs for labels for MS
                metadata.push(("x.label".to_owned(), page_attributes.0.symbol.to_owned()));
                metadata.push(("y.label".to_owned(), page_attributes.1.symbol.to_owned()));
            }
        }

        metadata
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

        let mut data = Self::map_block_data(block)?;

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

        let metadata = Self::map_block_metadata(block);

        Ok(Node {
            name,
            parameters,
            data,
            metadata,
            table,
            child_node_names,
        })
    }

    fn map_block_data(block: &JdxBlock<Box<dyn SeekBufRead>>) -> Result<Vec<PointXy>, JdxError> {
        let raw_data = if let Some(xy_data) = &block.xy_data {
            xy_data.get_data()?
        } else if let Some(ra_data) = &block.ra_data {
            ra_data.get_data()?
        } else if let Some(xy_points) = &block.xy_points {
            xy_points.get_data()?
        } else {
            vec![]
        };

        Ok(Self::map_xy_data(&raw_data))
    }

    fn map_xy_data(xy_data: &[(f64, f64)]) -> Vec<PointXy> {
        xy_data.iter().map(|(x, y)| PointXy::new(*x, *y)).collect()
    }

    fn map_block_metadata(block: &JdxBlock<Box<dyn SeekBufRead>>) -> Vec<(String, String)> {
        let mut metadata = vec![];

        if let Some(x_units) = block.get_ldr("XUNITS") {
            metadata.push(("x.unit".to_owned(), x_units.value.to_owned()));
        } else if let Some(r_units) = block.get_ldr("RUNITS") {
            metadata.push(("x.unit".to_owned(), r_units.value.to_owned()));
        }

        if let Some(x_unit_kvp) = metadata.get(0) {
            let x_unit = x_unit_kvp.1.to_lowercase();
            if x_unit == "1/cm" || x_unit == "1 / cm" || x_unit == "cm-1" || x_unit == "cm^-1" {
                metadata.push(("x.label".to_owned(), "Wavenumber".to_owned()));

                if let Some(data_type_ldr) = block.get_ldr("DATATYPE") {
                    let data_type = data_type_ldr.value.to_lowercase();
                    if data_type.contains("infrared") || data_type.contains("raman") {
                        metadata.push(("x.reverse".to_owned(), "true".to_owned()));
                    }
                }
            } else if x_unit == "nanometers"
                || x_unit == "micrometers"
                || x_unit == "nm"
                || x_unit == "um"
            {
                metadata.push(("x.label".to_owned(), "Wavelength".to_owned()));
            }
        }

        if let Some(y_units) = block.get_ldr("YUNITS") {
            metadata.push(("y.unit".to_owned(), y_units.value.to_owned()));
        } else if let Some(a_units) = block.get_ldr("AUNITS") {
            metadata.push(("x.unit".to_owned(), a_units.value.to_owned()));
        }

        if Self::is_peak_data(block) {
            metadata.push(("plot.style".to_owned(), "sticks".to_owned()));
        }

        metadata
    }

    fn map_peak_table(peak_table: &PeakTable<Box<dyn SeekBufRead>>) -> Result<Table, JdxError> {
        let peak_table_data = peak_table.get_data()?;
        let has_w = peak_table_data.iter().any(|peak| peak.w.is_some());
        let has_m = peak_table_data.iter().any(|peak| peak.m.is_some());

        let mut column_names = vec![];
        column_names.push(Column {
            key: "x".to_owned(),
            name: "Peak Position".to_owned(),
        });
        column_names.push(Column {
            key: "y".to_owned(),
            name: "Intensity".to_owned(),
        });
        if has_w {
            column_names.push(Column {
                key: "w".to_owned(),
                name: "Width".to_owned(),
            });
        }
        if has_m {
            column_names.push(Column {
                key: "m".to_owned(),
                name: "Multiplicity".to_owned(),
            });
        }

        let mut rows = vec![];
        for peak in &peak_table_data {
            let mut row = HashMap::<String, Value>::new();
            row.insert("x".to_owned(), Value::F64(peak.x));
            row.insert("y".to_owned(), Value::F64(peak.y));
            if let Some(w) = peak.w {
                row.insert("w".to_owned(), Value::F64(w));
            }
            if let Some(m) = &peak.m {
                row.insert("m".to_owned(), Value::String(m.to_owned()));
            }
            rows.push(row);
        }

        Ok(Table { column_names, rows })
    }

    fn map_peak_table_as_data(
        peak_table: &PeakTable<Box<dyn SeekBufRead>>,
    ) -> Result<Vec<PointXy>, JdxError> {
        let data = peak_table
            .get_data()?
            .iter()
            .map(|peak| PointXy::new(peak.x, peak.y))
            .collect();
        Ok(data)
    }

    fn map_data_as_peak_table(xy_data: &[(f64, f64)]) -> Table {
        let column_names = vec![
            Column {
                key: "x".to_owned(),
                name: "Peak Position".to_owned(),
            },
            Column {
                key: "y".to_owned(),
                name: "Intensity".to_owned(),
            },
        ];

        let rows = xy_data
            .iter()
            .map(|(x, y)| {
                HashMap::from([
                    ("x".to_owned(), Value::F64(*x)),
                    ("y".to_owned(), Value::F64(*y)),
                ])
            })
            .collect::<Vec<_>>();

        Table { column_names, rows }
    }

    fn map_peak_assignments(
        peak_assignments: &PeakAssignments<Box<dyn SeekBufRead>>,
    ) -> Result<Table, JdxError> {
        let peak_assignments_data = peak_assignments.get_data()?;
        let has_y = peak_assignments_data.iter().any(|peak| peak.y.is_some());
        let has_w = peak_assignments_data.iter().any(|peak| peak.w.is_some());
        let has_m = peak_assignments_data.iter().any(|peak| peak.m.is_some());

        let mut column_names = vec![];
        column_names.push(Column {
            key: "x".to_owned(),
            name: "Peak Position".to_owned(),
        });
        if has_y {
            column_names.push(Column {
                key: "y".to_owned(),
                name: "Intensity".to_owned(),
            });
        }
        if has_w {
            column_names.push(Column {
                key: "w".to_owned(),
                name: "Width".to_owned(),
            });
        }
        if has_m {
            column_names.push(Column {
                key: "m".to_owned(),
                name: "Multiplicity".to_owned(),
            });
        }
        column_names.push(Column {
            key: "a".to_owned(),
            name: "Assignment".to_owned(),
        });

        let mut rows = vec![];
        for peak in &peak_assignments_data {
            let mut row = HashMap::<String, Value>::new();
            row.insert("x".to_owned(), Value::F64(peak.x));
            if let Some(y) = peak.y {
                row.insert("y".to_owned(), Value::F64(y));
            }
            if let Some(w) = peak.w {
                row.insert("w".to_owned(), Value::F64(w));
            }
            if let Some(m) = &peak.m {
                row.insert("m".to_owned(), Value::String(m.to_owned()));
            }
            row.insert("a".to_owned(), Value::String(peak.a.to_owned()));
            rows.push(row);
        }

        Ok(Table { column_names, rows })
    }

    fn map_audit_trail(audit_trail: &AuditTrail<Box<dyn SeekBufRead>>) -> Result<Node, JdxError> {
        let audit_entries = audit_trail.get_data()?;
        let has_process = audit_entries.iter().any(|entry| entry.process.is_some());
        let has_version = audit_entries.iter().any(|entry| entry.version.is_some());

        let mut column_names = vec![];
        column_names.push(Column {
            key: "number".to_owned(),
            name: "NUMBER".to_owned(),
        });
        column_names.push(Column {
            key: "when".to_owned(),
            name: "WHEN".to_owned(),
        });
        column_names.push(Column {
            key: "who".to_owned(),
            name: "WHO".to_owned(),
        });
        column_names.push(Column {
            key: "where".to_owned(),
            name: "WHERE".to_owned(),
        });
        if has_process {
            column_names.push(Column {
                key: "process".to_owned(),
                name: "PROCESS".to_owned(),
            });
        }
        if has_version {
            column_names.push(Column {
                key: "version".to_owned(),
                name: "VERSION".to_owned(),
            });
        }
        column_names.push(Column {
            key: "what".to_owned(),
            name: "WHAT".to_owned(),
        });

        let mut rows = vec![];
        for entry in &audit_entries {
            let mut row = HashMap::<String, Value>::from([
                ("number".to_owned(), Value::U64(entry.number)),
                ("when".to_owned(), Value::String(entry.when.to_owned())),
                ("who".to_owned(), Value::String(entry.who.to_owned())),
                ("where".to_owned(), Value::String(entry.r#where.to_owned())),
                ("what".to_owned(), Value::String(entry.what.to_owned())),
            ]);
            if let Some(process) = &entry.process {
                row.insert("process".to_owned(), Value::String(process.to_owned()));
            }
            if let Some(version) = &entry.process {
                row.insert("version".to_owned(), Value::String(version.to_owned()));
            }
            rows.push(row);
        }

        Ok(Node {
            name: "AUDITTRAIL".to_owned(),
            parameters: vec![],
            data: vec![],
            metadata: vec![],
            table: Some(Table { column_names, rows }),
            child_node_names: vec![],
        })
    }
}
