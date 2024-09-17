use std::fmt;
use crate::logger::log;
use crate::{context::Context, AST::{word::Word, expr::Expr}};

pub struct DemonstrativeAdjectives<'a> {
    context: &'a Context,
    article: Word<'a>,
}

impl<'a> Expr<'a> for DemonstrativeAdjectives<'a> {
    fn new(context: &'a Context) -> Self {
        DemonstrativeAdjectives {
            context: &context,
            article: Word::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("DemonstrativeAdjectives::parse({})", input));
        let old_input = input.clone();
        let mut spaces = crate::AST::utils::skipable::Skipable::new(self.context);
        let (input, _) = spaces.parse(input.clone());
        let (input, success) = self.article.parse(input.clone());
        let determinants = &self.context.determinants.json.as_object().unwrap()["demonstrative_adjectives"];

        if !success {
            log::new().syntax(&format!("DemonstrativeAdjectives::parse({}) -> ({}, {})", old_input, input, true));
            return (input, true)
        }
        for determinant in determinants.as_array().unwrap() {
            if let serde_json::Value::String(ref value) = determinant {
                if value == self.article.get().to_lowercase().as_str() {
                    return (input, true)
                }
            } else {
                panic!("Invalid determinant type: {}", determinant);
            }
        }
        log::new().syntax(&format!("DemonstrativeAdjectives::parse({}) -> ({}, {})", old_input, old_input, false));
        (old_input, false)
    }
}

impl<'a> DemonstrativeAdjectives<'a> {
    pub fn get_word(&self) -> String {
        self.article.get()
    }
}

impl<'a> fmt::Display for DemonstrativeAdjectives<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"DemonstrativeAdjectives\": {{{}}},", self.article)
    }
}