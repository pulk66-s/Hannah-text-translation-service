use std::fmt;
use crate::logger::log;
use crate::AST::{
    expr::Expr,
    complements::COD::COD,
    utils::{
        or::Or,
        and::And,
        maybe::Maybe
    },
    noun::Noun,
    subject::Subject,
    preposition::Preposition
};
use crate::logger::log;
use crate::context::Context;

pub struct CDN<'a> {
    context: &'a Context,
    noun: Or<Noun, COD>,
    complement: And<Maybe<Preposition>, Or<Subject, Noun>>
}

impl<'a> Expr<'a> for CDN<'a> {
    fn new(context: &'a Context) -> Self {
        CDN {
            context: &context,
            noun: Or::new(&context),
            complement: And::new(&context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("CDN::parse({})", input));
        let old_input = input.clone();
        let (new_input, success) = self.noun.parse(input.clone());

        if !success {
            log::new().syntax(&format!("CDN::parse({}) -> false", input));
            return (old_input, false);
        }

        let (new_input, success) = self.complement.parse(new_input);

        if !success {
            log::new().syntax(&format!("CDN::parse({}) -> false", input));
            return (old_input, false);
        }
        (new_input, true)
    }
}

impl<'a> fmt::Display for CDN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CDN {{{} {}}}", self.noun, self.complement)
    }
}