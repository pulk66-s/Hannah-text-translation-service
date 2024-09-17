#[cfg(test)]
mod defined_articles {
    use crate::logger::log;
use crate::{tests::datas::{self, assert::assert_mul_parse}, AST::determinants::defined_articles::DefinedArticles};

    #[test]
    fn defined_articles() {
        let datas: Vec<String> = datas::determinants::defined_articles();

        assert_mul_parse::<DefinedArticles>(datas);
    }

    #[test]
    #[should_panic]
    fn unknown() {
        let data = "unknown";

        assert_mul_parse::<DefinedArticles>(vec![data.to_string()]);
    }
}