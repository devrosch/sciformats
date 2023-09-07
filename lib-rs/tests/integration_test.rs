use sf_rs::{andi_chrom::AndiChromParser, api::Parser};

use std::{path::Path, fs::File};

#[test]
fn test_andi_chrom_parsing_succeeds() {
    assert_eq!(5, 5);

    // Create a path to the desired file
    let path = Path::new("/home/rob/ACID.CDF");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let chrom = AndiChromParser::parse(path.to_str().unwrap(), file).unwrap();
    // TODO: actually test
    println!("netCDF revision: {}", chrom.admin_data.netcdf_revision);
    println!("{:?}", chrom);
}
