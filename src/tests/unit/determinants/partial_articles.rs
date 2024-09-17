#[cfg(test)]
mod partial_articles {
    use crate::logger::log;
use crate::{
        tests::datas::{
            self,
            assert::{
                assert_mul_parse,
                assert_parse
            }
        },
        AST::determinants::partial_articles::PartialArticles
    };

    #[test]
    fn test_partial_articles() {
        let datas: Vec<String> = datas::determinants::partial_articles();

        assert_mul_parse::<PartialArticles>(datas);
    }

    #[test]
    #[should_panic]
    fn test_unknown() {
        let data = "unknown";

        assert_parse::<PartialArticles>(data.to_string());
    }
}