pub mod simple;
pub mod complex;
pub mod proposition;

use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::{
            or::Or,
            skipable::Skipable
        },
        phrase::{
            simple::SimplePhrase,
            complex::ComplexPhrase
        }
    }
};

pub struct Phrase<'a> {
    phr: Or<'a, SimplePhrase<'a>, ComplexPhrase<'a>>,
    context: &'a Context
}

impl <'a> Phrase<'a> {
    pub fn phr(&self) -> &Or<'a, SimplePhrase<'a>, ComplexPhrase<'a>> {
        &self.phr
    }
}

impl<'a> Expr<'a> for Phrase<'a> {
    fn new(context: &'a Context) -> Self {
        Phrase {
            phr: Or::new(&context),
            context: &context
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Phrase::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.phr.parse(input.clone());

        if !res {
            log::new().syntax(&format!("Phrase::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }
        log::new().syntax(&format!("Phrase::parse({}) 2 -> ({}, {})", old_input, input, true));
        return (input, true);
    }
}

impl<'a> fmt::Display for Phrase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"Phrase\": {{{}}}", self.phr)
    }
}