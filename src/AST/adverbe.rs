use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        word::Word,
        utils::skipable::Skipable
    }
};
use std::fmt;

pub struct Adv<'a> {
    context: &'a Context,
    adv: Word<'a>
}

impl<'a> Adv<'a> {
    pub fn get(&self) -> String {
        self.adv.get()
    }
}

impl<'a> Expr<'a> for Adv<'a> {
    fn new(context: &'a Context) -> Self {
        Adv {
            context: &context,
            adv: Word::new(&context)
        }
    }

    // Function with json
    // fn parse(&mut self, input: String) -> (String, bool) {
    //     log::new().syntax(&format!("Adv::parse({})", input));
    //     let old_input = input.clone();
    //     let (input, _) = Skipable::new(self.context).parse(input.clone());
    //     let adverbes = &self.context.adverbes.json.as_object().unwrap()["adverbes"];
    //     let (input, res) = self.adv.parse(input.clone());

    //     if !res {
    //         log::new().syntax(&format!("Adv::parse({}) 1 -> ({}, {})", old_input, old_input, false));
    //         return (old_input, false)
    //     }
        
    //     let content = adverbes.as_array().unwrap();

    //     for adverbe in content {
    //         if let serde_json::Value::String(ref value) = adverbe {
    //             if value == self.adv.get().as_str() {
    //                 log::new().syntax(&format!("Adv::parse({}) 2 -> ({}, {})", old_input, input, true));
    //                 return (input, true)
    //             }
    //         } else {
    //             panic!("Invalid adverbe: {}", adverbe);
    //         }
    //     }
    //     log::new().syntax(&format!("Adv::parse({}) 3 -> ({}, {})", old_input, old_input, false));
    //     (old_input, false)
    // }

    // Function with mysql
    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Adv::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.adv.parse(input.clone());

        if !res {
            log::new().syntax(&format!("Adv::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false)
        }
        return match self.context.get_adverbe(&self.adv.get()) {
            Some(_) => {
                log::new().syntax(&format!("Adv::parse({}) 2 -> ({}, {})", old_input, input, true));
                (input, true)
            },
            None => {
                log::new().syntax(&format!("Adv::parse({}) 3 -> ({}, {})", old_input, old_input, false));
                (old_input, false)
            }
        }
    }
}

impl<'a> fmt::Display for Adv<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"Adv\": {{{}}}", self.adv)
    }
}