use crate::arg_parser::CmdLineElement;

#[derive(Debug, PartialEq)]
pub enum OptType {
    Flag,
    FlagAssignment(String),
    Assignment,
    Counter,
}

#[derive(Debug)]
pub enum Target {
    Variable(String),
    Function(String),
}

#[derive(Debug)]
pub struct OptConfig {
    // Every character in this string is a short option
    pub opt_chars: String,
    // Every string is a long option
    pub opt_strings: Vec<String>,
    // type of option
    pub opt_type: OptType,
    // Variable or function
    pub target: Target,
    // whether this option is required
    pub required: bool,
    // Whether this is a singleton option. If a singleton option is found, only its action is
    // executed and all other options and arguments are dropped (including other singletons).
    // Typically used for '--help' etc.
    pub singleton: bool,
    // Runtime: Whether this variable has been inited
    pub inited: bool,
    // Runtime: Whether this variable has been set
    pub assigned: bool,
    // Runtime: Count of a counting variable
    pub count_value: u16,
}

impl OptConfig {
    pub fn inc_count(&mut self) {
        self.count_value += 1;
    }
    pub fn set_count(&mut self, value: u16) {
        self.count_value = value;
    }

    pub fn match_option(&self, el: &CmdLineElement) -> bool {
        match el {
            CmdLineElement::ShortOption(c) => self.opt_chars.find(*c).is_some(),
            CmdLineElement::LongOption(s) => self.opt_strings.contains(&s),
            CmdLineElement::LongOptionValue(s, _) => self.opt_strings.contains(&s),
            _ => false
        }
    }

    /**
    Check whether duplicate usage of this option is allowed.
    This allowed for Counter options and options with a target type Function.
     */
    pub fn is_duplicate_allowed(&self) -> bool {
        self.opt_type == OptType::Counter || match self.target {
            Target::Function(_) => true,
            _ => false,
        }
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


pub mod opt_def_parser {
    use std::collections::HashMap;
    use chumsky::prelude::*;
    use chumsky::error::Cheap;
    use std::ops::Add;

    #[derive(PartialEq, Debug)]
    enum Token {
        ShortOpt(String),
        LongOpt(String),
        Flag(),
        Assignment(),
        Counter(),
        Required(),
        Singleton(),
        TargetVariable(String),
        TargetFunction(String),
        FlagAssignValue(String),
        NoOp(),
    }

    enum TargetType {
        Variable(),
        Function(),
    }

    pub fn parse(opt_def_string: &str) -> Result<Vec<super::OptConfig>, String> {
        let a_option = filter::<_, _, Cheap<char>>(|c: &char| c.is_alphanumeric() || *c == '_')
            .chain(
                filter::<_, _, Cheap<char>>(|c: &char| c.is_alphanumeric() || *c == '_' || *c == '-').repeated()
            )
            .collect::<String>()
            .map(|s| if s.len() == 1 { Token::ShortOpt(s) } else { Token::LongOpt(s) });

        let option_list = a_option
            .repeated().at_least(1).separated_by(just(':')).flatten();

        let bool_type = just('#').map(|_t| Token::Flag());
        let assign_type = just('=').map(|_t| Token::Assignment());
        let count_type = just('+').map(|_t| Token::Counter());

        let option_type = bool_type.or(assign_type).or(count_type);

        let required = just('*').map(|_t| Token::Required());
        let singleton = just('?').map(|_t| Token::Singleton());

        let option_attr = required.or(singleton).or_else(|_e| Result::Ok(Token::NoOp()));

        let target = filter::<_, _, Cheap<char>>(|c: &char| c.is_alphabetic() || *c == '_')
            .chain(
                filter::<_, _, Cheap<char>>(|c: &char| c.is_alphanumeric() || *c == '_').repeated()
            )
            .collect::<String>()
            .then(just("()").map(|_t| TargetType::Function()).or_else(|_e| Result::Ok(TargetType::Variable())))
            .map(|(name, typ)| match typ {
                TargetType::Function() => Token::TargetFunction(name),
                TargetType::Variable() => Token::TargetVariable(name),
            });

        let opt_assign = just("=")
            .ignore_then(filter::<_, _, Cheap<char>>(|c: &char| c.is_alphabetic() || *c == '_')
                .chain(
                    filter::<_, _, Cheap<char>>(|c: &char| c.is_alphanumeric() || *c == '_').repeated()
                )
                .collect::<String>()
                .map(Token::FlagAssignValue))
            .or_else(|_e| Result::Ok(Token::NoOp()));

        let parser = option_list
            .chain(option_type)
            .chain(option_attr)
            .chain(target)
            .chain(opt_assign)
            .map_with_span(|tok, span| (tok, span))
            .repeated().separated_by(just(','))
            .flatten()
            .then_ignore(end())
            ;

        let result = parser.parse(opt_def_string);

        match result {
            Ok(ast) => {
                let mut opt_cfg_list: Vec<super::OptConfig> = vec![];
                let mut name_map:HashMap<String, bool> = HashMap::new();
                for optdef_tokens in ast {
                    let mut shorts = String::new();
                    let mut long: Vec<String> = Vec::new();
                    let mut opt_type: Option<super::OptType> = None;
                    let mut target: Option<super::Target> = None;
                    let mut required = false;
                    let mut singleton = false;
                    for token in optdef_tokens.0 {
                        match token {
                            Token::ShortOpt(c) => { shorts = shorts.add(c.as_str()); }
                            Token::LongOpt(c) => { long.push(c); }
                            Token::Flag() => { opt_type = Some(super::OptType::Flag); }
                            Token::Assignment() => { opt_type = Some(super::OptType::Assignment); }
                            Token::Counter() => { opt_type = Some(super::OptType::Counter); }
                            Token::Required() => { required = true; }
                            Token::Singleton() => { singleton = true; }
                            Token::TargetVariable(name) => { target = Some(super::Target::Variable(name)); }
                            Token::TargetFunction(name) => { target = Some(super::Target::Function(name)); }
                            Token::FlagAssignValue(value) => { opt_type = Some(super::OptType::FlagAssignment(value)); }
                            Token::NoOp() => (),
                        }
                    }
                    let cfg = super::OptConfig {
                        opt_chars: shorts.clone(),
                        opt_strings: long.clone(),
                        opt_type: opt_type.unwrap(),
                        target: target.unwrap(),
                        required,
                        singleton,
                        assigned: false,
                        count_value: 0,
                        inited: false,
                    };

                    let flag_assign = match &cfg.opt_type {
                        super::OptType::FlagAssignment(_) => true,
                        _ => false,
                    };

                    let name = match &cfg.target {
                        super::Target::Function(name) => name,
                        super::Target::Variable(name) => name,
                    };

                    if flag_assign {
                        if name_map.contains_key(name) {
                            if true != *name_map.get(name).unwrap() {
                                return Err(format!("Duplicate use of variable/function '{}'", name));
                            }
                        } else {
                            name_map.insert(name.clone(), flag_assign);
                        }
                    } else {
                        if name_map.contains_key(name) {
                            return Err(format!("Duplicate use of variable/function '{}'", name));
                        } else {
                            name_map.insert(name.clone(), flag_assign);
                        }
                    }

                    opt_cfg_list.push(cfg);
                }
                return Ok(opt_cfg_list);
            }
            Err(errs) => {
                errs
                    .into_iter()
                    .for_each(|e| println!("Error parsing option definition: {:?}", e));
                return Err("FAILED".to_string());
            }
        }
    }
}

