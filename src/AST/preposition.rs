pub mod group_prepositionnal;

use std::fmt;
use crate::logger::log;
use crate::AST::{
    expr::Expr,
    word::Word,
    utils::skipable::Skipable,
};
use crate::context::Context;

pub struct Preposition<'a> {
    context: &'a Context,
    word: Word<'a>
}

impl<'a> Expr<'a> for Preposition<'a> {
    fn new(context: &'a Context) -> Self {
        Preposition {
            context: &context,
            word: Word::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Preposition::parse({})", input));
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (new_input, success) = self.word.parse(input.clone());
        let prepositions = &self.context.prepositions.json.as_object().unwrap()["prepositions"];

        if !success {
            log::new().syntax(&format!("Preposition::parse({}) 1 -> ({}, {})", input, input, false));
            return (new_input, false)
        }
        for preposition in prepositions.as_array().unwrap() {
            if let serde_json::Value::String(ref value) = preposition {
                if value == self.word.get().as_str() {
                    log::new().syntax(&format!("Preposition::parse({}) 2 -> ({}, {})", input, new_input, true));
                    return (new_input, true)
                }
            } else {
                panic!("Invalid preposition: {}", preposition);
            }
        }
        log::new().syntax(&format!("Preposition::parse({}) 3 -> ({}, {})", input, new_input, false));
        (input, false)
    }
}

impl<'a> fmt::Display for Preposition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Preposition {{{}}}", self.word)
    }
}
