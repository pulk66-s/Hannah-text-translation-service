use std::fmt;
use crate::logger::log;
use crate::AST::{
    expr::Expr,
    word::Word,
    utils::skipable::Skipable
};
use crate::context::Context;

pub struct ProperNoun<'a> {
    context: &'a Context,
    word: Word<'a>,
}

impl<'a> Expr<'a> for ProperNoun<'a> {
    fn new(context: &'a Context) -> Self {
        ProperNoun {
            context: &context,
            word: Word::new(&context),
        }
    }

    // Function using mysql
    // fn parse(&mut self, input: String) -> (String, bool) {
    //     log::new().syntax(&format!("ProperNoun::parse({})", input));
    //     let old_input = input.clone();
    //     let (input, _) = Skipable::new(self.context).parse(input.clone());
    //     let (input, success) = self.word.parse(input.clone());

    //     if !success {
    //         log::new().syntax(&format!("ProperNoun::parse({}) 2 -> ({}, {})", old_input, input, false));
    //         return (input, false)
    //     }

    //     let rows = self.context.mysql.nouns.find_with_type(&self.word.get(), "proper");

    //     if rows.len() == 0 {
    //         log::new().syntax(&format!("ProperNoun::parse({}) 3 -> ({}, {})", old_input, input, false));
    //         return (input, false)
    //     }
    //     log::new().syntax(&format!("ProperNoun::parse({}) 4 -> ({}, {})", old_input, input, false));
    //     (input, false)
    // }

    // Function using json
    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("ProperNoun::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (new_input, success) = self.word.parse(input.clone());
        let nouns = &self.context.proper_nouns.json.as_object().unwrap()["nouns"];

        if !success {
            log::new().syntax(&format!("ProperNoun::parse({}) 1 -> ({}, {})", old_input, new_input, false));
            return (new_input, false)
        }
        for noun in nouns.as_array().unwrap() {
            if let serde_json::Value::String(ref value) = noun {
                if value == self.word.get().as_str() {
                    log::new().syntax(&format!("ProperNoun::parse({}) 2 -> ({}, {})", old_input, new_input, true));
                    return (new_input, true)
                }
            } else {
                panic!("Invalid noun: {}", noun);
            }
        }
        log::new().syntax(&format!("ProperNoun::parse({}) 3 -> ({}, {})", old_input, input, false));
        (input, false)
    }
}

impl<'a> fmt::Display for ProperNoun<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"ProperNoun\": \"{{{}}}\",", self.word)
    }
}

impl<'a> ProperNoun<'a> {
    pub fn get_word(&self) -> String {
        self.word.get()
    }
}
