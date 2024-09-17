use crate::logger::log;
use crate::{
    context::Context,
    AST::expr::Expr
};

pub struct SpecialChar<'a> {
    context: &'a Context,
    c: Option<char>,
}

impl<'a> SpecialChar<'a> {
    pub fn c(&self) -> Option<char> {
        self.c
    }
}

impl<'a> Expr<'a> for SpecialChar<'a> {
    fn new(context: &'a Context) -> Self {
        SpecialChar {
            context: &context,
            c: None,
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("SpecialChar::parse({})", input));
        let mut chars = input.chars();
        let c = chars.next().unwrap();
        let mut result = false;

        if c == '\'' || c == '-' {
            result = true;
        }
        self.c = Some(c);

        log::new().syntax(&format!("SpecialChar::parse({}) -> {} {}", input, chars.as_str(), result));
        (chars.as_str().to_string(), result)
    }
}