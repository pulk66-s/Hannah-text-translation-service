use std::fmt;
use crate::logger::log;
use crate::{context::Context, AST::{word::Word, expr::Expr}};

pub struct PossessiveAdjectives<'a> {
    context: &'a Context,
    article: Word<'a>,
}

impl<'a> Expr<'a> for PossessiveAdjectives<'a> {
    fn new(context: &'a Context) -> Self {
        PossessiveAdjectives {
            context: &context,
            article: Word::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("PossessiveAdjectives::parse({})", input));
        let old_input = input.clone();
        let mut spaces = crate::AST::utils::skipable::Skipable::new(self.context);
        let (input, _) = spaces.parse(input.clone());
        let (input, success) = self.article.parse(input.clone());
        let determinants = &self.context.determinants.json.as_object().unwrap()["possessive_adjectives"];

        if !success {
            log::new().syntax(&format!("PossessiveAdjectives::parse({}) 1 -> ({}, {})", old_input, input, true));
            return (input, true)
        }
        for determinant in determinants.as_array().unwrap() {
            if let serde_json::Value::String(ref value) = determinant {
                if value == self.article.get().to_lowercase().as_str() {
                    log::new().syntax(&format!("PossessiveAdjectives::parse({}) 2 -> ({}, {})", old_input, input, true));
                    return (input, true)
                }
            } else {
                panic!("Invalid determinant type: {}", determinant);
            }
        }
        log::new().syntax(&format!("PossessiveAdjectives::parse({}) 3 -> ({}, {})", old_input, old_input, false));
        (old_input, false)
    }
}

impl<'a> PossessiveAdjectives<'a> {
    pub fn get_word(&self) -> String {
        self.article.get()
    }
}

impl<'a> fmt::Display for PossessiveAdjectives<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"PossessiveAdjectives\": {{{}}},", self.article)
    }
}
