use std::fmt;
use crate::logger::log;
use crate::AST::expr::Expr;
use crate::context::Context;

pub struct Maybe<'a, T> {
    context: &'a Context,
    expr: Option<T>,
}

impl<'a, T> Maybe<'a, T> {
    pub fn expr(&self) -> Option<&T> {
        self.expr.as_ref()
    }
}

impl<'a, T> Expr<'a> for Maybe<'a, T>
where T: Expr<'a>
{
    fn new(context: &'a Context) -> Self {
        Maybe {
            context: &context,
            expr: None
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Maybe::parse({})", input));
        let old_input = input.clone();
        let mut expr = T::new(self.context);
        let (input, success) = expr.parse(input.clone());

        if success {
            self.expr = Some(expr);
            log::new().syntax(&format!("Maybe::parse({}) 1 -> ({}, {})", old_input, input, success));
            return (input, true);
        }
        log::new().syntax(&format!("Maybe::parse({}) 2 -> ({}, {})", old_input, old_input, success));
        (old_input, true)
    }
}

impl<'a, T> fmt::Display for Maybe<'a, T>
where T: fmt::Display + Expr<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(expr) = &self.expr {
            write!(f, "\"Maybe\": {{{}}},", expr)
        } else {
            write!(f, "\"Nothing\": {{}},")
        }
    }
}