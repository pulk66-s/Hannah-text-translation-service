use crate::logger::log;
use crate::{
    AST::{
        expr::Expr,
        adverbe::Adv,
        utils::{
            skipable::Skipable,
            or::Or
        },
        complements::Complement
    },
    context::Context,
};
use std::fmt;

pub struct GroupPrepositionnal<'a> {
    context: &'a Context,
    preposition: Or<'a, Adv<'a>, Complement<'a>>,
}

impl <'a> GroupPrepositionnal<'a> {
    pub fn preposition(&self) -> &Or<'a, Adv<'a>, Complement<'a>> {
        &self.preposition
    }
}

impl<'a> Expr<'a> for GroupPrepositionnal<'a> {
    fn new(context: &'a Context) -> Self {
        GroupPrepositionnal {
            context: &context,
            preposition: Or::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("GroupPrepositionnal::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.preposition.parse(input.clone());

        if !res {
            log::new().syntax(&format!("GroupPrepositionnal::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false)
        }
        log::new().syntax(&format!("GroupPrepositionnal::parse({}) 2 -> ({}, {})", old_input, input, true));
        (input, true)
    }
}

impl<'a> fmt::Display for GroupPrepositionnal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"GroupPrepositionnal\": {{{}}}", self.preposition)
    }
}
