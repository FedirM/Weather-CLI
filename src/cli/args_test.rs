

use assert_cmd::Command;
use std::error::Error;

#[test]
fn test_help() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("weather")?;
    cmd.arg("--help").assert().success().stderr("");
    Ok(())
}

#[test]
fn test_configure_help() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("weather")?;
    cmd.arg("configure").arg("--help").assert().success().stderr("");
    Ok(())
}