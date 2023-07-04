#![allow(unused)]

use std::{fmt, ascii::escape_default};

const SHELL_TRUE: &str = "'true'";
const SHELL_FALSE: &str = "''";
const SHELL_EXIT: &str = "exit";

pub enum VarValue {
    StringValue(String),
    IntValue(i32),
    BoolValue(bool),
    None
}
impl VarValue {
    /// Escape a String for usage as shell value
    /// The value is enclosed in single quotes and a single quote in the value is replaced with
    /// "'\''".
    fn escape_string(value: &String) -> String {
        let mut esc = String::new();
        esc.push('\'');
        for c in value.chars() {
            if c == '\'' {
                esc.push_str("'\\''");
            } else {
                esc.push(c);
            }
        }
        esc.push('\'');
        esc
    }
}

impl fmt::Display for VarValue {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s : String = match self {
            VarValue::StringValue(s) => VarValue::escape_string(s),
            VarValue::IntValue(i) => i.to_string(),
            //VarValue::BoolValue(b) => if *b { "'true'".to_string() } else { "''".to_string() },
            VarValue::BoolValue(b) => if *b { SHELL_TRUE.to_string()} else { SHELL_FALSE.to_string() },
            VarValue::None => "".to_string(),
        };
        fmt.write_str(&s)
    }
}

//#[derive(Clone)]
pub enum CodeChunk {
    Separator,

    DeclareLocalVar(String),
    DeclareLocalIntVar(String),
    DeclareArrayVar(String),

    AssignVar(String, VarValue),
    AssignEmptyArray(String),
    AddToArray(String, VarValue),

    CheckForFunction(String),
    CallFunction(String, VarValue),

    SetArgs(Vec<String>),
    Exit(i32),
    FalseReturn,
}

#[derive(Clone, Copy)]
pub struct CodeTemplates {
    /** Whether the shell supports arrays. */
    pub supports_arrays: bool,
    /** Whether the shell supports typeset. */
    pub supports_local_vars: bool,
    statement_separator: &'static str,

    declare_local_variable: &'static str,
    declare_local_int_variable: &'static str,

    declare_array_variable: &'static str,

    assign_variable: &'static str,

    assign_empty_array: &'static str,
    add_to_array: &'static str,

    check_function_exists: &'static str,
    call_function: &'static str,
    exit: &'static str,

    false_return:  &'static str,

    set_args: &'static str,
}

impl CodeTemplates {
    pub fn format_vector(&self, chunks: &Vec<CodeChunk>) -> String {
        let mut str = String::new();
        let mut first = true;
        let separator = CodeChunk::Separator;
        for chunk in chunks.iter() {
            if ! first {
                str.push_str(&self.format(&separator));
            }
            str.push_str(&self.format(chunk));
            first = false;
        }
        str
    }

