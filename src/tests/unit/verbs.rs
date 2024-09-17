#[cfg(test)]
mod verbs {
    use crate::logger::log;
use crate::{
        tests::datas::{
            verbs::get_manger,
            assert::{assert_mul_parse, assert_parse},
        },
        AST::verb::Verb
    };

    #[test]
    fn test_verb() {
        let datas: Vec<String> = get_manger();

        assert_mul_parse::<Verb>(datas);
    }

    #[test]
    #[should_panic]
    fn test_unknown_verb() {
        let data: String = "faegeagea".to_string();

        assert_parse::<Verb>(data);
    }
}