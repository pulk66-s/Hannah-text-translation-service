use std::fmt;
use crate::logger::log;
use crate::AST::expr::Expr;
use crate::context::Context;

pub struct Or<'a, T, U> {
    context: &'a Context,
    left: Option<T>,
    right: Option<U>,
}

impl <'a, T, U> Or<'a, T, U> {
    pub fn left(&self) -> Option<&T> {
        self.left.as_ref()
    }

    pub fn right(&self) -> Option<&U> {
        self.right.as_ref()
    }
}

impl<'a, T, U> Expr<'a> for Or<'a, T, U>
where
    T: Expr<'a>,
    U: Expr<'a>,
{
    fn new(context: &'a Context) -> Or<'a, T, U> {
        Or {
            context: &context,
            left: None,
            right: None,
        }
    }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Or::parse({})", input));
        let old_input = input.clone();
        let mut left = T::new(self.context);
        let mut right = U::new(self.context);
        let (new_input, result) = left.parse(input.clone());

        if result {
            self.left = Some(left);
            log::new().syntax(&format!("Left Or::parse({}) 1 -> ({}, {})", old_input, new_input, true));
            return (new_input, true);
        }
        self.left = None;
        let (new_input, result) = right.parse(new_input);
        if result {
            self.right = Some(right);
            log::new().syntax(&format!("Right Or::parse({}) 2 -> ({}, {})", old_input, new_input, true));
            return (new_input, true);
        }
        self.right = None;
        log::new().syntax(&format!("Or::parse({}) 3 -> ({}, {})", old_input, new_input, false));
        (old_input, false)
    }
}

impl<'a, T, U> fmt::Display for Or<'a, T, U>
where
    T: fmt::Display,
    U: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = "\"Or\": {".to_string();
        if let Some(left) = &self.left {
            result.push_str(&format!(" {}", left));
        }
        if let Some(right) = &self.right {
            result.push_str(&format!(" {}", right));
        }
        write!(f, "{} }},", result)
    }
}