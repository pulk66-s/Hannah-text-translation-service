pub mod group_verbal;
pub mod infinitive_verb;

use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        time::Time,
        utils::skipable::Skipable,
        word::Word,
    }
};
use std::panic::UnwindSafe;
use crate::db::mysql::tables::verbs::Verb as dbVerb;

///
/// Verb is a struct that represents a verb in a sentence.
/// It's configuration file is located in `config/verbs.json`.
/// It's represented by a `String` and a `Time`.
/// 

pub struct Verb<'a> {
    pub time: Time,
    context: &'a Context,
    word: String,
}

impl<'a> UnwindSafe for Verb<'a> {}

impl<'a> Expr<'a> for Verb<'a> {
    // Function using mysql
    // fn parse(&mut self, input: String) -> (String, bool) {
    //     log::new().syntax(&format!("Verb::parse({})", input));
    //     let old_input = input.clone();
    //     let (input, _) = Skipable::new(self.context).parse(input.clone());
    //     let mut word = Word::new(self.context);
    //     let (input, _) = word.parse(input.clone());
    //     let key = word.get();
    //     let rows = self.context.mysql.verbs.find(&key);

    //     if rows.len() == 0 {
    //         log::new().syntax(&format!("Verb::parse({}) 2 -> ({}, {})", old_input, input, false));
    //         return (input, false)
    //     }

    //     let verb: &dbVerb = &rows[0];

    //     self.word = verb.value.clone();
    //     self.time = match verb.tense.as_str() {
    //         "present" => Time::Present,
    //         "past" => Time::Past,
    //         "future" => Time::Future,
    //         "infinitive" => Time::Infinitive,
    //         _ => Time::Unknown,
    //     };

    //     log::new().syntax(&format!("Verb::parse({}) 3 -> ({}, {})", old_input, input, true));
    //     (input, true)
    // }

    // Function using json
    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Verb::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let mut word = Word::new(self.context);
        let (input, _) = word.parse(input.clone());
        let key = word.get();
        let verb_data = match self.context.get_verb(&key) {
            Some(verb) => verb,
            None => {
                log::new().syntax(&format!("Verb::parse({}) 2 -> ({}, {})", old_input, input, false));
                return (input, false)
            }
        };

        self.time = match verb_data.tense.as_str() {
            "present" => Time::Present,
            "past" => Time::Past,
            "future" => Time::Future,
            "infinitive" => Time::Infinitive,
            _ => Time::Unknown,
        };
        self.word = verb_data.value.clone();

        // let verbs = &self.context.verbs.json["conjugations"].as_object().unwrap();

        // if !verbs.contains_key(key.as_str()) {
        //     log::new().syntax(&format!("Verb::parse({}) 1 -> ({}, {})", old_input, old_input, false));
        //     return (old_input, false);
        // }

        // let content = verbs[key.as_str()].as_object().unwrap();
        // let verb = match content["verb"] {
        //     serde_json::Value::String(ref value) => value,
        //     _ => panic!("Invalid json verb value: {}", key)
        // };
        // let tense = match content["tense"] {
        //     serde_json::Value::String(ref value) => value,
        //     _ => panic!("Invalid json tense value: {}", key)
        // };

        // self.word = verb.to_string();
        // self.time = match tense.as_str() {
        //     "present" => Time::Present,
        //     "past" => Time::Past,
        //     "future" => Time::Future,
        //     _ => panic!("Invalid verb: {}", key)
        // };
        // log::new().syntax(&format!("Verb::parse({}) 2 -> ({}, {})", old_input, input, true));
        (input, true)
    }

    fn new(context: &'a Context) -> Self {
        Verb {
            time: Time::Unknown,
            context: &context,
            word: String::new(),
        }
    }
}

impl<'a> fmt::Display for Verb<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"Verb\": {{\"time\": \"{}\", \"verb\": \"{}\"}},", self.time, self.word)
    }
}

impl<'a> Verb<'a> {
    pub fn get_inf(&self) -> String {
        self.word.clone()
    }
}