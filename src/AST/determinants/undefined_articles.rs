use std::fmt;
use crate::logger::log;
use crate::{context::Context, AST::{word::Word, expr::Expr}};

pub struct UndefinedArticles<'a> {
    context: &'a Context,
    article: Word<'a>,
}

impl<'a> Expr<'a> for UndefinedArticles<'a> {
    fn new(context: &'a Context) -> Self {
        UndefinedArticles {
            context: &context,
            article: Word::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("UndefinedArticles::parse({})", input));
        let old_input = input.clone();
        let mut spaces = crate::AST::utils::skipable::Skipable::new(self.context);
        let (input, _) = spaces.parse(input.clone());
        let (input, success) = self.article.parse(input.clone());
        let determinants = &self.context.determinants.json.as_object().unwrap()["undefined_articles"];

        if !success {
            log::new().syntax(&format!("UndefinedArticles::parse({}) 1 -> ({}, {})", old_input, input, true));
            return (input, true)
        }
        for determinant in determinants.as_array().unwrap() {
            if let serde_json::Value::String(ref value) = determinant {
                if value == self.article.get().to_lowercase().as_str() {
                    log::new().syntax(&format!("UndefinedArticles::parse({}) 2 -> ({}, {})", old_input, input, true));
                    return (input, true)
                }
            } else {
                panic!("Invalid determinant type: {}", determinant);
            }
        }
        log::new().syntax(&format!("UndefinedArticles::parse({}) 3 -> ({}, {})", old_input, old_input, false));
        (old_input, false)
    }
}

impl<'a> UndefinedArticles<'a> {
    pub fn get_word(&self) -> String {
        self.article.get()
    }
}

impl<'a> fmt::Display for UndefinedArticles<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"UndefinedArticles\": {{{}}},", self.article)
    }
}
