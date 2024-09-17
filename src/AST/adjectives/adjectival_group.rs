use std::fmt;
use crate::logger::log;
use crate::{
    AST::{
        expr::Expr,
        adjectives::Adjective,
        utils::{
            skipable::Skipable,
            many::Many,
            and::And,
            maybe::Maybe,
        },
        coordinating_conjunctions::CoordinatingConjunction
    },
    context::Context,
};

impl <'a> AdjectivalGroup<'a> {
    pub fn adj(&self) -> &Many<'a, And<'a, Maybe<'a, CoordinatingConjunction<'a>>, Adjective<'a>>> {
        &self.adj
    }
}

pub struct AdjectivalGroup<'a> {
    adj: Many<'a, And<'a, Maybe<'a, CoordinatingConjunction<'a>>, Adjective<'a>>>,
    context: &'a Context,
}

impl<'a> Expr<'a> for AdjectivalGroup<'a> {
    fn new(context: &'a Context) -> Self {
        AdjectivalGroup {
            context: &context,
            adj: Many::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("AdjectivalGroup::parse({})", input));
        let old_input = input.clone();
        let mut skips = Skipable::new(self.context);
        let (input, res) = skips.parse(input.clone());
        let (input, res) = self.adj.parse(input.clone());

        if !res {
            log::new().syntax(&format!("AdjectivalGroup::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input.to_string(), false);
        }
        log::new().syntax(&format!("AdjectivalGroup::parse({}) 2 -> ({}, {})", old_input, input, res));
        (input, res)
    }
}

impl<'a> fmt::Display for AdjectivalGroup<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"AdjectivalGroup\": {{{}}}", self.adj)
    }
}
