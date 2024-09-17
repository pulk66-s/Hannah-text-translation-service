use std::fmt;
use crate::AST::{
    expr::Expr,
    phrase::Phrase,
    utils::many::Many,
};
use crate::context::Context;

pub struct Text<'a> {
    pub phrases: Many<'a, Phrase<'a>>,
    context: &'a Context,
}

impl<'a> Expr<'a> for Text<'a> {
    fn new(context: &'a Context) -> Self {
        Text {
            phrases: Many::new(&context),
            context: &context,
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        self.phrases.parse(input.clone())
    }
}

impl<'a> fmt::Display for Text<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ \"Text\": {{{}}}}}", self.phrases)
    }
}
