/*
 * Part of parseargs - a command line options parser for shell scripts
 *
 * Copyright (c) 2023 Ralf Schandl
 * This code is licensed under MIT license (see LICENSE.txt for details).
 */

use crate::arg_parser::CmdLineElement;

/**
 * Target for a option. Parseargs either assigns a variable or calls
 * a function.
 */
#[derive(Debug, PartialEq)]
pub enum OptTarget {
    Variable(String),
    Function(String),
}

/**
 * Option attributes. The `*` or `?` before the option target.
 */
#[derive(Debug, PartialEq)]
enum OptAttribute {
    Required,
    Singleton,
}

/**
 * Type of the option.
 */
#[derive(Debug, PartialEq)]
pub enum OptType {
    /// Simple flag to set something to true (false is the default)
    Flag(OptTarget),
    /// Assign a value to a variable. Multiple ModeSwitches use the same
    /// variable with different values.
    ModeSwitch(OptTarget, String),
    /// Assignment option that requires an argument. Like `-o outfile`.
    Assignment(OptTarget),
    /// Counting occuences on the command line. Like -v, -vvv, -v  -vvv,...
    Counter(OptTarget),
}

/**
 * Describes a supported option.
 */
#[derive(Debug, PartialEq)]
pub struct OptConfig {
    /// Every character in this string is a short option.
    pub opt_chars: String,
    /// Every string in this vector is a long option.
    pub opt_strings: Vec<String>,
    // type of option
    pub opt_type: OptType,
    // whether this option is required
    pub required: bool,
    // Whether this is a singleton option. If a singleton option is found, only its action is
    // executed and all other options and arguments are dropped (including other singletons).
    // Typically used for '--help' etc.
    pub singleton: bool,
    // Runtime: Whether this variable has been set
    pub assigned: bool,
    // Runtime: Count of a counting variable
    pub count_value: u16,
}

impl OptConfig {
    /**
     * Returns whether this option is matched by the given command line element.
     *
     * TODO: This is the only reason why we import CmdLineElement. Could also
     *       be implemented for char or string.
     */
    pub fn match_option(&self, el: &CmdLineElement) -> bool {
        match el {
            CmdLineElement::ShortOption(c) => self.opt_chars.find(*c).is_some(),
            CmdLineElement::LongOption(s) => self.opt_strings.contains(s),
            CmdLineElement::LongOptionValue(s, _) => self.opt_strings.contains(s),
            _ => false,
        }
    }

    /**
     * Returns whether duplicate usage of this option is allowed.
     * This is allowed for Counter options and options with a target type Function.
     */
    pub fn is_duplicate_allowed(&self) -> bool {
        matches!(self.opt_type, OptType::Counter(_))
            || matches!(
                self.opt_type,
                OptType::Flag(OptTarget::Function(_))
                    | OptType::ModeSwitch(OptTarget::Function(_), _)
                    | OptType::Assignment(OptTarget::Function(_))
                    | OptType::Counter(OptTarget::Function(_))
            )
    }

    /**
     * Returns the name of the option target. The name could represent a
     * variable or a function.
     */
    pub fn get_target_name(&self) -> String {
        match &self.opt_type {
            OptType::Flag(OptTarget::Function(name))
            | OptType::Flag(OptTarget::Variable(name))
            | OptType::Assignment(OptTarget::Function(name))
            | OptType::Assignment(OptTarget::Variable(name))
            | OptType::Counter(OptTarget::Function(name))
            | OptType::Counter(OptTarget::Variable(name))
            | OptType::ModeSwitch(OptTarget::Function(name), _)
            | OptType::ModeSwitch(OptTarget::Variable(name), _) => name.clone(),
        }
    }

    /**
     * Returns the option target
     */
    pub fn get_target(&self) -> &OptTarget {
        match &self.opt_type {
            OptType::Flag(ot)
            | OptType::Assignment(ot)
            | OptType::Counter(ot)
            | OptType::ModeSwitch(ot, _) => ot,
        }
    }

    /**
     * Returns whether the option target is a function.
     */
    pub fn is_target_function(&self) -> bool {
        matches!(
            &self.opt_type,
            OptType::Flag(OptTarget::Function(_))
                | OptType::Assignment(OptTarget::Function(_))
                | OptType::Counter(OptTarget::Function(_))
                | OptType::ModeSwitch(OptTarget::Function(_), _)
        )
    }

