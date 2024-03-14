use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{self, Write};

use anyhow::{Context, Result};
use clap::Parser;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}
fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    let args = Cli::parse();

    let content =
        File::open(&args.path).with_context(|| format!("Could not read file {:?}", &args.path))?;
    let lines = BufReader::new(content).lines();

    for line in lines {
        let real_line = line.unwrap();
        if real_line.contains(&args.pattern) {
            writeln!(handle, "{}", real_line)?;
        }
    }

    Ok(())
}
