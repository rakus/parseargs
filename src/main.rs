/*
 * Part of parseargs - a command line options parser for shell scripts
 *
 * Copyright (c) 2023 Ralf Schandl
 * This code is licensed under MIT license (see LICENSE.txt for details).
 */

mod arg_parser;
mod opt_def;
mod shell_code;

use crate::shell_code::VarValue;
use shell_code::CodeChunk;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io::{stdout, IsTerminal};
use std::panic::catch_unwind;
use std::process::exit;

use crate::arg_parser::{CmdLineElement, CmdLineTokenizer};
use crate::opt_def::{OptConfig, OptTarget, OptType};
use clap::{CommandFactory, Parser};

const PARSEARGS: &str = env!("CARGO_PKG_NAME");

/**
 * The default shell, if '-s' is not given.
 * Can be overwritten using the environment variable 'PARSEARGS_SHELL'.
 */
const DEFAULT_SHELL: &str = "sh";

/**
 * Environment variable to set the default shell.
 * If '-s' is not given, this environment variable is checked.
 * If set, its value will be used as default shell. If not, 'sh' is used.
 */
const PARSEARGS_SHELL_VAR: &str = "PARSEARGS_SHELL";

/**
 * Command line arguments.
 */
#[derive(Parser, Debug)]
#[clap(
    disable_help_flag = true,
    disable_version_flag = true,
    verbatim_doc_comment
)]
#[command(version)]
struct CmdLineArgs {
    /// Definition of supported shell options
    #[arg(short = 'o', long = "options", value_name = "OPT-DEFs")]
    options: Option<String>,

    /// Name of calling shell script. Used as prefix for error messages.
    #[arg(short = 'n', long = "name")]
    name: Option<String>,

    /// Call function SHELL-FUNC to report program arguments.
    /// When used $# will always be 0 after parseargs call.
    #[arg(short = 'a', long = "arg-callback", value_name = "SHELL-FUNC", value_parser = parse_shell_name, verbatim_doc_comment)]
    arg_callback: Option<String>,

    /// On error call this function before exiting the calling script.
    #[arg(short = 'e', long = "error-callback", value_name = "SHELL-FUNC", value_parser = parse_shell_name)]
    error_callback: Option<String>,

    /// Collect all parameter behind a '--' in the named array.
    /// ONLY SUPPORTED WITH --shell bash, ksh, or zsh.
    #[arg(short = 'r', long = "remainder", value_name = "SHELL-VAR", value_parser = parse_shell_name, verbatim_doc_comment)]
    remainder: Option<String>,

    /// Stop option processing on first none-option
    #[arg(short = 'p', long = "posix")]
    posix: bool,

    /// Initialize all variables with '', except for counting variables,
    /// as they are always initialized with 0.
    #[arg(short = 'i', long = "init-vars")]
    init_vars: bool,

    /// Enable support for --help as script option.
    #[arg(short = 'h', long = "help-opt", verbatim_doc_comment)]
    help_opt: bool,

    /// Enable support for --version as script options.
    #[arg(short = 'v', long = "version-opt", verbatim_doc_comment)]
    version_opt: bool,

    /// Produce code for named shell. Supported: bash, ksh, zsh, sh.
    /// Default: sh
    #[arg(short = 's', long = "shell", value_name = "SHELL")]
    shell: Option<String>,

    /// Print help
    #[arg(long)]
    help: bool,

    /// Print version
    #[arg(long)]
    version: bool,

    // Disabled for now
    // /// enable debug output to STDERR.
    // #[arg(short = 'd', long = "debug")]
    // debug: bool,
    /// Shell script options
    #[arg(value_name = "SCRIPT-ARGS")]
    script_args: Vec<OsString>,
}

/**
 * Exit after printing an error message.
 */
fn die_internal(msg: String) -> ! {
    eprintln!("{}: {}", PARSEARGS, msg);
    println!("exit 1");

    exit(11);
}

/**
 * Used by Clap to validate a given str as shell variable/function name and to create a String from it.
 */
