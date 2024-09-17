pub mod COD;
pub mod COI;

use std::fmt;

use crate::logger::log;
use crate::{
    AST::{
        expr::Expr,
        utils::or::Or,
        complements::{
            COD::COD as CCOD,
            COI::COI as CCOI
        }
    },
    context::Context
};

pub struct Complement<'a> {
    pub complement: Or<'a, CCOD<'a>, CCOI<'a>>
}

impl<'a> Expr<'a> for Complement<'a> {
    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Complement::parse({})", input));
        let old_input = input.clone();
        let (input, res) = self.complement.parse(input.clone());

        if !res {
            log::new().syntax(&format!("Complement::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }
        log::new().syntax(&format!("Complement::parse({}) 2 -> ({}, {})", old_input, input, false));
        (input, true)
    }

    fn new(context: &'a Context) -> Self {
        Complement {
            complement: Or::new(&context)
        }
    }
}

impl<'a> fmt::Display for Complement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"Complement\": {{{}}},", self.complement)
    }
}

