///
/// Tests of all the Pronouns expression
/// - SubjectPronouns
/// 

pub mod subject_pronouns;
pub mod demonstrative_pronouns;

///
/// Test of all kind of Subject pronouns
/// Doc: doc/*Lang*/Lang/FR/Pronouns.md
/// 
#[cfg(test)]
mod subject {
    use crate::logger::log;
use crate::{
        tests::datas::{
            subject_pronouns,
            assert::assert_mul_parse
        }, 
        AST::pronouns::Pronoun
    };

    #[test]
    fn test_je() {
        let datas: Vec<String> = subject_pronouns::get_je();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    fn test_tu() {
        let datas: Vec<String> = subject_pronouns::get_tu();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    fn test_il() {
        let datas: Vec<String> = subject_pronouns::get_il();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    fn test_elle() {
        let datas: Vec<String> = subject_pronouns::get_elle();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    fn test_nous() {
        let datas: Vec<String> = subject_pronouns::get_nous();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    fn test_vous() {
        let datas: Vec<String> = subject_pronouns::get_vous();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    fn test_ils() {
        let datas: Vec<String> = subject_pronouns::get_ils();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    fn test_elles() {
        let datas: Vec<String> = subject_pronouns::get_elles();

        assert_mul_parse::<Pronoun>(datas);
    }
}

///
/// Test of all kind of Subject pronouns
/// Doc: doc/*Lang*/Lang/FR/Pronouns.md
/// 
#[cfg(test)]
mod demonstrative {
    use crate::logger::log;
use crate::{
        AST::pronouns::Pronoun,
        tests::datas::{
            demonstrative_pronouns,
            assert::assert_mul_parse
        },
    };

    #[test]
    pub fn test_ce() {
        let datas: Vec<String> = demonstrative_pronouns::get_ce();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_c() {
        let datas: Vec<String> = demonstrative_pronouns::get_c();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_ca() {
        let datas: Vec<String> = demonstrative_pronouns::get_ca();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_cela() {
        let datas: Vec<String> = demonstrative_pronouns::get_cela();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_ceci() {
        let datas: Vec<String> = demonstrative_pronouns::get_ceci();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_celui() {
        let datas: Vec<String> = demonstrative_pronouns::get_celui();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_celle() {
        let datas: Vec<String> = demonstrative_pronouns::get_celle();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_celuici() {
        let datas: Vec<String> = demonstrative_pronouns::get_celuici();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_cellela() {
        let datas: Vec<String> = demonstrative_pronouns::get_cellela();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_ceux() {
        let datas: Vec<String> = demonstrative_pronouns::get_ceux();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_celles() {
        let datas: Vec<String> = demonstrative_pronouns::get_celles();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_ceuxci() {
        let datas: Vec<String> = demonstrative_pronouns::get_ceuxci();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_ceuxla() {
        let datas: Vec<String> = demonstrative_pronouns::get_ceuxla();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_cellesci() {
        let datas: Vec<String> = demonstrative_pronouns::get_cellesci();

        assert_mul_parse::<Pronoun>(datas);
    }

    #[test]
    pub fn test_cellesla() {
        let datas: Vec<String> = demonstrative_pronouns::get_cellesla();

        assert_mul_parse::<Pronoun>(datas);
    }

}