fn parse_shell_name(arg: &str) -> Result<String, String> {
    for (idx, chr) in arg.chars().enumerate() {
        if idx == 0 && !chr.is_alphabetic() {
            Err("Not a valid shell variable or function name")?
        }
        if idx > 0 && !chr.is_alphanumeric() && chr != '_' {
            Err("Not a valid shell variable or function name")?
        }
    }
    Ok(arg.to_string())
}

/**
 * Produces the initial shell code. Like checking that required functions really exist and
 * typesetting the variables (if supported by shell).
 */
fn shell_init_code(
    opt_cfg_list: &Vec<OptConfig>,
    cmd_line_args: &CmdLineArgs,
    init_vars: bool,
) -> Vec<CodeChunk> {
    let mut init_code: Vec<CodeChunk> = vec![];

    // First function checks ...
    if let Some(func) = &cmd_line_args.arg_callback {
        init_code.push(CodeChunk::CheckForFunction(func.clone()));
    }
    if let Some(func) = &cmd_line_args.error_callback {
        init_code.push(CodeChunk::CheckForFunction(func.clone()));
    }

    // Iterating opt_cfg_list multiple time, but I want a certain order of
    // the generated code.

    // prevent multiple checks for same function (ModeSwitch)
    let mut func_name_vec: Vec<&String> = vec![];
    for opt_cfg in opt_cfg_list {
        if let OptTarget::Function(name) = &opt_cfg.get_target() {
            if !func_name_vec.contains(&name) {
                init_code.push(CodeChunk::CheckForFunction(name.clone()));
                func_name_vec.push(name);
            }
        }
    }
    // ... then typset and counter variables
    let mut handled_vars: Vec<String> = vec![];

    for opt_cfg in opt_cfg_list {
        let name = opt_cfg.get_target_name();

        if opt_cfg.is_target_variable() {
            if init_vars && !handled_vars.contains(&name) {
                match &opt_cfg.opt_type {
                    OptType::Flag(_) | OptType::Assignment(_) | OptType::ModeSwitch(_, _) => {
                        init_code.push(CodeChunk::AssignVar(
                            name.clone(),
                            VarValue::StringValue("".to_string()),
                        ))
                    }
                    OptType::Counter(_) => {
                        init_code.push(CodeChunk::AssignVar(name.clone(), VarValue::IntValue(0)));
                    }
                }
                handled_vars.push(name.clone());
            } else if let OptType::Counter(_) = &opt_cfg.opt_type {
                init_code.push(CodeChunk::AssignVar(name.clone(), VarValue::IntValue(0)));
            }
        }
    }

    if let Some(array) = &cmd_line_args.remainder {
        init_code.push(CodeChunk::DeclareArrayVar(array.clone()));
        init_code.push(CodeChunk::AssignEmptyArray(array.clone()));
    }

    init_code
}

/**
 * Optional String to bool.
 *
 * The values "true" and "yes" result in `true`.
 * The values "false" and "no" result in `false`.
 * Check is case-insensitive.
 *
 * `None` results in `false`.
 */
fn some_str_to_bool(ostr: Option<&String>, default: bool) -> Result<bool, String> {
    match ostr {
        Some(v) => match v.to_lowercase().trim() {
            "true" | "yes" => Ok(true),
            "false" | "no" => Ok(false),
            _ => Err(format!("Invalid boolean value: '{}'", v)),
        },
        None => Ok(default),
    }
}

/**
 * Assign a value to an option target.
 * Return either aCodeChunk for a variable assignment or a function call.
 */
fn assign_target(target: &OptTarget, value: VarValue) -> CodeChunk {
    match target {
        OptTarget::Variable(name) => CodeChunk::AssignVar(name.clone(), value),
        OptTarget::Function(name) => CodeChunk::CallFunction(name.clone(), value),
    }
}

/**
 * Parses the shell arguments based on the given option definition.
 * Returns a vector of CodeChunks.
 */
