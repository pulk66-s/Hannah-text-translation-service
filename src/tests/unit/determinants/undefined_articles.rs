#[cfg(test)]
mod undefined_articles {
    use crate::logger::log;
use crate::{
        tests::datas::{
            self,
            assert::{
                assert_mul_parse,
                assert_parse
            }
        },
        AST::determinants::undefined_articles::UndefinedArticles
    };

    #[test]
    fn test_undefined_articles() {
        let datas: Vec<String> = datas::determinants::undefined_articles();

        assert_mul_parse::<UndefinedArticles>(datas);
    }

    #[test]
    #[should_panic]
    fn test_unknown() {
        let data = "unknown";

        assert_parse::<UndefinedArticles>(data.to_string());
    }
}