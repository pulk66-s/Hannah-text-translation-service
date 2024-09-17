use crate::logger::log;
use crate::AST::pronouns::{Pronoun, PronounType};

pub fn translate_pronoun(pronoun: &Pronoun) -> String {
    let left = pronoun.pronoun().left();
    let right = pronoun.pronoun().right();
    let mut pronoun_type = &PronounType::Unknown;

    if let Some(left) = left {
        pronoun_type = left.get();
    } else if let Some(right) = right {
        pronoun_type = right.get();
    }
    translate_pronoun_type(pronoun_type)
}

pub fn translate_pronoun_type(pronoun_type: &PronounType) -> String {
    match pronoun_type {
        PronounType::FirstPersonSingular => "moi".to_string(),
        PronounType::SecondPersonSingular => "toi".to_string(),
        PronounType::ThirdPersonSingular => "il".to_string(),
        PronounType::FirstPersonPlural => "nous".to_string(),
        PronounType::SecondPersonPlural => "vous".to_string(),
        PronounType::ThirdPersonPlural => "ils".to_string(),
        _ => panic!("Invalid pronoun type: {}", pronoun_type)
    }
}