use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use clap::Parser;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    let content = File::open(&args.path).expect("Could not read file");
    let lines = BufReader::new(content).lines();
    for line in lines {
        let real_line = line.unwrap();
        if real_line.contains(&args.pattern) {
            println!("{}", real_line);
        }
    }
}
