use std::fmt;
use crate::logger::log;
use crate::AST::{
    expr::Expr,
    proper_noun::ProperNoun,
    commonNoun::CommonNoun,
    utils::{
        skipable::Skipable,
        or::Or
    },
};
use crate::context::Context;

pub struct Noun<'a> {
    context: &'a Context,
    noun: Or<'a, ProperNoun<'a>, CommonNoun<'a>>,
}

impl<'a> Expr<'a> for Noun<'a> {
    fn new(context: &'a Context) -> Self {
        Noun {
            context: &context,
            noun: Or::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Noun::parse({})", input));
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (new_input, success) = self.noun.parse(input.clone());

        if !success {
            return (input, false)
        }
        log::new().syntax(&format!("Noun::parse({}) -> ({}, {})", input, success, new_input));
        (new_input, true)
    }
}

impl<'a> fmt::Display for Noun<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"Noun\": {{{}}},", self.noun)
    }
}

impl<'a> Noun<'a> {
    pub fn get_word(&self) -> String {
        if let Some(left) = &self.noun.left() {
            return left.get_word();
        }
        if let Some(right) = &self.noun.right() {
            return right.get_word();
        }
        "".to_string()
    }
}
