use assert_cmd::Command;
use predicates::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parseargs() -> Command {
    Command::cargo_bin("parseargs").unwrap()
}

#[test]
fn test_help_usage_line() {
    parseargs()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Usage: parseargs [OPTIONS] -- [SCRIPT-ARGS]...",
        ));
}

#[test]
fn test_version() {
    parseargs()
        .arg("--version")
        .assert()
        .success()
        .stdout(format!("parseargs {}\n", VERSION));
}
