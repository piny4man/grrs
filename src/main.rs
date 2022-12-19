#![allow(unused)]

use ansi_term::Colour::Yellow;
use anyhow::{Context, Result, Ok};
use clap::Parser;
use std::io::{self, Write};
use indicatif::ProgressBar;
use log::{info, warn};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

// #[derive(Debug)]
// struct CustomError(String);

// LEARN: Add error handling to the function
fn find_matches(content: &str, pattern: &str, mut writer: impl Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

fn main() -> Result<()> {
    // USEFUL: run command `env RUST_LOG=info cargo run --bin grrs -- foo test.txt` to see logs
    env_logger::init();
    info!("Starting up");
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    let args = Cli::parse();

    // Progress bar example
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        // do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    // LEARN: Optimize it using BufReader to liberate read the whole file into memory
    // COMMENT: Error handling method 1
    // let result = std::fs::read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => { content }
    //     Err(error) => { return Err(error.into()); }
    // };
    // COMMENT: Error handling method 2
    // let content = std::fs::read_to_string(&args.path)?;

    // COMMENT: Error handling method 3
    // let content = std::fs::read_to_string(&args.path)
    //     .map_err(|err| CustomError(format!("Error reading `{}`: {}", &args.path.display(), err)))?;
    // COMMENT: Error handling method 4
    warn!("Going to check if file exists!");
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", &args.path.display()))?;
    writeln!(handle, "File content: {}", Yellow.paint(&content));
    // Ok(())

    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}

// USEFUL: cargo test
fn answer() -> i32 {
    42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
