///
/// Test of Subject expression,
/// Doc: doc/*Lang*/tests/unit/AST/fr/Subject.md
///

mod pronominal_subject;
mod nominal_subject;

///
/// Tests of all kind of nouns in Subject expression
/// 
///
/// Tests of all pronouns in Subject expression
///
#[cfg(test)]
mod pronouns {
    use crate::logger::log;
use crate::{
        tests::datas::{
            subject_pronouns, 
            assert::assert_mul_parse
        }, 
        AST::subject::Subject
    };

    ///
    /// Test of 'Je' French pronoun in Subject expression
    /// 
    #[test]
    fn test_je() {
        let pronouns: Vec<String> = subject_pronouns::get_je();

        assert_mul_parse::<Subject>(pronouns);
    }

    ///
    /// Test of 'Tu' French pronoun in Subject expression
    /// 
    #[test]
    fn test_tu() {
        let pronouns: Vec<String> = subject_pronouns::get_tu();

        assert_mul_parse::<Subject>(pronouns);
    }

    ///
    /// Test of 'Il' French pronoun in Subject expression
    /// 
    #[test]
    fn test_il() {
        let pronouns: Vec<String> = subject_pronouns::get_il();

        assert_mul_parse::<Subject>(pronouns);
    }

    ///
    /// Test of 'Elle' French pronoun in Subject expression
    /// 
    #[test]
    fn test_elle() {
        let pronouns: Vec<String> = subject_pronouns::get_elle();

        assert_mul_parse::<Subject>(pronouns);
    }

    ///
    /// Test of 'Nous' French pronoun in Subject expression
    /// 
    #[test]
    fn test_nous() {
        let pronouns: Vec<String> = subject_pronouns::get_nous();

        assert_mul_parse::<Subject>(pronouns);
    }

    ///
    /// Test of 'Vous' French pronoun in Subject expression
    /// 
    #[test]
    fn test_vous() {
        let pronouns: Vec<String> = subject_pronouns::get_vous();

        assert_mul_parse::<Subject>(pronouns);
    }

    ///
    /// Test of 'Ils' French pronoun in Subject expression
    /// 
    #[test]
    fn test_ils() {
        let pronouns: Vec<String> = subject_pronouns::get_ils();

        assert_mul_parse::<Subject>(pronouns);
    }

    ///
    /// Test of 'Elles' French pronoun in Subject expression
    /// 
    #[test]
    fn test_elles() {
        let pronouns: Vec<String> = subject_pronouns::get_elles();

        assert_mul_parse::<Subject>(pronouns);
    }
}

///
/// Tests of all kind of nouns in Subject expression
///
#[cfg(test)]
mod nouns {
    ///
    /// A Subject can be a noun with a determinant like:
    ///     - 'le chat'
    ///     - 'la souris'
    /// 
    #[cfg(test)]
    mod with_determinant {
        use crate::logger::log;
use crate::{
            tests::datas::assert::assert_parse,
            AST::subject::Subject
        };


        #[test]
        fn test_with_determinant() {
            let phrase: String = String::from("le chat");

            assert_parse::<Subject>(phrase)
        }
    }

    ///
    /// A Subject can be a noun without determinant like:
    ///     - 'chat'
    ///     - 'souris'
    #[cfg(test)]
    mod without_determinant {
        use crate::logger::log;
use crate::{tests::datas::assert::assert_parse, AST::subject::Subject};

        #[test]
        fn test_without_determinant() {
            let phrase: String = String::from("chat");

            assert_parse::<Subject>(phrase)
        }
    }
}
