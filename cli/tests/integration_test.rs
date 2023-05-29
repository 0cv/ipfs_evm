use anyhow::Result;
// use load_config::{load, ConfigType, SfdcCredential, SftpCredential};
// use rust_sync_force::response::QueryResponse;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

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
