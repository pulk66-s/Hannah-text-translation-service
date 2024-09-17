use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::{
            skipable::Skipable,
            and::And
        },
        phrase::proposition::Proposition,
        coordinating_conjunctions::CoordinatingConjunction
    }
};

pub struct Coordination<'a> {
    context: &'a Context,
    proposition: And<'a, Proposition<'a>, CoordinatingConjunction<'a>>
}

impl <'a> Coordination<'a> {
    pub fn proposition(&self) -> &And<'a, Proposition<'a>, CoordinatingConjunction<'a>> {
        &self.proposition
    }
}

impl<'a> Expr<'a> for Coordination<'a> {
    fn new(context: &'a Context) -> Self {
        Coordination {
            context: &context,
            proposition: And::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Coordination::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.proposition.parse(input.clone());

        if !res {
            log::new().syntax(&format!("Coordination::parse({}) 1 -> ({}, {})", old_input, input, false));
            return (input, false);
        }
        log::new().syntax(&format!("Coordination::parse({}) 2 -> ({}, {})", old_input, input, true));
        return (input, true);
    }
}

impl<'a> fmt::Display for Coordination<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"Coordination\": {{{}}}", self.proposition)
    }
}