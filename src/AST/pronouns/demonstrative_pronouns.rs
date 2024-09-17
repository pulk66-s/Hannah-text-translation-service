use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::skipable::Skipable,
        word::Word,
        pronouns::PronounType
    }
};


///
/// Demonstrative pronoun expression
/// See doc: doc/*Lang*/Lang/FR/Pronouns.md
/// A demonstrative pronoun has a person and it's listed in the:
/// There are listed in the config/demonstrative_pronouns.json
/// 
pub struct DemonstrativePronouns<'a> {
    context: &'a Context,
    person: PronounType
}

impl <'a> DemonstrativePronouns<'a> {
    pub fn get(&self) -> &PronounType {
        &self.person
    }
}

impl<'a> Expr<'a> for DemonstrativePronouns<'a> {
    fn new(context: &'a Context) -> Self {
        DemonstrativePronouns {
            context: &context,
            person: PronounType::Unknown
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let json_pronouns = self.context.demonstrative_pronouns.json.as_object().unwrap();
        let mut word = Word::new(self.context);
        let (new_input, success) = word.parse(input.clone());
        let key = word.get().to_lowercase();

        if !success {
            return (old_input, false);
        }
        if !json_pronouns.contains_key(key.as_str()) {
            return (old_input, false);
        }

        let value = match json_pronouns[key.as_str()].as_u64() {
            Some(value) => value,
            None => panic!("Invalid pronoun type: {}", key)
        };

        self.person = match value {
            1 => PronounType::FirstPersonSingular,
            2 => PronounType::SecondPersonSingular,
            3 => PronounType::ThirdPersonSingular,
            4 => PronounType::FirstPersonPlural,
            5 => PronounType::SecondPersonPlural,
            6 => PronounType::ThirdPersonPlural,
            e => panic!("Invalid pronoun type: {}", e),
        };

        (new_input, true)
    }
}

impl<'a> fmt::Display for DemonstrativePronouns<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.person)
    }
}