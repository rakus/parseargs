/*
 * Part of parseargs - a command line options parser for shell scripts
 *
 * Copyright (c) 2023 Ralf Schandl
 * This code is licensed under MIT license (see LICENSE.txt for details).
 */

use std::fmt;

/**
 * Element extracted from the command line.
 */
#[derive(Debug, PartialEq)]
pub enum CmdLineElement {
    /// A short option like '-l' without the leading dash
    ShortOption(char),
    /// A long option like '--long' without the leading dashes
    LongOption(String),
    /// A long option (without the leading dashes) with value (from --option=value)
    LongOptionValue(String, String),
    /// A program argument.
    Argument(String),
    /**
     * The option/arguments separator "--". Used by caller if the arguments before a '--' should
     * be handled differently than after it or it is just ignored.
     */
    Separator,
}

impl fmt::Display for CmdLineElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmdLineElement::ShortOption(c) => write!(f, "-{}", c),
            CmdLineElement::LongOption(c) => write!(f, "--{}", c),
            CmdLineElement::LongOptionValue(o, v) => write!(f, "--{}={}", o, v),
            CmdLineElement::Argument(v) => write!(f, "'{}'", v),
            CmdLineElement::Separator => write!(f, "--"),
        }
    }
}

pub struct CmdLineTokenizer {
    /// Vector with command line arguments
    cmd_line_args: Vec<String>,
    /// Index of the next argument to process
    cmd_line_args_idx: usize,
    /// Whether to stop option processing on the first non-option.
    posix: bool,
    /**
     * Whether to only returns Arguments. Switched to true when '--' is found
     * or on the first non-option if posix == true
     */
    args_only: bool,
    /**
     * Left over characters from combined short options. With -abc, this will
     * hold ['b', 'c'].
     */
    left_over: Vec<char>,
}

impl CmdLineTokenizer {
    pub fn new(args: Vec<String>, posix: bool) -> CmdLineTokenizer {
        CmdLineTokenizer {
            cmd_line_args: args,
            cmd_line_args_idx: 0,
            posix,
            args_only: false,
            left_over: Vec::new(),
        }
    }

    // Internal: get next part (seperated string) from the command line.
    fn next_part(&mut self) -> Option<String> {
        if self.cmd_line_args_idx >= self.cmd_line_args.len() {
            None
        } else {
            let r = Some(self.cmd_line_args[self.cmd_line_args_idx].to_string());
            self.cmd_line_args_idx += 1;
            r
        }
    }

    /**
     * Returns the next command line element.
     */
    pub fn next(&mut self) -> Option<CmdLineElement> {
        if !self.left_over.is_empty() {
            // next character from combined short options (-xyz)
            let chr = self.left_over.remove(0);
            Some(CmdLineElement::ShortOption(chr))
        } else if self.args_only {
            self.next_part().map(CmdLineElement::Argument)
        } else {
            match self.next_part() {
                None => None,
                Some(s) => {
                    if s.eq("--") {
                        self.args_only = true;
                        Some(CmdLineElement::Separator)
                    } else if s.eq("-") {
                        if self.posix {
                            self.args_only = true;
                        }
                        Some(CmdLineElement::Argument(s))
                    } else if let Some(opt_str) = s.strip_prefix("--") {
                        // parse long option
                        if let Some(eq_idx) = opt_str.find('=') {
                            let name = opt_str[..eq_idx].to_string();
                            let value = opt_str[eq_idx + 1..].to_string();
                            Some(CmdLineElement::LongOptionValue(name, value))
                        } else {
                            Some(CmdLineElement::LongOption(opt_str.to_string()))
                        }
                    } else if let Some(opt_str) = s.strip_prefix('-') {
                        // skip leading '-'
                        let mut cs = opt_str.chars();
                        let chr = cs.next().unwrap();
                        cs.for_each(|f| self.left_over.push(f));
                        Some(CmdLineElement::ShortOption(chr))
                    } else {
                        if self.posix {
                            self.args_only = true;
                        }
                        Some(CmdLineElement::Argument(s))
                    }
                }
            }
        }
    }

    /**
     * Returns an argument for a previous option.
     * Also handles combined options like `-ooutfile`.
     */
    pub fn get_option_argument(&mut self) -> Option<String> {
        if !self.left_over.is_empty() {
            let ret = Some(self.left_over.clone().into_iter().collect());
            self.left_over.clear();
            ret
        } else {
            self.next_part()
        }
    }
}

#[cfg(test)]
mod cmd_line_element_tests {
    use crate::arg_parser::CmdLineElement;

    #[test]
    fn test_to_string() {
        assert_eq!("-d", format!("{}", CmdLineElement::ShortOption('d')));
        assert_eq!(
            "--debug",
            format!("{}", CmdLineElement::LongOption("debug".to_string()))
        );
        assert_eq!(
            "--file=output",
            format!(
                "{}",
                CmdLineElement::LongOptionValue("file".to_string(), "output".to_string())
            )
        );
        assert_eq!(
            "'filename'",
            format!("{}", CmdLineElement::Argument("filename".to_string()))
        );
        assert_eq!("--", format!("{}", CmdLineElement::Separator));
    }
}
#[cfg(test)]
mod arg_parser_tests {
    use crate::arg_parser::{CmdLineElement, CmdLineTokenizer};

