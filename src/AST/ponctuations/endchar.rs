use std::fmt;
use crate::logger::log;
use crate::{
    AST::expr::Expr,
    context::Context,
};

pub struct EndChar<'a> {
    context: &'a Context,
    c: char
}

impl<'a> Expr<'a> for EndChar<'a> {
    fn new(context: &'a Context) -> Self {
        Self {
            c: ' ',
            context: &context,
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("EndChar::parse({})", input));
        if input.len() == 0 {
            log::new().syntax(&format!("EndChar::parse({}) 1 -> ({}, {})", input, input, false));
            return (input, false);
        }

        let c = input.chars().next().unwrap();

        match c {
            '.' | '?' | '!' => {
                self.c = c;
                log::new().syntax(&format!("EndChar::parse({}) 2 -> ({}, {})", input, (&input[1..]), true));
                (input[1..].to_string(), true)
            },
            _ => {
                log::new().syntax(&format!("EndChar::parse({}) 3 -> ({}, {})", input, input, false));
                (input, false)
            }
        }
    }
}

impl<'a> fmt::Display for EndChar<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"EndChar\": {{\"c\": \"{}\"}}", self.c)
    }
}