    pub fn format(&self, chunk: &CodeChunk) -> String {
        match chunk {
            CodeChunk::DeclareLocalVar(name) => {
                if !self.supports_local_vars { panic!("Local variables not supported") }
                self.format_code_name(self.declare_local_variable, name)
            }
            CodeChunk::DeclareLocalIntVar(name) => {
                if !self.supports_local_vars { panic!("Local variables not supported") }
                self.format_code_name(self.declare_local_int_variable, name)
            }
            CodeChunk::DeclareArrayVar(name) => {
                if !self.supports_arrays { panic!("Arrays not supported") }
                self.format_code_name(self.declare_array_variable, name)
            }
            CodeChunk::AssignVar(name, value) => {
                self.format_code_name_value(self.assign_variable, name, value)
            }
            CodeChunk::AssignEmptyArray(name) => {
                if !self.supports_arrays { panic!("Arrays not supported") }
                self.format_code_name(self.assign_empty_array, name)
            }
            CodeChunk::AddToArray(name, value) => {
                if !self.supports_arrays { panic!("Arrays not supported") }
                self.format_code_name_value(self.add_to_array, name, value)
            }
            CodeChunk::CheckForFunction(name) => {
                self.format_code_name(self.check_function_exists, name)
            }
            CodeChunk::CallFunction(name, value) => {
                self.format_code_name_value(self.call_function, name, value)
            }
            CodeChunk::Exit(exit_value) => self.format_code_int_value(self.exit, *exit_value),
            CodeChunk::FalseReturn => self.false_return.to_string(),
            CodeChunk::SetArgs(args) => self.format_code_args(self.set_args, args),
            CodeChunk::Separator => self.statement_separator().to_string(),
        }
    }
    fn format_code_name(&self, tmpl: &str, name: &String) -> String {
        let mut str = tmpl.replace("{NAME}", name);
        str
    }
    fn format_code_name_value(&self, tmpl: &str, name: &String, value: &VarValue) -> String {
        let mut str = tmpl.replace("{NAME}", name);
        str = str.replace("{VALUE}", &value.to_string());
        str
    }
    fn format_code_name_int_value(&self, tmpl: &str, name: &String, value: i32) -> String {
        let mut str = tmpl.replace("{NAME}", name);
        str = str.replace("{VALUE}", &value.to_string());
        str
    }
    fn format_code_int_value(&self, tmpl: &str, value: i32) -> String {
        let mut str = tmpl.replace("{VALUE}", &value.to_string());
        str
    }
    fn format_code_args(&self, tmpl: &str, args: &Vec<String>) -> String {
        let mut args_str = String::new();
        for (idx, a) in args.iter().enumerate() {
            if idx > 0 {
                args_str.push(' ');
            }
            args_str.push_str(&VarValue::escape_string(a));
        }
        let mut str = tmpl.replace("{ARGS}", &args_str);
        str
    }
    fn statement_separator(&self) -> &str {
        self.statement_separator
    }

}

const SH_TEMPLATE : CodeTemplates = CodeTemplates {
    supports_arrays : false,
    supports_local_vars: false,

    statement_separator : ";\n",

    declare_local_variable: "",
    declare_local_int_variable: "",
    declare_array_variable: "",

    assign_variable : "{NAME}={VALUE}",

    assign_empty_array : "",
    add_to_array : "",

    check_function_exists : "LC_ALL=C type {NAME} 2>/dev/null | grep function >/dev/null || (echo >&2 \"ERROR: Function '{NAME}' does not exist.\";exit 1) || exit 127" ,
    call_function : "{NAME} {VALUE} || exit $?",

    set_args : "set -- {ARGS}",
    exit : "exit {VALUE}",
    false_return: "false"
};

const BASH_TEMPLATE : CodeTemplates = CodeTemplates {
    supports_arrays : true,
    supports_local_vars: true,

    declare_local_variable: "typeset {NAME}",
    declare_local_int_variable: "typeset -i {NAME}",
    declare_array_variable: "typeset -a {NAME}",

    assign_empty_array : "{NAME}=()",
    add_to_array : "{NAME}+=({VALUE})",

    check_function_exists: "typeset -f {NAME} >/dev/null 2>&1 || { echo >&2 \"ERROR: Function '{NAME}' does not exist.\";exit 127; }",

    ..SH_TEMPLATE
};

const KSH_TEMPLATE: CodeTemplates = CodeTemplates {
    assign_empty_array: "set -A {NAME}",

    ..BASH_TEMPLATE
};

pub fn get_shell_template(shell: &str) -> Option<&CodeTemplates> {
    match shell {
        "bash" => Some(&BASH_TEMPLATE),
        "zsh"  => Some(&BASH_TEMPLATE),
        "ksh"  => Some(&KSH_TEMPLATE),
        "sh"   => Some(&SH_TEMPLATE),
        _ => None,
    }

    //SHELLS.get(shell)
}

#[cfg(test)]
mod var_value_tests {
    use super::VarValue;

    #[test]
    fn test_string_escape_simple() {
        assert_eq!("'test'".to_string(), VarValue::StringValue("test".to_string()).to_string());
    }

    #[test]
    fn test_string_escape_empty() {
        assert_eq!("''".to_string(), VarValue::StringValue("".to_string()).to_string());
    }

    #[test]
    fn test_string_escape_quote() {
        assert_eq!("'don'\\''t'".to_string(), VarValue::StringValue("don't".to_string()).to_string());
    }

    #[test]
    fn test_string_escape_quote_border() {
        assert_eq!("''\\''do'\\'''".to_string(), VarValue::StringValue("'do'".to_string()).to_string());
    }

