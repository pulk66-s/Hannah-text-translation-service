use crate::logger::log;
use crate::{
    AST::expr::Expr,
    context::Context,
};

pub struct Not<'a, T: Expr<'a>> {
    pub context: &'a Context,
    expr: T,
}

impl<'a, T> Expr<'a> for Not<'a, T> 
where
    T: Expr<'a>
{
    fn new(context: &'a Context) -> Self {
        Not {
            context: &context,
            expr: <T as Expr>::new(&context),
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Not::parse({})", input));
        let old_input = input.clone();
        let (new_input, success) = self.expr.parse(input.clone());

        if !success {
            log::new().syntax(&format!("Space::parse({}) -> 1 ({}, {})", input, old_input, true));
            return (old_input, true);
        } else {
            log::new().syntax(&format!("Space::parse({}) -> 2 ({}, {})", input, old_input, false));
            return (old_input, false);
        }
    }
}