    #[test]
    fn test_normal() {
        let args = [
            "-d",
            "-o",
            "outfile",
            "--name=parseargs",
            "--version",
            "1.0",
            "one",
            "two",
        ]
        .map(String::from)
        .to_vec();

        let mut pa = CmdLineTokenizer::new(args, false);

        assert_eq!(Some(CmdLineElement::ShortOption('d')), pa.next());
        assert_eq!(Some(CmdLineElement::ShortOption('o')), pa.next());
        assert_eq!(Some("outfile".to_string()), pa.get_option_argument());
        assert_eq!(
            Some(CmdLineElement::LongOptionValue(
                "name".to_string(),
                "parseargs".to_string()
            )),
            pa.next()
        );
        assert_eq!(
            Some(CmdLineElement::LongOption("version".to_string())),
            pa.next()
        );
        assert_eq!(Some("1.0".to_string()), pa.get_option_argument());
        assert_eq!(Some(CmdLineElement::Argument("one".to_string())), pa.next());
        assert_eq!(Some(CmdLineElement::Argument("two".to_string())), pa.next());
        assert_eq!(None, pa.next());
    }

    #[test]
    fn test_mixed() {
        let args = ["one", "-d", "-o", "outfile", "--name=parseargs", "two"]
            .map(String::from)
            .to_vec();

        let mut pa = CmdLineTokenizer::new(args, false);

        assert_eq!(Some(CmdLineElement::Argument("one".to_string())), pa.next());
        assert_eq!(Some(CmdLineElement::ShortOption('d')), pa.next());
        assert_eq!(Some(CmdLineElement::ShortOption('o')), pa.next());
        assert_eq!(Some("outfile".to_string()), pa.get_option_argument());
        assert_eq!(
            Some(CmdLineElement::LongOptionValue(
                "name".to_string(),
                "parseargs".to_string()
            )),
            pa.next()
        );
        assert_eq!(Some(CmdLineElement::Argument("two".to_string())), pa.next());
        assert_eq!(None, pa.next());
    }

    #[test]
    fn test_combined() {
        let args = ["-dooutfile", "one"].map(String::from).to_vec();

        let mut pa = CmdLineTokenizer::new(args, false);

        assert_eq!(Some(CmdLineElement::ShortOption('d')), pa.next());
        assert_eq!(Some(CmdLineElement::ShortOption('o')), pa.next());
        assert_eq!(Some("outfile".to_string()), pa.get_option_argument());
        assert_eq!(Some(CmdLineElement::Argument("one".to_string())), pa.next());
        assert_eq!(None, pa.next());
    }

    #[test]
    fn test_dash_dash() {
        let args = ["-d", "--", "-o"].map(String::from).to_vec();

        let mut pa = CmdLineTokenizer::new(args, false);

        assert_eq!(Some(CmdLineElement::ShortOption('d')), pa.next());
        assert_eq!(Some(CmdLineElement::Separator), pa.next());
        assert_eq!(Some(CmdLineElement::Argument("-o".to_string())), pa.next());
        assert_eq!(None, pa.next());
    }

    #[test]
    fn test_posix() {
        let args = ["-d", "one", "-o", "outfile", "--name=parseargs", "two"]
            .map(String::from)
            .to_vec();

        let mut pa = CmdLineTokenizer::new(args, true);

        assert_eq!(Some(CmdLineElement::ShortOption('d')), pa.next());
        assert_eq!(Some(CmdLineElement::Argument("one".to_string())), pa.next());
        assert_eq!(Some(CmdLineElement::Argument("-o".to_string())), pa.next());
        assert_eq!(
            Some(CmdLineElement::Argument("outfile".to_string())),
            pa.next()
        );
        assert_eq!(
            Some(CmdLineElement::Argument("--name=parseargs".to_string())),
            pa.next()
        );
        assert_eq!(Some(CmdLineElement::Argument("two".to_string())), pa.next());
        assert_eq!(None, pa.next());
    }

    #[test]
    fn test_posix_with_dash() {
        let args = ["-d", "-", "-o", "outfile", "--name=parseargs", "two"]
            .map(String::from)
            .to_vec();

        let mut pa = CmdLineTokenizer::new(args, true);

        assert_eq!(Some(CmdLineElement::ShortOption('d')), pa.next());
        assert_eq!(Some(CmdLineElement::Argument("-".to_string())), pa.next());
        assert_eq!(Some(CmdLineElement::Argument("-o".to_string())), pa.next());
        assert_eq!(
            Some(CmdLineElement::Argument("outfile".to_string())),
            pa.next()
        );
        assert_eq!(
            Some(CmdLineElement::Argument("--name=parseargs".to_string())),
            pa.next()
        );
        assert_eq!(Some(CmdLineElement::Argument("two".to_string())), pa.next());
        assert_eq!(None, pa.next());
    }
}
