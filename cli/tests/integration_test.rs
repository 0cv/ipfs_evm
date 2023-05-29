use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_no_such_file() -> Result<()> {
    let mut cmd = Command::cargo_bin("ipfs-evm")?;

    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn test_file_found() -> Result<()> {
    let mut cmd = Command::cargo_bin("ipfs-evm")?;

    cmd.arg("Cargo.toml");
    cmd.assert().stdout(predicate::str::contains(
        "View Transaction: https://mumbai.polygonscan.com/tx",
    ));

    Ok(())
}
