#![allow(unused)]

use ansi_term::Colour::Yellow;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

// #[derive(Debug)]
// struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();
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
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", &args.path.display()))?;
    println!("File content: {}", Yellow.paint(content));
    Ok(())


    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
}
