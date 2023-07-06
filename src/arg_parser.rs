use std::fmt;

#[derive(Debug)]
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
    The option/arguments separator "--". Used by caller if the arguments before a '--' should
    be handled differently than after it or it is just ignored.
     */
    Separator,
}

impl fmt::Display for CmdLineElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmdLineElement::ShortOption(c) => write!(f, "-{} ", c),
            CmdLineElement::LongOption(c) => write!(f, "--{} ", c),
            CmdLineElement::LongOptionValue(o, v) => write!(f, "--{}={} ", o, v),
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
    Whether to only returns Arguments. Switched to true when '--' is found
    or on the first non-option if posix == true
     */
    args_only: bool,
    /**
    Left over characters from combined short options. With -abc, this will
    hold ['b', 'c'].
     */
    left_over: Vec<char>,
}

impl CmdLineTokenizer {
    pub fn build(args: Vec<String>, posix: bool) -> CmdLineTokenizer {
        CmdLineTokenizer {
            cmd_line_args: args,
            cmd_line_args_idx: 0,
            posix,
            args_only: false,
            left_over: Vec::new(),
        }
    }

    fn next_arg(&mut self) -> Option<String> {
        if self.cmd_line_args_idx >= self.cmd_line_args.len() {
            None
        } else {
            let r = Some(self.cmd_line_args[self.cmd_line_args_idx].to_string());
            self.cmd_line_args_idx += 1;
            r
        }
    }

    pub fn next(&mut self) -> Option<CmdLineElement> {
        if !self.left_over.is_empty() {
            let chr = self.left_over.remove(0);
            Some(CmdLineElement::ShortOption(chr))
        } else if self.args_only {
            match self.next_arg() {
                None => None,
                Some(s) => Some(CmdLineElement::Argument(s.to_string())),
            }
        } else {
            match self.next_arg() {
                None => None,
                Some(s) => {
                    if s.eq("--") {
                        self.args_only = true;
                        Some(CmdLineElement::Separator)
                    } else if s.eq("-") {
                        if self.posix {
                            self.args_only = true;
                        }
                        Some(CmdLineElement::Argument(s.to_string()))
                    } else if s.starts_with("--") {
                        // parse long option
                        if let Some(eq_idx) = s.find("=") {
                            let name = s[2..eq_idx].to_string();
                            let value = s[eq_idx + 1..].to_string();
                            Some(CmdLineElement::LongOptionValue(name, value))
                        } else {
                            Some(CmdLineElement::LongOption(s[2..].to_string()))
                        }
                    } else if s.starts_with("-") {
                        // skip leading '-'
                        let mut cs = s[1..].chars();
                        let chr = cs.next().unwrap();
                        cs.for_each(|f| self.left_over.push(f));
                        Some(CmdLineElement::ShortOption(chr))
                    } else {
                        if self.posix {
                            self.args_only = true;
                        }
                        Some(CmdLineElement::Argument(s.to_string()))
                    }
                }
            }
        }
    }

    pub fn get_option_argument(&mut self) -> Option<String> {
        if !self.left_over.is_empty() {
            let ret = Some(self.left_over.clone().into_iter().collect());
            self.left_over.clear();
            ret
        } else {
            self.next_arg()
        }
    }
}
