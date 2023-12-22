//use crate::cmd_line::{CmdLineElement, CmdLineTokenizer};
use crate::opt_def;
use crate::opt_def::{OptConfig, OptTarget, OptType};
use crate::shell_code;
use crate::shell_code::CodeChunk;
use crate::shell_code::VarValue;
use crate::CmdLineArgs;
use std::cell::Cell;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::process::exit;

const PARSEARGS: &str = env!("CARGO_PKG_NAME");

/// The default shell, if '-s' is not given.
/// Can be overwritten using the environment variable 'PARSEARGS_SHELL'.
const DEFAULT_SHELL: &str = "sh";

/// Environment variable to set the default shell.
/// If '-s' is not given, this environment variable is checked.
/// If set, its value will be used as default shell. If not, 'sh' is used.
const PARSEARGS_SHELL_VAR: &str = "PARSEARGS_SHELL";

/// The actual parseargs logic.
///
/// The function does not return but exit.

pub fn parseargs(cmd_line_args: CmdLineArgs) -> ! {
    let script_name = match cmd_line_args.name {
        Some(ref n) => n,
        None => PARSEARGS,
    };

    // parse the option definition string
    let result = if cmd_line_args.options_list.is_some() {
        opt_def::parse(&cmd_line_args.options_list.clone().unwrap().join(","))
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
            assigned: Cell::new(false),
            count_value: Cell::new(0),
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
            assigned: Cell::new(false),
            count_value: Cell::new(0),
        });
    }

    if cmd_line_args.debug {
        for oc in &opt_cfg_list {
            eprintln!("{:?}", oc);
        }
    }

    // Determine shell. Either from option, environment var or the default.
    let shell = cmd_line_args
        .shell
        .clone()
        .unwrap_or(std::env::var(PARSEARGS_SHELL_VAR).unwrap_or(DEFAULT_SHELL.to_string()));

    if cmd_line_args.debug {
        eprintln!("Shell: {}", shell);
    }

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

    // println!("{}", shell_tmpl.format_vector(&code));
    let src_code = shell_tmpl.format_vector(&code);

    let bytes = match stfu8::decode_u8(&src_code) {
        Ok(s) => s,
        Err(e) => panic!("encoding/decoding failed of: {} - {}", src_code, e),
    };

    let _ = io::stdout().write_all(&bytes);
    let _ = io::stdout().write(b"\n");

    exit(rc);
}

/// Exit after printing an error message.
fn die_internal(msg: String) -> ! {
    eprintln!("{}: {}", PARSEARGS, msg);
    println!("exit 1");

    exit(11);
}

/// Produces the initial shell code. Like checking that required functions really exist and
/// typesetting the variables (if supported by shell).
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
                            VarValue::StringValue(OsString::from("")),
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

/// Validate the option definitions.
///
/// Check for:
///
/// * duplicate options
/// * duplicate usage of variables/functions (only allowed for ModeSwitch)
/// * ModeSwitch with same value
///
/// Does not allow function and variable with same name. For a shell script
/// this should work, but in our context it is most likely an error.
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

/// Optional String to bool.
///
/// The values "true" and "yes" result in `true`.
/// The values "false" and "no" result in `false`.
/// Check is case-insensitive.
///
/// `None` results in given default value.
fn optional_str_to_bool(ostr: Option<&OsStr>, default: bool) -> Result<bool, String> {
    match ostr {
        Some(v) => {
            let str_value = match v.to_os_string().into_string() {
                Ok(s) => Ok(s),
                Err(s) => Err(format!("Can't parse as boolean: {:?}", s)),
            }?;

            match str_value.to_lowercase().trim() {
                "true" | "yes" => Ok(true),
                "false" | "no" => Ok(false),
                _ => Err(format!("Invalid boolean value: '{}'", str_value)),
            }
        }
        None => Ok(default),
    }
}

/// Optional String to optional u16.
///
/// Returns Err on invalid value.
/// If input is None results in None
fn optional_string_to_optional_u16(value: Option<&OsStr>) -> Result<Option<u16>, String> {
    match value {
        Some(v) => {
            let str_value = match v.to_os_string().into_string() {
                Ok(s) => Ok(s),
                Err(s) => Err(format!("Can't parse as number: {:?}", s)),
            }?;
            let cnt = match str_value.parse::<u16>() {
                Ok(v) => v,
                Err(_) => Err(format!(
                    "Invalid unsigned integer (0-65535): '{}'",
                    str_value
                ))?,
            };
            Ok(Some(cnt))
        }
        None => Ok(None),
    }
}

