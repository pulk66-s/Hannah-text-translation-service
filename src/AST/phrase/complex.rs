pub mod juxtaposition;
pub mod coordination;
pub mod subordination;

use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::{skipable::Skipable, many::Many, or::Or},
        phrase::complex::{
            juxtaposition::Juxtaposition,
            coordination::Coordination,
            subordination::Subordination
        },
        ponctuations::endchar::EndChar
    }
};

pub struct ComplexPhrase<'a> {
    context: &'a Context,
    data: Many<'a, Or<'a, Juxtaposition<'a>, Or<'a, Coordination<'a>, Subordination<'a>>>>,
    end_char: EndChar<'a>
}

impl <'a> ComplexPhrase<'a> {
    pub fn data(&self) -> &Many<'a, Or<'a, Juxtaposition<'a>, Or<'a, Coordination<'a>, Subordination<'a>>>> {
        &self.data
    }

    pub fn end_char(&self) -> &EndChar<'a> {
        &self.end_char
    }
}

impl<'a> Expr<'a> for ComplexPhrase<'a> {
    fn new(context: &'a Context) -> Self {
        ComplexPhrase {
            context: &context,
            data: Many::new(&context),
            end_char: EndChar::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("ComplexPhrase::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.data.parse(input.clone());

        if !res {
            log::new().syntax(&format!("ComplexPhrase::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }

        let (input, res) = self.end_char.parse(input.clone());

        if !res {
            log::new().syntax(&format!("ComplexPhrase::parse({}) 2 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }
        log::new().syntax(&format!("ComplexPhrase::parse({}) 3 -> ({}, {})", old_input, input, true));
        return (input, true);
    }
}

impl<'a> fmt::Display for ComplexPhrase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"ComplexPhrase\": {{{}}}", self.data)
    }
}