    #[test]
    fn test_string_escape_quote_only() {
        assert_eq!("''\\'''\\'''\\'''".to_string(), VarValue::StringValue("'''".to_string()).to_string());
    }

    #[test]
    fn test_string_double_quote() {
        assert_eq!("'\"hello\"'".to_string(), VarValue::StringValue("\"hello\"".to_string()).to_string());
    }

    #[test]
    fn test_int_13() {
        assert_eq!("13".to_string(), VarValue::IntValue(13).to_string());
    }

    #[test]
    fn test_int_0() {
        assert_eq!("0".to_string(), VarValue::IntValue(0).to_string());
    }

    #[test]
    fn test_int_minus_13() {
        assert_eq!("-13".to_string(), VarValue::IntValue(-13).to_string());
    }

    #[test]
    fn test_bool_true() {
        assert_eq!("'true'".to_string(), VarValue::BoolValue(true).to_string());
    }

    #[test]
    fn test_bool_false() {
        assert_eq!("''".to_string(), VarValue::BoolValue(false).to_string());
    }
}

#[cfg(test)]
mod shell_template_test {
    use super::*;

    #[test]
    fn get_not_existing_shell_template() {
        let x = get_shell_template("rksh");
        assert!(if let Some(_) = x { false } else { true });
    }

    #[test]
    fn get_existing_shell_template() {
        let x = get_shell_template("sh");
        assert!(if let Some(_) = x { true } else { false }, "shell_template for SH not found");
        let x = get_shell_template("bash");
        assert!(if let Some(_) = x { true } else { false }, "shell_template for BASH not found");
        let x = get_shell_template("ksh");
        assert!(if let Some(_) = x { true } else { false }, "shell_template for KSH not found");
        let x = get_shell_template("zsh");
        assert!(if let Some(_) = x { true } else { false }, "shell_template for ZSH not found");
    }

