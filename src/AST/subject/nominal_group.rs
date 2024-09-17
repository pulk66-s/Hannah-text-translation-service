use std::fmt;
use actix_web::Either;

use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::{
            skipable::Skipable,
            maybe::Maybe,
            or::Or
        },
        determinants::Determinant,
        noun::Noun,
        adjectives::adjectival_group::AdjectivalGroup,
        pronouns::{
            Pronoun,
            PronounType
        }
    }
};

pub struct NominalGroup<'a> {
    context: &'a Context,
    determinant: Maybe<'a, Determinant<'a>>,
    noun: Or<'a, Pronoun<'a>, Noun<'a>>,
    adjectives: Maybe<'a, AdjectivalGroup<'a>>,
}

impl <'a> NominalGroup<'a> {
    pub fn determinant(&self) -> &Maybe<'a, Determinant<'a>> {
        &self.determinant
    }

    pub fn noun(&self) -> &Or<'a, Pronoun<'a>, Noun<'a>> {
        &self.noun
    }

    pub fn adjectives(&self) -> &Maybe<'a, AdjectivalGroup<'a>> {
        &self.adjectives
    }
}

impl<'a> Expr<'a> for NominalGroup<'a> {
    fn new(context: &'a Context) -> Self {
        Self {
            context: &context,
            determinant: Maybe::new(&context),
            noun: Or::new(&context),
            adjectives: Maybe::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("NominalGroup::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.determinant.parse(input.clone());

        if !res {
            log::new().syntax(&format!("NominalGroup::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }

        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.noun.parse(input.clone());

        if !res {
            log::new().syntax(&format!("NominalGroup::parse({}) 2 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }

        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.adjectives.parse(input.clone());

        if !res {
            log::new().syntax(&format!("NominalGroup::parse({}) 3 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }

        log::new().syntax(&format!("NominalGroup::parse({}) 4 -> ({}, {})", old_input, input, true));
        (input, true)
    }
}

impl<'a> fmt::Display for NominalGroup<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"NominalGroup\": {{{} {} {}}},", self.determinant, self.noun, self.adjectives)
    }
}

impl<'a> NominalGroup<'a> {
    pub fn get(&self) -> Either<&PronounType, String> {
        match self.noun.left() {
            Some(pronoun) => Either::Left(pronoun.get()),
            None => match self.noun.right() {
                Some(noun) => Either::Right(noun.get_word()),
                None => Either::Right(String::from("Unknown")),
            }
        }
    }
}
