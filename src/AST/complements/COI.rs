use std::fmt;
use crate::logger::log;
use crate::AST::{
    expr::Expr,
    subject::nominal_group::NominalGroup,
    preposition::Preposition,
    utils::skipable::Skipable,
};
use crate::context::Context;

pub struct COI<'a> {
    context: &'a Context,
    preposition: Preposition<'a>,
    subject: NominalGroup<'a>,
}

impl <'a> COI<'a> {
    pub fn preposition(&self) -> &Preposition<'a> {
        &self.preposition
    }

    pub fn subject(&self) -> &NominalGroup<'a> {
        &self.subject
    }
}

impl<'a> Expr<'a> for COI<'a> {
    fn new(context: &'a Context) -> Self {
        COI {
            context: &context,
            preposition: Preposition::new(&context),
            subject: NominalGroup::new(&context),
        }
    }
    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("COI::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, success) = self.preposition.parse(input.clone());

        if !success {
            log::new().syntax(&format!("COI::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false)
        }

        let (input, success) = self.subject.parse(input.clone());

        if !success {
            log::new().syntax(&format!("COI::parse({}) 2 -> ({}, {})", old_input, old_input, false));
            return (old_input, false)
        }
        log::new().syntax(&format!("COI::parse({}) 3 -> ({}, {})", old_input, input, true));
        (input, true)
    }
}

impl<'a> fmt::Display for COI<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "COI {{{}, {}}}", self.preposition, self.subject)
    }
}