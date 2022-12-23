use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

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
  pinyagrep::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
  assert_eq!(result, b"lorem ipsum\n");
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("grrs")?;

  cmd.arg("foobar").arg("/test/file/doesnt/exist");
  cmd.assert()
    .failure()
    .stderr(predicate::str::contains("Could not read file"));

  Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
  let file = assert_fs::NamedTempFile::new("sample.txt")?;
  file.write_str("A test\nActual content\nMore content\nAnother test")?;

  let mut cmd = Command::cargo_bin("grrs")?;
  cmd.arg("test").arg(file.path());
  cmd.assert()
    .success()
    .stdout(predicate::str::contains("test\nAnother test"));

  Ok(())
}

// LEARN: Add integration tests for passing an empty string as pattern. Adjust the program as needed.