use crate::logger::log;
use crate::{
    translation::lsf::pronouns::translate_pronoun,
    AST::subject::Subject
};

pub fn translate_subject(subject: &Subject) -> String {
    let mut res = String::new();

    if let Some(left) = subject.datas().left() {
        res += &left.noun().get_word();
    } else if let Some(right) = subject.datas().right() {
        res += &translate_pronoun(&right.pronoun());
    }
    res
}