#[cfg(test)]
mod possessive_adjectives {
    use crate::logger::log;
use crate::{
        tests::datas::{
            self,
            assert::{
                assert_mul_parse,
                assert_parse
            }
        },
        AST::determinants::possessive_adjectives::PossessiveAdjectives
    };

    #[test]
    fn test_possessive_adjectives() {
        let datas: Vec<String> = datas::determinants::possessive_adjectives();

        assert_mul_parse::<PossessiveAdjectives>(datas);
    }

    #[test]
    #[should_panic]
    fn test_unknown() {
        let data = "unknown";

        assert_parse::<PossessiveAdjectives>(data.to_string());
    }
}