use std::fmt;

use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        word::Word,
        utils::skipable::Skipable
    }
};

pub struct CoordinatingConjunction<'a> {
    context: &'a Context,
    conj: Word<'a>
}

impl<'a> Expr<'a> for CoordinatingConjunction<'a> {
    fn new(context: &'a Context) -> Self {
        CoordinatingConjunction {
            conj: Word::new(&context),
            context: &context
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("CoordinatingConjunction::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let conjs = &self.context.coordinating_conjunctions.json.as_object().unwrap()["conjunctions"];
        let (input, res) = self.conj.parse(input.clone());

        if !res {
            log::new().syntax(&format!("CoordinatingConjunction::parse({}) 1 -> ({}, {})", old_input, input, false));
            return (input, false);
        }
        for conj in conjs.as_array().unwrap() {
            if let serde_json::Value::String(ref value) = conj {
                if value == self.conj.get().as_str() {
                    log::new().syntax(&format!("CoordinatingConjunction::parse({}) 2 -> ({}, {})", old_input, input, true));
                    return (input, true);
                }
            } else {
                panic!("Invalid conj: {}", conj);
            }
        }
        log::new().syntax(&format!("CoordinatingConjunction::parse({}) 3 -> ({}, {})", old_input, input, false));
        (input, false)
    }
}

impl<'a> fmt::Display for CoordinatingConjunction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"CoordinatingConjunction\": {{{}}}", self.conj)
    }
}
