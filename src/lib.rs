use std::io::{Write};

// LEARN: Add error handling to the function
pub fn find_matches(content: &str, pattern: &str, mut writer: impl Write) {
  for line in content.lines() {
      if line.contains(pattern) {
          writeln!(writer, "{}", line);
      }
  }
}
