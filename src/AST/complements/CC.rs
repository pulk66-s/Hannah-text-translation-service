use std::fmt;
use crate::logger::log;
use crate::AST::expr::Expr;
use crate::logger::log;
use crate::context::Context;

pub struct CC<'a> {
    context: &Context
}

impl<'a> Expr<'a> for CC<'a> {
    fn new(context: &'a Context) -> Self {
        CC {
            context: &context
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("CC::parse({})", input));
        (input, false)
    }
}

impl<'a> fmt::Display for CC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CC")
    }
}