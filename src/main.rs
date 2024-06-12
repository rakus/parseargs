//
// Part of parseargs - a command line options parser for shell scripts
//
// Copyright (c) 2023 Ralf Schandl
// This code is licensed under MIT license (see LICENSE.txt for details).
//

//mod cmd_line;
mod opt_def;
mod parseargs;
mod shell_code;

use std::io::{stdout, IsTerminal};
use std::process::exit;
use std::{ffi::OsString, panic::catch_unwind};

use clap::{CommandFactory, Parser};

const GIT_HASH: &str = env!("GIT_HASH_STATUS");

/// Command line arguments.
#[derive(Parser, Debug)]
#[clap(
    disable_help_flag = true,
    disable_version_flag = true,
    verbatim_doc_comment
)]
#[command(version)]
struct CmdLineArgs {
    /// Definition of supported shell options.
    /// Can be given multiple times.
    #[arg(short = 'o', long = "options", value_name = "OPT-DEFs")]
    options_list: Option<Vec<String>>,

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

    /// Stop option processing on first none-option.
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

    /// Enable debug output to STDERR.
    #[arg(short = 'd', long = "debug")]
    debug: bool,

    /// Print help.
    #[arg(long)]
    help: bool,

    /// Print version.
    #[arg(long)]
    version: bool,

    /// Shell script options
    #[arg(value_name = "SCRIPT-ARGS")]
    script_args: Vec<OsString>,
}

/// Used by Clap to validate a given str as shell variable/function name and to create a String from it.
fn parse_shell_name(arg: &str) -> Result<String, String> {
    for (idx, chr) in arg.chars().enumerate() {
        if idx == 0 && !chr.is_ascii_alphabetic() {
            Err("Not a valid shell variable or function name")?
        }
        if idx > 0 && !chr.is_ascii_alphanumeric() && chr != '_' {
            Err("Not a valid shell variable or function name")?
        }
    }
    Ok(arg.to_string())
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
                let cmd = CmdLineArgs::command();
                let version_str = cmd.get_version().unwrap_or("UNKNOWN");
                println!("parseargs {} ({})", version_str, GIT_HASH);
                exit(0);
            }

            // Catch a panic and print `exit 1` to exit the calling script.
            match catch_unwind(|| parseargs::parseargs(c)) {
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
