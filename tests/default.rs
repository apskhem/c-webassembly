use std::error::Error;
use assert_cmd::Command;

#[test]
fn basic_syntax() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("c-webassembly")?;

    cmd.arg("tests/samples/simple.cwal").assert().success();

    return Ok(());
}