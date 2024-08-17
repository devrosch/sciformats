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
use std::{collections::HashMap, error::Error, path::Path};

pub struct JdxReader {
    path: String,
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
            path: path.to_owned(),
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

        for node_index in node_indices.iter().copied() {
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
                    return Err(generate_illegal_path_error(node_index, block));
                }
                return Self::map_bruker_relax_section(
                    &block.bruker_relax_sections[node_index - bruker_relax_start_index],
                );
            }
            if node_index >= bruker_params_start_index
                && node_index < bruker_params_end_index
                && !block.bruker_specific_parameters.is_empty()
            {
                // $$ Bruker specific parameters section
                if iteration_index < node_indices.len() - 1 {
                    // not a leaf node
                    return Err(generate_illegal_path_error(node_index, block));
                }
                return Self::map_bruker_specific_parameters(
                    &block.bruker_specific_parameters[node_index - bruker_params_start_index],
                );
            }
            if node_index == n_tuples_index && block.n_tuples.is_some() {
                // consider NTUPLES LDR as child node
                let n_tuples_indices = &node_indices[(iteration_index + 1)..];
                return Self::map_n_tuples(
                    block.n_tuples.as_ref().unwrap(),
                    n_tuples_indices,
                    Self::is_peak_data(block),
                );
            }
            if node_index == audit_trail_index && block.audit_trail.is_some() {
                // consider AUDIT TRAIL LDR as child node
                if iteration_index < node_indices.len() - 1 {
                    // not a leaf node
                    return Err(generate_illegal_path_error(node_index, block));
                }
                return Self::map_audit_trail(block.audit_trail.as_ref().unwrap());
            }
            let child_block = block.blocks.get(node_index - child_blocks_start_index);
            match child_block {
                None => return Err(generate_illegal_path_error(node_index, block)),
                Some(b) => {
                    block = b;
                    iteration_index += 1;
                }
            }
        }
        // block is leaf node
        let mut block_node = Self::map_block(block, Self::is_peak_data(block))?;
        if iteration_index == 0 {
            // root node
            // replace block node name with file name for root node
            let path = Path::new(&self.path);
            let file_name = path.file_name().and_then(|f| f.to_str());
            if let Some(name) = file_name {
                block_node.name = name.to_owned();
            }
        }
        Ok(block_node)
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

        if let Some(x_unit_kvp) = metadata.first() {
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

        let mut column_names = vec![
            Column {
                key: "number".to_owned(),
                name: "NUMBER".to_owned(),
            },
            Column {
                key: "when".to_owned(),
                name: "WHEN".to_owned(),
            },
            Column {
                key: "who".to_owned(),
                name: "WHO".to_owned(),
            },
            Column {
                key: "where".to_owned(),
                name: "WHERE".to_owned(),
            },
        ];
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
            if let Some(version) = &entry.version {
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

#[cfg(test)]
mod tests {
    use crate::{api::Parser, jdx::jdx_parser::JdxParser};

    use super::*;
    use std::io::{BufReader, Cursor};

    #[test]
    fn maps_valid_jdx_file() {
        let input = b"##TITLE= Root LINK BLOCK\n\
                                ##JCAMP-DX= 4.24\n\
                                ##DATA TYPE= LINK\n\
                                ##BLOCKS= 5\n\
                                ##TITLE= Data XYDATA (PAC) Block\n\
                                ##JCAMP-DX= 4.24\n\
                                ##DATA TYPE= INFRARED SPECTRUM\n\
                                ##XUNITS= 1/CM\n\
                                ##YUNITS= ABSORBANCE\n\
                                ##XFACTOR= 1.0\n\
                                ##YFACTOR= 1.0\n\
                                ##FIRSTX= 450\n\
                                ##LASTX= 451\n\
                                ##NPOINTS= 2\n\
                                ##FIRSTY= 10\n\
                                ##XYDATA= (X++(Y..Y))\n\
                                +450+10\n\
                                +451+11\n\
                                ##END=\n\
                                ##TITLE= Data RADATA (PAC) Block\n\
                                ##JCAMP-DX= 4.24\n\
                                ##DATA TYPE= INFRARED INTERFEROGRAM\n\
                                ##RUNITS= MICROMETERS\n\
                                ##AUNITS= ARBITRARY UNITS\n\
                                ##FIRSTR= 0\n\
                                ##LASTR= 2\n\
                                ##RFACTOR= 1.0\n\
                                ##AFACTOR= 1.0\n\
                                ##NPOINTS= 3\n\
                                ##RADATA= (R++(A..A))\n\
                                +0+10\n\
                                +1+11\n\
                                +2+12\n\
                                ##END=\n\
                                ##TITLE= Data XYPOINTS (AFFN) Block\n\
                                ##JCAMP-DX= 4.24\n\
                                ##DATA TYPE= RAMAN SPECTRUM\n\
                                ##XUNITS= 1/CM\n\
                                ##YUNITS= ABSORBANCE\n\
                                ##FIRSTX= 900.0\n\
                                ##LASTX= 922.0\n\
                                ##XFACTOR= 2.0\n\
                                ##YFACTOR= 10.0\n\
                                ##NPOINTS= 4\n\
                                ##XYPOINTS= (XY..XY)\n\
                                450.0, 10.0; 451.0, 11.0\n\
                                460.0, 20.0; 461.0, 21.0\n\
                                ##END=\n\
                                ##TITLE= NTUPLES Block\n\
                                ##JCAMP-DX= 5.00\n\
                                ##DATA TYPE= NMR SPECTRUM\n\
                                ##NTUPLES= NMR SPECTRUM\n\
                                ##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,    SPECTRUM/IMAG, PAGE NUMBER\n\
                                ##SYMBOL=             X,                R,                I,           N\n\
                                ##VAR_TYPE= INDEPENDENT,        DEPENDENT,        DEPENDENT,        PAGE\n\
                                ##VAR_FORM=        AFFN,             ASDF,             ASDF,        AFFN\n\
                                ##VAR_DIM=            4,                4,                4,           2\n\
                                ##UNITS=             HZ,  ARBITRARY UNITS,  ARBITRARY UNITS,            \n\
                                ##FIRST=            0.1,             50.0,            300.0,           1\n\
                                ##LAST=            0.25,            105.0,            410.0,           2\n\
                                ##MIN=              0.1,             50.0,            300.0,           1\n\
                                ##MAX=             0.25,            105.0,            410.0,           2\n\
                                ##FACTOR=           0.1,              5.0,             10.0,           1\n\
                                ##$CUSTOM_LDR=     VAL1,             VAL2,             VAL3,        VAL4,\n\
                                ##$CUSTOM_LDR2=        ,                 ,             VAL3,        VAL4\n\
                                ##PAGE= N=1\n\
                                ##DATA TABLE= (X++(R..R)), XYDATA   $$ Real data points\n\
                                1.0 +10+11\n\
                                2.0 +20+21\n\
                                ##PAGE= N=2\n\
                                ##DATA TABLE= (X++(I..I)), XYDATA   $$ Imaginary data points\n\
                                1.0 +30+31\n\
                                2.0 +40+41\n\
                                ##END NTUPLES= NMR SPECTRUM\n\
                                ##END=\n\
                                ##TITLE= PEAK TABLE (AFFN) Block\n\
                                ##JCAMP-DX= 4.24\n\
                                ##DATA TYPE= INFRARED PEAK TABLE\n\
                                ##XUNITS= 1/CM\n\
                                ##YUNITS= ABSORBANCE\n\
                                ##FIRSTX= 450.0\n\
                                ##LASTX= 461.0\n\
                                ##XFACTOR= 1.0\n\
                                ##YFACTOR= 1.0\n\
                                ##NPOINTS= 4\n\
                                ##PEAK TABLE= (XY..XY)\n\
                                450.0, 10.0; 451.0, 11.0\n\
                                460.0, 20.0; 461.0, 21.0\n\
                                ##END=\n\
                                ##TITLE= PEAK ASSIGNMENTS (AFFN) Block\n\
                                ##JCAMP-DX= 4.24\n\
                                ##DATA TYPE= NMR PEAK ASSIGNMENTS\n\
                                ##XUNITS= PPM\n\
                                ##YUNITS= ARBITRARY UNITS\n\
                                ##FIRSTX= 450.0\n\
                                ##LASTX= 461.0\n\
                                ##XFACTOR= 1.0\n\
                                ##YFACTOR= 1.0\n\
                                ##NPOINTS= 4\n\
                                ##PEAK ASSIGNMENTS= (XYMA)\n\
                                (450.0, 10.0, S, <1>)\n\
                                (451.0, 11.0, T, <2>)\n\
                                (460.0, 20.0, D, <3>)\n\
                                (461.0, 21.0, Q, <4>)\n\
                                ##END=\n\
                                ##TITLE= MS PEAK TABLE Block\n\
                                ##JCAMP-DX= 5.00\n\
                                ##DATA TYPE= MASS SPECTRUM\n\
                                ##DATA CLASS= PEAK TABLE\n\
                                ##XUNITS= M/Z\n\
                                ##YUNITS= RELATIVE ABUNDANCE\n\
                                ##NPOINTS= 4\n\
                                ##PEAK TABLE= (XY..XY)\n\
                                50.0, 10.0; 51.0, 11.0\n\
                                130.0, 20.0; 131.0, 21.0\n\
                                ##END=\n\
                                ##TITLE= MS NTUPLES PEAK TABLE\n\
                                ##JCAMP-DX= 5.00\n\
                                ##DATA TYPE= MASS SPECTRUM\n\
                                ##DATA CLASS= NTUPLES\n\
                                ##NTUPLES= MASS SPECTRUM\n\
                                ##VAR_NAME= MASS, INTENSITY, RETENTION TIME, \n\
                                ##SYMBOL= X, Y, T\n\
                                ##VAR_TYPE= INDEPENDENT, DEPENDENT, INDEPENDENT\n\
                                ##VAR_FORM= AFFN, AFFN, AFFN\n\
                                ##VAR_DIM= , , 1\n\
                                ##UNITS= M/Z, RELATIVE ABUNDANCE, SECONDS\n\
                                ##FIRST= , , 10\n\
                                ##LAST= , , 10\n\
                                ##PAGE= T=10\n\
                                ##NPOINTS= 4\n\
                                ##DATA TABLE= (XY..XY), PEAKS\n\
                                50.0, 10.0; 51.0, 11.0\n\
                                130.0, 20.0; 131.0, 21.0\n\
                                ##END NTUPLES= MASS SPECTRUM\n\
                                ##END=\n\
                                ##TITLE=\n\
                                ##JCAMPDX= 6.0\n\
                                ##DATA TYPE= NMR SPECTRUM\n\
                                ##OWNER= testuser\n\
                                ##AUDIT TRAIL=  $$ (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)\n\
                                (   1,<2023-08-06 08:00:00.000 +0200>,<testuser>,<loc01>,<proc1>,<SW 1.3>,\n\
                                      <acquisition\n\
                                      line 2\n\
                                      line 3>)\n\
                                (   2,<2023-08-06 08:10:00.000 +0200>,<testuser>,<loc01>,<proc1>,<SW 1.3>,\n\
                                      <raw data processing\n\
                                      line 2\n\
                                      line 3>)\n\
                                (   3,<2023-08-06 08:20:00.000 +0200>,<testuser>,<loc01>,<proc1>,<SW 1.3>,\n\
                                      <raw data processing\n\
                                      line 2\n\
                                      line 3>)\n\
                                $$ ##END=\n\
                                ##END=\n\
                                ##END= $$ Root LINK BLOCK\n\
                                \n";
        let path = "resources/CompoundFile.jdx";
        let cursor = Cursor::new(input);
        let buf_reader = BufReader::new(cursor);
        let buf_input: Box<dyn SeekBufRead> = Box::new(buf_reader);
        let file = JdxParser::parse(&path, buf_input).unwrap();
        let reader = JdxReader::new(&path, file);

        let root_node = &reader.read("/").unwrap();
        assert_eq!("CompoundFile.jdx", root_node.name);
        assert_eq!(4, root_node.parameters.len());
        assert!(root_node
            .parameters
            .contains(&Parameter::from_str_str("TITLE", "Root LINK BLOCK")));
        assert_eq!(4, root_node.parameters.len());
        assert!(root_node.data.is_empty());
        assert_eq!(9, root_node.child_node_names.len());

        let xy_data_node = &reader.read("/0").unwrap();
        assert_eq!("Data XYDATA (PAC) Block", xy_data_node.name);
        assert_eq!(11, xy_data_node.parameters.len());
        assert!(xy_data_node.child_node_names.is_empty());
        let xy_data_node_data = &xy_data_node.data;
        assert_eq!(2, xy_data_node_data.len());
        assert_eq!(PointXy::new(450.0, 10.0), xy_data_node_data[0]);

        let ra_data_node = &reader.read("/1").unwrap();
        assert_eq!("Data RADATA (PAC) Block", ra_data_node.name);
        assert_eq!(10, ra_data_node.parameters.len());
        assert!(ra_data_node.child_node_names.is_empty());
        let ra_data_node_data = &ra_data_node.data;
        assert_eq!(3, ra_data_node_data.len());
        assert_eq!(PointXy::new(0.0, 10.0), ra_data_node_data[0]);

        let xy_points_node = &reader.read("/2").unwrap();
        assert_eq!("Data XYPOINTS (AFFN) Block", xy_points_node.name);
        assert_eq!(10, xy_points_node.parameters.len());
        assert!(xy_points_node.child_node_names.is_empty());
        let xy_points_node_data = &xy_points_node.data;
        assert_eq!(4, xy_points_node_data.len());
        assert_eq!(PointXy::new(900.0, 100.0), xy_points_node_data[0]);

        let n_tuples_block_node = &reader.read("/3").unwrap();
        assert_eq!("NTUPLES Block", n_tuples_block_node.name);
        assert_eq!(3, n_tuples_block_node.parameters.len());
        assert_eq!(1, n_tuples_block_node.child_node_names.len());
        assert_eq!("NMR SPECTRUM", n_tuples_block_node.child_node_names[0]);

        let n_tuples_node = &reader.read("/3/0").unwrap();
        assert_eq!("NMR SPECTRUM", n_tuples_node.name);
        assert_eq!(13, n_tuples_node.parameters.len());
        assert_eq!(
            Parameter::from_str_str(
                "VARNAME",
                "FREQUENCY,    SPECTRUM/REAL,    SPECTRUM/IMAG, PAGE NUMBER"
            ),
            n_tuples_node.parameters[0]
        );
        assert_eq!(
            Parameter::from_str_str(
                "$CUSTOMLDR",
                "VAL1,             VAL2,             VAL3,        VAL4,"
            ),
            n_tuples_node.parameters[11]
        );
        assert_eq!(
            Parameter::from_str_str(
                "$CUSTOMLDR2",
                ",                 ,             VAL3,        VAL4"
            ),
            n_tuples_node.parameters[12]
        );
        assert!(n_tuples_node.data.is_empty());
        assert_eq!(2, n_tuples_node.child_node_names.len());

        let n_tuples_node_page1 = &reader.read("/3/0/0").unwrap();
        assert_eq!("N=1 - SPECTRUM/REAL", n_tuples_node_page1.name);
        assert_eq!(1, n_tuples_node_page1.parameters.len());
        assert_eq!(
            Parameter::from_str_str("Plot Descriptor", "XYDATA"),
            n_tuples_node_page1.parameters[0]
        );
        assert!(n_tuples_node_page1.child_node_names.is_empty());
        let n_tuples_node_page1_metadata = &n_tuples_node_page1.metadata;
        assert_eq!(4, n_tuples_node_page1_metadata.len());
        assert!(n_tuples_node_page1_metadata
            .iter()
            .any(|meta| meta == &("x.unit".to_owned(), "HZ".to_owned())));
        assert!(n_tuples_node_page1_metadata
            .iter()
            .any(|meta| meta == &("y.unit".to_owned(), "ARBITRARY UNITS".to_owned())));
        assert!(n_tuples_node_page1_metadata
            .iter()
            .any(|meta| meta == &("x.label".to_owned(), "X".to_owned())));
        assert!(n_tuples_node_page1_metadata
            .iter()
            .any(|meta| meta == &("y.label".to_owned(), "R".to_owned())));
        let n_tuples_node_page1_data = &n_tuples_node_page1.data;
        assert_eq!(PointXy::new(0.1, 50.0), n_tuples_node_page1_data[0]);
        assert_eq!(PointXy::new(0.25, 105.0), n_tuples_node_page1_data[3]);

        let n_tuples_node_page2 = &reader.read("/3/0/1").unwrap();
        assert_eq!("N=2 - SPECTRUM/IMAG", n_tuples_node_page2.name);
        assert_eq!(1, n_tuples_node_page2.parameters.len());
        assert_eq!(
            Parameter::from_str_str("Plot Descriptor", "XYDATA"),
            n_tuples_node_page2.parameters[0]
        );
        assert!(n_tuples_node_page2.child_node_names.is_empty());
        let n_tuples_node_page2_metadata = &n_tuples_node_page2.metadata;
        assert_eq!(4, n_tuples_node_page2_metadata.len());
        assert!(n_tuples_node_page2_metadata
            .iter()
            .any(|meta| meta == &("x.unit".to_owned(), "HZ".to_owned())));
        assert!(n_tuples_node_page2_metadata
            .iter()
            .any(|meta| meta == &("y.unit".to_owned(), "ARBITRARY UNITS".to_owned())));
        assert!(n_tuples_node_page2_metadata
            .iter()
            .any(|meta| meta == &("x.label".to_owned(), "X".to_owned())));
        assert!(n_tuples_node_page2_metadata
            .iter()
            .any(|meta| meta == &("y.label".to_owned(), "I".to_owned())));
        let n_tuples_node_page2_data = &n_tuples_node_page2.data;
        assert_eq!(PointXy::new(0.1, 300.0), n_tuples_node_page2_data[0]);
        assert_eq!(PointXy::new(0.25, 410.0), n_tuples_node_page2_data[3]);

        let peak_table_node = &reader.read("/4").unwrap();
        assert_eq!("PEAK TABLE (AFFN) Block", peak_table_node.name);
        assert_eq!(10, peak_table_node.parameters.len());
        assert!(peak_table_node.data.is_empty());
        assert!(peak_table_node.child_node_names.is_empty());
        let peak_table_node_table = peak_table_node.table.as_ref().unwrap();
        assert_eq!(2, peak_table_node_table.column_names.len());
        assert_eq!(
            vec![
                Column::new("x", "Peak Position"),
                Column::new("y", "Intensity")
            ],
            peak_table_node_table.column_names
        );
        assert_eq!(
            vec![
                HashMap::from([
                    ("x".to_owned(), Value::F64(450.0)),
                    ("y".to_owned(), Value::F64(10.0)),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(451.0)),
                    ("y".to_owned(), Value::F64(11.0)),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(460.0)),
                    ("y".to_owned(), Value::F64(20.0)),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(461.0)),
                    ("y".to_owned(), Value::F64(21.0)),
                ]),
            ],
            peak_table_node_table.rows
        );

        let peak_assignments_node = &reader.read("/5").unwrap();
        assert_eq!("PEAK ASSIGNMENTS (AFFN) Block", peak_assignments_node.name);
        assert_eq!(10, peak_assignments_node.parameters.len());
        assert!(peak_assignments_node.data.is_empty());
        assert!(peak_assignments_node.child_node_names.is_empty());
        let peak_assignments_node_table = peak_assignments_node.table.as_ref().unwrap();
        assert_eq!(4, peak_assignments_node_table.column_names.len());
        assert_eq!(
            vec![
                Column::new("x", "Peak Position"),
                Column::new("y", "Intensity"),
                Column::new("m", "Multiplicity"),
                Column::new("a", "Assignment"),
            ],
            peak_assignments_node_table.column_names
        );
        assert_eq!(
            vec![
                HashMap::from([
                    ("x".to_owned(), Value::F64(450.0)),
                    ("y".to_owned(), Value::F64(10.0)),
                    ("m".to_owned(), Value::String("S".to_owned())),
                    ("a".to_owned(), Value::String("1".to_owned())),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(451.0)),
                    ("y".to_owned(), Value::F64(11.0)),
                    ("m".to_owned(), Value::String("T".to_owned())),
                    ("a".to_owned(), Value::String("2".to_owned())),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(460.0)),
                    ("y".to_owned(), Value::F64(20.0)),
                    ("m".to_owned(), Value::String("D".to_owned())),
                    ("a".to_owned(), Value::String("3".to_owned())),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(461.0)),
                    ("y".to_owned(), Value::F64(21.0)),
                    ("m".to_owned(), Value::String("Q".to_owned())),
                    ("a".to_owned(), Value::String("4".to_owned())),
                ]),
            ],
            peak_assignments_node_table.rows
        );

        let ms_peak_table_node = &reader.read("/6").unwrap();
        assert_eq!("MS PEAK TABLE Block", ms_peak_table_node.name);
        assert_eq!(7, ms_peak_table_node.parameters.len());
        assert_eq!(3, ms_peak_table_node.metadata.len());
        assert!(ms_peak_table_node.child_node_names.is_empty());
        let ms_peak_table_node_data = &ms_peak_table_node.data;
        assert_eq!(4, ms_peak_table_node_data.len());
        assert_eq!(
            &vec![
                PointXy::new(50.0, 10.0),
                PointXy::new(51.0, 11.0),
                PointXy::new(130.0, 20.0),
                PointXy::new(131.0, 21.0),
            ],
            ms_peak_table_node_data
        );
        let ms_peak_table_node_table = ms_peak_table_node.table.as_ref().unwrap();
        assert_eq!(2, ms_peak_table_node_table.column_names.len());
        assert_eq!(
            vec![
                Column::new("x", "Peak Position"),
                Column::new("y", "Intensity"),
            ],
            ms_peak_table_node_table.column_names
        );
        assert_eq!(
            vec![
                HashMap::from([
                    ("x".to_owned(), Value::F64(50.0)),
                    ("y".to_owned(), Value::F64(10.0)),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(51.0)),
                    ("y".to_owned(), Value::F64(11.0)),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(130.0)),
                    ("y".to_owned(), Value::F64(20.0)),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(131.0)),
                    ("y".to_owned(), Value::F64(21.0)),
                ]),
            ],
            ms_peak_table_node_table.rows
        );
        let ms_peak_table_node_metadata = &ms_peak_table_node.metadata;
        assert_eq!(3, ms_peak_table_node_metadata.len());
        assert!(ms_peak_table_node_metadata
            .iter()
            .any(|meta| meta == &("x.unit".to_owned(), "M/Z".to_owned())));
        assert!(ms_peak_table_node_metadata
            .iter()
            .any(|meta| meta == &("y.unit".to_owned(), "RELATIVE ABUNDANCE".to_owned())));
        assert!(ms_peak_table_node_metadata
            .iter()
            .any(|meta| meta == &("plot.style".to_owned(), "sticks".to_owned())));

        let ms_n_tuples_page_node = &reader.read("/7/0/0").unwrap();
        assert_eq!("T=10 - INTENSITY", ms_n_tuples_page_node.name);
        assert_eq!(2, ms_n_tuples_page_node.parameters.len());
        assert_eq!(3, ms_n_tuples_page_node.metadata.len());
        assert!(ms_n_tuples_page_node.child_node_names.is_empty());
        let ms_n_tuples_page_node_data = &ms_n_tuples_page_node.data;
        assert_eq!(4, ms_n_tuples_page_node_data.len());
        assert_eq!(
            &vec![
                PointXy::new(50.0, 10.0),
                PointXy::new(51.0, 11.0),
                PointXy::new(130.0, 20.0),
                PointXy::new(131.0, 21.0),
            ],
            ms_n_tuples_page_node_data
        );
        let ms_n_tuples_page_node_table = ms_n_tuples_page_node.table.as_ref().unwrap();
        assert_eq!(2, ms_n_tuples_page_node_table.column_names.len());
        assert_eq!(
            vec![
                Column::new("x", "Peak Position"),
                Column::new("y", "Intensity"),
            ],
            ms_n_tuples_page_node_table.column_names
        );
        assert_eq!(
            vec![
                HashMap::from([
                    ("x".to_owned(), Value::F64(50.0)),
                    ("y".to_owned(), Value::F64(10.0)),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(51.0)),
                    ("y".to_owned(), Value::F64(11.0)),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(130.0)),
                    ("y".to_owned(), Value::F64(20.0)),
                ]),
                HashMap::from([
                    ("x".to_owned(), Value::F64(131.0)),
                    ("y".to_owned(), Value::F64(21.0)),
                ]),
            ],
            ms_n_tuples_page_node_table.rows
        );
        let ms_n_tuples_page_node_metadata = &ms_n_tuples_page_node.metadata;
        assert_eq!(3, ms_n_tuples_page_node_metadata.len());
        assert!(ms_n_tuples_page_node_metadata
            .iter()
            .any(|meta| meta == &("x.unit".to_owned(), "M/Z".to_owned())));
        assert!(ms_n_tuples_page_node_metadata
            .iter()
            .any(|meta| meta == &("y.unit".to_owned(), "RELATIVE ABUNDANCE".to_owned())));
        assert!(ms_n_tuples_page_node_metadata
            .iter()
            .any(|meta| meta == &("plot.style".to_owned(), "sticks".to_owned())));

        let audit_trail_node = &reader.read("/8/0").unwrap();
        assert_eq!("AUDITTRAIL", audit_trail_node.name);
        assert!(audit_trail_node.parameters.is_empty());
        assert!(audit_trail_node.data.is_empty());
        assert!(audit_trail_node.metadata.is_empty());
        assert!(audit_trail_node.child_node_names.is_empty());
        let audit_trail_node_table = audit_trail_node.table.as_ref().unwrap();
        assert_eq!(7, audit_trail_node_table.column_names.len());
        assert_eq!(
            vec![
                Column::new("number", "NUMBER"),
                Column::new("when", "WHEN"),
                Column::new("who", "WHO"),
                Column::new("where", "WHERE"),
                Column::new("process", "PROCESS"),
                Column::new("version", "VERSION"),
                Column::new("what", "WHAT"),
            ],
            audit_trail_node_table.column_names
        );
        assert_eq!(
            vec![
                HashMap::from([
                    ("number".to_owned(), Value::U64(1)),
                    (
                        "when".to_owned(),
                        Value::String("2023-08-06 08:00:00.000 +0200".to_owned())
                    ),
                    ("who".to_owned(), Value::String("testuser".to_owned())),
                    ("where".to_owned(), Value::String("loc01".to_owned())),
                    ("process".to_owned(), Value::String("proc1".to_owned())),
                    ("version".to_owned(), Value::String("SW 1.3".to_owned())),
                    (
                        "what".to_owned(),
                        Value::String("acquisition\nline 2\nline 3".to_owned())
                    ),
                ]),
                HashMap::from([
                    ("number".to_owned(), Value::U64(2)),
                    (
                        "when".to_owned(),
                        Value::String("2023-08-06 08:10:00.000 +0200".to_owned())
                    ),
                    ("who".to_owned(), Value::String("testuser".to_owned())),
                    ("where".to_owned(), Value::String("loc01".to_owned())),
                    ("process".to_owned(), Value::String("proc1".to_owned())),
                    ("version".to_owned(), Value::String("SW 1.3".to_owned())),
                    (
                        "what".to_owned(),
                        Value::String("raw data processing\nline 2\nline 3".to_owned())
                    ),
                ]),
                HashMap::from([
                    ("number".to_owned(), Value::U64(3)),
                    (
                        "when".to_owned(),
                        Value::String("2023-08-06 08:20:00.000 +0200".to_owned())
                    ),
                    ("who".to_owned(), Value::String("testuser".to_owned())),
                    ("where".to_owned(), Value::String("loc01".to_owned())),
                    ("process".to_owned(), Value::String("proc1".to_owned())),
                    ("version".to_owned(), Value::String("SW 1.3".to_owned())),
                    (
                        "what".to_owned(),
                        Value::String("raw data processing\nline 2\nline 3".to_owned())
                    ),
                ]),
            ],
            audit_trail_node_table.rows
        );

        assert!(reader.read("/0/0").is_err());
    }

    #[test]
    fn maps_bruker_specific_jdx() {
        let input = b"##TITLE= Bruker Relax Type NMR Spectrum\n\
                                    ##JCAMP-DX= 6.00\n\
                                    ##DATA TYPE= nD NMR SPECTRUM\n\
                                    ##DATA CLASS= NTUPLES\n\
                                    ##ORIGIN= Test\n\
                                    ##OWNER= nmrsu\n\
                                    ##$RELAX= \n\
                                    ##$BRUKER FILE EXP=file_name_1\n\
                                    $$ 1.0\n\
                                    $$ 0.0 1.0 2.0\n\
                                    ##$RELAX= \n\
                                    ##$BRUKER FILE PROC= file_name_2\n\
                                    $$ ##TITLE= Parameter file\n\
                                    $$ ##JCAMPDX= 5.0\n\
                                    $$ $$ c:/nmr/data/somepath\n\
                                    $$ ##$BLKPA= (0..15)\n\
                                    3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 \n\
                                    $$ ##$BLKTR= (0..15)\n\
                                    3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 \n\
                                    $$ ##$DE1= 2\n\
                                    $$ ##END=\n\
                                    ##$RELAX= \n\
                                    $$ Bruker specific parameters\n\
                                    $$ --------------------------\n\
                                    ##$DU= <C:/>\n\
                                    ##$NAME= <Jul11-2023>\n\
                                    ##$AQSEQ= 0\n\
                                    ##$AQ_mod= 3\n\
                                    $$ Bruker specific parameters for F1\n\
                                    $$ ---------------------------------\n\
                                    ##$AMP= (0..31)\n\
                                    100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 \n\
                                    100 100 100 100 100 100 100 100 100 100 100 100 100 100 \n\
                                    ##$AQSEQ= 0\n\
                                    ##$AQ_mod= 2\n\
                                    $$ End of Bruker specific parameters\n\
                                    $$ ---------------------------------\n\
                                    ##NTUPLES= NMR SPECTRUM\n\
                                    ##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,    SPECTRUM/IMAG, PAGE NUMBER\n\
                                    ##SYMBOL=             X,                R,                I,           N\n\
                                    ##VAR_TYPE= INDEPENDENT,        DEPENDENT,        DEPENDENT,        PAGE\n\
                                    ##VAR_FORM=        AFFN,             ASDF,             ASDF,        AFFN\n\
                                    ##VAR_DIM=            4,                4,                4,           2\n\
                                    ##UNITS=             HZ,  ARBITRARY UNITS,  ARBITRARY UNITS,            \n\
                                    ##FIRST=            0.1,             50.0,            300.0,           1\n\
                                    ##LAST=            0.25,            105.0,            410.0,           2\n\
                                    ##MIN=              0.1,             50.0,            300.0,           1\n\
                                    ##MAX=             0.25,            105.0,            410.0,           2\n\
                                    ##FACTOR=           0.1,              5.0,             10.0,           1\n\
                                    ##$CUSTOM_LDR=     VAL1,             VAL2,             VAL3,        VAL4,\n\
                                    ##$CUSTOM_LDR2=        ,                 ,             VAL3,        VAL4\n\
                                    ##PAGE= N=1\n\
                                    ##DATA TABLE= (X++(R..R)), XYDATA   $$ Real data points\n\
                                    1.0 +10+11\n\
                                    2.0 +20+21\n\
                                    ##PAGE= N=2\n\
                                    ##DATA TABLE= (X++(I..I)), XYDATA   $$ Imaginary data points\n\
                                    1.0 +30+31\n\
                                    2.0 +40+41\n\
                                    ##END NTUPLES= NMR SPECTRUM\n\
                                    ##END=";
        let path = "resources/Bruker_specific_relax.jdx";
        let cursor = Cursor::new(input);
        let buf_reader = BufReader::new(cursor);
        let buf_input: Box<dyn SeekBufRead> = Box::new(buf_reader);
        let file = JdxParser::parse(&path, buf_input).unwrap();
        let reader = JdxReader::new(&path, file);

        let root_node = &reader.read("/").unwrap();
        assert_eq!("Bruker_specific_relax.jdx", root_node.name);
        assert_eq!(6, root_node.parameters.len());
        assert!(root_node.parameters.contains(&Parameter::from_str_str(
            "TITLE",
            "Bruker Relax Type NMR Spectrum"
        )));
        assert!(root_node.data.is_empty());
        assert_eq!(5, root_node.child_node_names.len());
        assert_eq!(
            vec![
                "file_name_1",
                "file_name_2",
                "Bruker specific parameters",
                "Bruker specific parameters for F1",
                "NMR SPECTRUM"
            ],
            root_node.child_node_names
        );

        let bruker_relax_node0 = &reader.read("/0").unwrap();
        assert_eq!("file_name_1", bruker_relax_node0.name);
        assert!(bruker_relax_node0.data.is_empty());
        assert!(bruker_relax_node0.child_node_names.is_empty());
        assert_eq!(1, bruker_relax_node0.parameters.len());
        assert_eq!(
            vec![Parameter::from_str_str("", "1.0\n0.0 1.0 2.0\n")],
            bruker_relax_node0.parameters
        );

        let bruker_relax_node1 = &reader.read("/1").unwrap();
        assert_eq!("file_name_2", bruker_relax_node1.name);
        assert!(bruker_relax_node1.data.is_empty());
        assert!(bruker_relax_node1.child_node_names.is_empty());
        assert_eq!(1, bruker_relax_node1.parameters.len());
        assert_eq!(
            vec![Parameter::from_str_str(
                "",
                "##TITLE= Parameter file\n\
                ##JCAMPDX= 5.0\n\
                $$ c:/nmr/data/somepath\n\
                ##$BLKPA= (0..15)\n\
                3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 \n\
                ##$BLKTR= (0..15)\n\
                3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 \n\
                ##$DE1= 2\n\
                ##END=\n"
            )],
            bruker_relax_node1.parameters
        );

        let bruker_params_section = &reader.read("/2").unwrap();
        assert_eq!("Bruker specific parameters", bruker_params_section.name);
        assert!(bruker_params_section.data.is_empty());
        assert!(bruker_params_section.child_node_names.is_empty());
        assert_eq!(4, bruker_params_section.parameters.len());
        assert_eq!(
            vec![
                Parameter::from_str_str("$DU", "<C:/>"),
                Parameter::from_str_str("$NAME", "<Jul11-2023>"),
                Parameter::from_str_str("$AQSEQ", "0"),
                Parameter::from_str_str("$AQMOD", "3"),
            ],
            bruker_params_section.parameters
        );

        let bruker_params_section_f1 = &reader.read("/3").unwrap();
        assert_eq!(
            "Bruker specific parameters for F1",
            bruker_params_section_f1.name
        );
        assert!(bruker_params_section_f1.data.is_empty());
        assert!(bruker_params_section_f1.child_node_names.is_empty());
        assert_eq!(3, bruker_params_section_f1.parameters.len());
        assert_eq!(
            vec![
                Parameter::from_str_str(
                    "$AMP",
                    "(0..31)\n\
                    100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 \n\
                    100 100 100 100 100 100 100 100 100 100 100 100 100 100 "
                ),
                Parameter::from_str_str("$AQSEQ", "0"),
                Parameter::from_str_str("$AQMOD", "2"),
            ],
            bruker_params_section_f1.parameters
        );

        let error_bruker_relax_not_leaf = &reader.read("/0/0").unwrap_err();
        assert!(error_bruker_relax_not_leaf.to_string().contains("Illegal"));

        let error_bruker_params_not_leaf = &reader.read("/0/0").unwrap_err();
        assert!(error_bruker_params_not_leaf.to_string().contains("Illegal"));
    }
}
