use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::skipable::Skipable,
        ponctuations::endchar::EndChar,
        phrase::proposition::Proposition
    }
};

pub struct SimplePhrase<'a> {
    context: &'a Context,
    proposition: Proposition<'a>,
    end_char: EndChar<'a>,
}

impl <'a> SimplePhrase<'a> {
    pub fn proposition(&self) -> &Proposition<'a> {
        &self.proposition
    }

    pub fn end_char(&self) -> &EndChar<'a> {
        &self.end_char
    }
}

impl<'a> Expr<'a> for SimplePhrase<'a> {
    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("SimplePhrase::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.proposition.parse(input.clone());

        if !res {
            log::new().syntax(&format!("SimplePhrase::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }

        let (input, res) = self.end_char.parse(input.clone());

        if !res {
            log::new().syntax(&format!("SimplePhrase::parse({}) 2 -> ({}, {})", old_input, old_input, false));
            return (old_input, false);
        }
        log::new().syntax(&format!("SimplePhrase::parse({}) 3 -> ({}, {})", old_input, input, true));
        return (input, true);
    }

    fn new(context: &'a Context) -> Self {
        SimplePhrase {
            context: &context,
            end_char: EndChar::new(&context),
            proposition: Proposition::new(&context)
        }
    }
}

impl<'a> fmt::Display for SimplePhrase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"SimplePhrase\": {{{}, {}}},", self.proposition, self.end_char)
    }
}
