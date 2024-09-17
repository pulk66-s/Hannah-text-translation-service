use crate::logger::log;
use crate::{
    context::Context,
    AST::{
        text::Text,
        expr::Expr
    },
    translation::lsf::translate::lsf_translation
};

/**
 * Translate the given text into LSF
 * You must increase the stack if you want to translate a long text
 * cmd: RUST_MIN_STACK=104857600 cargo run
 */
pub fn lsf_translate(text: String) -> String {
    let mut context = Context::new(&text);

    context.init_text_data();

    let mut text_expr = Text::new(&context);
    let (input, result) = text_expr.parse(text);

    log::new().syntax(&format!("{}", text_expr));
    if !result {
        format!("Error: {}", input)
    } else {
        lsf_translation(text_expr)
    }
}
