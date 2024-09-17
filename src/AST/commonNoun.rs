use std::fmt;
use crate::logger::log;
use crate::AST::{
    expr::Expr,
    word::Word,
    utils::skipable::Skipable
};
use crate::context::Context;

pub struct CommonNoun<'a> {
    context: &'a Context,
    word: Word<'a>,
}

impl<'a> Expr<'a> for CommonNoun<'a> {
    fn new(context: &'a Context) -> Self {
        CommonNoun {
            context: &context,
            word: Word::new(&context),
        }
    }

    // Function using mysql
    // fn parse(&mut self, input: String) -> (String, bool) {
    //     log::new().syntax(&format!("CommonWord::parse({})", input));
    //     let old_input = input.clone();
    //     let (input, _) = Skipable::new(self.context).parse(input.clone());
    //     let (input, success) = self.word.parse(input.clone());

    //     if !success {
    //         log::new().syntax(&format!("CommonWord::parse({}) 2 -> ({}, {})", old_input, input, false));
    //         return (input, false)
    //     }

    //     let rows = self.context.mysql.nouns.find_with_type(&self.word.get(), "common");

    //     if rows.len() == 0 {
    //         log::new().syntax(&format!("CommonWord::parse({}) 3 -> ({}, {})", old_input, input, false));
    //         return (input, false)
    //     }
    //     log::new().syntax(&format!("CommonNoun::parse({}) 4 -> ({}, {})", old_input, input, false));
    //     (input, false)
    // }

    // Function using json
    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("CommonNoun::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (new_input, success) = self.word.parse(input.clone());
        let key = self.word.get();

        if !success {
            log::new().syntax(&format!("CommonNoun::parse({}) 1 -> ({}, {})", old_input, new_input, false));
            return (new_input, false)
        }
        return match self.context.get_noun(&key) {
            Some(n) => match n._type.as_str() {
                "common" => {
                    log::new().syntax(&format!("CommonNoun::parse({}) 2 -> ({}, {})", old_input, new_input, true));
                    (new_input, true)
                
                },
                _ => {
                    log::new().syntax(&format!("CommonNoun::parse({}) 3 -> ({}, {})", old_input, new_input, false));
                    (new_input, false)
                }
            },
            None => {
                log::new().syntax(&format!("CommonNoun::parse({}) 4 -> ({}, {})", old_input, new_input, false));
                (new_input, false)
            }
        };
    }
}

impl<'a> fmt::Display for CommonNoun<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"CommonNoun\": {{{}}},", self.word)
    }
}

impl<'a> CommonNoun<'a> {
    pub fn get_word(&self) -> String {
        self.word.get()
    }
}