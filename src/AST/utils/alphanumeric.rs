use std::fmt;
use crate::logger::log;
use crate::AST::expr::Expr;
use crate::context::Context;

pub struct Alphanumeric {
    pub c: Option<char>,
}

impl<'a> Expr<'a> for Alphanumeric {
    fn new(context: &'a Context) -> Self {
        Alphanumeric {
            c: None,
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Alphanumeric::parse({})", input));
        if input.len() == 0 {
            return (input, false);
        }

        let c: char = input.chars().next().unwrap();

        if c.is_alphanumeric() {
            self.c = Some(c);

            let mut chars = input.chars();

            chars.next();
            log::new().syntax(&format!("Alphanumeric::parse({}) -> {} {}", input, chars.as_str(), true));
            return (chars.as_str().to_string(), true);
        }
        log::new().syntax(&format!("Alphanumeric::parse({}) -> {} {}", input, input, false));
        (input, false)
    }
}

impl<'a> fmt::Display for Alphanumeric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alphanumeric {:?}", self.c)
    }
}
