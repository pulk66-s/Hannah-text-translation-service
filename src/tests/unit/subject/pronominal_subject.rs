///
/// Tests of all pronouns in Subject expression
///
#[cfg(test)]
mod pronouns {
    use crate::logger::log;
use crate::{
        tests::datas::{subject_pronouns, assert::assert_mul_parse}, AST::subject::pronominalSubject::PronominalSubject,
    };

    ///
    /// Test of 'Je' French pronoun in Subject expression
    /// 
    #[test]
    fn test_je() {
        let pronouns: Vec<String> = subject_pronouns::get_je();

        assert_mul_parse::<PronominalSubject>(pronouns);
    }

    ///
    /// Test of 'Tu' French pronoun in Subject expression
    /// 
    #[test]
    fn test_tu() {
        let pronouns: Vec<String> = subject_pronouns::get_tu();

        assert_mul_parse::<PronominalSubject>(pronouns);
    }

    ///
    /// Test of 'Il' French pronoun in Subject expression
    /// 
    #[test]
    fn test_il() {
        let pronouns: Vec<String> = subject_pronouns::get_il();

        assert_mul_parse::<PronominalSubject>(pronouns);
    }

    ///
    /// Test of 'Elle' French pronoun in Subject expression
    /// 
    #[test]
    fn test_elle() {
        let pronouns: Vec<String> = subject_pronouns::get_elle();

        assert_mul_parse::<PronominalSubject>(pronouns);
    }

    ///
    /// Test of 'Nous' French pronoun in Subject expression
    /// 
    #[test]
    fn test_nous() {
        let pronouns: Vec<String> = subject_pronouns::get_nous();

        assert_mul_parse::<PronominalSubject>(pronouns);
    }

    ///
    /// Test of 'Vous' French pronoun in Subject expression
    /// 
    #[test]
    fn test_vous() {
        let pronouns: Vec<String> = subject_pronouns::get_vous();

        assert_mul_parse::<PronominalSubject>(pronouns);
    }

    ///
    /// Test of 'Ils' French pronoun in Subject expression
    /// 
    #[test]
    fn test_ils() {
        let pronouns: Vec<String> = subject_pronouns::get_ils();

        assert_mul_parse::<PronominalSubject>(pronouns);
    }

    ///
    /// Test of 'Elles' French pronoun in Subject expression
    /// 
    #[test]
    fn test_elles() {
        let pronouns: Vec<String> = subject_pronouns::get_elles();

        assert_mul_parse::<PronominalSubject>(pronouns);
    }
}