fn parse_shell_options(
    opt_cfg_list: &mut Vec<OptConfig>,
    cmd_line_args: &CmdLineArgs,
) -> Result<Vec<CodeChunk>, String> {
    let mut shell_code: Vec<CodeChunk> = vec![];
    let mut arguments: Vec<String> = vec![];

    // Lookup table from target name to position in vector.
    // Needed for duplication checks of Mode-Switches.
    let mut shell_name_table: HashMap<String, Vec<usize>> = HashMap::new();

    for (pos, e) in opt_cfg_list.iter().enumerate() {
        let name = &e.get_target_name();

        if shell_name_table.contains_key(name) {
            shell_name_table.get_mut(name).unwrap().push(pos);
        } else {
            shell_name_table.insert(name.clone(), vec![pos]);
        }
    }

    let mut script_args = vec![];
    for oss in &cmd_line_args.script_args {
        let result = OsString::into_string(oss.clone());
        if let Ok(utf8) = result {
            script_args.push(utf8);
        } else {
            Err(format!(
                "Invalid UTF-8 char(s) in {:?}",
                result.unwrap_err()
            ))?
        }
    }

    let mut cl_tok = CmdLineTokenizer::build(script_args, cmd_line_args.posix);

    let mut after_separator = false;

    while let Some(e) = cl_tok.next() {
        if let CmdLineElement::Separator = e {
            after_separator = true;
            continue;
        } else if let CmdLineElement::Argument(value) = e {
            if after_separator && cmd_line_args.remainder.is_some() {
                if let Some(array) = &cmd_line_args.remainder {
                    shell_code.push(CodeChunk::AddToArray(
                        array.clone(),
                        VarValue::StringValue(value),
                    ));
                }
            } else if let Some(func) = &cmd_line_args.arg_callback {
                shell_code.push(CodeChunk::CallFunction(
                    func.clone(),
                    VarValue::StringValue(value),
                ));
            } else {
                arguments.push(value);
            }
        } else {
            let opt_value = match &e {
                CmdLineElement::LongOptionValue(_, v) => Some(v),
                _ => None,
            };

            let option = (
                opt_cfg_list.iter_mut().find(|cfg| cfg.match_option(&e)),
                opt_value,
            );

            if option.0.is_none() {
                return Err(format!("Unknown option: {}", e));
            } else if let Some(oc) = option.0 {
                // Check duplicate options. Counter options and options that trigger a function call
                // can be used multiple times.
                if oc.assigned && !oc.is_duplicate_allowed() {
                    return Err(format!("Duplicate option: {} ({})", e, oc.options_string()));
                }
                oc.assigned = true;

                if oc.singleton {
                    shell_code.clear();
                }

                match &oc.opt_type {
                    OptType::Flag(target) => {
                        let bool_val = VarValue::BoolValue(some_str_to_bool(option.1, true)?);
                        shell_code.push(assign_target(target, bool_val));
                    }
                    OptType::ModeSwitch(target, value) => {
                        if option.1.is_some() {
                            Err(format!("{}: No value supported.", oc.options_string()))?;
                        }
                        // Conflict detection is done at end of processing.
                        shell_code
                            .push(assign_target(target, VarValue::StringValue(value.clone())));
                    }
                    OptType::Assignment(target) => {
                        let opt_arg = match option.1 {
                            Some(v) => Some(v.clone()),
                            None => cl_tok.get_option_argument(),
                        };
                        if let Some(opt_arg) = opt_arg {
                            shell_code.push(assign_target(target, VarValue::StringValue(opt_arg)));
                        } else {
                            return Err(format!("Missing argument for: {}", e));
                        }
                    }
                    OptType::Counter(target) => {
                        let value = match option.1 {
                            Some(v) => {
                                let cnt = match v.parse::<u16>() {
                                    Ok(v) => v,
                                    Err(_) => {
                                        return Err(format!(
                                            "Invalid unsigned integer in value of option {}",
                                            e
                                        ));
                                    }
                                };
                                Some(cnt)
                            }
                            None => None,
                        };
                        match value {
                            Some(v) => {
                                oc.count_value = v;
                            }
                            None => {
                                oc.count_value += 1;
                            }
                        }

                        /*
                        TODO: -vvv should only output one 'verbose=3'
                        Also -vvv -d -v should output 'verbose=3; debug=true; verbose=4'
                        */
                        shell_code.push(assign_target(
                            target,
                            VarValue::IntValue(oc.count_value as i32),
                        ));
                    }
                }

                if oc.singleton {
                    shell_code.push(CodeChunk::Exit(0));
                    return Ok(shell_code);
                }
            }
        }
    }

    // Check duplicates for ModeSwitches
    // and handle required
    for name in shell_name_table.keys() {
        if shell_name_table.get(name).unwrap().len() > 1 {
            let mut used_tab = vec![];
            let mut all_tab = vec![];
            let mut required = false;
            for idx in shell_name_table.get(name).unwrap() {
                if opt_cfg_list[*idx].assigned {
                    used_tab.push(opt_cfg_list[*idx].options_string());
                }
                all_tab.push(opt_cfg_list[*idx].options_string());
                if opt_cfg_list[*idx].required {
                    required = true;
                }
            }
            if used_tab.len() > 1 {
                return Err(format!(
                    "Options are mutual exclusive: {}",
                    used_tab.join(", ")
                ));
            }
            if required && used_tab.is_empty() {
                return Err(format!(
                    "One of the following options is required: {}",
                    all_tab.join(", ")
                ));
            }
        }

        for oc in &mut *opt_cfg_list {
            match oc.opt_type {
                OptType::ModeSwitch(_, _) => (),
                _ => {
                    if oc.required && !oc.assigned {
                        return Err(format!(
                            "Required option not found: {}",
                            oc.options_string()
                        ));
                    }
                }
            }
        }
    }

    shell_code.push(CodeChunk::SetArgs(arguments));

    Ok(shell_code)
}

