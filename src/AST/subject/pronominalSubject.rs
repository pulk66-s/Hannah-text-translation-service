use std::fmt;
use crate::logger::log;
use crate::{
    AST::{
        expr::Expr,
        pronouns::Pronoun,
        utils::skipable::Skipable
    },
    context::Context
};

pub struct PronominalSubject<'a> {
    context: &'a Context,
    pronoun: Pronoun<'a>
}

impl <'a> PronominalSubject<'a> {
    pub fn pronoun(&self) -> &Pronoun<'a> {
        &self.pronoun
    }
}

impl<'a> Expr<'a> for PronominalSubject<'a> {
    fn new(context: &'a Context) -> Self {
        Self {
            context: &context,
            pronoun: Pronoun::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.pronoun.parse(input.clone());

        (input, res)
    }
}

impl<'a> fmt::Display for PronominalSubject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PronominalSubject {{{}}}", self.pronoun)
    }
}
