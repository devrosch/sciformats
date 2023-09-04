use sf_rs::parse_andi;

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

    parse_andi(path.to_str().unwrap(), file);
}
