mod exec;

///
/// Test error message from parsing a invalid option definition.
///
/// # Arguments
/// * `opt_def` - the option definition string
/// * `index` - index of fault in option definition string
/// * `message` - expected error message
///
fn test_parser_error_msg(opt_def: &str, index: usize, message: &str) {
    let mut expected_msg = String::new();
    expected_msg.push_str("parseargs: Error parsing option definition:\n");
    expected_msg.push_str(opt_def);
    expected_msg.push('\n');
    expected_msg.push_str(&" ".repeat(index));
    expected_msg.push_str("^\n");
    expected_msg.push_str(&" ".repeat(index));
    expected_msg.push_str(message);

    exec::test_parseargs_error_msg(&["-o", opt_def], &expected_msg);
}

///
/// Test error message from validation of the successfully parsed option definition.
///
/// # Arguments
/// * `opt_def` - the option definition string
/// * `message` - error message expected from validation of parsed option definition
///
fn test_validation_error_msg(opt_def: &str, message: &str) {
    let expected_msg = format!("parseargs: {}", message);
    exec::test_parseargs_error_msg(&["-o", opt_def], &expected_msg);
}

#[test]
fn test_parser_errors() {
    test_parser_error_msg("d", 0, "Expected #, = or + after this");
    test_parser_error_msg(" #debug", 0, "option char/string expected");
    test_parser_error_msg("x #debug", 0, "Expected #, = or + after this");
    test_parser_error_msg(
        "l#long,-#debug",
        6,
        "option char/string expected after this",
    );
    test_parser_error_msg(":#debug", 0, "option char/string expected");
    test_parser_error_msg(":d#debug", 0, "option char/string expected");
    test_parser_error_msg("d#", 1, "name expected after this");
    test_parser_error_msg("d#debug(", 7, "Unexpected character '('");
    test_parser_error_msg("d#debug)", 7, "Unexpected character ')'");
    test_parser_error_msg("d#debug,d", 8, "Expected #, = or + after this");
    test_parser_error_msg("\\=d#debug", 0, "'=' not allowed here");
    test_parser_error_msg("x\\=d#debug", 2, "'=' not allowed here");
    test_parser_error_msg("x=d#debug", 3, "Unexpected character '#'");
}

#[test]
fn test_validation_errors() {
    test_validation_error_msg("d#debug,d#dancing", "Duplicate definition of option '-d'");
    test_validation_error_msg(
        "debug#debug,debug#dancing",
        "Duplicate definition of option '--debug'",
    );
    test_validation_error_msg(
        "d#debug,x#debug",
        "Duplicate usage of variable/function 'debug'",
    );
    test_validation_error_msg(
        "d#debug,x#debug()",
        "Duplicate usage of variable/function 'debug'",
    );
    test_validation_error_msg(
        "c#mode,m#mode=move",
        "Duplicate usage of variable/function 'mode'",
    );
    test_validation_error_msg(
        "c#mode=copy,m#mode",
        "Duplicate usage of variable/function 'mode'",
    );
    test_validation_error_msg(
        "c#mode=copy,m#mode=copy",
        "Duplicate value 'copy' for mode 'mode'",
    );
}
