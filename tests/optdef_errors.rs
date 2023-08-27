use assert_cmd::Command;
use predicates::prelude::*;

fn parseargs() -> Command {
    Command::cargo_bin("parseargs").unwrap()
}

fn parser_error_msg_test(opt_def: &str, indent: usize, message: &str) {
    let mut expected_msg = String::new();
    expected_msg.push_str("parseargs: Error parsing option definition:\n");
    expected_msg.push_str(opt_def);
    expected_msg.push('\n');
    expected_msg.push_str(&" ".repeat(indent));
    expected_msg.push_str("^\n");
    expected_msg.push_str(&" ".repeat(indent));
    expected_msg.push_str(message);
    expected_msg.push('\n');

    parseargs()
        .arg("-o")
        .arg(opt_def)
        .assert()
        .code(11)
        .stderr(predicate::eq(expected_msg))
        .stdout("exit 1\n");
}

fn validate_msg_test(opt_def: &str, message: &str) {
    let expected_msg = format!("parseargs: {}\n", message);

    parseargs()
        .arg("-o")
        .arg(opt_def)
        .assert()
        .code(11)
        .stderr(predicate::eq(expected_msg))
        .stdout("exit 1\n");
}

#[test]
fn test_parser_errors() {
    parser_error_msg_test("d", 0, "Expected #, = or + after this");
    parser_error_msg_test(" #debug", 0, "option char/string expected");
    parser_error_msg_test("x #debug", 0, "Expected #, = or + after this");
    parser_error_msg_test(
        "l#long,-#debug",
        6,
        "option char/string expected after this",
    );
    parser_error_msg_test(":#debug", 0, "option char/string expected");
    parser_error_msg_test(":d#debug", 0, "option char/string expected");
    parser_error_msg_test("d#", 1, "name expected after this");
    parser_error_msg_test("d#debug(", 7, "Unexpected character '('");
    parser_error_msg_test("d#debug)", 7, "Unexpected character ')'");
    parser_error_msg_test("d#debug,d", 8, "Expected #, = or + after this");
    parser_error_msg_test("\\=d#debug", 0, "'=' not allowed here");
    parser_error_msg_test("x\\=d#debug", 2, "'=' not allowed here");
    parser_error_msg_test("x=d#debug", 3, "Unexpected character '#'");
}

#[test]
fn test_validation_errors() {
    validate_msg_test("d#debug,d#dancing", "Duplicate definition of option '-d'");
    validate_msg_test(
        "debug#debug,debug#dancing",
        "Duplicate definition of option '--debug'",
    );
    validate_msg_test(
        "d#debug,x#debug",
        "Duplicate usage of variable/function 'debug'",
    );
    validate_msg_test(
        "d#debug,x#debug()",
        "Duplicate usage of variable/function 'debug'",
    );
    validate_msg_test(
        "c#mode,m#mode=move",
        "Duplicate usage of variable/function 'mode'",
    );
    validate_msg_test(
        "c#mode=copy,m#mode",
        "Duplicate usage of variable/function 'mode'",
    );
    validate_msg_test(
        "c#mode=copy,m#mode=copy",
        "Duplicate value 'copy' for mode 'mode'",
    );
}
