use crate::logger::log;
use crate::{context::Context, AST::expr::Expr};
use std::fmt;

pub struct And<'a, T, U> {
    context: &'a Context,
    left: T,
    right: U,
}

impl<'a, T, U> And<'a, T, U>
where
    T: Expr<'a>,
    U: Expr<'a>
{
    pub fn left(&self) -> &T {
        &self.left
    }

    pub fn right(&self) -> &U {
        &self.right
    }
}

impl<'a, T, U> Expr<'a> for And<'a, T, U>
where
    T: Expr<'a>,
    U: Expr<'a>,
{
    fn new(context: &'a Context) -> Self {
        Self {
            context,
            left: T::new(context),
            right: U::new(context)
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("And::parse({})", input));
        let old_input = input.clone();
        let (new_input, result) = self.left.parse(input.clone());

        if !result {
            return (old_input, false);
        }

        let (new_input, result) = self.right.parse(new_input);

        if !result {
            return (old_input, false);
        }
        (new_input, true)
    }
}

impl<'a, T, U> fmt::Display for And<'a, T, U>
where
    T: fmt::Display,
    U: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"And\": {{{}, {}}}", self.left, self.right)
    }
}
