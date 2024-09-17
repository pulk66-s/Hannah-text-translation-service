use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::{and::And, maybe::Maybe},
        verb::{
            Verb, 
            infinitive_verb::InfinitiveVerb
        }, complements::COD
    },
};
use std::fmt;

pub struct GroupVerbal<'a> {
    pub data: And<
        'a,
        Verb<'a>,
        Maybe<'a, And<'a, InfinitiveVerb<'a>, COD::COD<'a>>>
    >
}

impl <'a> GroupVerbal<'a> {
    pub fn data(&self) -> &And<'a, Verb<'a>, Maybe<'a, And<'a, InfinitiveVerb<'a>, COD::COD<'a>>>> {
        &self.data
    }
}

impl<'a> Expr<'a> for GroupVerbal<'a> {
    fn new(context: &'a Context) -> Self {
        GroupVerbal {
            data: And::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("GroupVerbal::parse({})", input));
        let old_input = input.clone();
        let (input, res) = self.data.parse(input.clone());

        if !res {
            log::new().syntax(&format!("GroupVerbal::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }
        log::new().syntax(&format!("GroupVerbal::parse({}) 2 -> ({}, {})", old_input, input, true));
        (input, true)
    }
}

impl<'a> fmt::Display for GroupVerbal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"GroupVerbal\": {{{}}},", self.data)
    }
}