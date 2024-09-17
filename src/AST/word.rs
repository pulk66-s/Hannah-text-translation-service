use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::{
            alphanumeric::Alphanumeric,
            many::Many,
            specialChar::SpecialChar,
            and::And,
            maybe::Maybe,
        }
    }
};

pub struct Word<'a> {
    word: And<'a, Many<'a, Alphanumeric>, Maybe<'a, SpecialChar<'a>>>,
}

impl<'a> Word<'a> {
    pub fn get(&self) -> String {
        let mut result = String::new();

        for possibility in &self.word.left().items {
            match &possibility.c {
                Some(c) => result += &c.to_string(),
                None => (),
            }
        }
        if let Some(special_char) = self.word.right().expr() {
            match special_char.c() {
                Some(c) => result += &c.to_string(),
                None => (),
            }
        }
        result.to_lowercase()
    }
}

impl<'a> Expr<'a> for Word<'a> {
    fn new(context: &'a Context) -> Self {
        Word {
            word: And::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Word::parse({})", input));
        let (new_input, result) = self.word.parse(input.clone());

        log::new().syntax(&format!("Word::parse({}) -> ({}, {})", input, new_input, result));
        (new_input, result)
    }
}

impl<'a> fmt::Display for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let word = self.get();

        write!(f, "\"Word\": \"{}\"", word)
    }
}