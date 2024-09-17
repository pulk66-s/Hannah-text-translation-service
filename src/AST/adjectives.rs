pub mod adjectival_group;

use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        word::Word, utils::skipable::Skipable
    },
};


///
/// Adjectives Parsing
/// See doc: doc/*Lang*/Lang/FR/Adjectives.md
/// 
pub struct Adjective<'a> {
    context: &'a Context,
    adj: Word<'a>
}

impl<'a> Adjective<'a> {
    pub fn get(&self) -> String {
        self.adj.get()
    }
}

impl<'a> Expr<'a> for Adjective<'a> {
    fn new(context: &'a Context) -> Self {
        Adjective {
            context: &context,
            adj: Word::new(&context),
        }
    }

    // Function with json
    // fn parse(&mut self, input: String) -> (String, bool) {
    //     log::new().syntax(&format!("Adjectives::parse({})", input));
    //     let old_input = input.clone();
    //     let (input, _) = Skipable::new(self.context).parse(input.clone());
    //     let (input, res) = self.adj.parse(input.clone());

    //     if !res {
    //         return (old_input, false);
    //     }

    //     let json_adjectives = &self.context.adjectives.json["adjectives"];
    //     let key = self.adj.get().to_lowercase();

    //     for k in json_adjectives.as_array().unwrap() {
    //         if let serde_json::Value::String(ref value) = k {
    //             if value.to_lowercase() == key {
    //                 log::new().syntax(&format!("Adjectives::parse({}) -> {}", old_input, true));
    //                 return (input, true);
    //             }
    //         } else {
    //             panic!("Invalid adjective type: {}", k);
    //         }
    //     }
    //     log::new().syntax(&format!("Adjectives::parse({}) -> {}", old_input, false));
    //     return (old_input, false);
    // }

    // Function with mysql
    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Adjectives::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.adj.parse(input.clone());

        if !res {
            log::new().syntax(&format!("Adjectives::parse({}) 0 -> {}", old_input, false));
            return (old_input, false);
        }
        return match self.context.get_adjective(&self.adj.get()) {
            Some(_) => {
                log::new().syntax(&format!("Adjectives::parse({}) 1 -> {}", old_input, true));
                (input, true)
            },
            None => {
                log::new().syntax(&format!("Adjectives::parse({}) 2 -> {}", old_input, false));
                (old_input, false)
            }
        }
    }
}

impl<'a> fmt::Display for Adjective<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"Adjectives\": {{{}}}", self.adj)
    }
}