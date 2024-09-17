use std::fmt;
use crate::logger::log;
use crate::AST::expr::Expr;
use crate::context::Context;

pub struct Space<'a> {
    context: &'a Context,
    c: Option<char>,
}

impl<'a> Expr<'a> for Space<'a> {
    fn new(context: &'a Context) -> Self {
        Space {
            context: &context,
            c: None,
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Space::parse({})", input));
        if input.len() == 0 {
            return (input, false);
        }

        let c = input.chars().next().unwrap();

        if c.is_whitespace() {
            self.c = Some(c);
            return (input[1..].to_string(), true);
        }
        log::new().syntax(&format!("Space::parse({}) -> ({}, {})", input, input, false));
        (input, false)
    }
}

impl<'a> fmt::Display for Space<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Space")
    }
}
