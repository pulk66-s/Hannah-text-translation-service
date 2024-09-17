use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::{
            skipable::Skipable,
            and::And,
        },
        phrase::proposition::Proposition,
        ponctuations::phrase_splitting::PhraseSplitting,
    }
};

pub struct Juxtaposition<'a> {
    context: &'a Context,
    proposition: And<'a, Proposition<'a>, PhraseSplitting>
}

impl <'a> Juxtaposition<'a> {
    pub fn proposition(&self) -> &And<'a, Proposition<'a>, PhraseSplitting> {
        &self.proposition
    }
}

impl<'a> Expr<'a> for Juxtaposition<'a> {
    fn new(context: &'a Context) -> Self {
        Juxtaposition {
            context: &context,
            proposition: And::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Juxtaposition::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.proposition.parse(input.clone());

        if !res {
            log::new().syntax(&format!("Juxtaposition::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }
        log::new().syntax(&format!("Juxtaposition::parse({}) 2 -> ({}, {})", old_input, input, true));
        return (input, true);
    }
}

impl<'a> fmt::Display for Juxtaposition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"Juxtaposition\": {{{}}}", self.proposition)
    }
}
