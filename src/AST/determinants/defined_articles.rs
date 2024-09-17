use std::fmt;
use crate::logger::log;
use crate::{context::Context, AST::{word::Word, expr::Expr}};

pub struct DefinedArticles<'a> {
    context: &'a Context,
    article: Word<'a>,
}

impl<'a> Expr<'a> for DefinedArticles<'a> {
    fn new(context: &'a Context) -> Self {
        DefinedArticles {
            context: &context,
            article: Word::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("DefinedArticles::parse({})", input));
        let old_input = input.clone().clone();
        let mut spaces = crate::AST::utils::skipable::Skipable::new(self.context);
        let (input, _) = spaces.parse(input.clone());
        let (input, success) = self.article.parse(input.clone());
        let determinants = &self.context.determinants.json.as_object().unwrap()["defined_articles"];

        if !success {
            return (input, true)
        }
        for determinant in determinants.as_array().unwrap() {
            if let serde_json::Value::String(ref value) = determinant {
                if value == self.article.get().to_lowercase().as_str() {
                    log::new().syntax(&format!("DefinedArticles::parse({}) -> true", input));
                    return (input, true)
                }
            } else {
                panic!("Invalid determinant type: {}", determinant);
            }
        }
        log::new().syntax(&format!("DefinedArticles::parse({}) -> false", old_input));
        (old_input, false)
    }
}

impl<'a> DefinedArticles<'a> {
    pub fn get_word(&self) -> String {
        self.article.get()
    }
}

impl<'a> fmt::Display for DefinedArticles<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"Defined Articles\": {{{}}},", self.article)
    }
}