    /**
     * Returns whether the option target is a variable.
     */
    pub fn is_target_variable(&self) -> bool {
        !self.is_target_function()
    }

    /**
     * Formats the option for display. Most likely in error messages.
     * If the short option is `-l` and long `--long` it will return
     * something like `-l/--long`.
     */
    pub fn options_string(&self) -> String {
        let mut sb = String::new();

        if !self.opt_chars.is_empty() {
            for c in self.opt_chars.chars() {
                if !sb.is_empty() {
                    sb.push('/');
                }
                sb.push('-');
                sb.push(c);
            }
        }

        if !self.opt_strings.is_empty() {
            if !self.opt_chars.is_empty() {
                sb.push('/');
            }
            sb.push_str("--");
            sb.push_str(&self.opt_strings.join("/--"));
        }

        sb
    }
}

/**
 * Configuration of the option definition parser.
 */
pub struct ParserConfig {
    /// Whether UTF-8 characters are allowed as option characters.
    allow_utf8_options: bool,
}

/**
 * The source for parsing option definitions.
 */
pub struct ParserSource {
    /// Sequence of characters to parse.
    chars: Vec<char>,
    /// The index of the next character to read.
    index: usize,
    /// Number of characters in `chars`.
    length: usize,

    /// Stack to push/pop a position in the character sequence.
    position_stack: Vec<usize>,

    /// Configuration of the parser
    config: ParserConfig,
}

impl ParserSource {
    /**
     * Creates a new ParserSource.
     */
    pub fn new(string: &str) -> ParserSource {
        let array: Vec<char> = string.chars().collect();
        let len = array.len();
        ParserSource {
            chars: array,
            index: 0,
            length: len,
            position_stack: Vec::new(),
            config: ParserConfig {
                /*
                 * Don't allow UTF-8 chars in options. This would most likely
                 * result in portability problems. What if the parseargs option
                 * defines a smiley as option char, but the current system is
                 * configured with a single-byte character set?
                 */
                allow_utf8_options: false,
            },
        }
    }

    /// Get next character
    pub fn next(&mut self) -> Option<char> {
        if self.index < self.length {
            let res = Some(self.chars[self.index]);
            self.index += 1;
            res
        } else {
            None
        }
    }

