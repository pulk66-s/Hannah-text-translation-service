///
/// Test of all kind of Subject pronouns
/// Doc: doc/*Lang*/Lang/FR/Pronouns.md
/// 
#[cfg(test)]
mod subject_pronouns {
    use crate::logger::log;
use crate::{tests::datas::{
        subject_pronouns,
        assert::assert_mul_parse
    }, AST::pronouns::subject_pronouns::SubjectPronoun};

    #[test]
    fn test_je() {
        let datas: Vec<String> = subject_pronouns::get_je();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_tu() {
        let datas: Vec<String> = subject_pronouns::get_tu();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_il() {
        let datas: Vec<String> = subject_pronouns::get_il();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_elle() {
        let datas: Vec<String> = subject_pronouns::get_elle();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_nous() {
        let datas: Vec<String> = subject_pronouns::get_nous();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_vous() {
        let datas: Vec<String> = subject_pronouns::get_vous();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_ils() {
        let datas: Vec<String> = subject_pronouns::get_ils();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_elles() {
        let datas: Vec<String> = subject_pronouns::get_elles();

        assert_mul_parse::<SubjectPronoun>(datas);
    }


    #[test]
    fn test_me() {
        let datas: Vec<String> = subject_pronouns::get_me();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_te() {
        let datas: Vec<String> = subject_pronouns::get_te();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_le() {
        let datas: Vec<String> = subject_pronouns::get_le();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_la() {
        let datas: Vec<String> = subject_pronouns::get_la();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_les() {
        let datas: Vec<String> = subject_pronouns::get_les();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_lui() {
        let datas: Vec<String> = subject_pronouns::get_lui();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_leur() {
        let datas: Vec<String> = subject_pronouns::get_leur();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_en() {
        let datas: Vec<String> = subject_pronouns::get_en();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_y() {
        let datas: Vec<String> = subject_pronouns::get_y();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_moi() {
        let datas: Vec<String> = subject_pronouns::get_moi();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_toi() {
        let datas: Vec<String> = subject_pronouns::get_toi();

        assert_mul_parse::<SubjectPronoun>(datas);
    }

    #[test]
    fn test_eux() {
        let datas: Vec<String> = subject_pronouns::get_eux();

        assert_mul_parse::<SubjectPronoun>(datas);
    }
}