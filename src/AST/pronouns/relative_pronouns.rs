use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        word::Word, utils::skipable::Skipable
    }
};

pub struct RelativePronouns<'a> {
    context: &'a Context,
    pronoun: Word<'a>
}

impl<'a> Expr<'a> for RelativePronouns<'a> {
    fn new(context: &'a Context) -> Self {
        RelativePronouns {
            context: &context,
            pronoun: Word::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("RelativePronouns::parse({})", input));
        let old_input = input.clone().clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let pronouns = self.context.relative_pronouns.json["pronouns"].as_array().unwrap();
        let (input, res) = self.pronoun.parse(input.clone());

        if !res {
            log::new().syntax(&format!("RelativePronouns::parse({}) 1 -> ({}, {})", old_input, input, false));
            return (old_input, false);
        }
        for pronoun in pronouns {
            if let serde_json::Value::String(ref value) = pronoun {
                if value.to_lowercase() == self.pronoun.get().to_lowercase() {
                    log::new().syntax(&format!("RelativePronouns::parse({}) 2 -> ({}, {})", old_input, input, true));
                    return (input, true);
                }
            } else {
                panic!("Invalid pronoun type: {}", pronoun);
            }
        }
        log::new().syntax(&format!("RelativePronouns::parse({}) 2 -> ({}, {})", old_input, input, true));
        (input, true)
    }
}

impl<'a> fmt::Display for RelativePronouns<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"RelativePronouns\": {{{}}}", self.pronoun)
    }
}