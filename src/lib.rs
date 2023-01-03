use std::io::Write;

use crossbeam_channel::{bounded, Receiver};

// LEARN: Add error handling to the function
pub fn find_matches(content: &str, pattern: &str, mut writer: impl Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

pub fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}
