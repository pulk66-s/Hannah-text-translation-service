pub mod subject_pronouns;
pub mod demonstrative_pronouns;
pub mod relative_pronouns;

use std::fmt;
use crate::logger::log;
use crate::{
    AST::{
        expr::Expr,
        utils::skipable::Skipable,
        pronouns::subject_pronouns::SubjectPronoun,
        utils::or::Or,
    },
    context::Context,
};

use self::demonstrative_pronouns::DemonstrativePronouns;

#[derive(Clone, Copy)]
pub enum PronounType {
    FirstPersonSingular,
    SecondPersonSingular,
    ThirdPersonSingular,
    FirstPersonPlural,
    SecondPersonPlural,
    ThirdPersonPlural,
    Unknown,
}

///
/// Pronoun expression
/// See doc: doc/*Lang*/Lang/FR/Pronouns.md
/// A pronouns can be a:
/// - SubjectPronoun
/// - Other unidentified pronoun
/// 
pub struct Pronoun<'a> {
    context: &'a Context,
    pronoun: Or<'a, SubjectPronoun<'a>, DemonstrativePronouns<'a>>,
}

impl <'a> Pronoun<'a> {
    pub fn pronoun(&self) -> &Or<'a, SubjectPronoun<'a>, DemonstrativePronouns<'a>> {
        &self.pronoun
    }
}

impl<'a> Expr<'a> for Pronoun<'a> {
    fn new(context: &'a Context) -> Self {
        Pronoun {
            context: &context,
            pronoun: Or::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Pronoun::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, success) = self.pronoun.parse(input.clone());
        
        if !success {
            log::new().syntax(&format!("Pronoun::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }
        log::new().syntax(&format!("Pronoun::parse({}) 2 -> ({}, {})", old_input, input, true));
        (input, true)
    }
}

impl<'a> Pronoun<'a> {
    pub fn get(&self) -> &PronounType {
        match self.pronoun.left() {
            Some(pronoun) => pronoun.get(),
            None => match self.pronoun.right() {
                Some(pronoun) => pronoun.get(),
                None => &PronounType::Unknown,
            }
        }
    }
}

impl<'a> fmt::Display for Pronoun<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pronoun {{{}}}", self.pronoun)
    }
}

impl fmt::Display for PronounType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            PronounType::FirstPersonSingular => "FirstPersonSingular",
            PronounType::SecondPersonSingular => "SecondPersonSingular",
            PronounType::ThirdPersonSingular => "ThirdPersonSingular",
            PronounType::FirstPersonPlural => "FirstPersonPlural",
            PronounType::SecondPersonPlural => "SecondPersonPlural",
            PronounType::ThirdPersonPlural => "ThirdPersonPlural",
            PronounType::Unknown => "Unknown",
        };
        write!(f, "\"Pronoun\": \"{}\",", value)
    }
}
