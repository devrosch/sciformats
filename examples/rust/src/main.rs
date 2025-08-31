use sciformats::api::{Node, SeekRead};
use sciformats::common::ScannerRepository;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open file.
    let file_name = "CompoundFile.jdx";
    let file_path = format!("{}/../_resources/{}", env!("CARGO_MANIFEST_DIR"), file_name);
    let file = File::open(&file_path)?;
    let mut input: Box<dyn SeekRead> = Box::new(file);

    // Read file.
    let repo = ScannerRepository::init_all();
    // Ensure that the file has a supported format.
    assert!(repo.is_recognized(&file_path, &mut input));
    // Get a reader that through which data from the file is retrieved.
    let reader = repo.get_reader(&file_path, input)?;

    // Read the root node.
    let root_node_path = "/";
    let root_node = reader.read(root_node_path)?;
    print_node(root_node_path, &root_node);

    // Read the fourth child node. Indexing starts at 0. There are as many child nodes as elements in the child_node_names list.
    let child3_path = "/3";
    let child3_node = reader.read(child3_path)?;
    print_node(child3_path, &child3_node);

    // Read the first nested child node of the fourth root child node.
    let child30_path = "/3/0";
    let child30_node = reader.read(child30_path)?;
    print_node(child30_path, &child30_node);
    Ok(())
}

fn print_node(path: &str, node: &Node) {
    println!("node path: {path}");
    println!("name: {}", node.name);
    println!("parameters: {:?}", node.parameters);
    println!("data: {:?}", node.data);
    println!("metadata: {:?}", node.metadata);
    println!("table: {:?}", node.table);
    println!("childNodeNames: {:?}", node.child_node_names);
    println!();
}
