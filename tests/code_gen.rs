mod exec;

/// Create function check code for given function name for standard shell
fn sh_func_check(func_name: &str) -> String {
    format!("if ! LC_ALL=C command -V {func_name} 2>/dev/null | head -n1 | grep function >/dev/null; then echo >&2 \"ERROR: Function '{func_name}' does not exist.\"; exit 127; fi;")
}

#[test]
fn test_argument_quoting() {
    exec::test_code_gen(&[], &["Word"], &["set -- 'Word'"]);
    exec::test_code_gen(&[], &["Word"], &["set -- 'Word'"]);
    exec::test_code_gen(&[], &["Hello World"], &["set -- 'Hello World'"]);
    exec::test_code_gen(&[], &["Won't break"], &["set -- 'Won'\\''t break'"]);

    exec::test_code_gen(
        &[],
        &["Won't break", "this time"],
        &["set -- 'Won'\\''t break' 'this time'"],
    );
}

#[test]
fn test_option_quoting() {
    exec::test_code_gen(
        &["-o", "p=phrase"],
        &["-p", "Word"],
        &["phrase='Word';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "p=phrase"],
        &["-p", "Hello World"],
        &["phrase='Hello World';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "p=phrase"],
        &["-p", "Won't break"],
        &["phrase='Won'\\''t break';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "phrase=phrase"],
        &["--phrase=Won't break"],
        &["phrase='Won'\\''t break';", "set --"],
    );
}

#[test]
fn test_counting_option() {
    exec::test_code_gen(&["-o", "v+verbose"], &[], &["verbose=0;", "set --"]);

    exec::test_code_gen(
        &["-o", "v+verbose"],
        &["-v"],
        &["verbose=0;", "verbose=1;", "set --"],
    );

    exec::test_code_gen(
        &["-o", "v+verbose"],
        &["-vv"],
        &["verbose=0;", "verbose=2;", "set --"],
    );

    exec::test_code_gen(
        &["-o", "v+verbose"],
        &["-vvvvv"],
        &["verbose=0;", "verbose=5;", "set --"],
    );

    exec::test_code_gen(
        &["-o", "v+verbose,d#debug"],
        &["-vv", "-d", "-vv"],
        &[
            "verbose=0;",
            "verbose=2;",
            "debug='true';",
            "verbose=4;",
            "set --",
        ],
    );

    exec::test_code_gen(
        &["-o", "v:verbose+verbose,d#debug"],
        &["-v", "--verbose=4", "-d", "-vv"],
        &[
            "verbose=0;",
            "verbose=4;",
            "debug='true';",
            "verbose=6;",
            "set --",
        ],
    );

    exec::test_code_gen(
        &["-o", "v:verbose+verbose"],
        &["-v", "--verbose=0"],
        &["verbose=0;", "verbose=0;", "set --"],
    );

    exec::test_code_gen(
        &["-o", "v:verbose+verbose"],
        &["-v", "--verbose=65535"],
        &["verbose=0;", "verbose=65535;", "set --"],
    );

    exec::test_code_gen(
        &["-o", "v+verbose(),d#debug"],
        &["-vv", "-d", "-vv"],
        &[
            &sh_func_check("verbose"),
            "verbose 2 || exit $?;",
            "debug='true';",
            "verbose 4 || exit $?;",
            "set --",
        ],
    );

    exec::test_error_msg(
        &["-o", "v:verbose+verbose"],
        &["--verbose=-1"],
        "parseargs: Invalid unsigned integer (0-65535): '-1'",
    );

    exec::test_error_msg(
        &["-o", "v:verbose+verbose"],
        &["--verbose=65536"],
        "parseargs: Invalid unsigned integer (0-65535): '65536'",
    );

    exec::test_error_msg(
        &["-o", "v:verbose+verbose"],
        &["--verbose=all"],
        "parseargs: Invalid unsigned integer (0-65535): 'all'",
    );
}

#[test]
fn test_flag_option() {
    exec::test_code_gen(
        &["-o", "d:debug#debug"],
        &["-d"],
        &["debug='true';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "d:debug#debug"],
        &["--debug"],
        &["debug='true';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "d:debug#debug"],
        &["--debug=true"],
        &["debug='true';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "d:debug#debug"],
        &["--debug=yes"],
        &["debug='true';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "d:debug#debug"],
        &["--debug=false"],
        &["debug='';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "d:debug#debug"],
        &["--debug=no"],
        &["debug='';", "set --"],
    );

    exec::test_error_msg(
        &["-o", "d:debug#debug"],
        &["--debug=maybe"],
        "parseargs: Invalid boolean value: 'maybe'",
    );

    exec::test_error_msg(
        &["-o", "d:debug#debug"],
        &["--debug="],
        "parseargs: Invalid boolean value: ''",
    );
}

#[test]
fn test_assignment_option() {
    exec::test_code_gen(
        &["-o", "o:out-file=outfile"],
        &["-o", "out.txt"],
        &["outfile='out.txt';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "o:out-file=outfile"],
        &["-oout.txt"],
        &["outfile='out.txt';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "o:out-file=outfile"],
        &["--out-file", "out.txt"],
        &["outfile='out.txt';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "o:out-file=outfile"],
        &["--out-file=out.txt"],
        &["outfile='out.txt';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "o:out-file=outfile"],
        &["--out-file="],
        &["outfile='';", "set --"],
    );

    exec::test_error_msg(
        &["-o", "o:out-file=outfile"],
        &["--out-file"],
        "parseargs: Missing argument for: --out-file",
    );
}
#[test]
fn test_mode_switch_option() {
    exec::test_code_gen(
        &["-o", "c:copy#mode=copy,m:move#mode=move"],
        &["-c"],
        &["mode='copy';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "c:copy#mode=copy,m:move#mode=move"],
        &["--copy"],
        &["mode='copy';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "c:copy#mode=copy,m:move#mode=move"],
        &["-m"],
        &["mode='move';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "c:copy#mode=copy,m:move#mode=move"],
        &["--move"],
        &["mode='move';", "set --"],
    );

    exec::test_error_msg(
        &["-o", "c:copy#mode=copy,m:move#mode=move"],
        &["-cm"],
        "parseargs: Options are mutual exclusive: -c/--copy, -m/--move",
    );

    exec::test_error_msg(
        &["-o", "c:copy#mode=copy,m:move#mode=move"],
        &["-mc"],
        "parseargs: Options are mutual exclusive: -c/--copy, -m/--move",
    );
}

#[test]
fn test_combined_options() {
    let expected = &[
        "verbose=0;",
        "debug='true';",
        "long='true';",
        "verbose=1;",
        "file='file.txt';",
        "set --",
    ];

    exec::test_code_gen(
        &["-o", "d#debug,l#long,v+verbose,f=file"],
        &["-d", "-l", "-v", "-f", "file.txt"],
        expected,
    );

    exec::test_code_gen(
        &["-o", "d#debug,l#long,v+verbose,f=file"],
        &["-dlvf", "file.txt"],
        expected,
    );

    exec::test_code_gen(
        &["-o", "d#debug,l#long,v+verbose,f=file"],
        &["-dlvffile.txt"],
        expected,
    );
}

#[test]
fn test_script_name() {
    // without -n uses "parseargs" as prefix for messages
    exec::test_error_msg(&["-o", "d#debug"], &["-X"], "parseargs: Unknown option: -X");

    // with -n uses the given name
    exec::test_error_msg(
        &["-n", "test-script", "-o", "d#debug"],
        &["-X"],
        "test-script: Unknown option: -X",
    );
}

#[test]
fn test_initialize_variables() {
    exec::test_code_gen(
        &["-io", "d#debug,f=file,c#mode=copy,m#mode=move"],
        &[],
        &["debug='';", "file='';", "mode='';", "set --"],
    );

    // No init call for functions
    exec::test_code_gen(
        &["-io", "d#debug(),f=file(),c#mode()=copy,m#mode()=move"],
        &[],
        &[
            &sh_func_check("debug"),
            &sh_func_check("file"),
            &sh_func_check("mode"),
            "set --",
        ],
    );
}

#[test]
fn test_argument_callback() {
    exec::test_code_gen(
        &["-a", "arg_cb", "-o", "o:out-file=outfile"],
        &["-o", "out.txt", "input1", "input2"],
        &[
            &sh_func_check("arg_cb"),
            "outfile='out.txt';",
            "arg_cb 'input1' || exit $?;",
            "arg_cb 'input2' || exit $?;",
            "set --",
        ],
    );

    exec::test_code_gen(
        &["-a", "arg_cb", "-o", "o:out-file=outfile"],
        &["input1", "-o", "out.txt", "input2"],
        &[
            &sh_func_check("arg_cb"),
            "arg_cb 'input1' || exit $?;",
            "outfile='out.txt';",
            "arg_cb 'input2' || exit $?;",
            "set --",
        ],
    );
}

#[test]
fn test_remainder_array() {
    // Not supported by default
    exec::test_parseargs_error_msg(
        &["--remainder=rest"],
        "parseargs: Shell sh does not support arrays, so option -r/--remainder is not supported",
    );

    // Not supported with shell=sh
    exec::test_parseargs_error_msg(
        &["-ssh", "--remainder=rest"],
        "parseargs: Shell sh does not support arrays, so option -r/--remainder is not supported",
    );

    exec::test_code_gen(
        &["-sbash", "--remainder=rest"],
        &[],
        &["typeset -a rest;", "rest=();", "set --"],
    );

    exec::test_code_gen(
        &["-szsh", "--remainder=rest"],
        &[],
        &["typeset -a rest;", "rest=();", "set --"],
    );

    exec::test_code_gen(
        &["-sksh", "--remainder=rest"],
        &[],
        &["typeset -a rest;", "set -A rest;", "set --"],
    );

    exec::test_code_gen(
        &["-sbash", "--remainder=rest"],
        &["arg", "--", "r1", "r2"],
        &[
            "typeset -a rest;",
            "rest=();",
            "rest+=('r1');",
            "rest+=('r2');",
            "set -- 'arg'",
        ],
    );

    exec::test_code_gen(
        &["-szsh", "--remainder=rest"],
        &["arg", "--", "r1", "r2"],
        &[
            "typeset -a rest;",
            "rest=();",
            "rest+=('r1');",
            "rest+=('r2');",
            "set -- 'arg'",
        ],
    );

    exec::test_code_gen(
        &["-sksh", "--remainder=rest"],
        &["arg", "--", "r1", "r2"],
        &[
            "typeset -a rest;",
            "set -A rest;",
            "rest+=('r1');",
            "rest+=('r2');",
            "set -- 'arg'",
        ],
    );
}

#[test]
fn test_dash_dash() {
    exec::test_code_gen(
        &["-o", "d#debug"],
        &["-d", "--", "-d", "--", "test"],
        &["debug='true';", "set -- '-d' '--' 'test'"],
    );
}

#[test]
fn test_posix() {
    // without -p intermix of arguments and options
    exec::test_code_gen(
        &["-o", "d#debug"],
        &["test", "-d", "--", "test2"],
        &["debug='true';", "set -- 'test' 'test2'"],
    );

    // with -p first arg stops option handling
    exec::test_code_gen(
        &["-po", "d#debug"],
        &["test", "-d", "--", "test2"],
        &["set -- 'test' '-d' '--' 'test2'"],
    );

    exec::test_code_gen(
        &["-po", "d#debug"],
        &["test", "-d", "--", "test2"],
        &["set -- 'test' '-d' '--' 'test2'"],
    );
}

#[test]
fn test_multiple_o_options() {
    exec::test_code_gen(
        &["-o", "d#debug", "-o", "l#long"],
        &["-d", "-l"],
        &["debug='true';", "long='true';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "d#debug,,", "-o", ",,l#long"],
        &["-d", "-l"],
        &["debug='true';", "long='true';", "set --"],
    );

    exec::test_code_gen(
        &["-o", "d#debug", "-o", "l#long", "-o", "\\,#comma"],
        &["-d", "-l", "-,"],
        &["debug='true';", "long='true';", "comma='true';", "set --"],
    );
}
