use assert_cmd::prelude::*;
use std::process::Command;
#[test]
fn test_example() {
    let mut cmd = Command::cargo_bin("sample_main").unwrap();
    cmd.arg("demo");
    cmd.assert().success();
}