    #[test]
    fn format_sh() {
        let shell = get_shell_template("sh").unwrap();

        let var_name = "name".to_string();

        let chunk = CodeChunk::DeclareLocalVar(var_name.clone());
        assert!(std::panic::catch_unwind(|| shell.format(&chunk)).is_err());

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::StringValue("value".to_string()));
        assert_eq!("name='value'", shell.format(&chunk));

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::IntValue(13));
        assert_eq!("name=13", shell.format(&chunk));

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::BoolValue(true));
        assert_eq!("name='true'", shell.format(&chunk));

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::BoolValue(false));
        assert_eq!("name=''", shell.format(&chunk));

        let var_name = "func".to_string();

        let chunk = CodeChunk::CallFunction(var_name.clone(), VarValue::StringValue("value".to_string()));
        assert_eq!("func 'value' || exit $?", shell.format(&chunk));

        let chunk = CodeChunk::CheckForFunction(var_name.clone());
        assert_eq!("LC_ALL=C type func 2>/dev/null | grep function >/dev/null || (echo >&2 \"ERROR: Function 'func' does not exist.\";exit 1) || exit 127", shell.format(&chunk));

        let chunk = CodeChunk::SetArgs(vec!["one".to_string(), "don't".to_string(), "count".to_string()]);
        assert_eq!("set -- 'one' 'don'\\''t' 'count'", shell.format(&chunk));

        let chunk = CodeChunk::Exit(13);
        assert_eq!("exit 13", shell.format(&chunk));

        // Unsupported features

        // typeset is not supported
        let chunk = CodeChunk::DeclareLocalIntVar(var_name.clone());
        assert!(std::panic::catch_unwind(|| shell.format(&chunk)).is_err());

        let chunk = CodeChunk::DeclareArrayVar(var_name.clone());
        assert!(std::panic::catch_unwind(|| shell.format(&chunk)).is_err());

        // arrays are not supported
        let chunk = CodeChunk::AssignEmptyArray(var_name.clone());
        assert!(std::panic::catch_unwind(|| shell.format(&chunk)).is_err());

        let chunk = CodeChunk::AddToArray(var_name.clone(), VarValue::StringValue("test".to_string()));
        assert!(std::panic::catch_unwind(|| shell.format(&chunk)).is_err());
    }

    #[test]
    fn format_bash() {
        let shell = get_shell_template("bash").unwrap();

        let var_name = "name".to_string();

        let chunk = CodeChunk::DeclareLocalVar(var_name.clone());
        assert_eq!("typeset name", shell.format(&chunk));
        let chunk = CodeChunk::DeclareLocalIntVar(var_name.clone());
        assert_eq!("typeset -i name", shell.format(&chunk));
        let chunk = CodeChunk::DeclareArrayVar(var_name.clone());
        assert_eq!("typeset -a name", shell.format(&chunk));

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::StringValue("value".to_string()));
        assert_eq!("name='value'", shell.format(&chunk));

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::IntValue(13));
        assert_eq!("name=13", shell.format(&chunk));

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::BoolValue(true));
        assert_eq!("name='true'", shell.format(&chunk));

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::BoolValue(false));
        assert_eq!("name=''", shell.format(&chunk));

        let chunk = CodeChunk::AssignEmptyArray(var_name.clone());
        assert_eq!("name=()", shell.format(&chunk));

        let chunk = CodeChunk::AddToArray(var_name.clone(), VarValue::StringValue("test".to_string()));
        assert_eq!("name+=('test')", shell.format(&chunk));

        let var_name = "func".to_string();

        let chunk = CodeChunk::CallFunction(var_name.clone(), VarValue::StringValue("value".to_string()));
        assert_eq!("func 'value' || exit $?", shell.format(&chunk));

        let chunk = CodeChunk::CheckForFunction(var_name.clone());
        assert_eq!("typeset -f func >/dev/null 2>&1 || { echo >&2 \"ERROR: Function 'func' does not exist.\";exit 127; }", shell.format(&chunk));

        let chunk = CodeChunk::SetArgs(vec!["one".to_string(), "don't".to_string(), "count".to_string()]);
        assert_eq!("set -- 'one' 'don'\\''t' 'count'", shell.format(&chunk));

        let chunk = CodeChunk::Exit(13);
        assert_eq!("exit 13", shell.format(&chunk));
    }

    #[test]
    fn format_ksh() {
        let shell = get_shell_template("ksh").unwrap();

        let var_name = "name".to_string();

        let chunk = CodeChunk::DeclareLocalVar(var_name.clone());
        assert_eq!("typeset name", shell.format(&chunk));
        let chunk = CodeChunk::DeclareLocalIntVar(var_name.clone());
        assert_eq!("typeset -i name", shell.format(&chunk));
        let chunk = CodeChunk::DeclareArrayVar(var_name.clone());
        assert_eq!("typeset -a name", shell.format(&chunk));

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::StringValue("value".to_string()));
        assert_eq!("name='value'", shell.format(&chunk));

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::IntValue(13));
        assert_eq!("name=13", shell.format(&chunk));

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::BoolValue(true));
        assert_eq!("name='true'", shell.format(&chunk));

        let chunk = CodeChunk::AssignVar(var_name.clone(), VarValue::BoolValue(false));
        assert_eq!("name=''", shell.format(&chunk));

        let chunk = CodeChunk::AssignEmptyArray(var_name.clone());
        assert_eq!("set -A name", shell.format(&chunk));

        let chunk = CodeChunk::AddToArray(var_name.clone(), VarValue::StringValue("test".to_string()));
        assert_eq!("name+=('test')", shell.format(&chunk));

        let var_name = "func".to_string();

        let chunk = CodeChunk::CallFunction(var_name.clone(), VarValue::StringValue("value".to_string()));
        assert_eq!("func 'value' || exit $?", shell.format(&chunk));

        let chunk = CodeChunk::CheckForFunction(var_name.clone());
        assert_eq!("typeset -f func >/dev/null 2>&1 || { echo >&2 \"ERROR: Function 'func' does not exist.\";exit 127; }", shell.format(&chunk));

        let chunk = CodeChunk::SetArgs(vec!["one".to_string(), "don't".to_string(), "count".to_string()]);
        assert_eq!("set -- 'one' 'don'\\''t' 'count'", shell.format(&chunk));

        let chunk = CodeChunk::Exit(13);
        assert_eq!("exit 13", shell.format(&chunk));
    }
}
