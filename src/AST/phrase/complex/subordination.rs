use std::fmt;
use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        expr::Expr,
        utils::{
            skipable::Skipable,
            and::And
        },
        pronouns::relative_pronouns::RelativePronouns,
        phrase::proposition::Proposition,
    }
};

pub struct Subordination<'a> {
    context: &'a Context,
    proposition: And<'a, RelativePronouns<'a>, Proposition<'a>>
}

impl <'a> Subordination<'a> {
    pub fn proposition(&self) -> &And<'a, RelativePronouns<'a>, Proposition<'a>> {
        &self.proposition
    }
}

impl<'a> Expr<'a> for Subordination<'a> {
    fn new(context: &'a Context) -> Self {
        Subordination {
            context: &context,
            proposition: And::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Subordination::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, res) = self.proposition.parse(input.clone());

        if !res {
            log::new().syntax(&format!("Subordination::parse({}) 1 -> ({}, {})", old_input, input, false));
            return (input, false);
        }
        log::new().syntax(&format!("Subordination::parse({}) 2 -> ({}, {})", old_input, input, true));
        return (input, true);
    }
}

impl<'a> fmt::Display for Subordination<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"Subordination\": {{{}}}", self.proposition)
    }
}