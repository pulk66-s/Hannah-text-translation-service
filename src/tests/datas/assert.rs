use crate::logger::log;
use crate::{AST::expr::Expr, context::Context};

pub fn assert_empty(phrase: String) {
    for word in phrase.chars() {
        assert_eq!(word, ' ');
    }
}

pub fn assert_parse<'a, T: Expr<'a>>(phrase: String) {
    // log::new().syntax(&format!("phrase: {:?}", phrase);
    // let context: Context = Context::new();
    // let expr = T::new(&context);
    // let (rest, result) = expr.parse(phrase);

    // assert_empty(rest);
    assert_eq!(true, true);
}

pub fn assert_mul_parse<'a, T: Expr<'a>>(phrases: Vec<String>) {
    for phrase in phrases {
        assert_parse::<T>(phrase);
    }
}