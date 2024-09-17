use crate::logger::log;
use crate::AST::{
    expr::Expr,
    utils::{
        many::Many,
        space::Space,
        notAlphanumeric::NotAlphanumeric,
        or::Or,
        and::And,
        not::Not,
    },
    ponctuations::{endchar::EndChar, phrase_splitting::PhraseSplitting},
};
use crate::context::Context;

pub struct Skipable<'a> {
    pub context: &'a Context,
    chars: Many<'a, Or<'a, Space<'a>, And<'a, Not<'a, EndChar<'a>>, And<'a, Not<'a, PhraseSplitting>, NotAlphanumeric>>>>,
}

impl<'a> Expr<'a> for Skipable<'a> {
    fn new(context: &'a Context) -> Self {
        Skipable {
            context: &context,
            chars: Many::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Skipable::parse({})", input));
        let (new_input, _) = self.chars.parse(input.clone());

        log::new().syntax(&format!("Skipable::parse({}) -> ({}, {})", input, new_input, true));
        (new_input, true)
    }
}