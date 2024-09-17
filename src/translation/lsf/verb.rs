use crate::logger::log;
use crate::AST::verb::group_verbal::GroupVerbal;

use super::complement::translate_cod;

pub fn translate_group_verbal(verb: &GroupVerbal) -> String {
    let left = verb.data().left();
    let right = verb.data().right();

    if right.expr().is_none() {
        return left.get_inf() + " ";
    }

    let left_verb = left.get_inf();
    let right_verb = right.expr().as_ref().unwrap().left().verb().get();
    let right_complement = translate_cod(&right.expr().as_ref().unwrap().right());
    let res = format!("{} {} {} ", left_verb, right_verb, right_complement);

    log::new().syntax(&format!("translate_group_verbal: /{}/", res));
    return res;
}
