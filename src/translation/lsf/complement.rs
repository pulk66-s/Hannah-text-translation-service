use crate::logger::log;
use crate::AST::complements::{Complement, COD, COI};

use super::{nominal_group::translate_nominal_group, pronouns::translate_pronoun};

pub fn translate_complement(complement: &Complement) -> String {
    let mut res = String::new();
    let left = complement.complement.left();
    let right = complement.complement.right();

    if let Some(left) = left {
        res += &translate_cod(&left);
    } else if let Some(right) = right {
        res += &translate_coi(&right);
    }
    res
}

pub fn translate_cod(cod: &COD::COD) -> String {
    let left = cod.data().left();
    let right = cod.data().right();
    let mut str = String::new();

    if let Some(left) = left {
        str += &translate_nominal_group(&left);
    } else if let Some(right) = right {
        let left = &right.left();
        let right = &right.right();

        if left.is_some() {
            str += &left.as_ref().unwrap().get_inf()
        } else if right.is_some() {
            str += &translate_pronoun(&right.as_ref().unwrap())
        }
    }
    str
}

pub fn translate_coi(coi: &COI::COI) -> String {
    let mut res = String::new();

    res += &translate_nominal_group(coi.subject());
    res
}