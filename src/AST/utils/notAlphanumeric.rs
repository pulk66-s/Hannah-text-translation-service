use std::fmt;
use crate::logger::log;
use crate::AST::expr::Expr;
use crate::context::Context;

pub struct NotAlphanumeric {
    pub c: Option<char>,
}

impl<'a> Expr<'a> for NotAlphanumeric {
    fn new(context: &'a Context) -> Self {
        NotAlphanumeric {
            c: None,
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        let old_input = input.clone();
        log::new().syntax(&format!("NotAlphanumeric::parse({})", input));
        if input.len() == 0 {
            return (input, false);
        }

        let c: char = input.chars().next().unwrap();

        if !c.is_alphanumeric() {
            self.c = Some(c);

            let mut chars = input.chars();

            chars.next();
            log::new().syntax(&format!("NotAlphanumeric::parse({}) 1 -> ({}, {})", old_input, chars.as_str(), true));
            return (chars.as_str().to_string(), true);
        }
        log::new().syntax(&format!("NotAlphanumeric::parse({}) 2 -> ({}, {})", old_input, input, false));
        (input, false)
    }
}

impl fmt::Display for NotAlphanumeric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alphanumeric {:?}", self.c)
    }
}