/// Assign a value to an option target.
/// Return either aCodeChunk for a variable assignment or a function call.
fn assign_target(target: &OptTarget, value: VarValue) -> CodeChunk {
    match target {
        OptTarget::Variable(name) => CodeChunk::AssignVar(name.clone(), value),
        OptTarget::Function(name) => CodeChunk::CallFunction(name.clone(), value),
    }
}

/// Parses the shell arguments based on the given option definition.
/// Returns a vector of CodeChunks.
fn parse_shell_options(
    opt_cfg_list: &mut Vec<OptConfig>,
    cmd_line_args: &CmdLineArgs,
) -> Result<Vec<CodeChunk>, String> {
    let mut shell_code: Vec<CodeChunk> = vec![];
    let mut arguments: Vec<OsString> = vec![];

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

    //let mut script_args = vec![];
    //for oss in &cmd_line_args.script_args {
    //    let result = OsString::into_string(oss.clone());
    //    if let Ok(utf8) = result {
    //        script_args.push(utf8);
    //    } else {
    //        Err(format!(
    //            "Invalid UTF-8 char(s) in {:?}",
    //            result.unwrap_err()
    //        ))?
    //    }
    //}

    let raw_args = clap_lex::RawArgs::new(&cmd_line_args.script_args);
    let mut cursor = raw_args.cursor();

    let mut after_separator = false;
    let mut prev_counter: Option<(&OptTarget, u16)> = None;

    while let Some(arg) = raw_args.next(&mut cursor) {
        if !after_separator && arg.is_escape() {
            prev_counter = counter_assign(&mut shell_code, prev_counter);
            after_separator = true;
            continue;
        } else if after_separator {
            prev_counter = counter_assign(&mut shell_code, prev_counter);
            // Argument
            let argument = arg.to_value_os().to_owned();
            if let (true, Some(array)) = (after_separator, &cmd_line_args.remainder) {
                shell_code.push(CodeChunk::AddToArray(
                    array.clone(),
                    VarValue::StringValue(argument),
                ));
            } else if let Some(func) = &cmd_line_args.arg_callback {
                shell_code.push(CodeChunk::CallFunction(
                    func.clone(),
                    VarValue::StringValue(argument),
                ));
            } else {
                arguments.push(argument);
            }
        } else if let Some((name, value)) = arg.to_long() {
            let opt_name = match name {
                Ok(s) => s,
                Err(e) => return Err(format!("Can't parse {:?}", e)),
            };

            let opt_name_str = format!("--{}", opt_name);

            let opt_config = opt_cfg_list
                .iter()
                .find(|cfg| cfg.match_option_long(&opt_name.to_owned()));
            if opt_config.is_none() {
                return Err(format!("Unknown option: {}", opt_name_str));
            } else if let Some(oc) = opt_config {
                // Check duplicate options. Counter options and options that trigger a function call
                // can be used multiple times.
                if oc.assigned.get() && !oc.is_duplicate_allowed() {
                    return Err(format!(
                        "Duplicate option: {} ({})",
                        opt_name_str,
                        oc.options_string()
                    ));
                }
                oc.assigned.set(true);

                if oc.singleton {
                    shell_code.clear();
                }

                match &oc.opt_type {
                    OptType::Flag(target) => {
                        prev_counter = counter_assign(&mut shell_code, prev_counter);
                        let bool_val = VarValue::BoolValue(optional_str_to_bool(value, true)?);
                        shell_code.push(assign_target(target, bool_val));
                    }
                    OptType::ModeSwitch(target, mode_value) => {
                        prev_counter = counter_assign(&mut shell_code, prev_counter);
                        if value.is_some() {
                            Err(format!("{}: No value supported.", oc.options_string()))?;
                        }
                        // Conflict detection is done at end of processing.
                        shell_code.push(assign_target(
                            target,
                            VarValue::StringValue(OsString::from(mode_value)),
                        ));
                    }
                    OptType::Assignment(target) => {
                        prev_counter = counter_assign(&mut shell_code, prev_counter);
                        let opt_arg = match value {
                            Some(v) => Some(v),
                            None => raw_args.next_os(&mut cursor),
                        };
                        if let Some(opt_arg) = opt_arg {
                            shell_code.push(assign_target(
                                target,
                                VarValue::StringValue(opt_arg.to_owned()),
                            ));
                        } else {
                            return Err(format!("Missing argument for: {}", opt_name_str));
                        }
                    }
                    OptType::Counter(target) => {
                        if let Some((prev_target, _)) = prev_counter {
                            if prev_target != target {
                                counter_assign(&mut shell_code, prev_counter);
                            }
                        }

                        let value = optional_string_to_optional_u16(value)?;
                        oc.count_value
                            .set(value.unwrap_or(oc.count_value.get() + 1));

                        prev_counter = Some((target, oc.count_value.get()));
                    }
                }
            }
        } else if let Some(mut shorts) = arg.to_short() {
            while let Some(short) = shorts.next_flag() {
                let opt_char = match short {
                    Ok(s) => s,
                    Err(e) => return Err(format!("Can't parse {:?}", e)),
                };

                let opt_char_str: String = format!("-{}", opt_char);

                let opt_config = opt_cfg_list
                    .iter()
                    .find(|cfg| cfg.match_option_short(&opt_char));
                if opt_config.is_none() {
                    return Err(format!("Unknown option: {}", opt_char_str));
                } else if let Some(oc) = opt_config {
                    // Check duplicate options. Counter options and options that trigger a function call
                    // can be used multiple times.
                    if oc.assigned.get() && !oc.is_duplicate_allowed() {
                        return Err(format!(
                            "Duplicate option: {} ({})",
                            opt_char_str,
                            oc.options_string()
                        ));
                    }
                    oc.assigned.set(true);

                    if oc.singleton {
                        shell_code.clear();
                    }

                    match &oc.opt_type {
                        OptType::Flag(target) => {
                            prev_counter = counter_assign(&mut shell_code, prev_counter);
                            shell_code.push(assign_target(target, VarValue::BoolValue(true)));
                        }
                        OptType::ModeSwitch(target, mode_value) => {
                            prev_counter = counter_assign(&mut shell_code, prev_counter);
                            // Conflict detection is done at end of processing.
                            shell_code.push(assign_target(
                                target,
                                VarValue::StringValue(OsString::from(mode_value)),
                            ));
                        }
                        OptType::Assignment(target) => {
                            prev_counter = counter_assign(&mut shell_code, prev_counter);

                            let opt_arg = match shorts.next_value_os() {
                                Some(v) => Some(v),
                                None => raw_args.next_os(&mut cursor),
                            };

                            if let Some(opt_arg) = opt_arg {
                                shell_code.push(assign_target(
                                    target,
                                    VarValue::StringValue(opt_arg.to_owned()),
                                ));
                            } else {
                                return Err(format!("Missing argument for: {}", opt_char_str));
                            }
                        }
                        OptType::Counter(target) => {
                            if let Some((prev_target, _)) = prev_counter {
                                if prev_target != target {
                                    counter_assign(&mut shell_code, prev_counter);
                                }
                            }
                            oc.count_value.set(oc.count_value.get() + 1);

                            prev_counter = Some((target, oc.count_value.get()));
                        }
                    }
                }
            }
        } else {
            if cmd_line_args.posix {
                after_separator = true;
            }
            prev_counter = counter_assign(&mut shell_code, prev_counter);
            // Argument
            let argument = arg.to_value_os().to_owned();
            if let (true, Some(array)) = (after_separator, &cmd_line_args.remainder) {
                shell_code.push(CodeChunk::AddToArray(
                    array.clone(),
                    VarValue::StringValue(argument),
                ));
            } else if let Some(func) = &cmd_line_args.arg_callback {
                shell_code.push(CodeChunk::CallFunction(
                    func.clone(),
                    VarValue::StringValue(argument),
                ));
            } else {
                arguments.push(argument);
            }
        }
    }

    // let mut cl_tok = CmdLineTokenizer::new(cmd_line_args.script_args.clone(), cmd_line_args.posix);

    // let mut after_separator = false;
    // let mut prev_counter: Option<(&OptTarget, u16)> = None;

    // while let Some(e) = cl_tok.next()? {
    //     if let CmdLineElement::Separator = e {
    //         prev_counter = counter_assign(&mut shell_code, prev_counter);
    //         after_separator = true;
    //         continue;
    //     } else if let CmdLineElement::Argument(value) = e {
    //         prev_counter = counter_assign(&mut shell_code, prev_counter);
    //         if let (true, Some(array)) = (after_separator, &cmd_line_args.remainder) {
    //             shell_code.push(CodeChunk::AddToArray(
    //                 array.clone(),
    //                 VarValue::StringValue(value),
    //             ));
    //         } else if let Some(func) = &cmd_line_args.arg_callback {
    //             shell_code.push(CodeChunk::CallFunction(
    //                 func.clone(),
    //                 VarValue::StringValue(value),
    //             ));
    //         } else {
    //             arguments.push(value);
    //         }
    //     } else {
    //         let opt_value = match &e {
    //             CmdLineElement::LongOptionValue(_, v) => Some(v),
    //             _ => None,
    //         };

    //         let opt_config = opt_cfg_list.iter().find(|cfg| cfg.match_option_long(&name));

    //         if opt_config.is_none() {
    //             return Err(format!("Unknown option: {}", e));
    //         } else if let Some(oc) = opt_config {
    //             // Check duplicate options. Counter options and options that trigger a function call
    //             // can be used multiple times.
    //             if oc.assigned.get() && !oc.is_duplicate_allowed() {
    //                 return Err(format!("Duplicate option: {} ({})", e, oc.options_string()));
    //             }
    //             oc.assigned.set(true);

    //             if oc.singleton {
    //                 shell_code.clear();
    //             }

    //             match &oc.opt_type {
    //                 OptType::Flag(target) => {
    //                     prev_counter = counter_assign(&mut shell_code, prev_counter);
    //                     let bool_val = VarValue::BoolValue(optional_str_to_bool(opt_value, true)?);
    //                     shell_code.push(assign_target(target, bool_val));
    //                 }
    //                 OptType::ModeSwitch(target, value) => {
    //                     prev_counter = counter_assign(&mut shell_code, prev_counter);
    //                     if opt_value.is_some() {
    //                         Err(format!("{}: No value supported.", oc.options_string()))?;
    //                     }
    //                     // Conflict detection is done at end of processing.
    //                     shell_code.push(assign_target(
    //                         target,
    //                         VarValue::StringValue(OsString::from(value)),
    //                     ));
    //                 }
    //                 OptType::Assignment(target) => {
    //                     prev_counter = counter_assign(&mut shell_code, prev_counter);
    //                     let opt_arg = match opt_value {
    //                         Some(v) => Some(v.clone()),
    //                         None => cl_tok.get_option_argument(),
    //                     };
    //                     if let Some(opt_arg) = opt_arg {
    //                         shell_code.push(assign_target(target, VarValue::StringValue(opt_arg)));
    //                     } else {
    //                         return Err(format!("Missing argument for: {}", e));
    //                     }
    //                 }
    //                 OptType::Counter(target) => {
    //                     if let Some((prev_target, _)) = prev_counter {
    //                         if prev_target != target {
    //                             counter_assign(&mut shell_code, prev_counter);
    //                         }
    //                     }

    //                     let value = optional_string_to_optional_u16(opt_value)?;
    //                     oc.count_value
    //                         .set(value.unwrap_or(oc.count_value.get() + 1));

    //                     prev_counter = Some((target, oc.count_value.get()));
    //                 }
    //             }

    //             if oc.singleton {
    //                 shell_code.push(CodeChunk::Exit(0));
    //                 return Ok(shell_code);
    //             }
    //         }
    //     }
    // }
    counter_assign(&mut shell_code, prev_counter);

    // Check duplicates for ModeSwitches
    // and handle required
    for name in shell_name_table.keys() {
        if shell_name_table.get(name).unwrap().len() > 1 {
            let mut used_tab = vec![];
            let mut all_tab = vec![];
            let mut required = false;
            for idx in shell_name_table.get(name).unwrap() {
                if opt_cfg_list[*idx].assigned.get() {
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
                    if oc.required && !oc.assigned.get() {
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

/// If counter is not None, creates the counter assignment.
/// Always returns None
fn counter_assign<'a>(
    shell_code: &mut Vec<CodeChunk>,
    counter: Option<(&'a OptTarget, u16)>,
) -> Option<(&'a OptTarget, u16)> {
    if let Some((target, value)) = counter {
        shell_code.push(assign_target(target, VarValue::IntValue(value as i32)));
        None
    } else {
        counter
    }
}
