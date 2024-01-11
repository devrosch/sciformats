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
        (ANDI_CHROM_QUIRKS, "andi_chrom_quirks.cdf"),
        (ANDI_NON_ANDI_CDF, "non_andi.cdf"),
        (ANDI_NON_CDF_DUMMY, "dummy.cdf"),
        (ANDI_MS_LIBRARY, "andi_ms_library.cdf"),
        (ANDI_MS_CENTROID, "andi_ms_centroid.cdf"),
        (ANDI_MS_CONTINUUM, "andi_ms_continuum.cdf"),
        (ANDI_MS_SID, "andi_ms_sid.cdf"),
    )
);
