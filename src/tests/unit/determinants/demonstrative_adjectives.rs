#[cfg(test)]
mod demonstrative_adjectives {
    use crate::logger::log;
use crate::{
        tests::datas::{
            self,
            assert::{
                assert_mul_parse,
                assert_parse
            }
        },
        AST::determinants::demonstrative_adjectives::DemonstrativeAdjectives
    };

    #[test]
    fn test_demonstrative_adjectives() {
        let datas: Vec<String> = datas::determinants::demonstrative_adjectives();

        assert_mul_parse::<DemonstrativeAdjectives>(datas);
    }

    #[test]
    #[should_panic]
    fn test_unknown() {
        let data = "unknown";

        assert_parse::<DemonstrativeAdjectives>(data.to_string());
    }
}