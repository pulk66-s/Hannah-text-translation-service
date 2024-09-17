#[cfg(test)]
mod basic_integration_testing {
    use crate::logger::log;
use crate::{
        context::Context,
        AST::{
            text::Text,
            expr::Expr
        },
        translation::lsf::translate::lsf_translation, tests::datas::assert::assert_empty
    };

    #[test]
    fn black_cat() {
        let t = "le chat noir".to_string();
        let context: Context = Context::new(&t);
        let mut text: Text = Text::new(&context);
        let (input, success) = text.parse(t);

        assert_eq!(success, true);
        assert_empty(input);

        let res: String = lsf_translation(text);
        assert_eq!(res, "chat noir");
    }
}