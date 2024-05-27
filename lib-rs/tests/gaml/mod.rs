mod gaml_parser_tests;
mod gaml_reader_tests;

use super::open_files;

open_files!("resources/", ((GAML_SAMPLE_FILE, "sample_file.gaml"),));
