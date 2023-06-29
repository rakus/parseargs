use crate::arg_parser::CmdLineElement;

#[derive(Debug, PartialEq)]
pub enum OptTarget {
    Variable(String),
    Function(String),
}

#[derive(Debug, PartialEq)]
enum OptAttribute {
    Required,
    Singleton,
}

#[derive(Debug, PartialEq)]
pub enum OptType {
    Flag(OptTarget),
    ModeSwitch(OptTarget, String),
    Assignment(OptTarget),
    Counter(OptTarget),
}

#[derive(Debug, PartialEq)]
pub struct OptConfig {
    // Every character in this string is a short option
    pub opt_chars: String,
    // Every string is a long option
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
    pub fn match_option(&self, el: &CmdLineElement) -> bool {
        match el {
            CmdLineElement::ShortOption(c) => self.opt_chars.find(*c).is_some(),
            CmdLineElement::LongOption(s) => self.opt_strings.contains(&s),
            CmdLineElement::LongOptionValue(s, _) => self.opt_strings.contains(&s),
            _ => false,
        }
    }

    /**
    Check whether duplicate usage of this option is allowed.
    This allowed for Counter options and options with a target type Function.
     */
    pub fn is_duplicate_allowed(&self) -> bool {
        matches!(self.opt_type, OptType::Counter(_))
            || match self.opt_type {
                OptType::Flag(OptTarget::Function(_)) => true,
                OptType::ModeSwitch(OptTarget::Function(_), _) => true,
                OptType::Assignment(OptTarget::Function(_)) => true,
                OptType::Counter(OptTarget::Function(_)) => true,
                _ => false,
            }
    }

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

    pub fn get_target(&self) -> &OptTarget {
        match &self.opt_type {
            OptType::Flag(ot)
            | OptType::Assignment(ot)
            | OptType::Counter(ot)
            | OptType::ModeSwitch(ot, _) => ot,
        }
    }

    pub fn is_target_function(&self) -> bool {
        match &self.opt_type {
            OptType::Flag(OptTarget::Function(_))
            | OptType::Assignment(OptTarget::Function(_))
            | OptType::Counter(OptTarget::Function(_))
            | OptType::ModeSwitch(OptTarget::Function(_), _) => true,
            _ => false,
        }
    }

    pub fn is_target_variable(&self) -> bool {
        !self.is_target_function()
    }

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

pub struct ParserSource {
    chars: Vec<char>,
    index: usize,
    length: usize,

