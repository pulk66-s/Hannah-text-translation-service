use crate::logger::log;
use crate::AST::{
    text::Text,
    expr::Expr
};
use crate::context::Context;

pub fn parse<'a>(text: String, context: &Context) -> Text {
    let mut obj = Text::new(&context);
    let (rest, _) = obj.parse(text);

    log::new().syntax(&format!("rest {}", rest));
    obj
}
