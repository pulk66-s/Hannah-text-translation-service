use std::fmt;
use crate::logger::log;
use crate::AST::{
    expr::Expr,
    complements::{
        COD::COD,
        COI::COI
    },
    utils::{
        or::Or,
        and::And
    }
};
use crate::logger::log;
use crate::context::Context;

pub struct COS<'a> {
    context: &'a Context,
    complements: Or<And<COD, COI>, And<COI, COD>>
}

impl<'a> Expr<'a> for COS<'a> {
    fn new(context: &'a Context) -> Self {
        COS {
            context: &context,
            complements: Or::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("COS::parse({})", input));
        let old_input = input.clone();
        let (new_input, result) = self.complements.parse(input.clone());
        
        if !result {
            return (old_input, false);
        }
        (new_input, true)
    }
}

impl<'a> fmt::Display for COS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "COS {{{}}}", self.complements)
    }
}