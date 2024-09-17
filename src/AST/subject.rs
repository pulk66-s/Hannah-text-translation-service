pub mod nominalSubject;
pub mod pronominalSubject;
pub mod nominal_group;

use std::fmt;
use crate::AST::{
    expr::Expr,
    utils::{
        or::Or,
        skipable::Skipable,
    },
    subject::{
        nominalSubject::NominalSubject,
        pronominalSubject::PronominalSubject,
    },
};
use crate::context::Context;

/**
 * Subject Expression,
 * see doc: doc/*Lang*/Lang/FR/Subject.md to see what is a subject
 */
pub struct Subject<'a> {
    context: &'a Context,
    datas: Or<'a, NominalSubject<'a>, PronominalSubject<'a>>,
}

impl <'a> Subject<'a> {
    pub fn datas(&self) -> &Or<'a, NominalSubject<'a>, PronominalSubject<'a>> {
        &self.datas
    }
}

/**
 * A Subject is an expression that can be:
 * - Either:
 *  - A Pronoun
 *  - Maybe a Determinant and a Noun
 */
impl<'a> Expr<'a> for Subject<'a> {
    fn new(context: &'a Context) -> Self {
        Subject {
            context: &context,
            datas: Or::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, result) = self.datas.parse(input.clone());

        (input, result)
    }
}

impl<'a> fmt::Display for Subject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Subject {{{}}}", self.datas)
    }
}
