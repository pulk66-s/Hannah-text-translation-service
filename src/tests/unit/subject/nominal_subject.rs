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
use crate::{tests::datas::assert::assert_parse, AST::subject::nominalSubject::NominalSubject};


        #[test]
        fn test_with_determinant() {
            let phrase: String = String::from("le chat");

            assert_parse::<NominalSubject>(phrase)
        }
    }

    ///
    /// A Subject can be a noun without determinant like:
    ///     - 'chat'
    ///     - 'souris'
    #[cfg(test)]
    mod without_determinant {
        use crate::logger::log;
use crate::{tests::datas::assert::assert_parse, AST::subject::nominalSubject::NominalSubject};

        #[test]
        fn test_without_determinant() {
            let phrase: String = String::from("chat");

            assert_parse::<NominalSubject>(phrase)
        }
    }
}