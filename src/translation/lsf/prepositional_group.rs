use crate::logger::log;
use crate::{
    AST::preposition::group_prepositionnal::GroupPrepositionnal,
    translation::lsf::adverbe::translate_adverbe,
};

use super::complement::translate_complement;

pub fn translate_group_prepositional(group: &GroupPrepositionnal) -> String {
    match group.preposition().left() {
        Some(adv) => translate_adverbe(&adv),
        None => match group.preposition().right() {
            Some(complement) => translate_complement(complement),
            None => panic!("Error: translate_group_prepositional")
        },
    }
}