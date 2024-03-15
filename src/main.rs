use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
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
    env_logger::init();

    let args = Cli::parse();

    info!("Looking for {:?} file...", &args.path);
    let content =
        File::open(&args.path).with_context(|| format!("Could not read file {:?}", &args.path))?;
    let lines = BufReader::new(content).lines();

    let mut n = 0;
    for line in lines {
        let real_line = line.unwrap();
        if real_line.contains(&args.pattern) {
            writeln!(handle, "{}", real_line)?;
            n += 1;
        }
    }

    if n == 0 {
        warn!(
            "No lines found containing the pattern {:?} in file {:?}",
            &args.pattern, &args.path
        );
    }

    Ok(())
}

#[test]
fn find_a_match() -> Result<()> {
    let mut result = Vec::new();
    fn find_matches(content: &str, pattern: &str, mut writer: impl Write) -> Result<()> {
        for line in content.lines() {
            if line.contains(pattern) {
                writeln!(writer, "{}", line)?;
            }
        }
        Ok(())
    }
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
    assert_eq!(result, b"lorem ipsum\n");
    Ok(())
}
