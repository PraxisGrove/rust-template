#![allow(clippy::panic)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn cli_prints_greeting() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("{{project-name}}")?;

    command
        .arg("workspace")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, workspace!"));

    Ok(())
}
