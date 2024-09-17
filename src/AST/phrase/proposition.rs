use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::{
            skipable::Skipable,
            maybe::Maybe
        },
        subject::nominal_group::NominalGroup,
        verb::group_verbal::GroupVerbal,
        preposition::group_prepositionnal::GroupPrepositionnal
    },
};

pub struct Proposition<'a> {
    context: &'a Context,
    gn: Maybe<'a, NominalGroup<'a>>,
    verb: Maybe<'a, GroupVerbal<'a>>,
    complement: Maybe<'a, GroupPrepositionnal<'a>>,
}

impl <'a> Proposition<'a> {
    pub fn gn(&self) -> &Maybe<'a, NominalGroup<'a>> {
        &self.gn
    }

    pub fn verb(&self) -> &Maybe<'a, GroupVerbal<'a>> {
        &self.verb
    }

    pub fn complement(&self) -> &Maybe<'a, GroupPrepositionnal<'a>> {
        &self.complement
    }
}

impl<'a> Expr<'a> for Proposition<'a> {
    fn new(context: &'a Context) -> Self {
        Proposition {
            context: &context,
            gn: Maybe::new(&context),
            verb: Maybe::new(&context),
            complement: Maybe::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Proposition::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, result) = self.gn.parse(input.clone());

        if !result {
            log::new().syntax(&format!("Proposition::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }
        
        let (input, result) = self.verb.parse(input.clone());

        if !result {
            log::new().syntax(&format!("Proposition::parse({}) 2 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }

        let (input, result) = self.complement.parse(input.clone());

        if !result {
            log::new().syntax(&format!("Proposition::parse({}) 3 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }

        log::new().syntax(&format!("Proposition::parse({}) 4 -> ({}, {})", old_input, input, true));
        (input, true)
    }
}

impl<'a> fmt::Display for Proposition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"Proposition\": {{{}, {}, {}}}", self.gn, self.verb, self.complement)
    }
}
