use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr, pronouns::Pronoun, subject::nominal_group::NominalGroup, utils::or::Or,
        utils::skipable::Skipable, verb::Verb,
    },
};
use std::fmt;

pub struct COD<'a> {
    context: &'a Context,
    data: Or<'a, NominalGroup<'a>, Or<'a, Verb<'a>, Pronoun<'a>>>,
}

impl <'a> COD<'a> {
    pub fn data(&self) -> &Or<'a, NominalGroup<'a>, Or<'a, Verb<'a>, Pronoun<'a>>> {
        &self.data
    }
}

impl<'a> Expr<'a> for COD<'a> {
    fn new(context: &'a Context) -> Self {
        COD {
            context: &context,
            data: Or::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("COD::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.data.parse(input.clone());

        if !res {
            log::new().syntax(&format!("COD::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }
        log::new().syntax(&format!("COD::parse({}) 2 -> ({}, {})", old_input, input, true));
        (input, true)
    }
}

impl<'a> fmt::Display for COD<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"COD\": {{{}}}", self.data)
    }
}
