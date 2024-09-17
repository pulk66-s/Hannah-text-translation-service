use std::collections::HashMap;

use crate::json::Json;
use crate::{
    db::mysql::{
        tables::{
            adjectives::Adjective, adverbes::Adverbe, determinants::Determinant, nouns::Noun,
            verbs::Verb,
        },
        Connector, Mysql,
    },
    logger::log,
};

pub struct Context {
    pub mysql: Mysql,
    pub connector: Connector,
    pub verbs: Json,
    pub subject_pronouns: Json,
    pub determinants: Json,
    pub common_nouns: Json,
    pub prepositions: Json,
    pub proper_nouns: Json,
    pub demonstrative_pronouns: Json,
    pub adjectives: Json,
    pub adverbes: Json,
    pub coordinating_conjunctions: Json,
    pub relative_pronouns: Json,

    pub text: String,
    pub verbs_db: HashMap<String, Verb>,
    pub nouns_db: HashMap<String, Noun>,
    pub determinants_db: HashMap<String, Determinant>,
    pub adjective_db: HashMap<String, Adjective>,
    pub adverbes_db: HashMap<String, Adverbe>,
}

impl<'a> Context {
    pub fn new(text: &String) -> Self {
        let mut connector = Connector::new();

        connector.connect();

        let mut ctx = Context {
            mysql: Mysql::new(),
            connector: connector,
            verbs: Json::new("./config/verbs.json".to_string()),
            subject_pronouns: Json::new("./config/subject_pronouns.json".to_string()),
            determinants: Json::new("./config/determinants.json".to_string()),
            common_nouns: Json::new("./config/common_nouns.json".to_string()),
            prepositions: Json::new("./config/prepositions.json".to_string()),
            proper_nouns: Json::new("./config/proper_nouns.json".to_string()),
            demonstrative_pronouns: Json::new("./config/demonstrative_pronouns.json".to_string()),
            adjectives: Json::new("./config/adjectives.json".to_string()),
            adverbes: Json::new("./config/adverbes.json".to_string()),
            coordinating_conjunctions: Json::new(
                "./config/coordinating_conjunctions.json".to_string(),
            ),
            relative_pronouns: Json::new("./config/relative_pronouns.json".to_string()),
            text: text.to_string(),
            verbs_db: HashMap::new(),
            nouns_db: HashMap::new(),
            determinants_db: HashMap::new(),
            adjective_db: HashMap::new(),
            adverbes_db: HashMap::new(),
        };

        ctx.verbs.read();
        ctx.subject_pronouns.read();
        ctx.determinants.read();
        ctx.common_nouns.read();
        ctx.prepositions.read();
        ctx.proper_nouns.read();
        ctx.demonstrative_pronouns.read();
        ctx.adjectives.read();
        ctx.adverbes.read();
        ctx.coordinating_conjunctions.read();
        ctx.relative_pronouns.read();
        ctx
    }

    fn exist_data(&self, data: &str) -> bool {
        if self.verbs_db.contains_key(data) {
            return true;
        } else if self.nouns_db.contains_key(data) {
            return true;
        } else if self.determinants_db.contains_key(data) {
            return true;
        } else if self.adjective_db.contains_key(data) {
            return true;
        } else if self.adverbes_db.contains_key(data) {
            return true;
        }
        false
    }

    pub fn init_text_data(&mut self) {
        let words: Vec<&str> = self.text.split(|c: char| !c.is_alphanumeric()).collect();
        // filter empty words and trim / lowercase
        let words = words
            .iter()
            .filter(|word| !word.is_empty())
            .map(|word| word.trim().to_lowercase())
            .collect::<Vec<String>>();

        println!("words: {:?}", words);
        let verbs = self.mysql.verbs.find_in(&words);
        for verb in verbs {
            self.verbs_db.insert(verb.value.to_string(), verb.clone());
        }
        let adjectives = self.mysql.adjectives.find_in(&words);
        for adjective in adjectives {
            self.adjective_db
                .insert(adjective.value.to_string(), adjective.clone());
        }
        let adverbes = self.mysql.adverbes.find_in(&words);
        for adverbe in adverbes {
            self.adverbes_db
                .insert(adverbe.value.to_string(), adverbe.clone());
        }
        let nouns = self.mysql.nouns.find_in(&words);
        for noun in nouns {
            self.nouns_db.insert(noun.value.to_string(), noun.clone());
        }
        let determinants = self.mysql.determinants.find_in(&words);
        for determinant in determinants {
            self.determinants_db
                .insert(determinant.value.to_string(), determinant.clone());
        }
        // for word in words {
        //     if !self.exist_data(word) {
        //         log::new().db(&format!("word: {}", word));
        //         let v = self.mysql.verbs.find(&word.to_lowercase());
        //         if v.len() > 0 {
        //             log::new().db(&format!("v: {:?} is verb", v[0]));
        //             self.verbs_db.insert(word.to_string(), v[0].clone());
        //         } else {
        //             let n = self.mysql.nouns.find(&word.to_lowercase());
        //             if n.len() > 0 {
        //                 log::new().db(&format!("n: {:?} is noun", n[0]));
        //                 self.nouns_db.insert(word.to_string(), n[0].clone());
        //             } else {
        //                 let d = self.mysql.determinants.find(&word.to_lowercase());
        //                 if d.len() > 0 {
        //                     log::new().db(&format!("d: {:?} is determinant", d[0]));
        //                     self.determinants_db.insert(word.to_string(), d[0].clone());
        //                 } else {
        //                     let a = self.mysql.adjectives.find(&word.to_lowercase());
        //                     if a.len() > 0 {
        //                         // log::new().db(&format!("a: {:?} {:?} is adjective", word, a[0]));
        //                         println!("a: {:?} {:?} is adjective", word, a[0]);
        //                         self.adjective_db.insert(word.to_lowercase(), a[0].clone());
        //                     } else {
        //                         let ad = self.mysql.adverbes.find(&word.to_lowercase());
        //                         if ad.len() > 0 {
        //                             log::new().db(&format!("ad: {:?} is adverbe", ad[0]));
        //                             self.adverbes_db.insert(word.to_lowercase(), ad[0].clone());
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
        println!("End");
    }

    pub fn get_verb(&self, word: &str) -> Option<&Verb> {
        self.verbs_db.get(&word.to_lowercase())
    }

    pub fn get_noun(&self, word: &str) -> Option<&Noun> {
        self.nouns_db.get(&word.to_lowercase())
    }

    pub fn get_determinant(&self, word: &str) -> Option<&Determinant> {
        self.determinants_db.get(&word.to_lowercase())
    }

    pub fn get_adjective(&self, word: &str) -> Option<&Adjective> {
        self.adjective_db.get(&word.to_lowercase())
    }

    pub fn get_adverbe(&self, word: &str) -> Option<&Adverbe> {
        self.adverbes_db.get(&word.to_lowercase())
    }
}
