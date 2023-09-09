use assert_cmd::Command;
use predicates::prelude::*;

mod exec;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parseargs() -> Command {
    Command::cargo_bin("parseargs").unwrap()
}

#[test]
fn test_help_usage_line() {
    // Testing usage line, as the "--" between "[OPTIONS]" and "[SCRIPT-ARGS]"
    // is inserted manually.
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
        .stdout(predicate::str::starts_with(format!(
            "parseargs {} ",
            VERSION
        )));
}

#[test]
fn test_unknown_option() {
    parseargs()
        .arg("--unknown")
        .assert()
        .code(11)
        .stdout("exit 1\n")
        .stderr(predicate::str::contains(
            "error: unexpected argument '--unknown' found",
        ));
}

#[test]
fn test_invalid_remainder_name() {
    // This error message is created by Clap
    exec::test_parseargs_error_msg(
        &["--remainder=rest-array"],
        "error: invalid value 'rest-array' for '--remainder <SHELL-VAR>': Not a valid shell variable or function name\n",
    );

    exec::test_parseargs_error_msg(
        &["--remainder=ä_umlaut"],
        "error: invalid value 'ä_umlaut' for '--remainder <SHELL-VAR>': Not a valid shell variable or function name\n",
    );
}

#[test]
fn test_invalid_argument_callback_name() {
    // This error message is created by Clap
    exec::test_parseargs_error_msg(
        &["--arg-callback=arg-callback"],
        "error: invalid value 'arg-callback' for '--arg-callback <SHELL-FUNC>': Not a valid shell variable or function name\n",
    );
}

#[test]
fn test_invalid_error_callback_name() {
    // This error message is created by Clap
    exec::test_parseargs_error_msg(
        &["--error-callback=error-callback"],
        "error: invalid value 'error-callback' for '--error-callback <SHELL-FUNC>': Not a valid shell variable or function name\n",
    );
}