/**
 * Validate the option definitions.
 *
 * Check for:
 *
 * * duplicate options
 * * duplicate usage of variables/functions (only allowed for ModeSwitch)
 * * ModeSwitch with same value
 *
 * Does not allow function and variable with same name. For a shell script
 * this should work, but in our context it is most likely an error.
 */
fn validate_option_definitions(opt_def_list: &Vec<OptConfig>) {
    let mut all_short_options = String::new();
    let mut all_long_options: Vec<&String> = vec![];
    let mut all_variables: Vec<(String, bool, bool)> = vec![];
    let mut mode_values_map: HashMap<String, Vec<&String>> = HashMap::new();

    for oc in opt_def_list {
        for chr in oc.opt_chars.chars() {
            match all_short_options.find(chr) {
                Some(_) => {
                    die_internal(format!("Duplicate definition of option '-{}'", chr));
                }
                None => {
                    all_short_options.push(chr);
                }
            }
        }
        for lng in &oc.opt_strings {
            if all_long_options.contains(&lng) {
                die_internal(format!("Duplicate definition of option '--{}'", lng));
            } else {
                all_long_options.push(lng);
            }
        }

        let name = oc.get_target_name();
        let is_function = oc.is_target_function();
        let is_mode_switch = matches!(oc.opt_type, OptType::ModeSwitch(_, _));

        match all_variables.iter().find(|x| x.0 == name) {
            Some(o) => {
                if is_mode_switch {
                    if !o.2 {
                        die_internal(format!("Duplicate usage of variable/function '{}'", name));
                    } else if is_function != o.1 {
                        die_internal(format!(
                            "Used as variable and function in mod-switch option: '{}'",
                            name
                        ));
                    }
                } else {
                    die_internal(format!("Duplicate usage of variable/function '{}'", name));
                }
            }
            None => {
                all_variables.push((name.clone(), is_function, is_mode_switch));
            }
        }
        if let OptType::ModeSwitch(_, value) = &oc.opt_type {
            if mode_values_map.contains_key(&name) {
                if let Some(v) = mode_values_map.get(&name) {
                    if v.contains(&value) {
                        die_internal(format!("Duplicate value '{}' for mode '{}'", value, name));
                    }
                }
            } else {
                mode_values_map.insert(name.clone(), vec![value]);
            }
        }
    }
}

/**
 * The actual parseargs logic.
 *
 * The function does not return but exit.
 */
