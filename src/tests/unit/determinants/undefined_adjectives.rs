#[cfg(test)]
mod undefined_adjectives {
    use crate::logger::log;
use crate::{
        tests::datas::{
            self,
            assert::{
                assert_mul_parse,
                assert_parse
            }
        },
        AST::determinants::undefined_adjectives::UndefinedAdjectives
    };

    #[test]
    fn test_undefined_adjectives() {
        let datas: Vec<String> = datas::determinants::undefined_adjectives();

        assert_mul_parse::<UndefinedAdjectives>(datas);
    }

    #[test]
    #[should_panic]
    fn test_unknown() {
        let data = "unknown";

        assert_parse::<UndefinedAdjectives>(data.to_string());
    }
}