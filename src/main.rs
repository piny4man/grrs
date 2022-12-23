#![allow(unused)]

use ansi_term::Colour::Yellow;
use anyhow::{Context, Result, Ok};
use clap::Parser;
use std::io::{self, Write};
use indicatif::ProgressBar;
use log::{info, warn};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error, thread, time::Duration};
use crossbeam_channel::{bounded, tick, Receiver, select, internal::select};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

// #[derive(Debug)]
// struct CustomError(String);

fn main() -> Result<()> {
    // COMMENT: Working with ctrl_c cratee
    // ctrlc::set_handler(move || {
    //     println!("Exiting...");
    // })
    // .expect("Error setting Ctrl-C handler");

    // COMMENT: Working with signals
    // let mut signals = Signals::new(&[SIGINT])?;

    // thread::spawn(move || {
    //     for sig in signals.forever() {
    //         println!("Received signal {:?}", sig);
    //         break;
    //     }
    // });

    // COMMENT: Working with channels
    let ctrl_c_events = pinyagrep::ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working!");
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    // Following code does the actual work, and can be interrupted by pressing
    // Ctrl-C. As an example: Let's wait a few seconds.
    thread::sleep(Duration::from_secs(2));

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

    pinyagrep::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
