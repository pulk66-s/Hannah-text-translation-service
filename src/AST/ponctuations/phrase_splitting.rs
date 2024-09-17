use std::fmt;
use crate::logger::log;
use crate::AST::expr::Expr;
use crate::context::Context;

pub struct PhraseSplitting {
    pub c: char
}

impl<'a> Expr<'a> for PhraseSplitting {
    fn new(_context: &Context) -> Self {
        Self {
            c: ' '
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("PhraseSplitting::parse({})", input));
        if input.len() == 0 {
            log::new().syntax(&format!("PhraseSplitting::parse({}) 1 -> ({}, {})", input, input, false));
            return (input, false);
        }

        let c = input.chars().next().unwrap();

        match c {
            ',' | ';' => {
                self.c = c;
                log::new().syntax(&format!("PhraseSplitting::parse({}) 2 -> ({}, {})", input, (&input[1..]), true));
                (input[1..].to_string(), true)
            },
            _ => {
                log::new().syntax(&format!("PhraseSplitting::parse({}) 3 -> ({}, {})", input, input, false));
                (input, false)
            }
        }
    }
}

impl<'a> fmt::Display for PhraseSplitting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"PhraseSplitting\": {{\"c\": \"{}\"}}", self.c)
    }
}