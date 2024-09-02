mod andi_chrom_parser_tests;
mod andi_chrom_reader_tests;
mod andi_ms_parser_tests;
mod andi_ms_reader_tests;
mod andi_scanner_tests;

use super::open_files;

open_files!(
    "resources/",
    (
        (ANDI_CHROM_VALID, "andi_chrom_valid.cdf"),
        (ANDI_MS_CENTROID, "andi_ms_centroid.cdf"),
    )
);