fn parseargs(cmd_line_args: CmdLineArgs) -> ! {
    let script_name = match cmd_line_args.name {
        Some(ref n) => n,
        None => PARSEARGS,
    };

    // parse the option definition string
    let result = if cmd_line_args.options.is_some() {
        opt_def::parse(&cmd_line_args.options.clone().unwrap())
    } else {
        Ok(Vec::new())
    };

    let mut opt_cfg_list = match result {
        Ok(list) => list,
        Err(error) => {
            die_internal(format!("Error parsing option definition:\n{}", error));
        }
    };

    validate_option_definitions(&opt_cfg_list);

    // Add support for `--help` if requested.
    // As this is added to the end of the list, a custom '--help' has precedence.
    if cmd_line_args.help_opt {
        opt_cfg_list.push(OptConfig {
            opt_chars: "".to_string(),
            opt_strings: vec!["help".to_string()],
            opt_type: OptType::Flag(OptTarget::Function("show_help".to_string())),
            required: false,
            singleton: true,
            assigned: false,
            count_value: 0,
        });
    }
    // Add support for `--version` if requested.
    // As this is added to the end of the list, a custom '--version' has precedence.
    if cmd_line_args.version_opt {
        opt_cfg_list.push(OptConfig {
            opt_chars: "".to_string(),
            opt_strings: vec!["version".to_string()],
            opt_type: OptType::Flag(OptTarget::Function("show_version".to_string())),
            required: false,
            singleton: true,
            assigned: false,
            count_value: 0,
        });
    }

    // Determine shell. Either from option, environment var or the default.
    let shell = cmd_line_args
        .shell
        .clone()
        .unwrap_or(std::env::var(PARSEARGS_SHELL_VAR).unwrap_or(DEFAULT_SHELL.to_string()));

    // get the shell templates
    let shell_tmpl = shell_code::get_shell_template(shell.as_str());
    if shell_tmpl.is_none() {
        die_internal(format!("Unknown shell '{}'", shell));
    }
    let shell_tmpl = shell_tmpl.unwrap();

    if !shell_tmpl.supports_arrays && cmd_line_args.remainder.is_some() {
        die_internal(format!(
            "Shell {} does not support arrays, so option -r/--remainder is not supported",
            shell
        ));
    }

    let mut code: Vec<CodeChunk> = vec![];

    // generate initialization code. Check for functions, initialize variables
    let mut init_code = shell_init_code(&opt_cfg_list, &cmd_line_args, cmd_line_args.init_vars);

    // let options_code = parse_shell_options(&opt_cfg_list, &cmd_line_args);
    let rc = match parse_shell_options(&mut opt_cfg_list, &cmd_line_args) {
        Ok(mut c) => {
            code.append(&mut init_code);
            code.append(&mut c);
            0
        }
        Err(msg) => {
            eprintln!("{}: {}", script_name, msg);
            if let Some(func) = cmd_line_args.error_callback {
                code.push(CodeChunk::CallFunction(func, VarValue::None));
            }
            code.push(CodeChunk::Exit(1));
            1
        }
    };

    println!("{}", shell_tmpl.format_vector(&code));

    exit(rc);
}

fn main() {
    match CmdLineArgs::try_parse() {
        Ok(c) => {
            if c.help {
                // insert an additional '--' in the help output.
                // Didn't find a way to tell Clap to insert it, so we do it manually.
                // This is fragile.
                let mut help_str = CmdLineArgs::command().render_help().to_string();
                help_str = help_str.replace("] [SCRIPT-ARGS]", "] -- [SCRIPT-ARGS]");
                println!("{}", help_str);
                exit(0);
            } else if c.version {
                let version_str = CmdLineArgs::command().render_version();
                println!("{}", version_str);
                exit(0);
            }

            // Catch a panic and print `exit 1` to exit the calling script.
            match catch_unwind(|| parseargs(c)) {
                // Ok should never be reached, as parseargs exits
                Ok(_) => exit(97),
                Err(_) => {
                    println!("exit 1");
                    exit(13);
                }
            }
        }
        Err(e) => {
            if e.exit_code() == 0 {
                // help or version output
                if stdout().is_terminal() {
                    println!("{}", e);
                } else {
                    eprintln!("{}", e);
                    println!("exit 0");
                }
                exit(0);
            } else {
                eprintln!("{}", e);

                println!("exit 1");
                exit(11);
            }
        }
    }
}
