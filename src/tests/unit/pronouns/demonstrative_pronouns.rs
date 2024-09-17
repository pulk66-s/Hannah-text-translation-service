///
/// Tests of all the DemonstrativePronouns expression
/// - DemonstrativePronouns
/// 
#[cfg(test)]
mod demonstrative_pronouns {
    use crate::logger::log;
use crate::{
        AST::pronouns::demonstrative_pronouns::DemonstrativePronouns,
        tests::datas::{
            demonstrative_pronouns,
            assert::assert_mul_parse
        },
    };

    #[test]
    pub fn test_ce() {
        let datas: Vec<String> = demonstrative_pronouns::get_ce();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_c() {
        let datas: Vec<String> = demonstrative_pronouns::get_c();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_ca() {
        let datas: Vec<String> = demonstrative_pronouns::get_ca();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_cela() {
        let datas: Vec<String> = demonstrative_pronouns::get_cela();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_ceci() {
        let datas: Vec<String> = demonstrative_pronouns::get_ceci();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_celui() {
        let datas: Vec<String> = demonstrative_pronouns::get_celui();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_celle() {
        let datas: Vec<String> = demonstrative_pronouns::get_celle();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_celuici() {
        let datas: Vec<String> = demonstrative_pronouns::get_celuici();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_cellela() {
        let datas: Vec<String> = demonstrative_pronouns::get_cellela();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_ceux() {
        let datas: Vec<String> = demonstrative_pronouns::get_ceux();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_celles() {
        let datas: Vec<String> = demonstrative_pronouns::get_celles();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_ceuxci() {
        let datas: Vec<String> = demonstrative_pronouns::get_ceuxci();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_ceuxla() {
        let datas: Vec<String> = demonstrative_pronouns::get_ceuxla();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_cellesci() {
        let datas: Vec<String> = demonstrative_pronouns::get_cellesci();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

    #[test]
    pub fn test_cellesla() {
        let datas: Vec<String> = demonstrative_pronouns::get_cellesla();

        assert_mul_parse::<DemonstrativePronouns>(datas);
    }

}