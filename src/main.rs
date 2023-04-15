use std::fs;
use tree::tree;

fn main() {
    // Get command line arguments
    let mut args = std::env::args();
    args.next(); // Skip first arg

    // Create tree
    let output = tree(&args.next().unwrap_or(".".to_string()));

    // If filename given in args, write to file, otherwise print
    if let Some(filename) = args.next() {
        fs::write(filename, output).expect("Could not write file");
    } else {
        println!("{}", output);
    }
}
