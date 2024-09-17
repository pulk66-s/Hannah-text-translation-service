use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        pronouns::PronounType,
        expr::Expr, utils::skipable::Skipable, word::Word
    },
};

///
/// Subject pronoun expression
/// See doc: doc/*Lang*/Lang/FR/Pronouns.md
/// A subject pronoun has a person and it's listed in the:
/// config/subjectPronouns.json
///
pub struct SubjectPronoun<'a> {
    context: &'a Context,
    person: PronounType
}

impl <'a> SubjectPronoun<'a> {
    pub fn get(&self) -> &PronounType {
        &self.person
    }
}

impl<'a> Expr<'a> for SubjectPronoun<'a> {
    fn new(context: &'a Context) -> Self {
        SubjectPronoun {
            context: &context,
            person: PronounType::Unknown
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("SubjectPronoun::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let json_pronouns = self.context.subject_pronouns.json.as_object().unwrap();
        let mut word = Word::new(self.context);
        let (new_input, success) = word.parse(input.clone());
        let key = word.get().to_lowercase();

        if !success {
            log::new().syntax(&format!("SubjectPronoun::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }
        if !json_pronouns.contains_key(key.as_str()) {
            log::new().syntax(&format!("SubjectPronoun::parse({}) 2 -> ({}, {})", old_input, old_input, false));
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
        log::new().syntax(&format!("SubjectPronoun::parse({}) 3 -> ({}, {})", old_input, new_input, true));
        (new_input, true)
    }
}

impl<'a> fmt::Display for SubjectPronoun<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pronoun {{{}}}", self.person)
    }
}
