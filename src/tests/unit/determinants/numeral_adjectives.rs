#[cfg(test)]
mod numerals_adjectives {
    use crate::logger::log;
use crate::{
        tests::datas::{
            self,
            assert::{
                assert_mul_parse,
                assert_parse
            }
        },
        AST::determinants::numerals_adjectives::NumeralsAdjectives
    };

    #[test]
    fn test_numerals_adjectives() {
        let datas: Vec<String> = datas::determinants::numerals_adjectives();

        assert_mul_parse::<NumeralsAdjectives>(datas);
    }

    #[test]
    #[should_panic]
    fn test_unknown() {
        let data = "unknown";

        assert_parse::<NumeralsAdjectives>(data.to_string());
    }
}