use std::fmt;
use crate::logger::log;
use crate::AST::expr::Expr;
use crate::context::Context;

pub struct Many<'a, T> {
    pub items: Vec<T>,
    context: &'a Context,
}

impl<'a, T> Expr<'a> for Many<'a, T>
where
    T: Expr<'a>,
{
    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Many::parse({})", input));
        let mut result = true;
        let mut input = input.clone();

        while result {
            if input.len() == 0 {
                break;
            }

            let mut item: T = T::new(self.context);
            let (new_input, new_result) = item.parse(input.clone());

            if new_result {
                self.items.push(item);
                input = new_input;
            }
            result = new_result;
        }

        let res = self.items.len() > 0;

        log::new().syntax(&format!("Many::parse({}) -> ({}, {})", input, input, res));
        (input, res)
    }

    fn new(context: &'a Context) -> Self {
        Many {
            items: Vec::new(),
            context: &context,
        }
    }
}

impl<'a, T> Many<'a, T> {
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
}

impl<'a, T> fmt::Display for Many<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        result.push_str("\"Many\": [");
        for item in &self.items {
            result.push_str(&format!(" {{{}}},", item));
        }
        write!(f, "{} ],", result)
    }
}