use std::fmt;
use crate::logger::log;
use crate::context::Context;
use crate::logger::log;
use crate::AST::expr::Expr;

pub struct CCC<'a> {
    context: &Context
}

impl<'a> Expr<'a> for CCC<'a> {
    fn new(context: &'a Context) -> Self {
        CCC {
            context: &context
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("CCC::parse({})", input));
        (input, false)
    }
}

impl<'a> fmt::Display for CCC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CCC")
    }
}