    // Get next character if it matches the predicate
    pub fn next_if<P>(&mut self, mut predicate: P) -> Option<char>
    where
        P: FnMut(char) -> bool,
    {
        match self.peek() {
            Some(c) => {
                if (predicate)(c) {
                    self.next()
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Peek at the next character without advancing
    pub fn peek(&mut self) -> Option<char> {
        if self.index < self.length {
            Some(self.chars[self.index])
        } else {
            None
        }
    }

    /// Step back one character in the source
    pub fn back(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    /// Push the current position on the position stack
    pub fn push_pos(&mut self) {
        self.position_stack.push(self.index)
    }

    /// Pop the top of the position stack and jump to that position
    pub fn pop_pos(&mut self) {
        if let Some(idx) = self.position_stack.pop() {
            self.index = idx;
        } else {
            panic!("ParserSource.pop_pos: position_stack is empty")
        }
    }

    /// Jump to the top of the position stack without popping it
    pub fn reset_pos(&mut self) {
        if let Some(idx) = self.position_stack.last() {
            self.index = *idx;
        } else {
            panic!("ParserSource.reset_pos: position_stack is empty")
        }
    }

    /// Drop the top of the position stack
    pub fn drop_pos(&mut self) {
        if self.position_stack.pop().is_none() {
            panic!("ParserSource.drop_pos: position_stack is empty")
        }
    }

    /// Reset the ParserSource toi start parsing from the beginning again.
    /// Currently only used in test.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.index = 0;
        self.position_stack.clear();
    }
}

/**
 * Errors of the parser.
 */
#[derive(Debug, PartialEq)]
pub enum ParsingError {
    /// Nothing parsable found at all
    Empty,
    /// Error with Message
    Error(String),
}

/**
 * Check whether the given character is a valid character for an option.
 *
 * * `chr` - The character to check
 * * `first` - whether this is the first character of an option
 * * `allow_utf8` - whether characters outside the ASCII range are allowed
 */
fn is_valid_opt_char(chr: char, first: bool, allow_utf8: bool) -> bool {
    if allow_utf8 {
        if first {
            chr != '-' && !chr.is_whitespace() && !chr.is_ascii_control()
        } else {
            !chr.is_whitespace() && !chr.is_ascii_control()
        }
    } else if first {
        chr != '-' && chr.is_ascii() && !chr.is_ascii_whitespace() && !chr.is_ascii_control()
    } else {
        chr.is_ascii() && !chr.is_ascii_whitespace() && !chr.is_ascii_control()
    }
}

/**
 * Gets the next character for an option from the source.
 * This handles backslash-escapes for certain characters.
 */
fn get_option_char(ps: &mut ParserSource, first: bool) -> Option<char> {
    let forbidden = vec![':', '#', '=', '+', '%'];

    match ps.next() {
        Some(c) => {
            if c == '\\' {
                match ps.next() {
                    Some(c2) => {
                        if is_valid_opt_char(c2, first, ps.config.allow_utf8_options) {
                            Some(c2)
                        } else {
                            ps.back();
                            None
                        }
                    }
                    None => Some(c),
                }
            } else if is_valid_opt_char(c, first, ps.config.allow_utf8_options)
                && !forbidden.contains(&c)
            {
                Some(c)
            } else {
                ps.back();
                None
            }
        }
        None => None,
    }
}

/**
 * Parses an option. The resulting string either contains a single character
 * for short options or a multiple for long options.
 */
fn parse_option(ps: &mut ParserSource) -> Result<String, ParsingError> {
    let mut option = String::new();

    ps.push_pos();

    match get_option_char(ps, true) {
        Some(c) => {
            option.push(c);
        }
        None => {
            return Err(ParsingError::Empty);
        }
    }

    let mut is_long = false;

    while let Some(c) = get_option_char(ps, false) {
        if c == '=' {
            return Err(ParsingError::Error("'=' not allowed here".to_string()));
        }
        option.push(c);
        is_long = true;
    }

    if is_long && option.starts_with('=') {
        ps.reset_pos();
        Err(ParsingError::Error("'=' not allowed here".to_string()))?;
    }

    ps.drop_pos();
    Ok(option)
}

/**
 * Parses a name. A name is a valid name for a shell variable or function.
 */
fn parse_name(ps: &mut ParserSource) -> Result<String, ParsingError> {
    let mut name = String::new();

    match ps.next_if(|c| c.is_alphabetic()) {
        Some(c) => {
            name.push(c);
        }
        None => {
            return Err(ParsingError::Empty);
        }
    };

    while let Some(c) = ps.next_if(|c| c.is_alphanumeric() || c == '_') {
        name.push(c);
    }

    Ok(name)
}

/**
 * Parses a value. Currently a value is the same as a name.
 *
 */
fn parse_value(ps: &mut ParserSource) -> Result<String, ParsingError> {
    /*
     * TODO: This should be extended to also parse strings delimited by single or double quote.
     */
    parse_name(ps)
}

/**
 * Parse the option attribute `*` (required) or `?` (singleton).
 */
fn parse_option_attribute(ps: &mut ParserSource) -> Option<OptAttribute> {
    match ps.next_if(|c| c == '*' || c == '?') {
        Some('*') => Some(OptAttribute::Required),
        Some('?') => Some(OptAttribute::Singleton),
        _ => None,
    }
}

/**
 * Parse a Flag or a Mode-Option.
 */
fn parse_flag_mode(ps: &mut ParserSource) -> Result<(OptType, Option<OptAttribute>), ParsingError> {
    // must start with `#`
    match ps.next() {
        Some('#') => (),
        _ => Err(ParsingError::Empty)?,
    }

    let attr = parse_option_attribute(ps);

    let target_name = match parse_name(ps) {
        Ok(name) => name,
        Err(ParsingError::Empty) => Err(ParsingError::Error("name expected".to_string()))?,
        Err(ParsingError::Error(msg)) => Err(ParsingError::Error(msg))?,
    };

    ps.push_pos();
    let target = if ps.next() == Some('(') && ps.next() == Some(')') {
        ps.drop_pos();
        OptTarget::Function(target_name)
    } else {
        ps.pop_pos();
        OptTarget::Variable(target_name)
    };

    if ps.next_if(|c| c == '=').is_none() {
        Ok((OptType::Flag(target), attr))
    } else {
        match parse_value(ps) {
            Ok(value) => Ok((OptType::ModeSwitch(target, value), attr)),
            Err(ParsingError::Empty) => Ok((OptType::ModeSwitch(target, "".to_string()), attr)),
            Err(ParsingError::Error(msg)) => Err(ParsingError::Error(msg))?,
        }
    }
}

/**
 * Parse an assignment.
 */
fn parse_assignment(
    ps: &mut ParserSource,
) -> Result<(OptType, Option<OptAttribute>), ParsingError> {
    match ps.next() {
        Some('=') => (),
        _ => Err(ParsingError::Empty)?,
    }

    let attr = parse_option_attribute(ps);

    let target_name = match parse_name(ps) {
        Ok(name) => name,
        Err(ParsingError::Empty) => Err(ParsingError::Error("name expected".to_string()))?,
        Err(ParsingError::Error(msg)) => Err(ParsingError::Error(msg))?,
    };

    ps.push_pos();
    let target = if ps.next() == Some('(') && ps.next() == Some(')') {
        ps.drop_pos();
        OptTarget::Function(target_name)
    } else {
        ps.pop_pos();
        OptTarget::Variable(target_name)
    };

    Ok((OptType::Assignment(target), attr))
}

/**
 * Parse a counting option.
 */
fn parse_counter(ps: &mut ParserSource) -> Result<(OptType, Option<OptAttribute>), ParsingError> {
    match ps.next() {
        Some('+') => (),
        _ => Err(ParsingError::Empty)?,
    }

    let attr = parse_option_attribute(ps);

    let target_name = match parse_name(ps) {
        Ok(name) => name,
        Err(ParsingError::Empty) => Err(ParsingError::Error("name expected".to_string()))?,
        Err(ParsingError::Error(msg)) => Err(ParsingError::Error(msg))?,
    };

    ps.push_pos();
    let target = if ps.next() == Some('(') && ps.next() == Some(')') {
        ps.drop_pos();
        OptTarget::Function(target_name)
    } else {
        ps.pop_pos();
        OptTarget::Variable(target_name)
    };

    Ok((OptType::Counter(target), attr))
}

/**
 * Parse a single option definition.
 */
fn parse_opt_def(ps: &mut ParserSource) -> Result<OptConfig, ParsingError> {
    let mut short = String::new();
    let mut long: Vec<String> = Vec::new();

    // first options
    loop {
        match parse_option(ps) {
            Ok(o) => {
                if o.chars().count() == 1 {
                    short.push(o.chars().next().unwrap());
                } else {
                    long.push(o);
                }
            }
            Err(ParsingError::Empty) => {
                return Err(ParsingError::Error("option expected".to_string()));
            }
            Err(ParsingError::Error(msg)) => {
                return Err(ParsingError::Error(msg));
            }
        }

        if ps.next_if(|c| c == ':').is_none() {
            break;
        }
    }

    // parse option type and name
    ps.push_pos();
    let opt_type = match parse_flag_mode(ps) {
        Ok(ot) => Some(ot),
        Err(ParsingError::Error(s)) => Err(ParsingError::Error(s))?,
        Err(ParsingError::Empty) => {
            // jump back to the last pushed position
            ps.reset_pos();
            match parse_assignment(ps) {
                Ok(ot) => Some(ot),
                Err(ParsingError::Error(s)) => Err(ParsingError::Error(s))?,
                Err(ParsingError::Empty) => {
                    ps.pop_pos();
                    ps.push_pos();
                    match parse_counter(ps) {
                        Ok(ot) => Some(ot),
                        Err(ParsingError::Empty) => None,
                        Err(ParsingError::Error(s)) => Err(ParsingError::Error(s))?,
                    }
                }
            }
        }
    };

    if opt_type.is_none() {
        Err(ParsingError::Error(
            "option type char (#=+) expected".to_string(),
        ))?
    }

    let tuple = opt_type.unwrap();
    let opt_attr = tuple.1;

    Ok(OptConfig {
        opt_chars: short,
        opt_strings: long,
        opt_type: tuple.0,
        required: opt_attr == Some(OptAttribute::Required),
        singleton: opt_attr == Some(OptAttribute::Singleton),
        assigned: false,
        count_value: 0,
    })
}

/**
 * Format a parsing error to give the user a hint where something got wrong.
 * Returns a multi-line string.
 */
fn format_parsing_error(opt_def_str: &str, index: usize, msg: &String) -> String {
    let mut msg_list = Vec::new();
    msg_list.push(opt_def_str.to_owned());
    let pre = if index > 0 {
        " ".repeat(index - 1)
    } else {
        String::new()
    };
    msg_list.push(format!("{}^", pre));
    msg_list.push(format!("{}{}", pre, msg));

    msg_list.join("\n")
}

/**
 * Entry function to parse a comma-separated list of option definitions.
 *
 * Returns a (possibly empty) vector of OptConfig on success.
 */
pub fn parse(opt_def_str: &String) -> Result<Vec<OptConfig>, String> {
    if opt_def_str.is_empty() {
        Ok(Vec::new())
    } else {
        let mut ps = ParserSource::new(opt_def_str);
        match parse_opt_def_list(&mut ps) {
            Ok(vec) => Ok(vec),
            Err(ParsingError::Error(msg)) => Err(format_parsing_error(opt_def_str, ps.index, &msg)),
            _ => Err(format_parsing_error(
                opt_def_str,
                ps.index,
                &"Can't parse".to_string(),
            )),
        }
    }
}

/**
 * Parses a list of option definitions from a ParserSource.
 */
fn parse_opt_def_list(ps: &mut ParserSource) -> Result<Vec<OptConfig>, ParsingError> {
    let mut opt_def_list: Vec<OptConfig> = Vec::new();
    loop {
        match parse_opt_def(ps) {
            Ok(od) => {
                opt_def_list.push(od);
            }
            Err(pe) => Err(pe)?,
        }

        if ps.next_if(|c| c == ',').is_none() {
            break;
        }
    }

    if let Some(c) = ps.next() {
        Err(ParsingError::Error(format!("Unexpected character '{}'", c)))?
    }

    Ok(opt_def_list)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn get_od_debug() -> OptConfig {
        OptConfig {
            opt_chars: String::from("d"),
            opt_strings: vec![String::from("debug")],
            opt_type: OptType::Flag(OptTarget::Variable(String::from("debug"))),
            required: false,
            singleton: false,
            assigned: false,
            count_value: 0,
        }
    }

    fn get_od_mode() -> OptConfig {
        OptConfig {
            opt_chars: String::from("c"),
            opt_strings: vec![String::from("copy")],
            opt_type: OptType::ModeSwitch(
                OptTarget::Variable(String::from("mode")),
                String::from("copy"),
            ),
            required: false,
            singleton: false,
            assigned: false,
            count_value: 0,
        }
    }

    fn get_od_out_file() -> OptConfig {
        OptConfig {
            opt_chars: String::from("o"),
            opt_strings: vec![String::from("out-file")],
            opt_type: OptType::Assignment(OptTarget::Variable(String::from("output_file"))),
            required: false,
            singleton: false,
            assigned: false,
            count_value: 0,
        }
    }

    fn get_od_verbosity() -> OptConfig {
        OptConfig {
            opt_chars: String::from("v"),
            opt_strings: vec![String::from("verbose")],
            opt_type: OptType::Counter(OptTarget::Variable(String::from("verbosity"))),
            required: false,
            singleton: false,
            assigned: false,
            count_value: 0,
        }
    }

    #[test]
    fn test_opt_config_flag() {
        let oc = get_od_debug();
        assert!(oc.match_option(&CmdLineElement::ShortOption('d')));
        assert!(oc.match_option(&CmdLineElement::LongOption("debug".to_string())));
        assert!(oc.match_option(&CmdLineElement::LongOptionValue(
            "debug".to_string(),
            "true".to_string()
        )));
        assert!(!oc.match_option(&CmdLineElement::Separator));

        assert!(!oc.is_duplicate_allowed());
        assert!(!oc.is_target_function());
        assert!(oc.is_target_variable());
        assert_eq!("debug", oc.get_target_name());
    }

    #[test]
    fn test_parse_opt_def_flag() {
        let mut ps = ParserSource::new("d:debug#debug");

        match parse_opt_def(&mut ps) {
            Ok(od) => {
                assert_eq!(get_od_debug(), od);
            }
            Err(err) => {
                panic!("Parsing failed: {} {:?}", ps.index, err)
            }
        }
    }

    #[test]
    fn test_parse_opt_def_mode_switch() {
        let mut ps = ParserSource::new("c:copy#mode=copy");

        match parse_opt_def(&mut ps) {
            Ok(od) => {
                assert_eq!(get_od_mode(), od);
            }
            Err(err) => {
                panic!("Parsing failed: {} {:?}", ps.index, err)
            }
        }
    }

    #[test]
    fn test_parse_opt_def_assignment() {
        let mut ps = ParserSource::new("o:out-file=output_file");

        match parse_opt_def(&mut ps) {
            Ok(od) => {
                assert_eq!(get_od_out_file(), od);
            }
            Err(err) => {
                panic!("Parsing failed: {} {:?}", ps.index, err)
            }
        }
    }

    #[test]
    fn test_parse_opt_def_counter() {
        let mut ps = ParserSource::new("v:verbose+verbosity");

        match parse_opt_def(&mut ps) {
            Ok(od) => {
                assert_eq!(get_od_verbosity(), od);
            }
            Err(err) => {
                panic!("Parsing failed: {} {:?}", ps.index, err)
            }
        }
    }

    #[test]
    fn test_parse_opt_def_list() {
        let mut ps =
            ParserSource::new("c:copy#mode=copy,o:out-file=output_file,v:verbose+verbosity");

        match parse_opt_def_list(&mut ps) {
            Ok(od_list) => {
                assert_eq!(3, od_list.len())
            }
            Err(pe) => {
                panic!("Parsing error: {:?}", pe)
            }
        }
    }

    #[test]
    fn test_parse_option_short() {
        let mut ps = ParserSource::new("d#debug");
        assert_eq!(Ok("d".to_string()), parse_option(&mut ps));
    }

    #[test]
    fn test_parse_option_long() {
        let mut ps = ParserSource::new("debug#debug");
        assert_eq!(Ok("debug".to_string()), parse_option(&mut ps));
    }

    #[test]
    fn test_parse_option_short_long() {
        let mut ps = ParserSource::new("d:debug#debug");
        assert_eq!(Ok("d".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("debug".to_string()), parse_option(&mut ps));
        assert_eq!(Some('#'), ps.next());
    }

    #[test]
    fn test_parse_option_long_short() {
        let mut ps = ParserSource::new("debug:d#debug");
        assert_eq!(Ok("debug".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("d".to_string()), parse_option(&mut ps));
        assert_eq!(Some('#'), ps.next());
    }

    #[test]
    fn test_parse_unicode() {
        // Creating ParserSource manually, to enable UTF-8 support
        let array: Vec<char> = "😀:d#debug".chars().collect();
        let len = array.len();
        let mut ps = ParserSource {
            chars: array,
            index: 0,
            length: len,
            position_stack: Vec::new(),
            config: ParserConfig {
                allow_utf8_options: true,
            },
        };

        assert_eq!(Ok("😀".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("d".to_string()), parse_option(&mut ps));
        assert_eq!(Some('#'), ps.next());
    }

    #[test]
    fn test_parse_option() {
        let mut ps = ParserSource::new("test-case:debug:d:test\\%case:\\##debug");

        assert_eq!(Ok("test-case".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("debug".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("d".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("test%case".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        // assert_eq!(Ok("💖".to_string()), parse_option(&mut ps));
        // assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("#".to_string()), parse_option(&mut ps));
        assert_eq!(Some('#'), ps.next());
    }

    #[test]
    fn test_parser_source() {
        let mut ps = ParserSource::new("ABCD");

        assert_eq!(Some('A'), ps.next());
        assert_eq!(Some('B'), ps.next());
        // Push current position
        ps.push_pos();
        assert_eq!(Some('C'), ps.next());
        assert_eq!(Some('D'), ps.next());
        assert_eq!(None, ps.next());
        // Pop saved position
        ps.pop_pos();
        assert_eq!(Some('C'), ps.next());
        assert_eq!(Some('D'), ps.next());
        assert_eq!(None, ps.next());

        ps.reset();
        ps.push_pos();
        assert_eq!(Some('A'), ps.next());
        ps.push_pos();
        assert_eq!(Some('B'), ps.next());
        ps.push_pos();
        assert_eq!(Some('C'), ps.next());
        ps.push_pos();
        assert_eq!(Some('D'), ps.next());

        ps.pop_pos();
        assert_eq!(Some('D'), ps.next());
        ps.pop_pos();
        assert_eq!(Some('C'), ps.next());
        ps.pop_pos();
        assert_eq!(Some('B'), ps.next());
        ps.pop_pos();
        assert_eq!(Some('A'), ps.next());
    }

    #[test]
    #[should_panic]
    fn test_parser_source_pos_stack_empty_pop() {
        let mut ps = ParserSource::new("ABCD");

        ps.pop_pos();
    }

    #[test]
    #[should_panic]
    fn test_parser_source_pos_stack_empty_drop() {
        let mut ps = ParserSource::new("ABCD");

        ps.drop_pos();
    }
}
