

use assert_cmd::Command;
use color_eyre::eyre::Result;

#[test]
fn test_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("weather")?;
    cmd.arg("--help").assert().success().stderr("");
    Ok(())
}

#[test]
fn test_configure_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("weather")?;
    cmd.arg("configure").arg("--help").assert().success().stderr("");
    Ok(())
}