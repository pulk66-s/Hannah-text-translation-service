use crate::logger::log;
use crate::context::Context;

pub trait Expr<'a> {
    fn parse(&mut self, input: String) -> (String, bool);
    fn new(context: &'a Context) -> Self;
}
