use crate::logger::log;
use crate::{
    AST::{expr::Expr, utils::skipable::Skipable, word::Word},
    context::Context,
};
use std::fmt;

pub struct InfinitiveVerb<'a> {
    context: &'a Context,
    verb: Word<'a>
}

impl<'a> InfinitiveVerb<'a> {
    pub fn verb(&self) -> &Word<'a> {
        &self.verb
    }
}

impl<'a> Expr<'a> for InfinitiveVerb<'a> {
    fn new(context: &'a Context) -> Self {
        Self {
            context: &context,
            verb: Word::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("InfinitiveVerb::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.verb.parse(input.clone());
        let verbs = &self.context.verbs.json.as_object().unwrap()["infinitives"];

        if !res {
            log::new().syntax(&format!("InfinitiveVerb::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }

        let exists = verbs.as_array().unwrap().iter().any(|verb| {
            if let serde_json::Value::String(ref value) = verb {
                value == self.verb.get().to_lowercase().as_str()
            } else {
                panic!("Invalid verb type: {}", verb);
            }
        });

        if exists {
            log::new().syntax(&format!("InfinitiveVerb::parse({}) 2 -> ({}, {})", old_input, input, true));
            return (input, true);
        }
        log::new().syntax(&format!("InfinitiveVerb::parse({}) 3 -> ({}, {})", old_input, old_input, false));
        (old_input, false)
    }
}

impl<'a> fmt::Display for InfinitiveVerb<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"InfinitiveVerb\": {{{}}}", self.verb)
    }
}
