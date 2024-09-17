use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::{
            maybe::Maybe,
            skipable::Skipable
        },
        determinants::Determinant,
        noun::Noun
    }
};
use std::fmt;

pub struct NominalSubject<'a> {
    context: &'a Context,
    determinant: Maybe<'a, Determinant<'a>>,
    noun: Noun<'a>
}

impl <'a> NominalSubject<'a> {
    pub fn determinant(&self) -> &Maybe<'a, Determinant<'a>> {
        &self.determinant
    }

    pub fn noun(&self) -> &Noun<'a> {
        &self.noun
    }
}

impl<'a> Expr<'a> for NominalSubject<'a> {
    fn new(context: &'a Context) -> Self {
        Self {
            context: &context,
            determinant: Maybe::new(&context),
            noun: Noun::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, _) = self.determinant.parse(input.clone());
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.noun.parse(input.clone());

        (input, res)
    }
}

impl<'a> fmt::Display for NominalSubject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NominalSubject {{{} {}}}", self.determinant, self.noun)
    }
}