    position_stack: Vec<usize>,
}

impl ParserSource {
    pub fn new(string: &String) -> ParserSource {
        let array: Vec<char> = string.chars().collect();
        let len = array.len();
        ParserSource {
            chars: array,
            index: 0,
            length: len,
            position_stack: Vec::new(),
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
    pub fn back(&mut self) -> () {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    /// Push the current position on the position stack
    pub fn push_pos(&mut self) -> () {
        self.position_stack.push(self.index)
    }

    /// Pop the top of the position stack
    pub fn pop_pos(&mut self) -> () {
        if let Some(idx) = self.position_stack.pop() {
            self.index = idx;
        } else {
            panic!("ParserSource.pop_pos: position_stack is empty")
        }
    }

    /// Jump to the top of the position stack without popping it
    pub fn reset_pos(&mut self) -> () {
        if let Some(idx) = self.position_stack.last() {
            self.index = *idx;
        } else {
            panic!("ParserSource.reset_pos: position_stack is empty")
        }
    }

    /// Drop the top of the position stack
    pub fn drop_pos(&mut self) -> () {
        if let None = self.position_stack.pop() {
            panic!("ParserSource.drop_pos: position_stack is empty")
        }
    }

    pub fn reset(&mut self) -> () {
        self.index = 0;
        self.position_stack.clear();
    }
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    // Nothing parsable found at all
    Empty,
    // Error with Message
    Error(String),
}

fn get_option_char(ps: &mut ParserSource, first: bool) -> Option<char> {
    let forbidden = vec![':', '#', '=', '+', '%'];

    match ps.next() {
        Some(c) => {
            if c == '\\' {
                match ps.next() {
                    Some(c2) => {
                        if c2.is_whitespace() || (first && c2 == '-') {
                            ps.back();
                            None
                        } else {
                            Some(c2)
                        }
                    }
                    None => Some(c),
                }
            } else if c.is_whitespace() || forbidden.contains(&c) || (first && c == '-') {
                ps.back();
                None
            } else {
                Some(c)
            }
        }
        None => None,
    }
}

fn parse_option(ps: &mut ParserSource) -> Result<String, ParsingError> {
    let mut option = String::new();

    match get_option_char(ps, true) {
        Some(c) => {
            option.push(c);
        }
        None => {
            return Err(ParsingError::Empty);
        }
    }

    while let Some(c) = get_option_char(ps, false) {
        option.push(c);
    }

    Ok(option)
}

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

fn parse_value(ps: &mut ParserSource) -> Result<String, ParsingError> {
    /*
     * This should be extended to also parse strings delimited by single or double quote.
     */
    parse_name(ps)
}

fn parse_option_attribute(ps: &mut ParserSource) -> Option<OptAttribute> {
    match ps.next_if(|c| c == '*' || c == '?') {
        Some('*') => Some(OptAttribute::Required),
        Some('?') => Some(OptAttribute::Singleton),
        _ => None,
    }
}

fn parse_flag_mode(ps: &mut ParserSource) -> Result<(OptType, Option<OptAttribute>), ParsingError> {
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

    if ps.next_if(|c| c == '=') == None {
        Ok((OptType::Flag(target), attr))
    } else {
        match parse_value(ps) {
            Ok(value) => Ok((OptType::ModeSwitch(target, value), attr)),
            Err(ParsingError::Empty) => Ok((OptType::ModeSwitch(target, "".to_string()), attr)),
            Err(ParsingError::Error(msg)) => Err(ParsingError::Error(msg))?,
        }
    }
}

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

fn parse_opt_def(ps: &mut ParserSource) -> Result<OptConfig, ParsingError> {
    let mut short = String::new();
    let mut long: Vec<String> = Vec::new();

    // first options
    loop {
        match parse_option(ps) {
            Ok(o) => {
                if o.len() == 1 {
                    short.push(o.chars().next().unwrap());
                } else {
                    long.push(o);
                }
            }
            Err(_) => {
                return Err(ParsingError::Error("option expected".to_string()));
            }
        }

        if ps.next_if(|c| c == ':') == None {
            break;
        }
    }

    // parse option type and name
    ps.push_pos();
    let opt_type = match parse_flag_mode(ps) {
        Ok(ot) => Some(ot),
        Err(ParsingError::Error(s)) => Err(ParsingError::Error(s))?,
        Err(ParsingError::Empty) => {
            ps.pop_pos();
            ps.push_pos();
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

fn format_parsing_error(opt_def_str: &String, index: usize, msg: &String) -> String {
    let mut msg_list = Vec::new();
    msg_list.push(opt_def_str.clone());
    let pre = if index > 0 {
        " ".repeat(index - 1)
    } else {
        String::new()
    };
    msg_list.push(format!("{}^", pre));
    msg_list.push(format!("{}{}", pre, msg));

    msg_list.join("\n")
}

pub fn parse(opt_def_str: &String) -> Result<Vec<OptConfig>, String> {
    if opt_def_str.len() == 0 {
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

pub fn parse_opt_def_list(ps: &mut ParserSource) -> Result<Vec<OptConfig>, ParsingError> {
    let mut opt_def_list: Vec<OptConfig> = Vec::new();
    loop {
        match parse_opt_def(ps) {
            Ok(od) => {
                opt_def_list.push(od);
            }
            Err(pe) => Err(pe)?,
        }

        if ps.next_if(|c| c == ',') == None {
            break;
        }
    }
    match ps.next() {
        Some(c) => Err(ParsingError::Error(format!("Unexpected character '{}'", c)))?,
        None => (),
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
    fn test_parse_opt_def_flag() {
        let mut ps = ParserSource::new(&"d:debug#debug".to_string());

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
        let mut ps = ParserSource::new(&"c:copy#mode=copy".to_string());

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
        let mut ps = ParserSource::new(&"o:out-file=output_file".to_string());

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
        let mut ps = ParserSource::new(&"v:verbose+verbosity".to_string());

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
        let mut ps = ParserSource::new(
            &"c:copy#mode=copy,o:out-file=output_file,v:verbose+verbosity".to_string(),
        );

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
        let mut ps = ParserSource::new(&"d#debug".to_string());
        assert_eq!(Ok("d".to_string()), parse_option(&mut ps));
    }

    #[test]
    fn test_parse_option_long() {
        let mut ps = ParserSource::new(&"debug#debug".to_string());
        assert_eq!(Ok("debug".to_string()), parse_option(&mut ps));
    }

    #[test]
    fn test_parse_option_short_long() {
        let mut ps = ParserSource::new(&"d:debug#debug".to_string());
        assert_eq!(Ok("d".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("debug".to_string()), parse_option(&mut ps));
        assert_eq!(Some('#'), ps.next());
    }

    #[test]
    fn test_parse_option_long_short() {
        let mut ps = ParserSource::new(&"debug:d#debug".to_string());
        assert_eq!(Ok("debug".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("d".to_string()), parse_option(&mut ps));
        assert_eq!(Some('#'), ps.next());
    }

    #[test]
    fn test_parse_option() {
        let mut ps = ParserSource::new(&"test-case:debug:d:test\\%case:💖:\\##debug".to_string());

        assert_eq!(Ok("test-case".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("debug".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("d".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("test%case".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("💖".to_string()), parse_option(&mut ps));
        assert_eq!(Some(':'), ps.next());
        assert_eq!(Ok("#".to_string()), parse_option(&mut ps));
        assert_eq!(Some('#'), ps.next());
    }

    #[test]
    fn test_parser_source() {
        let mut ps = ParserSource::new(&"ABCD".to_string());

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
        let mut ps = ParserSource::new(&"ABCD".to_string());

        ps.pop_pos();
    }

    #[test]
    #[should_panic]
    fn test_parser_source_pos_stack_empty_drop() {
        let mut ps = ParserSource::new(&"ABCD".to_string());

        ps.drop_pos();
    }
}
