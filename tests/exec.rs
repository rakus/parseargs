use assert_cmd::Command;

fn parseargs() -> Command {
    Command::cargo_bin("parseargs").unwrap()
}

fn test_parseargs(
    pa_args: &[&str],
    script_args: &[&str],
    exit_code: i32,
    code_lines: &[&str],
    error_lines: &[&str],
) {
    let mut expected_code = String::new();
    for code_line in code_lines {
        expected_code.push_str(code_line);
        expected_code.push('\n');
    }

    let mut expected_error_msg = String::new();
    for error_line in error_lines {
        expected_error_msg.push_str(error_line);
        expected_error_msg.push('\n');
    }

    parseargs()
        .args(pa_args)
        .arg("--")
        .args(script_args)
        .assert()
        .code(exit_code)
        .stderr(expected_error_msg)
        .stdout(expected_code);
}

/// Test the generated code.
///
///  # Arguments
/// * `pa_args` - parseargs arguments (before the `--`)
/// * `script_args` - script arguments (after the `--`)
/// * `code_lines` - the expected shell code lines. All but the `set --...` need a trailing semicolon.
///
#[allow(dead_code)] // because it is not used in ALL tests
pub fn test_code_gen(pa_args: &[&str], script_args: &[&str], code_lines: &[&str]) {
    test_parseargs(pa_args, script_args, 0, code_lines, &[])
}

/// Test an error message from processing script args, resulting in exit code 1
///
///  # Arguments
/// * `pa_args` - parseargs arguments (before the `--`)
/// * `script_args` - script arguments (after the `--`)
/// * `error_msg` - the expected error message on stderr
///
#[allow(dead_code)] // because it is not used in ALL tests
pub fn test_error_msg(pa_args: &[&str], script_args: &[&str], error_msg: &str) {
    test_parseargs(pa_args, script_args, 1, &["exit 1"], &[error_msg])
}

/// Test an error message from processing parseargs args, resulting in exit code 11
///
///  # Arguments
/// * `pa_args` - parseargs arguments (before the `--`)
/// * `error_msg` - the expected error message on stderr
///
pub fn test_parseargs_error_msg(pa_args: &[&str], error_msg: &str) {
    test_parseargs(pa_args, &[], 11, &["exit 1"], &[error_msg])
}
